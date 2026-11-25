use crate::db::TrackRecord;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;

const USER_AGENT_STR: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15";

pub struct AppleMusicClient {
    client: Client,
}

impl AppleMusicClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .connect_timeout(std::time::Duration::from_secs(6))
            .timeout(std::time::Duration::from_secs(12))
            .pool_idle_timeout(std::time::Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .tcp_keepalive(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { client }
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STR));
        headers
    }

    // 1. Search Apple Music / iTunes Catalog
    pub async fn search_songs(&self, keyword: &str, limit: Option<u32>) -> Result<Vec<TrackRecord>, String> {
        let lim = limit.unwrap_or(25);
        let url = format!(
            "https://itunes.apple.com/search?term={}&entity=song&limit={}",
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

        if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
            for item in results {
                let track_id = item.get("trackId").and_then(|v| v.as_i64()).unwrap_or(0);
                if track_id == 0 {
                    continue;
                }

                let id = format!("apple-{}", track_id);
                let title = item
                    .get("trackName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("未知歌曲")
                    .to_string();

                let artist = item
                    .get("artistName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Apple Music")
                    .to_string();

                let album = item
                    .get("collectionName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Apple Digital Master")
                    .to_string();

                // High-resolution artwork: replace 100x100bb with 600x600bb
                let raw_cover = item.get("artworkUrl100").and_then(|v| v.as_str()).unwrap_or("");
                let cover = if !raw_cover.is_empty() {
                    Some(raw_cover.replace("100x100bb.jpg", "600x600bb.jpg"))
                } else {
                    None
                };

                let duration_ms = item.get("trackTimeMillis").and_then(|v| v.as_f64()).unwrap_or(180000.0);
                let duration = duration_ms / 1000.0;

                let preview_url = item.get("previewUrl").and_then(|v| v.as_str()).unwrap_or("");

                list.push(TrackRecord {
                    id,
                    title,
                    artist,
                    album,
                    duration,
                    path: if !preview_url.is_empty() {
                        preview_url.to_string()
                    } else {
                        format!("apple://{}", track_id)
                    },
                    format: "Apple AAC 256k".to_string(),
                    bitrate: Some(256),
                    year: None,
                    genre: item.get("primaryGenreName").and_then(|g| g.as_str()).map(|s| s.to_string()),
                    cover,
                    is_favorite: false,
                    play_count: 0,
                });
            }
        }

        Ok(list)
    }

    // 2. Get Song URL (lookup by track ID)
    pub async fn get_song_url(&self, track_id: &str) -> Result<Option<String>, String> {
        let url = format!("https://itunes.apple.com/lookup?id={}&entity=song", track_id);

        let res = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

        if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
            if let Some(first) = results.first() {
                if let Some(preview) = first.get("previewUrl").and_then(|p| p.as_str()) {
                    return Ok(Some(preview.to_string()));
                }
            }
        }

        Ok(None)
    }
}

