use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheInfo {
    pub current_path: String,
    pub total_bytes: u64,
    pub formatted_total: String,
    pub audio_bytes: u64,
    pub formatted_audio: String,
    pub cover_bytes: u64,
    pub formatted_cover: String,
    pub other_bytes: u64,
    pub formatted_other: String,
    pub file_count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemPresetDir {
    pub id: String,
    pub name: String,
    pub path: String,
    pub icon_type: String, // "music", "download", "desktop", "drive"
    pub exists: bool,
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn get_default_cache_path() -> PathBuf {
    dirs::cache_dir()
        .or_else(dirs::data_local_dir)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("time-music")
        .join("cache")
}

pub fn resolve_cache_dir(custom_path: Option<String>) -> PathBuf {
    if let Some(custom) = custom_path {
        let trimmed = custom.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    get_default_cache_path()
}

pub fn inspect_cache(cache_dir: &Path) -> CacheInfo {
    let mut total_bytes: u64 = 0;
    let mut audio_bytes: u64 = 0;
    let mut cover_bytes: u64 = 0;
    let mut other_bytes: u64 = 0;
    let mut file_count: usize = 0;

    if cache_dir.exists() {
        for entry in WalkDir::new(cache_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(meta) = entry.metadata() {
                    let len = meta.len();
                    total_bytes += len;
                    file_count += 1;

                    let ext = entry
                        .path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_lowercase())
                        .unwrap_or_default();

                    match ext.as_str() {
                        "mp3" | "flac" | "wav" | "m4a" | "aac" | "ogg" | "audio" | "part" => {
                            audio_bytes += len;
                        }
                        "png" | "jpg" | "jpeg" | "webp" | "gif" | "cover" => {
                            cover_bytes += len;
                        }
                        _ => {
                            other_bytes += len;
                        }
                    }
                }
            }
        }
    } else {
        let _ = std::fs::create_dir_all(cache_dir);
    }

    CacheInfo {
        current_path: cache_dir.to_string_lossy().to_string(),
        total_bytes,
        formatted_total: format_bytes(total_bytes),
        audio_bytes,
        formatted_audio: format_bytes(audio_bytes),
        cover_bytes,
        formatted_cover: format_bytes(cover_bytes),
        other_bytes,
        formatted_other: format_bytes(other_bytes),
        file_count,
    }
}

pub fn clear_cache_directory(cache_dir: &Path) -> Result<u64, String> {
    if !cache_dir.exists() {
        return Ok(0);
    }

    let mut freed_bytes: u64 = 0;
    for entry in WalkDir::new(cache_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(meta) = entry.metadata() {
                freed_bytes += meta.len();
            }
            let _ = std::fs::remove_file(entry.path());
        }
    }
    Ok(freed_bytes)
}

pub fn get_system_preset_dirs() -> Vec<SystemPresetDir> {
    let mut list = Vec::new();

    if let Some(p) = dirs::audio_dir() {
        list.push(SystemPresetDir {
            id: "user_music".to_string(),
            name: "用户音乐目录".to_string(),
            path: p.to_string_lossy().to_string(),
            icon_type: "music".to_string(),
            exists: p.exists(),
        });
    }

    if let Some(p) = dirs::download_dir() {
        list.push(SystemPresetDir {
            id: "user_downloads".to_string(),
            name: "系统下载目录".to_string(),
            path: p.to_string_lossy().to_string(),
            icon_type: "download".to_string(),
            exists: p.exists(),
        });
    }

    if let Some(p) = dirs::desktop_dir() {
        list.push(SystemPresetDir {
            id: "user_desktop".to_string(),
            name: "桌面快捷目录".to_string(),
            path: p.to_string_lossy().to_string(),
            icon_type: "desktop".to_string(),
            exists: p.exists(),
        });
    }

    #[cfg(target_os = "windows")]
    {
        let drive_candidates = [
            ("D:\\Music", "D 盘音乐专属", "music"),
            ("D:\\TimeMusicCache", "D 盘 TimeMusic 专用缓存", "drive"),
            ("E:\\Music", "E 盘音乐专属", "music"),
            ("D:\\", "本地磁盘 (D:)", "drive"),
            ("E:\\", "本地磁盘 (E:)", "drive"),
        ];

        for (p, name, icon) in drive_candidates {
            let path = PathBuf::from(p);
            if path.exists() {
                list.push(SystemPresetDir {
                    id: format!("win_{}", p.replace([':', '\\'], "_")),
                    name: name.to_string(),
                    path: p.to_string(),
                    icon_type: icon.to_string(),
                    exists: true,
                });
            }
        }
    }

    list
}

