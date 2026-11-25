use crate::db::{DbManager, PlaylistRecord, TrackRecord};
use crate::netease::NeteaseClient;
use crate::qqmusic::QQMusicClient;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const USER_AGENT_STR: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedTrackItem {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedPlaylistPreview {
    pub platform: String,      // "spotify", "apple", "netease", "qq"
    pub platform_name: String, // "Spotify", "Apple Music", "网易云音乐", "QQ 音乐"
    pub name: String,
    pub description: Option<String>,
    pub cover: Option<String>,
    pub tracks_count: usize,
    pub tracks: Vec<ImportedTrackItem>,
}

pub struct PlaylistImporter {
    client: Client,
}

impl PlaylistImporter {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap_or_else(|_| Client::new());
        Self { client }
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STR));
        headers
    }

    pub async fn parse_link(
        &self,
        url: &str,
        netease_client: &NeteaseClient,
        qqmusic_client: &QQMusicClient,
    ) -> Result<ImportedPlaylistPreview, String> {
        let trimmed = url.trim();

        if trimmed.contains("spotify.com") {
            self.parse_spotify(trimmed).await
        } else if trimmed.contains("music.apple.com") {
            self.parse_apple_music(trimmed).await
        } else if trimmed.contains("163.com") {
            self.parse_netease(trimmed, netease_client).await
        } else if trimmed.contains("qq.com") {
            self.parse_qqmusic(trimmed, qqmusic_client).await
        } else {
            Err("不支持的链接格式。请提供 Spotify、Apple Music、网易云音乐或 QQ 音乐的歌单分享链接。".to_string())
        }
    }

    // 1. Parse Spotify Playlist Embed
    async fn parse_spotify(&self, url: &str) -> Result<ImportedPlaylistPreview, String> {
        // Extract playlist id
        let id = if let Some(pos) = url.find("/playlist/") {
            let sub = &url[pos + 10..];
            let end = sub.find('?').or_else(|| sub.find('/')).unwrap_or(sub.len());
            sub[..end].to_string()
        } else {
            return Err("无法从 Spotify 链接中提取歌单 ID".to_string());
        };

        let embed_url = format!("https://open.spotify.com/embed/playlist/{}", id);
        let res = self
            .client
            .get(&embed_url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| format!("请求 Spotify 页面失败: {}", e))?;

        let html = res.text().await.map_err(|e| e.to_string())?;

        // Extract JSON from <script id="__NEXT_DATA__" type="application/json">
        let marker = "<script id=\"__NEXT_DATA__\" type=\"application/json\">";
        if let Some(start) = html.find(marker) {
            let json_start = start + marker.len();
            if let Some(json_end) = html[json_start..].find("</script>") {
                let json_str = &html[json_start..json_start + json_end];
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
                    if let Some(entity) = val
                        .get("props")
                        .and_then(|p| p.get("pageProps"))
                        .and_then(|pp| pp.get("state"))
                        .and_then(|s| s.get("data"))
                        .and_then(|d| d.get("entity"))
                    {
                        let name = entity
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Spotify 导入歌单")
                            .to_string();

                        let description = entity
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());

                        let cover = entity
                            .get("coverArt")
                            .and_then(|c| c.get("sources"))
                            .and_then(|s| s.as_array())
                            .and_then(|arr| arr.first())
                            .and_then(|img| img.get("url"))
                            .and_then(|u| u.as_str())
                            .map(|s| s.to_string());

                        let mut tracks = Vec::new();
                        if let Some(track_list) = entity.get("trackList").and_then(|t| t.as_array()) {
                            for item in track_list {
                                let title = item
                                    .get("title")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                if title.is_empty() {
                                    continue;
                                }

                                let artist = item
                                    .get("subtitle")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Spotify Artist")
                                    .to_string();

                                let duration_ms = item.get("duration").and_then(|v| v.as_f64()).unwrap_or(180000.0);

                                tracks.push(ImportedTrackItem {
                                    title,
                                    artist,
                                    album: Some(name.clone()),
                                    duration: duration_ms / 1000.0,
                                });
                            }
                        }

                        let tracks_count = tracks.len();
                        return Ok(ImportedPlaylistPreview {
                            platform: "spotify".to_string(),
                            platform_name: "Spotify".to_string(),
                            name,
                            description,
                            cover,
                            tracks_count,
                            tracks,
                        });
                    }
                }
            }
        }

        Err("无法解析 Spotify 歌单内容，请确认该歌单为公开可见".to_string())
    }

    // 2. Parse Apple Music Playlist
    async fn parse_apple_music(&self, url: &str) -> Result<ImportedPlaylistPreview, String> {
        let res = self
            .client
            .get(url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| format!("请求 Apple Music 页面失败: {}", e))?;

        let html = res.text().await.map_err(|e| e.to_string())?;

        // Fallback meta tags
        let mut name = "Apple Music 导入歌单".to_string();
        if let Some(pos) = html.find("property=\"og:title\" content=\"") {
            let sub = &html[pos + 30..];
            if let Some(end) = sub.find('"') {
                name = sub[..end].to_string();
            }
        }

        let mut description: Option<String> = None;
        if let Some(pos) = html.find("property=\"og:description\" content=\"") {
            let sub = &html[pos + 36..];
            if let Some(end) = sub.find('"') {
                description = Some(sub[..end].to_string());
            }
        }

        let mut cover: Option<String> = None;
        if let Some(pos) = html.find("property=\"og:image\" content=\"") {
            let sub = &html[pos + 30..];
            if let Some(end) = sub.find('"') {
                cover = Some(sub[..end].to_string());
            }
        }

        let mut tracks = Vec::new();

        // Extract <script type="application/ld+json">
        let marker = "<script type=\"application/ld+json\">";
        let mut search_from = 0;
        while let Some(start) = html[search_from..].find(marker) {
            let json_start = search_from + start + marker.len();
            if let Some(json_end) = html[json_start..].find("</script>") {
                let json_str = &html[json_start..json_start + json_end];
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
                    if let Some(track_arr) = val.get("track").and_then(|t| t.as_array()) {
                        for item in track_arr {
                            let title = item.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
                            if title.is_empty() {
                                continue;
                            }

                            let artist = item
                                .get("byArtist")
                                .and_then(|a| a.get("name"))
                                .and_then(|n| n.as_str())
                                .unwrap_or("Apple Music")
                                .to_string();

                            let iso_duration = item.get("duration").and_then(|d| d.as_str()).unwrap_or("");
                            let duration = parse_iso_duration(iso_duration);

                            tracks.push(ImportedTrackItem {
                                title,
                                artist,
                                album: Some(name.clone()),
                                duration,
                            });
                        }
                    }
                    if !tracks.is_empty() {
                        break;
                    }
                }
                search_from = json_start + json_end;
            } else {
                break;
            }
        }

        let tracks_count = tracks.len();
        if tracks_count == 0 {
            return Err("未在 Apple Music 页面中检索到公开曲目".to_string());
        }

        Ok(ImportedPlaylistPreview {
            platform: "apple".to_string(),
            platform_name: "Apple Music".to_string(),
            name,
            description,
            cover,
            tracks_count,
            tracks,
        })
    }

    // 3. Parse NetEase Playlist
    async fn parse_netease(
        &self,
        url: &str,
        netease_client: &NeteaseClient,
    ) -> Result<ImportedPlaylistPreview, String> {
        let id = extract_id_from_url(url, "id=")?;
        let detail = netease_client.get_playlist(&id).await?;

        let tracks: Vec<ImportedTrackItem> = detail
            .tracks
            .into_iter()
            .map(|t| ImportedTrackItem {
                title: t.title,
                artist: t.artist,
                album: Some(t.album),
                duration: t.duration,
            })
            .collect();

        Ok(ImportedPlaylistPreview {
            platform: "netease".to_string(),
            platform_name: "网易云音乐".to_string(),
            name: detail.name,
            description: detail.description,
            cover: detail.cover,
            tracks_count: tracks.len(),
            tracks,
        })
    }

    // 4. Parse QQ Music Playlist
    async fn parse_qqmusic(
        &self,
        url: &str,
        qqmusic_client: &QQMusicClient,
    ) -> Result<ImportedPlaylistPreview, String> {
        let id = extract_id_from_url(url, "id=").or_else(|_| extract_id_from_url(url, "playlist/"))?;
        let detail = qqmusic_client.get_playlist(&id).await?;

        let tracks: Vec<ImportedTrackItem> = detail
            .tracks
            .into_iter()
            .map(|t| ImportedTrackItem {
                title: t.title,
                artist: t.artist,
                album: Some(t.album),
                duration: t.duration,
            })
            .collect();

        Ok(ImportedPlaylistPreview {
            platform: "qq".to_string(),
            platform_name: "QQ 音乐".to_string(),
            name: detail.name,
            description: detail.description,
            cover: detail.cover,
            tracks_count: tracks.len(),
            tracks,
        })
    }

    // 5. Commit import into SQLite Database
    pub fn save_imported_playlist(
        &self,
        preview: ImportedPlaylistPreview,
        db_state: &Arc<DbManager>,
    ) -> Result<PlaylistRecord, String> {
        let created = db_state.create_playlist(
            &preview.name,
            preview.description,
            preview.cover.clone(),
        )?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        for (idx, item) in preview.tracks.into_iter().enumerate() {
            let track_id = format!("imp-{}-{}", now, idx);
            let path = format!("search://{}", urlencoding::encode(&format!("{} {}", item.title, item.artist)));

            let track = TrackRecord {
                id: track_id.clone(),
                title: item.title,
                artist: item.artist,
                album: item.album.unwrap_or_else(|| format!("{} 导入歌单", preview.platform_name)),
                duration: item.duration,
                path,
                format: format!("{} 导入", preview.platform_name),
                bitrate: Some(320),
                year: None,
                genre: Some("流媒体".to_string()),
                cover: preview.cover.clone(),
                is_favorite: false,
                play_count: 0,
            };

            let _ = db_state.upsert_track(&track);
            let _ = db_state.add_track_to_playlist(&created.id, &track_id);
        }

        Ok(created)
    }
}

// Helpers
fn extract_id_from_url(url: &str, keyword: &str) -> Result<String, String> {
    if let Some(pos) = url.find(keyword) {
        let sub = &url[pos + keyword.len()..];
        let end = sub
            .find(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
            .unwrap_or(sub.len());
        let id = &sub[..end];
        if !id.is_empty() {
            return Ok(id.to_string());
        }
    }
    Err(format!("无法从链接中匹配到歌单 ID: {}", url))
}

fn parse_iso_duration(iso: &str) -> f64 {
    // Format: PT3M45S or PT215S
    let s = iso.trim_start_matches("PT");
    let mut total_secs = 0.0;
    if let Some(m_pos) = s.find('M') {
        let mins: f64 = s[..m_pos].parse().unwrap_or(0.0);
        total_secs += mins * 60.0;
        let rem = &s[m_pos + 1..];
        if let Some(s_pos) = rem.find('S') {
            let secs: f64 = rem[..s_pos].parse().unwrap_or(0.0);
            total_secs += secs;
        }
    } else if let Some(s_pos) = s.find('S') {
        let secs: f64 = s[..s_pos].parse().unwrap_or(0.0);
        total_secs += secs;
    }
    if total_secs > 0.0 {
        total_secs
    } else {
        180.0
    }
}
