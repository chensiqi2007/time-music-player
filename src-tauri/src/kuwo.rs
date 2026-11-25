use crate::db::TrackRecord;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, REFERER, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

const USER_AGENT_STR: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
const REFERER_STR: &str = "https://www.kuwo.cn/";

pub struct KuwoClient {
    client: Client,
    user_cookie: Mutex<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuwoLyricResult {
    pub lrc: String,
}

impl KuwoClient {
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

    fn decode_html_entities(input: &str) -> String {
        input
            .replace("&nbsp;", " ")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
    }

    /// Search tracks on Kuwo Music.
    pub async fn search_songs(&self, keyword: &str, limit: Option<u32>) -> Result<Vec<TrackRecord>, String> {
        let lim = limit.unwrap_or(25);
        let url = format!(
            "https://search.kuwo.cn/r.s?all={}&ft=music&itemset=web_2013&client=kt&pn=0&rn={}&rformat=json&encoding=utf8",
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

        let raw_text = res.text().await.map_err(|e| e.to_string())?;
        // Kuwo r.s often returns single-quoted JSON-like objects; normalize safely if needed.
        let json: serde_json::Value = match serde_json::from_str(&raw_text) {
            Ok(v) => v,
            Err(_) => {
                let normalized = Self::normalize_kuwo_json(&raw_text);
                serde_json::from_str(&normalized).map_err(|e| format!("Kuwo JSON parse error: {}", e))?
            }
        };

        let mut list = Vec::new();
        if let Some(abslist) = json.get("abslist").and_then(|v| v.as_array()) {
            for item in abslist {
                let music_rid = item
                    .get("MUSICRID")
                    .or_else(|| item.get("DC_TARGETID"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let rid = music_rid.trim_start_matches("MUSIC_").trim();
                if rid.is_empty() {
                    continue;
                }

                let raw_title = item
                    .get("SONGNAME")
                    .or_else(|| item.get("NAME"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("未知歌曲");
                let title = Self::decode_html_entities(raw_title);

                let raw_artist = item.get("ARTIST").and_then(|v| v.as_str()).unwrap_or("未知歌手");
                let artist = Self::decode_html_entities(raw_artist).replace('&', " / ");

                let raw_album = item.get("ALBUM").and_then(|v| v.as_str()).unwrap_or("");
                let album = if raw_album.is_empty() {
                    "酷我音乐精选".to_string()
                } else {
                    Self::decode_html_entities(raw_album)
                };

                let duration = item
                    .get("DURATION")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
                    .or_else(|| item.get("DURATION").and_then(|v| v.as_f64()))
                    .unwrap_or(180.0);

                let pic_short = item
                    .get("web_albumpic_short")
                    .or_else(|| item.get("web_artistpic_short"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let cover = if !pic_short.is_empty() {
                    if pic_short.starts_with("http") {
                        Some(pic_short.replace("http://", "https://"))
                    } else {
                        Some(format!("https://img4.kuwo.cn/star/albumcover/{}", pic_short.trim_start_matches('/')))
                    }
                } else {
                    None
                };

                list.push(TrackRecord {
                    id: format!("kuwo-{}", rid),
                    title,
                    artist,
                    album,
                    duration,
                    path: format!("kuwo://{}", rid),
                    format: "酷我 MP3 320k".to_string(),
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

    /// Convert Kuwo's single-quoted JS-style JSON into valid RFC 8259 JSON.
    fn normalize_kuwo_json(raw: &str) -> String {
        let mut out = String::with_capacity(raw.len());
        let mut in_single = false;
        let mut escaped = false;
        for ch in raw.chars() {
            if escaped {
                if ch == '\'' {
                    out.push('\'');
                } else {
                    out.push('\\');
                    out.push(ch);
                }
                escaped = false;
                continue;
            }
            if ch == '\\' && in_single {
                escaped = true;
                continue;
            }
            if ch == '\'' {
                in_single = !in_single;
                out.push('"');
            } else if ch == '"' && in_single {
                out.push_str("\\\"");
            } else {
                out.push(ch);
            }
        }
        out
    }

    /// Resolve direct audio stream URL for a Kuwo track ID.
    pub async fn get_song_url(&self, rid: &str) -> Result<Option<String>, String> {
        let clean_rid = rid.trim_start_matches("MUSIC_").trim();
        if clean_rid.is_empty() {
            return Ok(None);
        }

        // If user is logged in with Cookie, try official web playUrl endpoint first for VIP tracks
        if self.get_cookie().is_some() {
            let vip_url = format!(
                "https://www.kuwo.cn/api/v1/www/music/playUrl?mid={}&type=music&httpsStatus=1",
                clean_rid
            );
            if let Ok(res) = self.client.get(&vip_url).headers(self.build_headers()).send().await {
                if let Ok(json) = res.json::<serde_json::Value>().await {
                    if let Some(u) = json.get("data").and_then(|d| d.get("url")).and_then(|v| v.as_str()) {
                        if !u.is_empty() && u.starts_with("http") {
                            return Ok(Some(u.replace("http://", "https://")));
                        }
                    }
                }
            }
        }

        let url = format!(
            "https://antiserver.kuwo.cn/anti.s?type=convert_url3&rid={}&format=mp3",
            clean_rid
        );

        let res = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        if let Some(stream_url) = json.get("url").and_then(|u| u.as_str()) {
            if !stream_url.is_empty() && stream_url.starts_with("http") {
                return Ok(Some(stream_url.replace("http://", "https://")));
            }
        }

        Ok(None)
    }

    /// Fetch and format Kuwo lyrics into standard LRC text.
    pub async fn get_lyric(&self, rid: &str) -> Result<KuwoLyricResult, String> {
        let clean_rid = rid.trim_start_matches("MUSIC_").trim();
        let url = format!(
            "https://m.kuwo.cn/newh5/singles/songinfoandlrc?musicId={}&httpsStatus=1",
            clean_rid
        );

        let res = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        let mut lrc_lines = Vec::new();

        if let Some(lrclist) = json
            .get("data")
            .and_then(|d| d.get("lrclist"))
            .and_then(|l| l.as_array())
        {
            for item in lrclist {
                let time_sec = item
                    .get("time")
                    .and_then(|t| t.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
                    .or_else(|| item.get("time").and_then(|t| t.as_f64()))
                    .unwrap_or(0.0);
                let text = item
                    .get("lineLyric")
                    .and_then(|l| l.as_str())
                    .unwrap_or("")
                    .trim();
                if text.is_empty() {
                    continue;
                }
                let mins = (time_sec / 60.0).floor() as u32;
                let secs = time_sec % 60.0;
                lrc_lines.push(format!("[{:02}:{:05.2}]{}", mins, secs, text));
            }
        }

        Ok(KuwoLyricResult {
            lrc: lrc_lines.join("\n"),
        })
    }
}
