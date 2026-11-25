use crate::db::TrackRecord;
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, REFERER, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

const USER_AGENT_STR: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
const REFERER_STR: &str = "https://www.kugou.com/";

pub struct KugouClient {
    client: Client,
    user_cookie: Mutex<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KugouLyricResult {
    pub lrc: String,
}

impl KugouClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .connect_timeout(std::time::Duration::from_secs(6))
            .timeout(std::time::Duration::from_secs(12))
            .pool_idle_timeout(std::time::Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            user_cookie: Mutex::new(None),
        }
    }

    pub fn set_cookie(&self, cookie: String) {
        if let Ok(mut c) = self.user_cookie.lock() {
            let trimmed = cookie.trim().to_string();
            *c = if trimmed.is_empty() { None } else { Some(trimmed) };
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

    /// Search tracks on Kugou Music.
    pub async fn search_songs(&self, keyword: &str, limit: Option<u32>) -> Result<Vec<TrackRecord>, String> {
        let lim = limit.unwrap_or(25);
        let url = format!(
            "https://songsearch.kugou.com/song_search_v2?keyword={}&page=1&pagesize={}&platform=WebFilter",
            urlencoding::encode(keyword),
            lim
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

        if let Some(items) = json
            .get("data")
            .and_then(|d| d.get("lists"))
            .and_then(|l| l.as_array())
        {
            for item in items {
                let file_hash = item
                    .get("FileHash")
                    .or_else(|| item.get("HQFileHash"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if file_hash.is_empty() {
                    continue;
                }

                let clip_hash = item
                    .get("trans_param")
                    .and_then(|tp| tp.get("hash_offset"))
                    .and_then(|ho| ho.get("clip_hash"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                let title = item
                    .get("SongName")
                    .or_else(|| item.get("OriSongName"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("未知歌曲")
                    .replace("<em>", "")
                    .replace("</em>", "");

                let artist = item
                    .get("SingerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("未知歌手")
                    .replace("<em>", "")
                    .replace("</em>", "")
                    .replace('、', " / ");

                let album_raw = item.get("AlbumName").and_then(|v| v.as_str()).unwrap_or("");
                let album = if album_raw.is_empty() {
                    "酷狗音乐精选".to_string()
                } else {
                    album_raw.replace("<em>", "").replace("</em>", "")
                };

                let duration = item
                    .get("Duration")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(180.0);

                let raw_img = item
                    .get("Image")
                    .or_else(|| item.get("trans_param").and_then(|tp| tp.get("union_cover")))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let cover = if !raw_img.is_empty() {
                    Some(raw_img.replace("{size}", "400").replace("http://", "https://"))
                } else {
                    None
                };

                let path = if !clip_hash.is_empty() {
                    format!("kugou://{}:{}", file_hash, clip_hash)
                } else {
                    format!("kugou://{}", file_hash)
                };

                list.push(TrackRecord {
                    id: format!("kugou-{}", file_hash),
                    title,
                    artist,
                    album,
                    duration,
                    path,
                    format: "酷狗 Hi-Fi".to_string(),
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

    async fn fetch_play_info_url(&self, hash: &str) -> Option<String> {
        // If user is logged in with Cookie, try Kugou web play/getdata first for VIP tracks
        if self.get_cookie().is_some() {
            let vip_url = format!(
                "https://wwwapi.kugou.com/yy/index.php?r=play/getdata&hash={}",
                hash.trim()
            );
            if let Ok(res) = self.client.get(&vip_url).headers(self.build_headers()).send().await {
                if let Ok(json) = res.json::<serde_json::Value>().await {
                    if let Some(u) = json.get("data").and_then(|d| d.get("play_url")).and_then(|v| v.as_str()) {
                        if !u.is_empty() && u.starts_with("http") {
                            return Some(u.replace("http://", "https://"));
                        }
                    }
                }
            }
        }

        let url = format!(
            "https://m.kugou.com/app/i/getSongInfo.php?cmd=playInfo&hash={}",
            hash.trim()
        );
        let res = self.client.get(&url).headers(self.build_headers()).send().await.ok()?;
        let json: serde_json::Value = res.json().await.ok()?;

        if let Some(u) = json.get("url").and_then(|v| v.as_str()) {
            if !u.is_empty() && u.starts_with("http") {
                return Some(u.replace("http://", "https://"));
            }
        }
        if let Some(backup) = json.get("backup_url").and_then(|b| b.as_array()) {
            for item in backup {
                if let Some(u) = item.as_str() {
                    if !u.is_empty() && u.starts_with("http") {
                        return Some(u.replace("http://", "https://"));
                    }
                }
            }
        }
        None
    }

    /// Resolve stream URL for Kugou track hash (or `hash:clip_hash` pair).
    pub async fn get_song_url(&self, hash_or_pair: &str) -> Result<Option<String>, String> {
        let parts: Vec<&str> = hash_or_pair.split(':').collect();
        let main_hash = parts[0].trim();
        if main_hash.is_empty() {
            return Ok(None);
        }

        if let Some(url) = self.fetch_play_info_url(main_hash).await {
            return Ok(Some(url));
        }

        if parts.len() > 1 && !parts[1].trim().is_empty() {
            if let Some(clip_url) = self.fetch_play_info_url(parts[1].trim()).await {
                return Ok(Some(clip_url));
            }
        }

        Ok(None)
    }

    /// Fetch Base64-encoded LRC lyrics from Kugou KRC search & download API.
    pub async fn get_lyric(&self, hash_or_pair: &str, keyword: Option<&str>) -> Result<KugouLyricResult, String> {
        let main_hash = hash_or_pair.split(':').next().unwrap_or(hash_or_pair).trim();
        let search_url = format!(
            "https://krcs.kugou.com/search?ver=1&man=yes&client=mobi&keyword={}&duration=&hash={}",
            urlencoding::encode(keyword.unwrap_or("")),
            main_hash
        );

        let res = self
            .client
            .get(&search_url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

        if let Some(candidate) = json
            .get("candidates")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
        {
            let id = candidate.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let accesskey = candidate.get("accesskey").and_then(|v| v.as_str()).unwrap_or("");
            if !id.is_empty() && !accesskey.is_empty() {
                let dl_url = format!(
                    "https://lyrics.kugou.com/download?ver=1&client=pc&id={}&accesskey={}&fmt=lrc&charset=utf8",
                    id, accesskey
                );
                if let Ok(dl_res) = self.client.get(&dl_url).headers(self.build_headers()).send().await {
                    if let Ok(dl_json) = dl_res.json::<serde_json::Value>().await {
                        if let Some(b64) = dl_json.get("content").and_then(|v| v.as_str()) {
                            if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(b64) {
                                if let Ok(lrc) = String::from_utf8(decoded) {
                                    return Ok(KugouLyricResult { lrc });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(KugouLyricResult { lrc: String::new() })
    }
}
