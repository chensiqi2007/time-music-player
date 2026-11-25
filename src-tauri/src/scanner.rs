use crate::db::{DbManager, TrackRecord};
use base64::prelude::*;
use lofty::config::WriteOptions;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::read_from_path;
use lofty::tag::{Accessor, TagExt};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub total_found: usize,
    pub successfully_added: usize,
    pub errors: usize,
}

const SUPPORTED_EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "aac", "ogg", "m4a", "ape", "wma"];

pub fn is_supported_audio(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str())
    } else {
        false
    }
}

pub fn scan_directory(dir_path: &str, db: Arc<DbManager>) -> ScanResult {
    let mut total_found = 0;
    let mut successfully_added = 0;
    let mut errors = 0;

    let path = Path::new(dir_path);
    if !path.exists() {
        return ScanResult {
            total_found: 0,
            successfully_added: 0,
            errors: 0,
        };
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let file_path = entry.path();
        if file_path.is_file() && is_supported_audio(file_path) {
            total_found += 1;
            match parse_audio_file(file_path) {
                Ok(record) => {
                    if db.upsert_track(&record).is_ok() {
                        successfully_added += 1;
                    } else {
                        errors += 1;
                    }
                }
                Err(err) => {
                    eprintln!("[Scanner] Error parsing {:?}: {}", file_path, err);
                    errors += 1;
                }
            }
        }
    }

    ScanResult {
        total_found,
        successfully_added,
        errors,
    }
}

pub fn parse_audio_file(path: &Path) -> Result<TrackRecord, String> {
    let tagged_file = read_from_path(path).map_err(|e| e.to_string())?;
    let properties = tagged_file.properties();

    let duration = properties.duration().as_secs_f64();
    let bitrate = properties.audio_bitrate();

    let format = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("audio")
        .to_uppercase();

    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未知曲目")
        .to_string();

    let mut title = file_stem.clone();
    let mut artist = "未知艺术家".to_string();
    let mut album = "未知专辑".to_string();
    let mut genre: Option<String> = None;
    let mut year: Option<u32> = None;
    let mut cover_data_uri: Option<String> = None;

    if let Some(tag) = tagged_file.primary_tag().or_else(|| tagged_file.first_tag()) {
        if let Some(t) = tag.title() {
            let t_str = t.trim();
            if !t_str.is_empty() {
                title = t_str.to_string();
            }
        }

        if let Some(a) = tag.artist() {
            let a_str = a.trim();
            if !a_str.is_empty() {
                artist = a_str.to_string();
            }
        }

        if let Some(al) = tag.album() {
            let al_str = al.trim();
            if !al_str.is_empty() {
                album = al_str.to_string();
            }
        }

        if let Some(g) = tag.genre() {
            let g_str = g.trim();
            if !g_str.is_empty() {
                genre = Some(g_str.to_string());
            }
        }

        if let Some(d) = tag.date() {
            year = Some(d.year as u32);
        }

        // Extract first picture if available
        if let Some(pic) = tag.pictures().first() {
            let mime = pic
                .mime_type()
                .map(|m| m.as_str())
                .unwrap_or("image/jpeg");
            let b64 = BASE64_STANDARD.encode(pic.data());
            cover_data_uri = Some(format!("data:{};base64,{}", mime, b64));
        }
    }

    let path_str = path.to_string_lossy().to_string();
    // Unique ID based on path hash
    let id = format!("track-{:x}", md5_hash(&path_str));

    Ok(TrackRecord {
        id,
        title,
        artist,
        album,
        duration,
        path: path_str,
        format,
        bitrate,
        year,
        genre,
        cover: cover_data_uri,
        is_favorite: false,
        play_count: 0,
    })
}

pub fn save_audio_metadata(
    path_str: &str,
    new_title: &str,
    new_artist: &str,
    new_album: &str,
) -> Result<(), String> {
    let path = Path::new(path_str);
    let mut tagged_file = read_from_path(path).map_err(|e| e.to_string())?;

    if let Some(tag) = tagged_file.primary_tag_mut() {
        tag.set_title(new_title.to_string());
        tag.set_artist(new_artist.to_string());
        tag.set_album(new_album.to_string());
        tag.save_to_path(path, WriteOptions::default())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn md5_hash(input: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

