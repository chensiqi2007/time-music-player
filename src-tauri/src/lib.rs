pub mod ai;
pub mod applemusic;
pub mod audio;
pub mod db;
pub mod importer;
pub mod kugou;
pub mod kuwo;
pub mod netease;
pub mod passport;
pub mod qishui;
pub mod qqmusic;
pub mod scanner;
pub mod storage;
pub mod ytmusic;

use ai::{AiClient, TestConnectionResult};
use applemusic::AppleMusicClient;
use audio::{AudioCommand, AudioPlayerManager, AudioStatusPayload};
use db::{DbManager, PlaylistRecord, TrackRecord};
use importer::{ImportedPlaylistPreview, PlaylistImporter};
use kugou::{KugouClient, KugouLyricResult};
use kuwo::{KuwoClient, KuwoLyricResult};
use netease::{
    NeteaseClient, NeteaseDiscoverPlaylist, NeteaseLyricData, NeteasePlaylistDetail,
    NeteaseToplistItem, NeteaseUserProfile, QrCheckResult, QrKeyResult,
};
use passport::{PassportAuthResult, PassportClient, PassportLoginStart, PassportUserProfile};
use qishui::{QishuiClient, QishuiLyricResult};
use qqmusic::{
    QQMusicClient, QQMusicLyricResult, QQPlaylistDetail, QQPlaylistSummary, QQQrCheckResult,
    QQQrCodeResult, QQUserProfile,
};
use scanner::{ScanResult, scan_directory};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{Manager, State, Window};
use ytmusic::{
    YTDeviceCheckResult, YTDeviceCodeResult, YTPlaylistDetail, YTPlaylistSummary, YTUserProfile,
    YouTubeMusicClient,
};

#[tauri::command]
fn minimize_window(window: Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_maximize(window: Window) -> Result<bool, String> {
    let is_max = window.is_maximized().map_err(|e| e.to_string())?;
    if is_max {
        window.unmaximize().map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        window.maximize().map_err(|e| e.to_string())?;
        Ok(true)
    }
}

#[tauri::command]
fn close_window(window: Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
fn is_maximized(window: Window) -> Result<bool, String> {
    window.is_maximized().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_default_music_dirs() -> Vec<String> {
    let mut dirs_list = Vec::new();

    if let Some(user_music) = dirs::audio_dir() {
        if let Some(path_str) = user_music.to_str() {
            dirs_list.push(path_str.to_string());
        }
    }

    #[cfg(target_os = "windows")]
    {
        let common_paths = ["D:\\Music", "E:\\Music", "D:\\音乐", "E:\\音乐"];
        for p in common_paths {
            let path = PathBuf::from(p);
            if path.exists() {
                dirs_list.push(p.to_string());
            }
        }
    }

    dirs_list
}

// Audio Engine Commands
#[tauri::command]
fn audio_play(path: String, state: State<'_, Arc<AudioPlayerManager>>) -> Result<(), String> {
    state.send(AudioCommand::Play(path))
}

#[tauri::command]
fn audio_pause(state: State<'_, Arc<AudioPlayerManager>>) -> Result<(), String> {
    state.send(AudioCommand::Pause)
}

#[tauri::command]
fn audio_resume(state: State<'_, Arc<AudioPlayerManager>>) -> Result<(), String> {
    state.send(AudioCommand::Resume)
}

#[tauri::command]
fn audio_stop(state: State<'_, Arc<AudioPlayerManager>>) -> Result<(), String> {
    state.send(AudioCommand::Stop)
}

#[tauri::command]
fn audio_seek(position_secs: f64, state: State<'_, Arc<AudioPlayerManager>>) -> Result<(), String> {
    state.send(AudioCommand::Seek(position_secs))
}

#[tauri::command]
fn audio_set_volume(volume: f32, state: State<'_, Arc<AudioPlayerManager>>) -> Result<(), String> {
    state.send(AudioCommand::SetVolume(volume))
}

#[tauri::command]
fn audio_set_speed(speed: f32, state: State<'_, Arc<AudioPlayerManager>>) -> Result<(), String> {
    state.send(AudioCommand::SetSpeed(speed))
}

#[tauri::command]
fn audio_get_status(
    state: State<'_, Arc<AudioPlayerManager>>,
) -> Result<AudioStatusPayload, String> {
    state.get_status()
}

// Database & Scanner Commands
#[tauri::command]
fn scan_music_directories(
    dirs: Vec<String>,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<ScanResult, String> {
    let mut total_found = 0;
    let mut successfully_added = 0;
    let mut errors = 0;

    for dir in dirs {
        let res = scan_directory(&dir, Arc::clone(&db_state));
        total_found += res.total_found;
        successfully_added += res.successfully_added;
        errors += res.errors;
    }

    Ok(ScanResult {
        total_found,
        successfully_added,
        errors,
    })
}

#[tauri::command]
fn get_local_tracks(db_state: State<'_, Arc<DbManager>>) -> Result<Vec<TrackRecord>, String> {
    db_state.get_all_tracks()
}

#[tauri::command]
fn toggle_track_favorite(
    track_id: String,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<bool, String> {
    db_state.toggle_favorite(&track_id)
}

#[tauri::command]
fn update_track_tags(
    track_id: String,
    title: String,
    artist: String,
    album: String,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<(), String> {
    let path = db_state.update_track_metadata(&track_id, &title, &artist, &album)?;
    let _ = scanner::save_audio_metadata(&path, &title, &artist, &album);
    Ok(())
}

// Playlist Commands
#[tauri::command]
fn create_playlist(
    name: String,
    description: Option<String>,
    cover: Option<String>,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<PlaylistRecord, String> {
    db_state.create_playlist(&name, description, cover)
}

#[tauri::command]
fn update_playlist(
    id: String,
    name: String,
    description: Option<String>,
    cover: Option<String>,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<(), String> {
    db_state.update_playlist(&id, &name, description, cover)
}

#[tauri::command]
fn get_playlists(db_state: State<'_, Arc<DbManager>>) -> Result<Vec<PlaylistRecord>, String> {
    db_state.get_playlists()
}

#[tauri::command]
fn delete_playlist(id: String, db_state: State<'_, Arc<DbManager>>) -> Result<(), String> {
    db_state.delete_playlist(&id)
}

#[tauri::command]
fn add_track_to_playlist(
    playlist_id: String,
    track_id: String,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<(), String> {
    db_state.add_track_to_playlist(&playlist_id, &track_id)
}

#[tauri::command]
fn remove_track_from_playlist(
    playlist_id: String,
    track_id: String,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<(), String> {
    db_state.remove_track_from_playlist(&playlist_id, &track_id)
}

// History & Recommendation Commands
#[tauri::command]
fn record_play_history(
    track_id: String,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<(), String> {
    db_state.record_play_history(&track_id)
}

#[tauri::command]
fn get_recent_played(
    limit: u32,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<Vec<TrackRecord>, String> {
    db_state.get_recent_played(limit)
}

#[tauri::command]
fn get_daily_recommendations(
    limit: u32,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<Vec<TrackRecord>, String> {
    db_state.get_daily_recommendations(limit)
}

// --- NetEase Cloud Music Commands ---
#[tauri::command]
async fn netease_get_qr_key(state: State<'_, Arc<NeteaseClient>>) -> Result<QrKeyResult, String> {
    state.get_qr_key().await
}

#[tauri::command]
async fn netease_check_qr(
    key: String,
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<QrCheckResult, String> {
    state.check_qr(&key).await
}

#[tauri::command]
async fn netease_get_user_profile(
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<NeteaseUserProfile, String> {
    state.get_user_profile().await
}

#[tauri::command]
async fn netease_get_song_url(
    song_id: String,
    br: Option<u32>,
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<Option<String>, String> {
    state.get_song_url(&song_id, br).await
}

#[tauri::command]
async fn netease_get_lyric(
    song_id: String,
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<NeteaseLyricData, String> {
    state.get_lyric(&song_id).await
}

#[tauri::command]
async fn netease_search(
    keyword: String,
    limit: Option<u32>,
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<Vec<TrackRecord>, String> {
    state.search_songs(&keyword, limit).await
}

#[tauri::command]
async fn netease_get_playlist(
    playlist_id: String,
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<NeteasePlaylistDetail, String> {
    state.get_playlist(&playlist_id).await
}

#[tauri::command]
async fn netease_get_user_playlists(
    uid: String,
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<Vec<PlaylistRecord>, String> {
    state.get_user_playlists(&uid).await
}

#[tauri::command]
fn netease_set_cookie(cookie: String, state: State<'_, Arc<NeteaseClient>>) -> Result<(), String> {
    state.set_cookie(cookie);
    Ok(())
}

#[tauri::command]
fn netease_logout(state: State<'_, Arc<NeteaseClient>>) -> Result<(), String> {
    state.set_cookie("".to_string());
    Ok(())
}

#[tauri::command]
async fn netease_get_discover_playlists(
    cat: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<Vec<NeteaseDiscoverPlaylist>, String> {
    state
        .get_discover_playlists(cat.as_deref(), limit, offset)
        .await
}

#[tauri::command]
async fn netease_get_daily_songs(
    limit: Option<u32>,
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<Vec<TrackRecord>, String> {
    state.get_daily_songs(limit).await
}

#[tauri::command]
async fn netease_get_toplists(
    state: State<'_, Arc<NeteaseClient>>,
) -> Result<Vec<NeteaseToplistItem>, String> {
    state.get_toplists().await
}

// --- QQ Music Commands ---
#[tauri::command]
async fn qqmusic_search(
    keyword: String,
    limit: Option<u32>,
    state: State<'_, Arc<QQMusicClient>>,
) -> Result<Vec<TrackRecord>, String> {
    state.search_songs(&keyword, limit).await
}

#[tauri::command]
async fn qqmusic_get_song_url(
    songmid: String,
    state: State<'_, Arc<QQMusicClient>>,
) -> Result<Option<String>, String> {
    state.get_song_url(&songmid).await
}

#[tauri::command]
async fn qqmusic_get_lyric(
    songmid: String,
    state: State<'_, Arc<QQMusicClient>>,
) -> Result<QQMusicLyricResult, String> {
    state.get_lyric(&songmid).await
}

// --- QQ Music Login & Playlist Commands ---
#[tauri::command]
async fn qqmusic_get_qr_key(
    state: State<'_, Arc<QQMusicClient>>,
) -> Result<QQQrCodeResult, String> {
    state.get_qr_key().await
}

#[tauri::command]
async fn qqmusic_check_qr(
    qrsig: String,
    state: State<'_, Arc<QQMusicClient>>,
) -> Result<QQQrCheckResult, String> {
    state.check_qr(&qrsig).await
}

#[tauri::command]
async fn qqmusic_get_user_profile(
    uin: String,
    state: State<'_, Arc<QQMusicClient>>,
) -> Result<QQUserProfile, String> {
    state.get_user_profile(&uin).await
}

#[tauri::command]
async fn qqmusic_get_user_playlists(
    uin: String,
    state: State<'_, Arc<QQMusicClient>>,
) -> Result<Vec<QQPlaylistSummary>, String> {
    state.get_user_playlists(&uin).await
}

#[tauri::command]
async fn qqmusic_get_playlist(
    disstid: String,
    state: State<'_, Arc<QQMusicClient>>,
) -> Result<QQPlaylistDetail, String> {
    state.get_playlist(&disstid).await
}

#[tauri::command]
fn qqmusic_set_cookie(cookie: String, state: State<'_, Arc<QQMusicClient>>) -> Result<(), String> {
    state.set_cookie(cookie);
    Ok(())
}

#[tauri::command]
fn qqmusic_logout(state: State<'_, Arc<QQMusicClient>>) -> Result<(), String> {
    state.logout();
    Ok(())
}

// --- External Playlist Importer Commands ---
#[tauri::command]
async fn parse_external_playlist_link(
    url: String,
    importer: State<'_, Arc<PlaylistImporter>>,
    netease_client: State<'_, Arc<NeteaseClient>>,
    qqmusic_client: State<'_, Arc<QQMusicClient>>,
) -> Result<ImportedPlaylistPreview, String> {
    importer
        .parse_link(&url, &netease_client, &qqmusic_client)
        .await
}

#[tauri::command]
fn confirm_import_external_playlist(
    preview: ImportedPlaylistPreview,
    importer: State<'_, Arc<PlaylistImporter>>,
    db_state: State<'_, Arc<DbManager>>,
) -> Result<PlaylistRecord, String> {
    importer.save_imported_playlist(preview, &db_state)
}

// --- Apple Music Commands ---
#[tauri::command]
async fn applemusic_search(
    keyword: String,
    limit: Option<u32>,
    state: State<'_, Arc<AppleMusicClient>>,
) -> Result<Vec<TrackRecord>, String> {
    state.search_songs(&keyword, limit).await
}

#[tauri::command]
async fn applemusic_get_song_url(
    track_id: String,
    state: State<'_, Arc<AppleMusicClient>>,
) -> Result<Option<String>, String> {
    state.get_song_url(&track_id).await
}

// --- YouTube Music Commands ---
#[tauri::command]
async fn ytmusic_search(
    keyword: String,
    limit: Option<u32>,
    state: State<'_, Arc<YouTubeMusicClient>>,
) -> Result<Vec<TrackRecord>, String> {
    state.search_songs(&keyword, limit).await
}

#[tauri::command]
async fn ytmusic_get_song_url(
    video_id: String,
    state: State<'_, Arc<YouTubeMusicClient>>,
) -> Result<Option<String>, String> {
    state.get_song_url(&video_id).await
}

#[tauri::command]
async fn ytmusic_get_device_code(
    state: State<'_, Arc<YouTubeMusicClient>>,
) -> Result<YTDeviceCodeResult, String> {
    state.get_device_code().await
}

#[tauri::command]
async fn ytmusic_check_device_code(
    device_code: String,
    state: State<'_, Arc<YouTubeMusicClient>>,
) -> Result<YTDeviceCheckResult, String> {
    state.check_device_code(&device_code).await
}

#[tauri::command]
fn ytmusic_set_cookie(
    cookie: String,
    state: State<'_, Arc<YouTubeMusicClient>>,
) -> Result<(), String> {
    state.set_cookie(cookie);
    Ok(())
}

#[tauri::command]
fn ytmusic_set_token(
    token: String,
    state: State<'_, Arc<YouTubeMusicClient>>,
) -> Result<(), String> {
    state.set_token(token);
    Ok(())
}

#[tauri::command]
async fn ytmusic_get_user_profile(
    state: State<'_, Arc<YouTubeMusicClient>>,
) -> Result<YTUserProfile, String> {
    state.get_user_profile().await
}

#[tauri::command]
async fn ytmusic_get_user_playlists(
    state: State<'_, Arc<YouTubeMusicClient>>,
) -> Result<Vec<YTPlaylistSummary>, String> {
    state.get_user_playlists().await
}

#[tauri::command]
async fn ytmusic_get_playlist(
    playlist_id: String,
    state: State<'_, Arc<YouTubeMusicClient>>,
) -> Result<YTPlaylistDetail, String> {
    state.get_playlist(&playlist_id).await
}

#[tauri::command]
fn ytmusic_logout(state: State<'_, Arc<YouTubeMusicClient>>) -> Result<(), String> {
    state.logout();
    Ok(())
}

// --- AI Commands ---
#[tauri::command]
async fn ai_chat_completion(
    base_url: String,
    api_key: String,
    model: String,
    system_prompt: String,
    user_prompt: String,
    state: State<'_, Arc<AiClient>>,
) -> Result<String, String> {
    state
        .chat_completion(&base_url, &api_key, &model, &system_prompt, &user_prompt)
        .await
}

#[tauri::command]
async fn ai_test_connection(
    base_url: String,
    api_key: String,
    model: String,
    state: State<'_, Arc<AiClient>>,
) -> Result<TestConnectionResult, String> {
    state.test_connection(&base_url, &api_key, &model).await
}

// Storage & Directory Picker Commands
#[tauri::command]
async fn pick_folder(default_path: Option<String>) -> Result<Option<String>, String> {
    let mut dialog = rfd::AsyncFileDialog::new().set_title("选择文件夹");
    if let Some(ref p) = default_path {
        let trimmed = p.trim();
        if !trimmed.is_empty() {
            dialog = dialog.set_directory(trimmed);
        }
    }
    let folder = dialog.pick_folder().await;
    Ok(folder.map(|f| f.path().to_string_lossy().to_string()))
}

#[tauri::command]
fn get_cache_info(custom_path: Option<String>) -> Result<storage::CacheInfo, String> {
    let path = storage::resolve_cache_dir(custom_path);
    Ok(storage::inspect_cache(&path))
}

#[tauri::command]
fn clear_cache(custom_path: Option<String>) -> Result<u64, String> {
    let path = storage::resolve_cache_dir(custom_path);
    storage::clear_cache_directory(&path)
}

#[tauri::command]
fn get_preset_directories() -> Result<Vec<storage::SystemPresetDir>, String> {
    Ok(storage::get_system_preset_dirs())
}

#[tauri::command]
fn open_folder_in_explorer(path: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if !p.exists() {
        let _ = std::fs::create_dir_all(&p);
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("explorer").arg(&path).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&path).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(&path).spawn();
    }
    Ok(())
}

// URL Time Passport SSO Commands
#[tauri::command]
async fn passport_start_login(
    app: tauri::AppHandle,
    state: State<'_, Arc<PassportClient>>,
) -> Result<PassportLoginStart, String> {
    state.start_login(app).await
}

#[tauri::command]
async fn passport_exchange_code(
    code: String,
    redirect_uri: Option<String>,
    client_secret: Option<String>,
    state: State<'_, Arc<PassportClient>>,
) -> Result<PassportAuthResult, String> {
    state
        .exchange_code(&code, redirect_uri, client_secret)
        .await
}

#[tauri::command]
async fn passport_get_user_info(
    token: String,
    state: State<'_, Arc<PassportClient>>,
) -> Result<PassportUserProfile, String> {
    state.get_user_info(&token).await
}

#[tauri::command]
async fn passport_refresh_token(
    refresh_token: String,
    state: State<'_, Arc<PassportClient>>,
) -> Result<PassportAuthResult, String> {
    state.refresh_token(&refresh_token).await
}

#[tauri::command]
fn passport_set_client_secret(
    secret: Option<String>,
    state: State<'_, Arc<PassportClient>>,
) -> Result<(), String> {
    state.set_client_secret(secret);
    Ok(())
}

#[tauri::command]
async fn passport_logout(
    token: Option<String>,
    state: State<'_, Arc<PassportClient>>,
) -> Result<(), String> {
    state.logout(token).await;
    Ok(())
}

// Kuwo Music Commands
#[tauri::command]
async fn kuwo_search(
    keyword: String,
    limit: Option<u32>,
    state: State<'_, Arc<KuwoClient>>,
) -> Result<Vec<TrackRecord>, String> {
    state.search_songs(&keyword, limit).await
}

#[tauri::command]
async fn kuwo_get_song_url(
    rid: String,
    state: State<'_, Arc<KuwoClient>>,
) -> Result<Option<String>, String> {
    state.get_song_url(&rid).await
}

#[tauri::command]
async fn kuwo_get_lyric(
    rid: String,
    state: State<'_, Arc<KuwoClient>>,
) -> Result<KuwoLyricResult, String> {
    state.get_lyric(&rid).await
}

#[tauri::command]
fn kuwo_set_cookie(cookie: String, state: State<'_, Arc<KuwoClient>>) -> Result<(), String> {
    state.set_cookie(cookie);
    Ok(())
}

#[tauri::command]
fn kuwo_logout(state: State<'_, Arc<KuwoClient>>) -> Result<(), String> {
    state.logout();
    Ok(())
}

// Kugou Music Commands
#[tauri::command]
async fn kugou_search(
    keyword: String,
    limit: Option<u32>,
    state: State<'_, Arc<KugouClient>>,
) -> Result<Vec<TrackRecord>, String> {
    state.search_songs(&keyword, limit).await
}

#[tauri::command]
async fn kugou_get_song_url(
    hash: String,
    state: State<'_, Arc<KugouClient>>,
) -> Result<Option<String>, String> {
    state.get_song_url(&hash).await
}

#[tauri::command]
async fn kugou_get_lyric(
    hash: String,
    keyword: Option<String>,
    state: State<'_, Arc<KugouClient>>,
) -> Result<KugouLyricResult, String> {
    state.get_lyric(&hash, keyword.as_deref()).await
}

#[tauri::command]
fn kugou_set_cookie(cookie: String, state: State<'_, Arc<KugouClient>>) -> Result<(), String> {
    state.set_cookie(cookie);
    Ok(())
}

#[tauri::command]
fn kugou_logout(state: State<'_, Arc<KugouClient>>) -> Result<(), String> {
    state.logout();
    Ok(())
}

// Qishui Music Commands
#[tauri::command]
async fn qishui_search(
    keyword: String,
    limit: Option<u32>,
    state: State<'_, Arc<QishuiClient>>,
) -> Result<Vec<TrackRecord>, String> {
    state.search_songs(&keyword, limit).await
}

#[tauri::command]
async fn qishui_get_song_url(
    track_id: String,
    state: State<'_, Arc<QishuiClient>>,
) -> Result<Option<String>, String> {
    state.get_song_url(&track_id).await
}

#[tauri::command]
async fn qishui_get_lyric(
    track_id: String,
    state: State<'_, Arc<QishuiClient>>,
) -> Result<QishuiLyricResult, String> {
    state.get_lyric(&track_id).await
}

#[tauri::command]
fn qishui_set_cookie(cookie: String, state: State<'_, Arc<QishuiClient>>) -> Result<(), String> {
    state.set_cookie(cookie);
    Ok(())
}

#[tauri::command]
fn qishui_logout(state: State<'_, Arc<QishuiClient>>) -> Result<(), String> {
    state.logout();
    Ok(())
}

#[tauri::command]
async fn open_platform_login_window(
    app: tauri::AppHandle,
    platform: String,
) -> Result<(), String> {
    let (label, title, target_url) = match platform.as_str() {
        "kuwo" => ("login-kuwo", "登录酷我音乐 (扫码 / 手机 / 密码)", "https://www.kuwo.cn/"),
        "kugou" => ("login-kugou", "登录酷狗音乐 (扫码 / 手机 / 密码)", "https://www.kugou.com/"),
        "qishui" => ("login-qishui", "登录汽水音乐 (抖音扫码 / 手机号)", "https://music.douyin.com/"),
        _ => return Err("不支持的平台".to_string()),
    };

    if let Some(existing) = app.get_webview_window(label) {
        let _ = existing.set_focus();
        return Ok(());
    }

    let parsed_url: tauri::Url = target_url.parse().map_err(|e| format!("URL 解析失败: {}", e))?;
    tauri::WebviewWindowBuilder::new(&app, label, tauri::WebviewUrl::External(parsed_url))
        .title(title)
        .inner_size(980.0, 680.0)
        .center()
        .build()
        .map_err(|e| format!("无法打开登录窗口: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn sync_platform_window_cookies(
    app: tauri::AppHandle,
    platform: String,
    kuwo_state: State<'_, Arc<KuwoClient>>,
    kugou_state: State<'_, Arc<KugouClient>>,
    qishui_state: State<'_, Arc<QishuiClient>>,
) -> Result<String, String> {
    let (label, target_url) = match platform.as_str() {
        "kuwo" => ("login-kuwo", "https://www.kuwo.cn/"),
        "kugou" => ("login-kugou", "https://www.kugou.com/"),
        "qishui" => ("login-qishui", "https://music.douyin.com/"),
        _ => return Err("不支持的平台".to_string()),
    };

    let win = app
        .get_webview_window(label)
        .ok_or_else(|| "请先点击打开登录窗口并完成登录".to_string())?;

    let parsed_url: tauri::Url = target_url.parse().map_err(|e| format!("URL 解析失败: {}", e))?;
    let cookies = win
        .cookies_for_url(parsed_url)
        .map_err(|e| format!("读取登录会话失败: {}", e))?;

    let cookie_str = cookies
        .iter()
        .map(|c| format!("{}={}", c.name(), c.value()))
        .collect::<Vec<_>>()
        .join("; ");

    if cookie_str.trim().is_empty() {
        return Err("尚未检测到有效登录 Cookie，请确认已在窗口内完成登录".to_string());
    }

    match platform.as_str() {
        "kuwo" => kuwo_state.set_cookie(cookie_str.clone()),
        "kugou" => kugou_state.set_cookie(cookie_str.clone()),
        "qishui" => qishui_state.set_cookie(cookie_str.clone()),
        _ => {}
    }

    let _ = win.close();
    Ok(cookie_str)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let audio_mgr = Arc::new(AudioPlayerManager::new(app.handle().clone()));
            let db_mgr = Arc::new(DbManager::new().expect("Failed to initialize SQLite database"));
            let netease_client = Arc::new(NeteaseClient::new());
            let qqmusic_client = Arc::new(QQMusicClient::new());
            let applemusic_client = Arc::new(AppleMusicClient::new());
            let ytmusic_client = Arc::new(YouTubeMusicClient::new());
            let importer = Arc::new(PlaylistImporter::new());
            let ai_client = Arc::new(AiClient::new());
            let passport_client = Arc::new(PassportClient::new());
            let kuwo_client = Arc::new(KuwoClient::new());
            let kugou_client = Arc::new(KugouClient::new());
            let qishui_client = Arc::new(QishuiClient::new());

            app.manage(audio_mgr);
            app.manage(db_mgr);
            app.manage(netease_client);
            app.manage(qqmusic_client);
            app.manage(applemusic_client);
            app.manage(ytmusic_client);
            app.manage(importer);
            app.manage(ai_client);
            app.manage(passport_client);
            app.manage(kuwo_client);
            app.manage(kugou_client);
            app.manage(qishui_client);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            minimize_window,
            toggle_maximize,
            close_window,
            is_maximized,
            get_default_music_dirs,
            audio_play,
            audio_pause,
            audio_resume,
            audio_stop,
            audio_seek,
            audio_set_volume,
            audio_set_speed,
            audio_get_status,
            scan_music_directories,
            get_local_tracks,
            toggle_track_favorite,
            update_track_tags,
            create_playlist,
            update_playlist,
            get_playlists,
            delete_playlist,
            add_track_to_playlist,
            remove_track_from_playlist,
            record_play_history,
            get_recent_played,
            get_daily_recommendations,
            netease_get_qr_key,
            netease_check_qr,
            netease_get_user_profile,
            netease_get_song_url,
            netease_get_lyric,
            netease_search,
            netease_get_playlist,
            netease_get_user_playlists,
            netease_set_cookie,
            netease_logout,
            netease_get_discover_playlists,
            netease_get_daily_songs,
            netease_get_toplists,
            qqmusic_search,
            qqmusic_get_song_url,
            qqmusic_get_lyric,
            qqmusic_get_qr_key,
            qqmusic_check_qr,
            qqmusic_get_user_profile,
            qqmusic_get_user_playlists,
            qqmusic_get_playlist,
            qqmusic_set_cookie,
            qqmusic_logout,
            applemusic_search,
            applemusic_get_song_url,
            ytmusic_search,
            ytmusic_get_song_url,
            ytmusic_get_device_code,
            ytmusic_check_device_code,
            ytmusic_set_cookie,
            ytmusic_set_token,
            ytmusic_get_user_profile,
            ytmusic_get_user_playlists,
            ytmusic_get_playlist,
            ytmusic_logout,
            parse_external_playlist_link,
            confirm_import_external_playlist,
            ai_chat_completion,
            ai_test_connection,
            pick_folder,
            get_cache_info,
            clear_cache,
            get_preset_directories,
            open_folder_in_explorer,
            passport_start_login,
            passport_exchange_code,
            passport_get_user_info,
            passport_refresh_token,
            passport_set_client_secret,
            passport_logout,
            kuwo_search,
            kuwo_get_song_url,
            kuwo_get_lyric,
            kuwo_set_cookie,
            kuwo_logout,
            kugou_search,
            kugou_get_song_url,
            kugou_get_lyric,
            kugou_set_cookie,
            kugou_logout,
            qishui_search,
            qishui_get_song_url,
            qishui_get_lyric,
            qishui_set_cookie,
            qishui_logout,
            open_platform_login_window,
            sync_platform_window_cookies
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
