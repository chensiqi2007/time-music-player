use crate::db::TrackRecord;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, REFERER, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

const USER_AGENT_STR: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
const REFERER_STR: &str = "https://music.douyin.com/";

pub struct QishuiClient {
    client: Client,
    user_cookie: Mutex<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QishuiLyricResult {
    pub lrc: String,
}

impl QishuiClient {
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

    /// Search tracks from Qishui Music (汽水音乐 / music.douyin.com).
    /// Attempts SSR `_ROUTER_DATA` extraction from the public web search page.
    pub async fn search_songs(&self, keyword: &str, limit: Option<u32>) -> Result<Vec<TrackRecord>, String> {
        let lim = limit.unwrap_or(20) as usize;
        let url = format!(
            "https://music.douyin.com/qishui/share/search?keyword={}",
            urlencoding::encode(keyword)
        );

        let res = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let html = res.text().await.map_err(|e| e.to_string())?;
        let mut list = Vec::new();

        // Extract embedded SSR JSON if present in `_ROUTER_DATA`
        if let Some(router_json) = Self::extract_router_data(&html) {
            Self::collect_tracks_recursive(&router_json, &mut list, lim);
        }

        Ok(list)
    }

    fn extract_router_data(html: &str) -> Option<serde_json::Value> {
        let marker = "_ROUTER_DATA";
        let pos = html.find(marker)?;
        let rest = &html[pos..];
        let eq_pos = rest.find('=')?;
        let after_eq = rest[eq_pos + 1..].trim_start();
        let end_pos = after_eq.find("</script>")?;
        let json_str = after_eq[..end_pos].trim().trim_end_matches(';');
        serde_json::from_str(json_str).ok()
    }

    fn collect_tracks_recursive(val: &serde_json::Value, out: &mut Vec<TrackRecord>, limit: usize) {
        if out.len() >= limit {
            return;
        }
        match val {
            serde_json::Value::Array(arr) => {
                for item in arr {
                    Self::collect_tracks_recursive(item, out, limit);
                    if out.len() >= limit {
                        break;
                    }
                }
            }
            serde_json::Value::Object(map) => {
                // Qishui track objects typically carry `id` + `name` + `artists`/`album`
                let track_obj = map.get("track").unwrap_or(val);
                if let (Some(id_val), Some(name_val)) = (
                    track_obj.get("id").and_then(|v| v.as_str()),
                    track_obj.get("name").and_then(|v| v.as_str()),
                ) {
                    if !id_val.is_empty() && !name_val.is_empty() && track_obj.get("artists").is_some() {
                        let full_id = format!("qishui-{}", id_val);
                        if !out.iter().any(|t| t.id == full_id) {
                            let artist = track_obj
                                .get("artists")
                                .and_then(|a| a.as_array())
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|x| x.get("name").and_then(|n| n.as_str()))
                                        .collect::<Vec<_>>()
                                        .join(" / ")
                                })
                                .filter(|s| !s.is_empty())
                                .unwrap_or_else(|| "汽水音乐".to_string());

                            let album = track_obj
                                .get("album")
                                .and_then(|a| a.get("name"))
                                .and_then(|n| n.as_str())
                                .unwrap_or("汽水音乐精选")
                                .to_string();

                            let cover = track_obj
                                .get("album")
                                .and_then(|a| a.get("url_cover"))
                                .and_then(|c| c.get("urls"))
                                .and_then(|u| u.as_array())
                                .and_then(|arr| arr.first())
                                .and_then(|u| u.as_str())
                                .map(|s| s.to_string());

                            let duration = track_obj
                                .get("duration")
                                .and_then(|d| d.as_f64())
                                .map(|ms| if ms > 1000.0 { ms / 1000.0 } else { ms })
                                .unwrap_or(180.0);

                            out.push(TrackRecord {
                                id: full_id,
                                title: name_val.to_string(),
                                artist,
                                album,
                                duration,
                                path: format!("qishui://{}", id_val),
                                format: "汽水音乐".to_string(),
                                bitrate: Some(320),
                                year: None,
                                genre: Some("流行".to_string()),
                                cover,
                                is_favorite: false,
                                play_count: 0,
                            });
                        }
                    }
                }
                for child in map.values() {
                    Self::collect_tracks_recursive(child, out, limit);
                    if out.len() >= limit {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    /// Resolve direct Qishui audio URL from public track share page when unencrypted.
    pub async fn get_song_url(&self, track_id: &str) -> Result<Option<String>, String> {
        let clean_id = track_id.trim();
        if clean_id.is_empty() {
            return Ok(None);
        }
        let url = format!("https://music.douyin.com/qishui/share/track?track_id={}", clean_id);
        let res = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let html = res.text().await.map_err(|e| e.to_string())?;

        if let Some(router_json) = Self::extract_router_data(&html) {
            if let Some(play_url) = Self::find_unencrypted_audio_url(&router_json) {
                return Ok(Some(play_url));
            }
        }

        // Return None when DRM/spade_a protected so player.ts triggers cross-catalog fallback cleanly.
        Ok(None)
    }

    fn find_unencrypted_audio_url(val: &serde_json::Value) -> Option<String> {
        match val {
            serde_json::Value::Object(map) => {
                if let Some(main_url) = map.get("main_url").and_then(|v| v.as_str()) {
                    // Only return if no `spade_a` AES encryption key is present
                    let has_spade = map
                        .get("spade_a")
                        .and_then(|v| v.as_str())
                        .map(|s| !s.is_empty())
                        .unwrap_or(false);
                    if !has_spade && main_url.starts_with("http") {
                        return Some(main_url.to_string());
                    }
                }
                for v in map.values() {
                    if let Some(found) = Self::find_unencrypted_audio_url(v) {
                        return Some(found);
                    }
                }
                None
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    if let Some(found) = Self::find_unencrypted_audio_url(item) {
                        return Some(found);
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub async fn get_lyric(&self, _track_id: &str) -> Result<QishuiLyricResult, String> {
        Ok(QishuiLyricResult { lrc: String::new() })
    }
}
