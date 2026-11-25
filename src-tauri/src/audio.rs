use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStatusPayload {
    pub is_playing: bool,
    pub current_time: f64,
    pub duration: f64,
    pub volume: f32,
    pub speed: f32,
}

pub enum AudioCommand {
    Play(String), // File path
    Pause,
    Resume,
    Stop,
    Seek(f64), // Seconds
    SetVolume(f32),
    SetSpeed(f32),
}

pub struct AudioPlayerManager {
    cmd_sender: Mutex<Sender<AudioCommand>>,
    status: Arc<Mutex<AudioStatusPayload>>,
}

impl AudioPlayerManager {
    pub fn new(app_handle: AppHandle) -> Self {
        let (tx, rx) = channel::<AudioCommand>();
        let status = Arc::new(Mutex::new(AudioStatusPayload {
            is_playing: false,
            current_time: 0.0,
            duration: 0.0,
            volume: 1.0,
            speed: 1.0,
        }));

        let status_clone = Arc::clone(&status);
        thread::spawn(move || {
            audio_worker_thread(rx, status_clone, app_handle);
        });

        Self {
            cmd_sender: Mutex::new(tx),
            status,
        }
    }

    pub fn send(&self, cmd: AudioCommand) -> Result<(), String> {
        let sender = self.cmd_sender.lock().map_err(|e| e.to_string())?;
        sender.send(cmd).map_err(|e| e.to_string())
    }

    pub fn get_status(&self) -> Result<AudioStatusPayload, String> {
        let st = self.status.lock().map_err(|e| e.to_string())?;
        Ok(st.clone())
    }
}

fn audio_worker_thread(
    rx: Receiver<AudioCommand>,
    status: Arc<Mutex<AudioStatusPayload>>,
    app_handle: AppHandle,
) {
    // Attempt to acquire default audio output sink
    let sink_handle_res = DeviceSinkBuilder::open_default_sink();
    let (sink_handle, player): (Option<MixerDeviceSink>, Option<Player>) = match sink_handle_res {
        Ok(handle) => {
            let p = Player::connect_new(&handle.mixer());
            (Some(handle), Some(p))
        }
        Err(e) => {
            eprintln!("[Audio Engine] Failed to open default audio stream: {}", e);
            (None, None)
        }
    };

    let mut total_duration = 0.0;
    let mut volume = 1.0f32;
    let mut speed = 1.0f32;
    let mut track_loaded = false;

    loop {
        // Drain any incoming commands
        while let Ok(cmd) = rx.try_recv() {
            match cmd {
                AudioCommand::Play(path) => {
                    if let Some(ref p) = player {
                        p.clear();
                        if let Ok(file) = File::open(&path) {
                            let len = file.metadata().map(|m| m.len()).unwrap_or(0);
                            let decoder_res = Decoder::builder()
                                .with_data(file)
                                .with_byte_len(len)
                                .with_seekable(true)
                                .build();

                            match decoder_res {
                                Ok(source) => {
                                    total_duration = source
                                        .total_duration()
                                        .map(|d| d.as_secs_f64())
                                        .unwrap_or(0.0);
                                    p.append(source);
                                    p.set_volume(volume);
                                    p.set_speed(speed);
                                    p.play();
                                    track_loaded = true;

                                    if let Ok(mut st) = status.lock() {
                                        st.is_playing = true;
                                        st.duration = total_duration;
                                        st.current_time = 0.0;
                                    }
                                }
                                Err(err) => {
                                    eprintln!("[Audio Engine] Decoder build error for '{}': {}", path, err);
                                }
                            }
                        } else {
                            eprintln!("[Audio Engine] Failed to open file: {}", path);
                        }
                    }
                }
                AudioCommand::Pause => {
                    if let Some(ref p) = player {
                        p.pause();
                        if let Ok(mut st) = status.lock() {
                            st.is_playing = false;
                        }
                    }
                }
                AudioCommand::Resume => {
                    if let Some(ref p) = player {
                        p.play();
                        if let Ok(mut st) = status.lock() {
                            st.is_playing = true;
                        }
                    }
                }
                AudioCommand::Stop => {
                    if let Some(ref p) = player {
                        p.clear();
                        track_loaded = false;
                        if let Ok(mut st) = status.lock() {
                            st.is_playing = false;
                            st.current_time = 0.0;
                        }
                    }
                }
                AudioCommand::Seek(pos) => {
                    if let Some(ref p) = player {
                        let dur = Duration::from_secs_f64(pos.max(0.0));
                        let _ = p.try_seek(dur);
                        if let Ok(mut st) = status.lock() {
                            st.current_time = pos;
                        }
                    }
                }
                AudioCommand::SetVolume(vol) => {
                    volume = vol.clamp(0.0, 1.0);
                    if let Some(ref p) = player {
                        p.set_volume(volume);
                    }
                    if let Ok(mut st) = status.lock() {
                        st.volume = volume;
                    }
                }
                AudioCommand::SetSpeed(spd) => {
                    speed = spd.clamp(0.5, 2.0);
                    if let Some(ref p) = player {
                        p.set_speed(speed);
                    }
                    if let Ok(mut st) = status.lock() {
                        st.speed = speed;
                    }
                }
            }
        }

        // Periodic ticker and state sync
        let mut emit_tick = false;
        let mut has_ended = false;

        if let Some(ref p) = player {
            if track_loaded {
                let is_paused = p.is_paused();
                let is_empty = p.empty();

                if is_empty && !is_paused {
                    track_loaded = false;
                    has_ended = true;
                    if let Ok(mut st) = status.lock() {
                        st.is_playing = false;
                        st.current_time = total_duration;
                    }
                } else if !is_paused {
                    let pos = p.get_pos().as_secs_f64();
                    if let Ok(mut st) = status.lock() {
                        st.is_playing = true;
                        st.current_time = pos;
                        emit_tick = true;
                    }
                }
            }
        }

        if emit_tick {
            if let Ok(st) = status.lock() {
                let _ = app_handle.emit("player:status", st.clone());
            }
        }

        if has_ended {
            let _ = app_handle.emit("player:ended", ());
        }

        // Keep sink_handle referenced so it doesn't get dropped
        let _ = &sink_handle;

        thread::sleep(Duration::from_millis(200));
    }
}
