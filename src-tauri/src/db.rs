use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackRecord {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub path: String,
    pub format: String,
    pub bitrate: Option<u32>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    pub cover: Option<String>,
    pub is_favorite: bool,
    pub play_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumDto {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub cover: Option<String>,
    pub year: Option<u32>,
    pub tracks_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistDto {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub tracks_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistRecord {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover: Option<String>,
    pub track_ids: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct DbManager {
    conn: Mutex<Connection>,
}

impl DbManager {
    pub fn new() -> Result<Self, String> {
        let db_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("TimeMusic");
        let _ = fs::create_dir_all(&db_dir);
        let db_path = db_dir.join("library.db");

        let conn = Connection::open(db_path).map_err(|e| e.to_string())?;

        // SQLite High-Performance Tuning: WAL mode, memory cache, fast sync
        let _ = conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -64000;
             PRAGMA temp_store = MEMORY;
             PRAGMA mmap_size = 268435456;"
        );

        // 1. Tracks table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tracks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                artist TEXT NOT NULL,
                album TEXT NOT NULL,
                duration REAL NOT NULL,
                path TEXT NOT NULL UNIQUE,
                format TEXT NOT NULL,
                bitrate INTEGER,
                year INTEGER,
                genre TEXT,
                cover TEXT,
                is_favorite INTEGER NOT NULL DEFAULT 0,
                play_count INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL
            );",
            [],
        )
        .map_err(|e| e.to_string())?;

        // 2. Scan directories table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS scan_directories (
                path TEXT PRIMARY KEY,
                last_scanned INTEGER NOT NULL
            );",
            [],
        )
        .map_err(|e| e.to_string())?;

        // 3. Playlists table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS playlists (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                cover TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );",
            [],
        )
        .map_err(|e| e.to_string())?;

        // 4. Playlist tracks junction table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS playlist_tracks (
                playlist_id TEXT NOT NULL,
                track_id TEXT NOT NULL,
                position INTEGER NOT NULL,
                PRIMARY KEY (playlist_id, track_id)
            );",
            [],
        )
        .map_err(|e| e.to_string())?;

        // 5. Play history table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS play_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                track_id TEXT NOT NULL,
                played_at INTEGER NOT NULL
            );",
            [],
        )
        .map_err(|e| e.to_string())?;

        // High-Performance Query Indexes
        let _ = conn.execute_batch(
            "CREATE INDEX IF NOT EXISTS idx_tracks_artist ON tracks(artist);
             CREATE INDEX IF NOT EXISTS idx_tracks_album ON tracks(album);
             CREATE INDEX IF NOT EXISTS idx_tracks_favorite ON tracks(is_favorite);
             CREATE INDEX IF NOT EXISTS idx_tracks_play_count ON tracks(play_count DESC);
             CREATE INDEX IF NOT EXISTS idx_play_history_played_at ON play_history(played_at DESC);
             CREATE INDEX IF NOT EXISTS idx_playlist_tracks_pos ON playlist_tracks(playlist_id, position);"
        );

        // Seed default playlists if empty
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM playlists", [], |r| r.get(0))
            .unwrap_or(0);
        if count == 0 {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;

            let _ = conn.execute(
                "INSERT INTO playlists (id, name, description, cover, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    "pl-favorites",
                    "我喜欢的音乐",
                    "收藏的所有心动旋律",
                    "https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=500&auto=format&fit=crop&q=80",
                    now,
                    now
                ],
            );

            let _ = conn.execute(
                "INSERT INTO playlists (id, name, description, cover, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    "pl-ambient",
                    "Time OS 极光氛围",
                    "沉浸代码与专注时刻的太空氛围原声",
                    "https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=500&auto=format&fit=crop&q=80",
                    now,
                    now
                ],
            );
        }

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn upsert_track(&self, t: &TrackRecord) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO tracks (id, title, artist, album, duration, path, format, bitrate, year, genre, cover, is_favorite, play_count, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
             ON CONFLICT(path) DO UPDATE SET
                title = excluded.title,
                artist = excluded.artist,
                album = excluded.album,
                duration = excluded.duration,
                format = excluded.format,
                bitrate = excluded.bitrate,
                year = excluded.year,
                genre = excluded.genre,
                cover = COALESCE(excluded.cover, tracks.cover);",
            params![
                t.id,
                t.title,
                t.artist,
                t.album,
                t.duration,
                t.path,
                t.format,
                t.bitrate,
                t.year,
                t.genre,
                t.cover,
                if t.is_favorite { 1 } else { 0 },
                t.play_count,
                now,
            ],
        )
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_all_tracks(&self) -> Result<Vec<TrackRecord>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT id, title, artist, album, duration, path, format, bitrate, year, genre, cover, is_favorite, play_count
                 FROM tracks ORDER BY title COLLATE NOCASE ASC",
            )
            .map_err(|e| e.to_string())?;

        let track_iter = stmt
            .query_map([], |row| {
                let fav_int: i32 = row.get(11)?;
                Ok(TrackRecord {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    artist: row.get(2)?,
                    album: row.get(3)?,
                    duration: row.get(4)?,
                    path: row.get(5)?,
                    format: row.get(6)?,
                    bitrate: row.get(7)?,
                    year: row.get(8)?,
                    genre: row.get(9)?,
                    cover: row.get(10)?,
                    is_favorite: fav_int != 0,
                    play_count: row.get(12)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut list = Vec::new();
        for t in track_iter {
            if let Ok(record) = t {
                list.push(record);
            }
        }
        Ok(list)
    }

    pub fn toggle_favorite(&self, track_id: &str) -> Result<bool, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT is_favorite FROM tracks WHERE id = ?1")
            .map_err(|e| e.to_string())?;

        let current_fav: Result<i32, _> = stmt
            .query_row(params![track_id], |row| row.get(0));

        let new_fav = match current_fav {
            Ok(fav) => {
                let n = if fav == 0 { 1 } else { 0 };
                let _ = conn.execute(
                    "UPDATE tracks SET is_favorite = ?1 WHERE id = ?2",
                    params![n, track_id],
                );
                n != 0
            }
            Err(_) => {
                let is_in_fav: i64 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM playlist_tracks WHERE playlist_id = 'pl-favorites' AND track_id = ?1",
                        params![track_id],
                        |r| r.get(0),
                    )
                    .unwrap_or(0);
                is_in_fav == 0
            }
        };

        if new_fav {
            let next_pos: i64 = conn
                .query_row(
                    "SELECT COALESCE(MAX(position), 0) + 1 FROM playlist_tracks WHERE playlist_id = 'pl-favorites'",
                    [],
                    |r| r.get(0),
                )
                .unwrap_or(1);
            let _ = conn.execute(
                "INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position) VALUES ('pl-favorites', ?1, ?2)",
                params![track_id, next_pos],
            );
        } else {
            let _ = conn.execute(
                "DELETE FROM playlist_tracks WHERE playlist_id = 'pl-favorites' AND track_id = ?1",
                params![track_id],
            );
        }

        Ok(new_fav)
    }

    pub fn update_track_metadata(
        &self,
        track_id: &str,
        title: &str,
        artist: &str,
        album: &str,
    ) -> Result<String, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT path FROM tracks WHERE id = ?1")
            .map_err(|e| e.to_string())?;

        let path: String = stmt
            .query_row(params![track_id], |row| row.get(0))
            .map_err(|e| e.to_string())?;

        conn.execute(
            "UPDATE tracks SET title = ?1, artist = ?2, album = ?3 WHERE id = ?4",
            params![title, artist, album, track_id],
        )
        .map_err(|e| e.to_string())?;

        Ok(path)
    }

    // --- Playlist Operations ---
    pub fn create_playlist(
        &self,
        name: &str,
        description: Option<String>,
        cover: Option<String>,
    ) -> Result<PlaylistRecord, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        let id = format!("pl-{}", now);

        conn.execute(
            "INSERT INTO playlists (id, name, description, cover, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&id, name, &description, &cover, now, now],
        )
        .map_err(|e| e.to_string())?;

        Ok(PlaylistRecord {
            id,
            name: name.to_string(),
            description,
            cover,
            track_ids: Vec::new(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn update_playlist(
        &self,
        id: &str,
        name: &str,
        description: Option<String>,
        cover: Option<String>,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        conn.execute(
            "UPDATE playlists SET name = ?1, description = ?2, cover = ?3, updated_at = ?4 WHERE id = ?5",
            params![name, &description, &cover, now, id],
        )
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_playlists(&self) -> Result<Vec<PlaylistRecord>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, name, description, cover, created_at, updated_at FROM playlists ORDER BY created_at ASC")
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                ))
            })
            .map_err(|e| e.to_string())?;

        let mut playlists = Vec::new();
        for r in rows {
            if let Ok((id, name, description, cover, created_at, updated_at)) = r {
                // Fetch associated track IDs
                let mut tr_stmt = conn
                    .prepare("SELECT track_id FROM playlist_tracks WHERE playlist_id = ?1 ORDER BY position ASC")
                    .map_err(|e| e.to_string())?;
                let tr_iter = tr_stmt
                    .query_map(params![&id], |t_row| t_row.get::<_, String>(0))
                    .map_err(|e| e.to_string())?;

                let mut track_ids = Vec::new();
                for t_id in tr_iter {
                    if let Ok(tid) = t_id {
                        track_ids.push(tid);
                    }
                }

                playlists.push(PlaylistRecord {
                    id,
                    name,
                    description,
                    cover,
                    track_ids,
                    created_at,
                    updated_at,
                });
            }
        }

        Ok(playlists)
    }

    pub fn delete_playlist(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM playlist_tracks WHERE playlist_id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM playlists WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn add_track_to_playlist(&self, playlist_id: &str, track_id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let next_pos: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(position), 0) + 1 FROM playlist_tracks WHERE playlist_id = ?1",
                params![playlist_id],
                |r| r.get(0),
            )
            .unwrap_or(1);

        conn.execute(
            "INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position) VALUES (?1, ?2, ?3)",
            params![playlist_id, track_id, next_pos],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn remove_track_from_playlist(&self, playlist_id: &str, track_id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "DELETE FROM playlist_tracks WHERE playlist_id = ?1 AND track_id = ?2",
            params![playlist_id, track_id],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    // --- Play History & Recommendations ---
    pub fn record_play_history(&self, track_id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO play_history (track_id, played_at) VALUES (?1, ?2)",
            params![track_id, now],
        )
        .map_err(|e| e.to_string())?;

        conn.execute(
            "UPDATE tracks SET play_count = play_count + 1 WHERE id = ?1",
            params![track_id],
        )
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_recent_played(&self, limit: u32) -> Result<Vec<TrackRecord>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT DISTINCT t.id, t.title, t.artist, t.album, t.duration, t.path, t.format, t.bitrate, t.year, t.genre, t.cover, t.is_favorite, t.play_count
                 FROM play_history h
                 JOIN tracks t ON h.track_id = t.id
                 ORDER BY h.played_at DESC
                 LIMIT ?1",
            )
            .map_err(|e| e.to_string())?;

        let track_iter = stmt
            .query_map(params![limit], |row| {
                let fav_int: i32 = row.get(11)?;
                Ok(TrackRecord {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    artist: row.get(2)?,
                    album: row.get(3)?,
                    duration: row.get(4)?,
                    path: row.get(5)?,
                    format: row.get(6)?,
                    bitrate: row.get(7)?,
                    year: row.get(8)?,
                    genre: row.get(9)?,
                    cover: row.get(10)?,
                    is_favorite: fav_int != 0,
                    play_count: row.get(12)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut list = Vec::new();
        for t in track_iter {
            if let Ok(rec) = t {
                list.push(rec);
            }
        }
        Ok(list)
    }

    pub fn get_daily_recommendations(&self, limit: u32) -> Result<Vec<TrackRecord>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        // Recommendation logic (scoring: favorite * 10 + play_count * 2 + random jitter)
        let mut stmt = conn
            .prepare(
                "SELECT id, title, artist, album, duration, path, format, bitrate, year, genre, cover, is_favorite, play_count
                 FROM tracks
                 ORDER BY (is_favorite * 10 + play_count * 2 + (RANDOM() % 5)) DESC
                 LIMIT ?1",
            )
            .map_err(|e| e.to_string())?;

        let track_iter = stmt
            .query_map(params![limit], |row| {
                let fav_int: i32 = row.get(11)?;
                Ok(TrackRecord {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    artist: row.get(2)?,
                    album: row.get(3)?,
                    duration: row.get(4)?,
                    path: row.get(5)?,
                    format: row.get(6)?,
                    bitrate: row.get(7)?,
                    year: row.get(8)?,
                    genre: row.get(9)?,
                    cover: row.get(10)?,
                    is_favorite: fav_int != 0,
                    play_count: row.get(12)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut list = Vec::new();
        for t in track_iter {
            if let Ok(rec) = t {
                list.push(rec);
            }
        }
        Ok(list)
    }
}
