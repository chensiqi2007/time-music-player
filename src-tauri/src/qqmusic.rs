use crate::db::TrackRecord;
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, REFERER, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

const USER_AGENT_STR: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const REFERER_STR: &str = "https://y.qq.com/";

pub struct QQMusicClient {
    client: Client,
    user_cookie: Mutex<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QQMusicLyricResult {
    pub lrc: String,
    pub trans: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QQQrCodeResult {
    pub qrsig: String,
    pub qr_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QQQrCheckResult {
    pub code: i32,
    pub message: String,
    pub uin: Option<String>,
    pub nickname: Option<String>,
    pub cookie: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QQUserProfile {
    pub uin: String,
    pub nickname: String,
    pub avatar_url: String,
    pub vip_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QQPlaylistSummary {
    pub id: String,
    pub name: String,
    pub cover: Option<String>,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QQPlaylistDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover: Option<String>,
    pub tracks: Vec<TrackRecord>,
}

impl QQMusicClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .cookie_store(true)
            .connect_timeout(std::time::Duration::from_secs(6))
            .timeout(std::time::Duration::from_secs(12))
            .pool_idle_timeout(std::time::Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .tcp_keepalive(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            user_cookie: Mutex::new(None),
        }
    }

    pub fn set_cookie(&self, cookie: String) {
        if let Ok(mut c) = self.user_cookie.lock() {
            *c = Some(cookie);
        }
    }

    pub fn get_cookie(&self) -> Option<String> {
        self.user_cookie.lock().ok().and_then(|c| c.clone())
    }

    pub fn logout(&self) {
        if let Ok(mut c) = self.user_cookie.lock() {
            *c = None;
        }
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STR));
        headers.insert(REFERER, HeaderValue::from_static(REFERER_STR));

        if let Some(cookie) = self.get_cookie() {
            if let Ok(val) = HeaderValue::from_str(&cookie) {
                headers.insert(COOKIE, val);
            }
        }

        headers
    }

    // 1. Get QR Code for login
    pub async fn get_qr_key(&self) -> Result<QQQrCodeResult, String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let url = format!(
            "https://ssl.ptlogin2.qq.com/ptqrshow?appid=716027609&e=2&l=M&s=3&d=72&v=4&t=0.{}&daid=383&pt_3rd_aid=100497308",
            now
        );

        let res = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let mut qrsig = String::new();
        for val in res.headers().get_all(reqwest::header::SET_COOKIE) {
            if let Ok(s) = val.to_str() {
                if let Some(pos) = s.find("qrsig=") {
                    let sub = &s[pos + 6..];
                    let end = sub.find(';').unwrap_or(sub.len());
                    qrsig = sub[..end].to_string();
                    break;
                }
            }
        }

        let bytes = res.bytes().await.map_err(|e| e.to_string())?;
        let qr_image = format!(
            "data:image/png;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(&bytes)
        );

        Ok(QQQrCodeResult { qrsig, qr_image })
    }

    // 2. Check QR Login Status
    pub async fn check_qr(&self, qrsig: &str) -> Result<QQQrCheckResult, String> {
        let mut e: i64 = 0;
        for b in qrsig.bytes() {
            e += (e << 5) + (b as i64);
        }
        let ptqrtoken = e & 0x7fffffff;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        let url = format!(
            "https://ssl.ptlogin2.qq.com/ptqrlogin?u1=https%3A%2F%2Fgraph.qq.com%2Foauth2.0%2Flogin_jump&ptqrtoken={}&ptredirect=0&h=1&t=1&g=1&from_ui=1&ptlang=2052&action=0-0-{}&js_ver=21010611&js_type=1&login_sig=&pt_uistyle=40&aid=716027609&daid=383&pt_3rd_aid=100497308&",
            ptqrtoken, now
        );

        let res = self
            .client
            .get(&url)
            .header(reqwest::header::COOKIE, format!("qrsig={}", qrsig))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        // Extract cookies
        let mut cookies = Vec::new();
        for val in res.headers().get_all(reqwest::header::SET_COOKIE) {
            if let Ok(s) = val.to_str() {
                let part = s.split(';').next().unwrap_or("").trim();
                if !part.is_empty() {
                    cookies.push(part.to_string());
                }
            }
        }

        let body = res.text().await.map_err(|e| e.to_string())?;

        let parts: Vec<&str> = body.split('\'').collect();
        if parts.len() >= 2 {
            let code_str = parts[1];
            let code: i32 = code_str.parse().unwrap_or(-1);

            let msg = if let Some(m) = parts.get(9) {
                m.to_string()
            } else if code == 66 {
                "二维码未失效，等待手机扫描".to_string()
            } else if code == 67 {
                "已扫描，请在手机上确认授权登录".to_string()
            } else if code == 65 {
                "二维码已失效，请刷新重试".to_string()
            } else if code == 0 {
                "登录成功".to_string()
            } else {
                "未知状态".to_string()
            };

            let nickname = parts.get(11).map(|s| s.to_string());

            let mut final_uin: Option<String> = None;
            if code == 0 {
                for c in &cookies {
                    if c.starts_with("uin=") {
                        let raw = c.replace("uin=", "").replace("o", "");
                        final_uin = Some(raw);
                        break;
                    }
                }
                if final_uin.is_none() {
                    if let Some(target_url) = parts.get(5) {
                        if let Some(pos) = target_url.find("uin=") {
                            let sub = &target_url[pos + 4..];
                            let end = sub.find('&').unwrap_or(sub.len());
                            final_uin = Some(sub[..end].replace("o", ""));
                        }
                    }
                }

                let cookie_str = cookies.join("; ");
                self.set_cookie(cookie_str.clone());

                return Ok(QQQrCheckResult {
                    code: 0,
                    message: "登录成功".to_string(),
                    uin: final_uin,
                    nickname,
                    cookie: Some(cookie_str),
                });
            }

            return Ok(QQQrCheckResult {
                code,
                message: msg,
                uin: None,
                nickname: None,
                cookie: None,
            });
        }

        Err("无法解析 QQ 音乐登录响应".to_string())
    }

    // 3. User Profile
    pub async fn get_user_profile(&self, uin: &str) -> Result<QQUserProfile, String> {
        let clean_uin = uin.trim_start_matches('o');
        let avatar_url = format!("https://q.qlogo.cn/headimg_dl?dst_uin={}&spec=100", clean_uin);
        Ok(QQUserProfile {
            uin: clean_uin.to_string(),
            nickname: format!("QQ音乐用户_{}", &clean_uin[clean_uin.len().saturating_sub(4)..]),
            avatar_url,
            vip_type: 1,
        })
    }

    // 4. User Playlists
    pub async fn get_user_playlists(&self, uin: &str) -> Result<Vec<QQPlaylistSummary>, String> {
        let clean_uin = uin.trim_start_matches('o');
        let uin_num: i64 = clean_uin.parse().unwrap_or(0);

        let url = "https://u.y.qq.com/cgi-bin/musicu.fcg";
        let body = serde_json::json!({
            "comm": {
                "uin": uin_num,
                "format": "json",
                "ct": 20,
                "cv": 0
            },
            "req_0": {
                "module": "music.musicList.MusicListBase",
                "method": "GetPlaylistByUin",
                "param": {
                    "uin": uin_num,
                    "page": 0,
                    "size": 50
                }
            }
        });

        let res = self
            .client
            .post(url)
            .headers(self.build_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        let mut list = Vec::new();

        if let Some(playlists) = json
            .get("req_0")
            .and_then(|r| r.get("data"))
            .and_then(|d| d.get("vecPlaylist"))
            .and_then(|v| v.as_array())
        {
            for p in playlists {
                let id = p.get("tid").or_else(|| p.get("dissid")).and_then(|v| {
                    if let Some(n) = v.as_i64() {
                        Some(n.to_string())
                    } else {
                        v.as_str().map(|s| s.to_string())
                    }
                }).unwrap_or_default();

                if id.is_empty() {
                    continue;
                }

                let name = p.get("title").or_else(|| p.get("dissname")).and_then(|v| v.as_str()).unwrap_or("QQ歌单").to_string();
                let cover = p.get("pic_url").or_else(|| p.get("dir_pic")).and_then(|v| v.as_str()).map(|s| s.to_string());
                let count = p.get("cnt").and_then(|v| v.as_i64()).unwrap_or(0);

                list.push(QQPlaylistSummary {
                    id: format!("qq-pl-{}", id),
                    name,
                    cover,
                    count,
                });
            }
        }

        if list.is_empty() {
            list.push(QQPlaylistSummary {
                id: format!("qq-pl-fav-{}", clean_uin),
                name: "我喜欢的音乐 (QQ)".to_string(),
                cover: Some(format!("https://q.qlogo.cn/headimg_dl?dst_uin={}&spec=100", clean_uin)),
                count: 0,
            });
        }

        Ok(list)
    }

    // 5. Get Playlist Detail
    pub async fn get_playlist(&self, disstid: &str) -> Result<QQPlaylistDetail, String> {
        let clean_id = disstid.trim_start_matches("qq-pl-");
        let url = "https://u.y.qq.com/cgi-bin/musicu.fcg";
        let body = serde_json::json!({
            "comm": { "uin": 0, "format": "json", "ct": 20, "cv": 0 },
            "req_0": {
                "module": "music.srfDissInfo.DissInfo",
                "method": "CgiGetDiss",
                "param": {
                    "disstid": clean_id.parse::<i64>().unwrap_or(0),
                    "userinfo": 1,
                    "tag": 1
                }
            }
        });

        let res = self
            .client
            .post(url)
            .headers(self.build_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        let mut tracks = Vec::new();

        let dirinfo = json.get("req_0").and_then(|r| r.get("data")).and_then(|d| d.get("dirinfo"));
        let name = dirinfo.and_then(|d| d.get("title")).and_then(|v| v.as_str()).unwrap_or("QQ音乐歌单").to_string();
        let desc = dirinfo.and_then(|d| d.get("desc")).and_then(|v| v.as_str()).map(|s| s.to_string());
        let cover = dirinfo.and_then(|d| d.get("picurl")).and_then(|v| v.as_str()).map(|s| s.to_string());

        if let Some(songlist) = json
            .get("req_0")
            .and_then(|r| r.get("data"))
            .and_then(|d| d.get("songlist"))
            .and_then(|s| s.as_array())
        {
            for s in songlist {
                let songmid = s.get("mid").or_else(|| s.get("songmid")).and_then(|v| v.as_str()).unwrap_or("");
                if songmid.is_empty() {
                    continue;
                }

                let id = format!("qq-{}", songmid);
                let title = s.get("name").or_else(|| s.get("songname")).and_then(|v| v.as_str()).unwrap_or("未知歌曲").to_string();

                let mut artist = "QQ音乐".to_string();
                if let Some(singers) = s.get("singer").and_then(|a| a.as_array()) {
                    let names: Vec<String> = singers
                        .iter()
                        .filter_map(|art| art.get("name").and_then(|n| n.as_str()).map(|n| n.to_string()))
                        .collect();
                    if !names.is_empty() {
                        artist = names.join(" / ");
                    }
                }

                let album = s.get("album").and_then(|a| a.get("name")).or_else(|| s.get("albumname")).and_then(|v| v.as_str()).unwrap_or("QQ音乐精选").to_string();
                let albummid = s.get("album").and_then(|a| a.get("mid")).or_else(|| s.get("albummid")).and_then(|v| v.as_str()).unwrap_or("");
                let song_cover = if !albummid.is_empty() {
                    Some(format!("https://y.gtimg.cn/music/photo_new/T002R300x300M000{}.jpg", albummid))
                } else {
                    cover.clone()
                };

                let duration = s.get("interval").and_then(|d| d.as_f64()).unwrap_or(180.0);

                let media_mid = s
                    .get("file")
                    .and_then(|f| f.get("media_mid"))
                    .or_else(|| s.get("media_mid"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(songmid);

                tracks.push(TrackRecord {
                    id,
                    title,
                    artist,
                    album,
                    duration,
                    path: format!("qq://{}:{}", songmid, media_mid),
                    format: "QQ音乐 320k".to_string(),
                    bitrate: Some(320),
                    year: None,
                    genre: Some("流行".to_string()),
                    cover: song_cover,
                    is_favorite: false,
                    play_count: 0,
                });
            }
        }

        Ok(QQPlaylistDetail {
            id: format!("qq-pl-{}", clean_id),
            name,
            description: desc,
            cover,
            tracks,
        })
    }

    // 6. Search Songs
    pub async fn search_songs(&self, keyword: &str, limit: Option<u32>) -> Result<Vec<TrackRecord>, String> {
        let lim = limit.unwrap_or(25);
        let url = format!(
            "https://c.y.qq.com/soso/fcgi-bin/client_search_cp?p=1&n={}&w={}&format=json",
            lim,
            urlencoding::encode(keyword)
        );

        let res = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        let mut list = Vec::new();

        if let Some(songs) = json
            .get("data")
            .and_then(|d| d.get("song"))
            .and_then(|s| s.get("list"))
            .and_then(|l| l.as_array())
        {
            for s in songs {
                let songmid = s.get("songmid").and_then(|v| v.as_str()).unwrap_or("");
                if songmid.is_empty() {
                    continue;
                }

                let id = format!("qq-{}", songmid);
                let title = s.get("songname").and_then(|v| v.as_str()).unwrap_or("未知歌曲").to_string();

                let mut artist = "未知歌手".to_string();
                if let Some(singers) = s.get("singer").and_then(|a| a.as_array()) {
                    let names: Vec<String> = singers
                        .iter()
                        .filter_map(|art| art.get("name").and_then(|n| n.as_str()).map(|n| n.to_string()))
                        .collect();
                    if !names.is_empty() {
                        artist = names.join(" / ");
                    }
                }

                let album = s
                    .get("albumname")
                    .and_then(|v| v.as_str())
                    .unwrap_or("QQ音乐精选")
                    .to_string();

                let albummid = s.get("albummid").and_then(|v| v.as_str()).unwrap_or("");
                let cover = if !albummid.is_empty() {
                    Some(format!(
                        "https://y.gtimg.cn/music/photo_new/T002R300x300M000{}.jpg",
                        albummid
                    ))
                } else {
                    None
                };

                let duration = s.get("interval").and_then(|d| d.as_f64()).unwrap_or(180.0);
                let media_mid = s
                    .get("media_mid")
                    .or_else(|| s.get("strMediaMid"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(songmid);

                list.push(TrackRecord {
                    id,
                    title,
                    artist,
                    album,
                    duration,
                    path: format!("qq://{}:{}", songmid, media_mid),
                    format: "QQ音乐 320k".to_string(),
                    bitrate: Some(320),
                    year: None,
                    genre: Some("流行".to_string()),
                    cover,
                    is_favorite: false,
                    play_count: 0,
                });
            }
        }

        Ok(list)
    }

    // Helper: Extract uin from cookie
    fn extract_uin(&self) -> String {
        if let Some(cookie) = self.get_cookie() {
            for part in cookie.split(';') {
                let part = part.trim();
                if part.starts_with("uin=") {
                    let uin = part.trim_start_matches("uin=").trim_start_matches('o');
                    if !uin.is_empty() {
                        return uin.to_string();
                    }
                } else if part.starts_with("wxuin=") {
                    let uin = part.trim_start_matches("wxuin=");
                    if !uin.is_empty() {
                        return uin.to_string();
                    }
                }
            }
        }
        "0".to_string()
    }

    // Helper: Query track detail to get media_mid
    async fn get_media_mid(&self, songmid: &str) -> Option<String> {
        let url = "https://u.y.qq.com/cgi-bin/musicu.fcg";
        let body = serde_json::json!({
            "comm": { "ct": 24, "cv": 0 },
            "songinfo": {
                "method": "get_song_detail_yqq",
                "param": { "song_type": 0, "song_mid": songmid },
                "module": "music.pf_song_detail_svr"
            }
        });
        let res = self
            .client
            .post(url)
            .headers(self.build_headers())
            .json(&body)
            .send()
            .await
            .ok()?;
        let json: serde_json::Value = res.json().await.ok()?;
        json.get("songinfo")
            .and_then(|s| s.get("data"))
            .and_then(|d| d.get("track_info"))
            .and_then(|t| t.get("file"))
            .and_then(|f| f.get("media_mid"))
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
    }

    // 7. Get Song Stream URL
    pub async fn get_song_url(&self, songmid_or_pair: &str) -> Result<Option<String>, String> {
        let parts: Vec<&str> = songmid_or_pair.split(':').collect();
        let songmid = parts[0].trim();
        if songmid.is_empty() {
            return Ok(None);
        }

        let media_mid = if parts.len() > 1 && !parts[1].trim().is_empty() {
            parts[1].trim().to_string()
        } else {
            self.get_media_mid(songmid).await.unwrap_or_else(|| songmid.to_string())
        };

        let uin = self.extract_uin();
        let guid = "1819638027";

        let mut filenames = vec![
            format!("C400{}.m4a", media_mid),
            format!("M500{}.mp3", media_mid),
        ];
        if media_mid != songmid {
            filenames.push(format!("C400{}.m4a", songmid));
            filenames.push(format!("M500{}.mp3", songmid));
        }

        let songmids = vec![songmid; filenames.len()];
        let songtypes = vec![0; filenames.len()];

        let body = serde_json::json!({
            "comm": {
                "uin": uin.parse::<i64>().unwrap_or(0),
                "format": "json",
                "ct": 24,
                "cv": 0
            },
            "req_0": {
                "module": "vkey.GetVkeyServer",
                "method": "CgiGetVkey",
                "param": {
                    "filename": filenames,
                    "guid": guid,
                    "songmid": songmids,
                    "songtype": songtypes,
                    "uin": uin,
                    "loginflag": 1,
                    "platform": "20"
                }
            }
        });

        let url = "https://u.y.qq.com/cgi-bin/musicu.fcg";
        let res = self
            .client
            .post(url)
            .headers(self.build_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

        if let Some(req_data) = json.get("req_0").and_then(|r| r.get("data")) {
            let sip_vec: Vec<String> = req_data
                .get("sip")
                .and_then(|s| s.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            // Default to HTTPS domain for security & preventing mixed-content blocking
            let base_sip = sip_vec
                .iter()
                .find(|s| s.starts_with("https://"))
                .cloned()
                .unwrap_or_else(|| {
                    if let Some(first) = sip_vec.first() {
                        first.replace("http://", "https://")
                    } else {
                        "https://isure.stream.qqmusic.qq.com/".to_string()
                    }
                });

            if let Some(midurlinfo) = req_data.get("midurlinfo").and_then(|m| m.as_array()) {
                for item in midurlinfo {
                    if let Some(purl) = item.get("purl").and_then(|p| p.as_str()) {
                        if !purl.is_empty() {
                            let stream_url = if purl.starts_with("http://") || purl.starts_with("https://") {
                                purl.to_string()
                            } else {
                                format!("{}{}", base_sip, purl)
                            };
                            return Ok(Some(stream_url));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    // 8. Get Lyrics
    pub async fn get_lyric(&self, songmid_or_pair: &str) -> Result<QQMusicLyricResult, String> {
        let clean_mid = songmid_or_pair.split(':').next().unwrap_or(songmid_or_pair).trim();
        let url = format!(
            "https://c.y.qq.com/lyric/fcgi-bin/fcg_query_lyric_new.fcg?songmid={}&format=json&nobase64=1",
            clean_mid
        );

        let res = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        let lrc = json
            .get("lyric")
            .and_then(|l| l.as_str())
            .unwrap_or("")
            .to_string();

        let trans = json
            .get("trans")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        Ok(QQMusicLyricResult { lrc, trans })
    }
}
