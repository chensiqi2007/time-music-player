import { defineStore } from 'pinia';
import type { Track, PlaybackMode, LrcLine } from '@/types/music';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { parseLrc } from '@/utils/lrcParser';
import { useSettingsStore } from './settings';
import { useLibraryStore } from './library';
import { usePlaylistStore } from './playlist';

// Mock initial tracks for demonstration
export const SAMPLE_TRACKS: Track[] = [
  {
    id: 'demo-1',
    title: 'Time Journey (时光旅程)',
    artist: 'Time OS Studio',
    album: 'Time OS 1.0 Original Sounds',
    duration: 218,
    genre: 'Electronic / Ambient',
    year: 2026,
    format: 'FLAC 24bit/96kHz',
    bitrate: 1411,
    isFavorite: true,
    cover: 'https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'demo-2',
    title: 'Neon Horizon (霓虹地平线)',
    artist: 'Cyber Pulse',
    album: 'Synthwave Nightfall',
    duration: 185,
    genre: 'Synthwave',
    year: 2026,
    format: 'MP3 320kbps',
    bitrate: 320,
    isFavorite: false,
    cover: 'https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'demo-3',
    title: 'Rainy Cafe in Seattle',
    artist: 'Acoustic Dreams',
    album: 'Nordic Reflections',
    duration: 242,
    genre: 'Lo-Fi / Chill',
    year: 2025,
    format: 'FLAC',
    bitrate: 980,
    isFavorite: true,
    cover: 'https://images.unsplash.com/photo-1501386761578-eac5c94b800a?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'demo-4',
    title: 'Starlight Symphony',
    artist: 'Aurora Ensemble',
    album: 'Cosmic Harmonies',
    duration: 310,
    genre: 'Classical / Ambient',
    year: 2026,
    format: 'WAV 32bit',
    bitrate: 2304,
    isFavorite: false,
    cover: 'https://images.unsplash.com/photo-1451187580459-43490279c0fa?w=500&auto=format&fit=crop&q=80',
  },
];

export const SAMPLE_LYRICS: LrcLine[] = [
  { time: 0, text: '时光漫漫，代码在指尖流淌', translation: 'Time flows gently as code ripples through our fingers' },
  { time: 8, text: '在喧嚣的城市中寻找内心的宁静', translation: 'Finding inner serenity amidst the bustling city' },
  { time: 16, text: 'Time OS 伴随着每一个音符跳跃', translation: 'Time OS leaps with every melody note' },
  { time: 25, text: '黑夜与极光的交织，星河在此刻定格', translation: 'Where night merges with aurora, the galaxy pauses' },
  { time: 36, text: '呼吸沉淀在旋律的微风中', translation: 'Breathing settles within the breeze of melody' },
  { time: 48, text: '让无损音质唤醒沉睡的耳朵', translation: 'Let lossless audio awaken slumbering ears' },
  { time: 62, text: '这一首时光之旅，送给每一次奔赴', translation: 'This journey of time is dedicated to every departure' },
  { time: 78, text: '穿梭在数位与现实的梦境之间', translation: 'Drifting between digital realms and tangible dreams' },
  { time: 95, text: '心随声动，无远弗届', translation: 'Where the heart moves with sound, boundless and free' },
  { time: 120, text: '感谢聆听 Time Music', translation: 'Thank you for listening to Time Music' },
];

let tickInterval: any = null;
let webAudio: HTMLAudioElement | null = null;

// High-performance In-Memory Stream URL Cache
const streamUrlCache = new Map<string, { url: string; time: number }>();
const STREAM_CACHE_TTL = 15 * 60 * 1000; // 15 mins

function getCachedStreamUrl(key: string): string | null {
  const cached = streamUrlCache.get(key);
  if (cached && Date.now() - cached.time < STREAM_CACHE_TTL) {
    return cached.url;
  }
  return null;
}

function setCachedStreamUrl(key: string, url: string) {
  if (streamUrlCache.size > 150) {
    const oldestKey = streamUrlCache.keys().next().value;
    if (oldestKey) streamUrlCache.delete(oldestKey);
  }
  streamUrlCache.set(key, { url, time: Date.now() });
}

// Persistent Favorites LocalStorage keys & loaders
const STORAGE_KEY_FAV_IDS = 'time_music_fav_ids_v1';
const STORAGE_KEY_FAV_CACHE = 'time_music_fav_cache_v1';

function loadStoredFavIds(): Set<string> {
  if (typeof window === 'undefined') return new Set(['demo-1', 'demo-3']);
  try {
    const raw = localStorage.getItem(STORAGE_KEY_FAV_IDS);
    if (raw) {
      const arr = JSON.parse(raw);
      if (Array.isArray(arr) && arr.length > 0) return new Set(arr);
    }
  } catch (e) {}
  return new Set(['demo-1', 'demo-3']);
}

function loadStoredFavCache(): Track[] {
  if (typeof window === 'undefined') return [];
  try {
    const raw = localStorage.getItem(STORAGE_KEY_FAV_CACHE);
    if (raw) {
      const arr = JSON.parse(raw);
      if (Array.isArray(arr)) return arr;
    }
  } catch (e) {}
  return [];
}

export const usePlayerStore = defineStore('player', {
  state: () => ({
    currentTrack: SAMPLE_TRACKS[0] as Track | null,
    isPlaying: false,
    currentTime: 0,
    duration: 218,
    volume: 0.85,
    isMuted: false,
    mode: 'loop' as PlaybackMode,
    playbackRate: 1.0,
    queue: [...SAMPLE_TRACKS],
    queueIndex: 0,
    lyrics: SAMPLE_LYRICS,
    activeLyricIndex: 0,
    showFullscreenLyrics: false,
    showQueueDrawer: false,
    eqEnabled: false,
    bassBoost: 4,
    surroundSound: true,
    audioEngine: 'mock' as 'native' | 'web' | 'mock',
    isNativePlaying: false,
    isInitialized: false,
    isLoadingStream: false,
    isRecoveringStream: false,
    favoriteIds: loadStoredFavIds(),
    favoriteCache: loadStoredFavCache(),
    toastMessage: '',
    toastTimer: null as any,
  }),

  getters: {
    currentTrackTitle: (state) => state.currentTrack?.title ?? '未在播放',
    currentTrackArtist: (state) => state.currentTrack?.artist ?? 'Time Music',
    currentTrackCover: (state) => state.currentTrack?.cover ?? '',
    progressPercentage: (state) => {
      if (!state.duration || state.duration === 0) return 0;
      return (state.currentTime / state.duration) * 100;
    },
    formattedCurrentTime: (state) => formatTime(state.currentTime),
    formattedDuration: (state) => formatTime(state.duration),
  },

  actions: {
    showToast(message: string, duration = 3000) {
      this.toastMessage = message;
      if (this.toastTimer) clearTimeout(this.toastTimer);
      this.toastTimer = setTimeout(() => {
        this.toastMessage = '';
        this.toastTimer = null;
      }, duration);
    },

    async initAudioEvents() {
      if (this.isInitialized) return;
      this.isInitialized = true;

      // Setup Web Audio instance
      if (typeof window !== 'undefined' && !webAudio) {
        webAudio = new Audio();
        webAudio.preload = 'auto';

        webAudio.ontimeupdate = () => {
          if (this.audioEngine === 'web') {
            this.currentTime = webAudio!.currentTime;
            this.updateActiveLyric();
          }
        };

        webAudio.onloadedmetadata = () => {
          if (this.audioEngine === 'web' && webAudio!.duration && !isNaN(webAudio!.duration)) {
            this.duration = webAudio!.duration;
          }
        };

        webAudio.onended = () => {
          if (this.audioEngine === 'web') {
            this.handleTrackEnded();
          }
        };

        webAudio.onerror = (e) => {
          console.warn('[Web Audio] Playback error encountered:', e);
          this.isLoadingStream = false;
          this.recoverAudioError();
        };
      }

      // Native Rodio Status Listener
      try {
        await listen<{
          is_playing: boolean;
          current_time: number;
          duration: number;
          volume: number;
          speed: number;
        }>('player:status', (event) => {
          if (this.audioEngine === 'native') {
            this.currentTime = event.payload.current_time;
            if (event.payload.duration > 0) {
              this.duration = event.payload.duration;
            }
            this.isPlaying = event.payload.is_playing;
            this.updateActiveLyric();
          }
        });

        await listen('player:ended', () => {
          if (this.audioEngine === 'native') {
            this.handleTrackEnded();
          }
        });
      } catch (err) {
        console.warn('Tauri event listener registration failed or running in browser:', err);
      }
    },

    async recoverAudioError() {
      if (!this.currentTrack || this.isRecoveringStream) return;
      this.isRecoveringStream = true;
      const track = this.currentTrack;
      console.log(`[Audio Recovery] Attempting auto-recovery for: ${track.title} - ${track.artist}`);

      try {
        const kuwoMatches = await invoke<Track[]>('kuwo_search', {
          keyword: `${track.title} ${track.artist}`,
          limit: 3,
        });
        if (kuwoMatches && kuwoMatches.length > 0) {
          for (const kTrack of kuwoMatches) {
            const rid = kTrack.path ? kTrack.path.replace('kuwo://', '') : kTrack.id.replace('kuwo-', '');
            const kuwoUrl = await invoke<string | null>('kuwo_get_song_url', { rid });
            if (kuwoUrl && kuwoUrl.startsWith('http')) {
              this.showToast(`已自动匹配《${track.title}》酷我高保真完整音频流`);
              await this.playWebStream(kuwoUrl);
              this.isRecoveringStream = false;
              return;
            }
          }
        }
      } catch (e) {
        console.warn('[Audio Recovery] Kuwo fallback failed:', e);
      }

      try {
        const appleMatches = await invoke<Track[]>('applemusic_search', {
          keyword: `${track.title} ${track.artist}`,
          limit: 3,
        });
        const match = appleMatches?.find((m) => m.path && m.path.startsWith('http'));
        if (match?.path) {
          this.showToast(`已自动恢复《${track.title}》高保真音频流`);
          await this.playWebStream(match.path);
          this.isRecoveringStream = false;
          return;
        }
      } catch (e) {
        console.warn('[Audio Recovery] Apple fallback failed:', e);
      }

      try {
        const neteaseMatches = await invoke<Track[]>('netease_search', {
          keyword: `${track.title} ${track.artist}`,
          limit: 3,
        });
        if (neteaseMatches && neteaseMatches.length > 0) {
          for (const nTrack of neteaseMatches) {
            const nSongId = nTrack.id.replace('netease-', '');
            const neteaseUrl = await invoke<string | null>('netease_get_song_url', {
              songId: nSongId,
              br: 320000,
            });
            if (neteaseUrl && neteaseUrl.startsWith('http')) {
              this.showToast(`已自动恢复《${track.title}》网易云音频流`);
              await this.playWebStream(neteaseUrl);
              this.isRecoveringStream = false;
              return;
            }
          }
        }
      } catch (e) {
        console.warn('[Audio Recovery] NetEase fallback failed:', e);
      }

      this.isRecoveringStream = false;
    },

    async playWebStream(streamUrl: string) {
      try {
        await invoke('audio_stop');
      } catch (e) {
        // Ignored
      }

      if (typeof window !== 'undefined') {
        if (!webAudio) {
          webAudio = new Audio();
          webAudio.preload = 'auto';

          webAudio.ontimeupdate = () => {
            if (this.audioEngine === 'web') {
              this.currentTime = webAudio!.currentTime;
              this.updateActiveLyric();
            }
          };

          webAudio.onloadedmetadata = () => {
            if (this.audioEngine === 'web' && webAudio!.duration && !isNaN(webAudio!.duration)) {
              this.duration = webAudio!.duration;
            }
          };

          webAudio.onended = () => {
            if (this.audioEngine === 'web') {
              this.handleTrackEnded();
            }
          };

          webAudio.onerror = (e) => {
            console.warn('[Web Audio] Playback error encountered:', e, webAudio?.error);
            this.isLoadingStream = false;
            this.recoverAudioError();
          };
        }

        const targetVol = this.isMuted ? 0 : this.volume;
        webAudio.volume = targetVol;
        webAudio.playbackRate = this.playbackRate;
        webAudio.src = streamUrl;

        try {
          await webAudio.play();
        } catch (e) {
          console.warn('[Web Audio] play() prevented or failed:', e);
        }
      }

      this.audioEngine = 'web';
      this.isNativePlaying = false;
      this.isPlaying = true;
      this.isLoadingStream = false;
      this.stopSimulatedTicker();
    },

    async playTrack(track: Track, queueList?: Track[]) {
      this.currentTrack = track;
      this.currentTime = 0;
      this.duration = track.duration || 180;

      // Queue management
      if (queueList && queueList.length > 0) {
        this.queue = [...queueList];
        this.queueIndex = this.queue.findIndex((t) => t.id === track.id);
      } else if (!this.queue.some((t) => t.id === track.id)) {
        this.queue.push(track);
        this.queueIndex = this.queue.length - 1;
      }

      // Record play history in SQLite
      try {
        await invoke('record_play_history', { trackId: track.id });
      } catch (e) {
        // Ignored
      }

      // 0. PRIORITY 1: Direct HTTP / HTTPS audio stream (e.g. Apple Music AAC / direct CDN stream)
      if (track.path && (track.path.startsWith('http://') || track.path.startsWith('https://'))) {
        this.isLoadingStream = true;
        this.lyrics = [
          { time: 0, text: `${track.title} - ${track.artist}` },
          { time: 3, text: `${track.album || '高保真音频流'} • 正在播放` },
        ];
        await this.playWebStream(track.path);
        return;
      }

      // 1. Check if track is an Apple Music protocol track
      const isApple = track.id.startsWith('apple-') || (track.path && track.path.startsWith('apple://'));
      if (isApple) {
        this.isLoadingStream = true;
        this.lyrics = [
          { time: 0, text: `${track.title} - ${track.artist}` },
          { time: 3, text: 'Apple Digital Master • 高保真流媒体' },
        ];

        const trackId = track.path ? track.path.replace('apple://', '') : track.id.replace('apple-', '');
        const cacheKey = `apple:${trackId}`;
        const cachedUrl = getCachedStreamUrl(cacheKey);
        if (cachedUrl) {
          await this.playWebStream(cachedUrl);
          return;
        }

        try {
          const streamUrl = await invoke<string | null>('applemusic_get_song_url', { trackId });
          if (streamUrl) {
            setCachedStreamUrl(cacheKey, streamUrl);
            await this.playWebStream(streamUrl);
            return;
          }
        } catch (err) {}
      }

      // 2. Check if track is a NetEase Cloud Music stream
      const isNetease = (track.path && track.path.startsWith('netease://')) || track.id.startsWith('netease-');
      if (isNetease) {
        this.isLoadingStream = true;
        const songId = track.path ? track.path.replace('netease://', '') : track.id.replace('netease-', '');

        // Fetch lyric asynchronously
        invoke<{ lrc: string; tlyric?: string }>('netease_get_lyric', { songId })
          .then((lyricData) => {
            if (lyricData && lyricData.lrc) {
              this.lyrics = parseLrc(lyricData.lrc, lyricData.tlyric);
            } else {
              this.lyrics = [
                { time: 0, text: `${track.title} - ${track.artist}` },
                { time: 5, text: '享受音乐，暂无滚动歌词' },
              ];
            }
          })
          .catch(() => {
            this.lyrics = [
              { time: 0, text: `${track.title} - ${track.artist}` },
              { time: 5, text: '享受音乐，暂无滚动歌词' },
            ];
          });

        // Fast path: In-memory stream cache hit
        const cacheKey = `netease:${songId}`;
        const cachedUrl = getCachedStreamUrl(cacheKey);
        if (cachedUrl) {
          await this.playWebStream(cachedUrl);
          return;
        }

        // Resolve stream URL
        try {
          const streamUrl = await invoke<string | null>('netease_get_song_url', {
            songId,
            br: 320000,
          });

          if (streamUrl) {
            setCachedStreamUrl(cacheKey, streamUrl);
            await this.playWebStream(streamUrl);
            return;
          }
        } catch (err) {
          console.warn('Failed to resolve NetEase stream URL:', err);
        }

        // SMART AUTO-FALLBACK: If NetEase stream is VIP/restricted, automatically find audio stream
        try {
          const appleMatches = await invoke<Track[]>('applemusic_search', {
            keyword: `${track.title} ${track.artist}`,
            limit: 3,
          });
          if (appleMatches && appleMatches.length > 0 && appleMatches[0].path && appleMatches[0].path.startsWith('http')) {
            const fallbackUrl = appleMatches[0].path;
            setCachedStreamUrl(cacheKey, fallbackUrl);
            this.showToast(`已为您智能匹配《${track.title}》高保真音频流`);
            await this.playWebStream(fallbackUrl);
            return;
          }
        } catch (fallbackErr) {}

        this.showToast(`《${track.title}》暂未取得在线播放权限，建议切换其他音源`);
        this.isLoadingStream = false;
        this.isPlaying = false;
        if (this.queue.length > 1 && this.queueIndex < this.queue.length - 1) {
          setTimeout(() => this.nextTrack(), 1800);
        }
        return;
      }

      // 3. Check if track is a QQ Music stream
      const isQQ = (track.path && track.path.startsWith('qq://')) || track.id.startsWith('qq-');
      if (isQQ) {
        this.isLoadingStream = true;
        const songmid = track.path ? track.path.replace('qq://', '') : track.id.replace('qq-', '');

        // Fetch lyric asynchronously
        invoke<{ lrc: string; trans?: string }>('qqmusic_get_lyric', { songmid })
          .then((res) => {
            if (res && res.lrc) {
              this.lyrics = parseLrc(res.lrc, res.trans);
            } else {
              this.lyrics = [
                { time: 0, text: `${track.title} - ${track.artist}` },
                { time: 5, text: 'QQ 音乐高保真播放中' },
              ];
            }
          })
          .catch(() => {
            this.lyrics = [{ time: 0, text: `${track.title} - ${track.artist}` }];
          });

        // Fast path: In-memory stream cache hit
        const cacheKey = `qq:${songmid}`;
        const cachedUrl = getCachedStreamUrl(cacheKey);
        if (cachedUrl) {
          await this.playWebStream(cachedUrl);
          return;
        }

        // Resolve stream URL
        try {
          const streamUrl = await invoke<string | null>('qqmusic_get_song_url', { songmid });
          if (streamUrl) {
            setCachedStreamUrl(cacheKey, streamUrl);
            await this.playWebStream(streamUrl);
            return;
          }
        } catch (err) {
          console.warn('Failed to resolve QQ Music stream URL:', err);
        }

        // SMART AUTO-FALLBACK for QQ Music (when VIP copyright or stream unavailable)
        try {
          const appleMatches = await invoke<Track[]>('applemusic_search', {
            keyword: `${track.title} ${track.artist}`,
            limit: 5,
          });
          if (appleMatches && appleMatches.length > 0) {
            const match = appleMatches.find((m) => m.path && m.path.startsWith('http'));
            if (match && match.path) {
              const fallbackUrl = match.path;
              setCachedStreamUrl(cacheKey, fallbackUrl);
              this.showToast(`已为您智能匹配《${track.title}》高保真音频流`);
              await this.playWebStream(fallbackUrl);
              return;
            }
          }
        } catch (fallbackErr) {}

        // Fallback 2: NetEase Cloud Music stream
        try {
          const neteaseMatches = await invoke<Track[]>('netease_search', {
            keyword: `${track.title} ${track.artist}`,
            limit: 5,
          });
          if (neteaseMatches && neteaseMatches.length > 0) {
            for (const nTrack of neteaseMatches) {
              const nSongId = nTrack.id.replace('netease-', '');
              const neteaseUrl = await invoke<string | null>('netease_get_song_url', {
                songId: nSongId,
                br: 320000,
              });
              if (neteaseUrl && neteaseUrl.startsWith('http')) {
                setCachedStreamUrl(cacheKey, neteaseUrl);
                this.showToast(`已为您智能匹配《${track.title}》网易云音频流`);
                await this.playWebStream(neteaseUrl);
                return;
              }
            }
          }
        } catch (neteaseErr) {}

        this.showToast(`《${track.title}》暂未取得 QQ 音乐播放流，建议尝试其他音源`);
        this.isLoadingStream = false;
        this.isPlaying = false;
        if (this.queue.length > 1 && this.queueIndex < this.queue.length - 1) {
          setTimeout(() => this.nextTrack(), 1800);
        }
        return;
      }

      // 3b. Check if track is a Kuwo Music track (酷我音乐)
      const isKuwo = (track.path && track.path.startsWith('kuwo://')) || track.id.startsWith('kuwo-');
      if (isKuwo) {
        this.isLoadingStream = true;
        const rid = track.path ? track.path.replace('kuwo://', '') : track.id.replace('kuwo-', '');

        invoke<{ lrc: string }>('kuwo_get_lyric', { rid })
          .then((res) => {
            if (res && res.lrc) {
              this.lyrics = parseLrc(res.lrc);
            } else {
              this.lyrics = [{ time: 0, text: `${track.title} - ${track.artist}` }];
            }
          })
          .catch(() => {
            this.lyrics = [{ time: 0, text: `${track.title} - ${track.artist}` }];
          });

        const cacheKey = `kuwo:${rid}`;
        const cachedUrl = getCachedStreamUrl(cacheKey);
        if (cachedUrl) {
          await this.playWebStream(cachedUrl);
          return;
        }

        try {
          const streamUrl = await invoke<string | null>('kuwo_get_song_url', { rid });
          if (streamUrl) {
            setCachedStreamUrl(cacheKey, streamUrl);
            await this.playWebStream(streamUrl);
            return;
          }
        } catch (err) {
          console.warn('Failed to resolve Kuwo stream URL:', err);
        }

        await this.recoverAudioError();
        if (!this.isPlaying) {
          this.showToast(`《${track.title}》暂未取得酷我音乐播放流`);
          this.isLoadingStream = false;
        }
        return;
      }

      // 3c. Check if track is a Kugou Music track (酷狗音乐)
      const isKugou = (track.path && track.path.startsWith('kugou://')) || track.id.startsWith('kugou-');
      if (isKugou) {
        this.isLoadingStream = true;
        const hash = track.path ? track.path.replace('kugou://', '') : track.id.replace('kugou-', '');

        invoke<{ lrc: string }>('kugou_get_lyric', {
          hash,
          keyword: `${track.artist} - ${track.title}`,
        })
          .then((res) => {
            if (res && res.lrc) {
              this.lyrics = parseLrc(res.lrc);
            } else {
              this.lyrics = [{ time: 0, text: `${track.title} - ${track.artist}` }];
            }
          })
          .catch(() => {
            this.lyrics = [{ time: 0, text: `${track.title} - ${track.artist}` }];
          });

        const cacheKey = `kugou:${hash}`;
        const cachedUrl = getCachedStreamUrl(cacheKey);
        if (cachedUrl) {
          await this.playWebStream(cachedUrl);
          return;
        }

        try {
          const streamUrl = await invoke<string | null>('kugou_get_song_url', { hash });
          if (streamUrl) {
            setCachedStreamUrl(cacheKey, streamUrl);
            await this.playWebStream(streamUrl);
            return;
          }
        } catch (err) {
          console.warn('Failed to resolve Kugou stream URL:', err);
        }

        await this.recoverAudioError();
        if (!this.isPlaying) {
          this.showToast(`《${track.title}》暂未取得酷狗音乐播放流`);
          this.isLoadingStream = false;
        }
        return;
      }

      // 3d. Check if track is a Qishui Music track (汽水音乐)
      const isQishui = (track.path && track.path.startsWith('qishui://')) || track.id.startsWith('qishui-');
      if (isQishui) {
        this.isLoadingStream = true;
        const trackId = track.path ? track.path.replace('qishui://', '') : track.id.replace('qishui-', '');
        this.lyrics = [
          { time: 0, text: `${track.title} - ${track.artist}` },
          { time: 4, text: '汽水音乐 • 发现好音乐' },
        ];

        const cacheKey = `qishui:${trackId}`;
        const cachedUrl = getCachedStreamUrl(cacheKey);
        if (cachedUrl) {
          await this.playWebStream(cachedUrl);
          return;
        }

        try {
          const streamUrl = await invoke<string | null>('qishui_get_song_url', { trackId });
          if (streamUrl) {
            setCachedStreamUrl(cacheKey, streamUrl);
            await this.playWebStream(streamUrl);
            return;
          }
        } catch (err) {
          console.warn('Failed to resolve Qishui stream URL:', err);
        }

        await this.recoverAudioError();
        if (!this.isPlaying) {
          this.showToast(`《${track.title}》暂未取得汽水音乐播放流`);
          this.isLoadingStream = false;
        }
        return;
      }

      // 4. Check if track is a YouTube Music track
      const isYT = (track.path && track.path.startsWith('yt://')) || track.id.startsWith('yt-');
      if (isYT) {
        this.isLoadingStream = true;
        this.lyrics = [
          { time: 0, text: `${track.title} - ${track.artist}` },
          { time: 4, text: 'YouTube Music • 全球高清音频流' },
        ];

        const videoId = track.path ? track.path.replace('yt://', '') : track.id.replace('yt-', '');

        // Fast path: In-memory stream cache hit
        const cacheKey = `yt:${videoId}`;
        const cachedUrl = getCachedStreamUrl(cacheKey);
        if (cachedUrl) {
          await this.playWebStream(cachedUrl);
          return;
        }

        try {
          const streamUrl = await invoke<string | null>('ytmusic_get_song_url', { videoId });
          if (streamUrl) {
            setCachedStreamUrl(cacheKey, streamUrl);
            await this.playWebStream(streamUrl);
            return;
          }
        } catch (err) {
          console.warn('Failed to resolve YouTube Music stream:', err);
        }

        // SMART AUTO-FALLBACK for YouTube
        try {
          const appleMatches = await invoke<Track[]>('applemusic_search', {
            keyword: `${track.title} ${track.artist}`,
            limit: 3,
          });
          if (appleMatches && appleMatches.length > 0 && appleMatches[0].path && appleMatches[0].path.startsWith('http')) {
            const fallbackUrl = appleMatches[0].path;
            setCachedStreamUrl(cacheKey, fallbackUrl);
            this.showToast(`已为您智能匹配《${track.title}》高保真音频流`);
            await this.playWebStream(fallbackUrl);
            return;
          }
        } catch (fallbackErr) {}

        this.showToast(`无法获取《${track.title}》的音频流`);
        this.isLoadingStream = false;
        this.isPlaying = false;
        if (this.queue.length > 1 && this.queueIndex < this.queue.length - 1) {
          setTimeout(() => this.nextTrack(), 1800);
        }
        return;
      }

      // 5. Check if track is an imported external track needing smart online stream match
      if (track.path && track.path.startsWith('search://')) {
        this.isLoadingStream = true;
        const query = decodeURIComponent(track.path.replace('search://', ''));
        const cacheKey = `search:${query}`;
        const cachedUrl = getCachedStreamUrl(cacheKey);
        if (cachedUrl) {
          this.isLoadingStream = false;
          await this.playWebStream(cachedUrl);
          return;
        }

        try {
          // Attempt QQ Music match first
          const qqResults = await invoke<Track[]>('qqmusic_search', { keyword: query, limit: 1 });
          if (qqResults && qqResults.length > 0) {
            const matched = qqResults[0];
            const songmid = matched.path ? matched.path.replace('qq://', '') : matched.id.replace('qq-', '');
            const streamUrl = await invoke<string | null>('qqmusic_get_song_url', { songmid });
            if (streamUrl) {
              invoke<{ lrc: string; trans?: string }>('qqmusic_get_lyric', { songmid })
                .then((res) => {
                  if (res && res.lrc) this.lyrics = parseLrc(res.lrc, res.trans);
                })
                .catch(() => {});
              setCachedStreamUrl(cacheKey, streamUrl);
              this.isLoadingStream = false;
              await this.playWebStream(streamUrl);
              return;
            }
          }

          // Fallback to NetEase search
          const neteaseResults = await invoke<Track[]>('netease_search', { keyword: query, limit: 1 });
          if (neteaseResults && neteaseResults.length > 0) {
            const matched = neteaseResults[0];
            const songId = matched.path ? matched.path.replace('netease://', '') : matched.id.replace('netease-', '');
            const streamUrl = await invoke<string | null>('netease_get_song_url', {
              songId: songId.toString(),
              br: 320000,
            });
            if (streamUrl) {
              invoke<{ lrc: string; tlyric?: string }>('netease_get_lyric', { songId: songId.toString() })
                .then((lyricData) => {
                  if (lyricData && lyricData.lrc) this.lyrics = parseLrc(lyricData.lrc, lyricData.tlyric);
                })
                .catch(() => {});
              setCachedStreamUrl(cacheKey, streamUrl);
              this.isLoadingStream = false;
              await this.playWebStream(streamUrl);
              return;
            }
          }
        } catch (err) {
          console.warn('Smart match failed for imported track:', err);
        } finally {
          this.isLoadingStream = false;
        }
      }

      // 6. Check if track is direct HTTP / HTTPS stream (e.g. Apple preview)
      if (track.path && (track.path.startsWith('http://') || track.path.startsWith('https://'))) {
        await this.playWebStream(track.path);
        return;
      }

      // 3. Check if track is a native local file path
      if (track.path && !track.path.startsWith('mock:')) {
        // Pause Web Audio if playing
        if (webAudio) {
          webAudio.pause();
          webAudio.src = '';
        }

        try {
          await invoke('audio_play', { path: track.path });
          await invoke('audio_set_volume', { volume: this.isMuted ? 0 : this.volume });
          await invoke('audio_set_speed', { speed: this.playbackRate });

          this.audioEngine = 'native';
          this.isNativePlaying = true;
          this.isPlaying = true;
          this.stopSimulatedTicker();
          return;
        } catch (err) {
          console.warn('Failed to play via native Rust Rodio, falling back to simulated pipeline:', err);
        }
      }

      // 4. Fallback: Demo simulated ticker
      if (webAudio) {
        webAudio.pause();
        webAudio.src = '';
      }
      this.lyrics = SAMPLE_LYRICS;
      this.audioEngine = 'mock';
      this.isNativePlaying = false;
      this.isPlaying = true;
      this.startSimulatedTicker();
    },

    async togglePlay() {
      if (!this.currentTrack && this.queue.length > 0) {
        await this.playTrack(this.queue[0]);
        return;
      }

      this.isPlaying = !this.isPlaying;

      if (this.audioEngine === 'web' && webAudio) {
        if (this.isPlaying) {
          webAudio.play().catch(console.warn);
        } else {
          webAudio.pause();
        }
      } else if (this.audioEngine === 'native') {
        try {
          if (this.isPlaying) {
            await invoke('audio_resume');
          } else {
            await invoke('audio_pause');
          }
        } catch (err) {
          console.warn('Native pause/resume invoke failed:', err);
        }
      } else {
        if (this.isPlaying) {
          this.startSimulatedTicker();
        } else {
          this.stopSimulatedTicker();
        }
      }
    },

    nextTrack() {
      if (this.queue.length === 0) return;
      if (this.mode === 'shuffle') {
        const nextIdx = Math.floor(Math.random() * this.queue.length);
        this.queueIndex = nextIdx;
      } else {
        this.queueIndex = (this.queueIndex + 1) % this.queue.length;
      }
      this.playTrack(this.queue[this.queueIndex]);
    },

    prevTrack() {
      if (this.queue.length === 0) return;
      if (this.currentTime > 3) {
        this.seek(0);
        return;
      }
      this.queueIndex = (this.queueIndex - 1 + this.queue.length) % this.queue.length;
      this.playTrack(this.queue[this.queueIndex]);
    },

    playNext(track: Track) {
      if (this.queue.length === 0) {
        this.playTrack(track);
        return;
      }
      if (this.currentTrack?.id === track.id) {
        this.showToast(`《${track.title}》正在播放中`);
        return;
      }
      const existingIdx = this.queue.findIndex((t) => t.id === track.id);
      if (existingIdx !== -1) {
        this.queue.splice(existingIdx, 1);
        if (existingIdx < this.queueIndex) {
          this.queueIndex = Math.max(0, this.queueIndex - 1);
        }
      }
      const insertIdx = Math.min(this.queue.length, Math.max(0, this.queueIndex + 1));
      this.queue.splice(insertIdx, 0, track);
      this.showToast(`已将《${track.title}》设为下一首播放`);
    },

    addToQueue(track: Track) {
      if (this.queue.some((t) => t.id === track.id)) {
        this.showToast(`《${track.title}》已在播放队列中`);
        return;
      }
      this.queue.push(track);
      this.showToast(`已将《${track.title}》加入播放队列`);
    },

    async seek(seconds: number) {
      this.currentTime = Math.max(0, Math.min(seconds, this.duration));
      this.updateActiveLyric();

      if (this.audioEngine === 'web' && webAudio) {
        webAudio.currentTime = this.currentTime;
      } else if (this.audioEngine === 'native') {
        try {
          await invoke('audio_seek', { positionSecs: this.currentTime });
        } catch (err) {
          console.warn('Native seek failed:', err);
        }
      }
    },

    async setVolume(vol: number) {
      this.volume = Math.max(0, Math.min(1, vol));
      if (this.volume > 0) {
        this.isMuted = false;
      }

      const settingsStore = useSettingsStore();
      if ((this.volume === 0 || this.isMuted) && settingsStore.autoCaptionsOnMute) {
        this.showFullscreenLyrics = true;
      }

      const effectiveVol = this.isMuted ? 0 : this.volume;
      if (webAudio) {
        webAudio.volume = effectiveVol;
      }
      if (this.audioEngine === 'native') {
        try {
          await invoke('audio_set_volume', { volume: effectiveVol });
        } catch (err) {
          console.warn('Native volume set failed:', err);
        }
      }
    },

    async toggleMute() {
      this.isMuted = !this.isMuted;
      const settingsStore = useSettingsStore();
      if (this.isMuted && settingsStore.autoCaptionsOnMute) {
        this.showFullscreenLyrics = true;
      }

      const effectiveVol = this.isMuted ? 0 : this.volume;

      if (webAudio) {
        webAudio.volume = effectiveVol;
      }
      if (this.audioEngine === 'native') {
        try {
          await invoke('audio_set_volume', { volume: effectiveVol });
        } catch (err) {
          console.warn('Native mute toggle failed:', err);
        }
      }
    },

    async setPlaybackRate(rate: number) {
      this.playbackRate = rate;

      if (webAudio) {
        webAudio.playbackRate = rate;
      }
      if (this.audioEngine === 'native') {
        try {
          await invoke('audio_set_speed', { speed: rate });
        } catch (err) {
          console.warn('Native speed set failed:', err);
        }
      }
    },

    cycleMode() {
      const modes: PlaybackMode[] = ['sequence', 'loop', 'single', 'shuffle'];
      const currentIdx = modes.indexOf(this.mode);
      this.mode = modes[(currentIdx + 1) % modes.length];
    },

    handleTrackEnded() {
      if (this.mode === 'single') {
        if (this.currentTrack) {
          this.playTrack(this.currentTrack);
        }
      } else if (this.mode === 'sequence') {
        if (this.queueIndex < this.queue.length - 1) {
          this.nextTrack();
        } else {
          this.isPlaying = false;
          this.currentTime = 0;
        }
      } else {
        this.nextTrack();
      }
    },

    toggleFullscreenLyrics() {
      this.showFullscreenLyrics = !this.showFullscreenLyrics;
    },

    toggleQueueDrawer() {
      this.showQueueDrawer = !this.showQueueDrawer;
    },

    isIdFavorite(trackId?: string): boolean {
      if (!trackId) return false;
      return this.favoriteIds.has(trackId);
    },

    saveFavoritesToStorage() {
      if (typeof window === 'undefined') return;
      try {
        localStorage.setItem(STORAGE_KEY_FAV_IDS, JSON.stringify(Array.from(this.favoriteIds)));
        localStorage.setItem(STORAGE_KEY_FAV_CACHE, JSON.stringify(this.favoriteCache));
      } catch (e) {
        // Ignored
      }
    },

    toggleFavorite(trackOrId?: Track | string) {
      let targetId: string | undefined;
      let targetTrack: Track | undefined;

      if (typeof trackOrId === 'object' && trackOrId !== null) {
        targetTrack = trackOrId;
        targetId = trackOrId.id;
      } else if (typeof trackOrId === 'string') {
        targetId = trackOrId;
      } else {
        targetId = this.currentTrack?.id;
        targetTrack = this.currentTrack || undefined;
      }

      if (!targetId) return;

      // Determine new favorite state
      const currentlyFav = targetTrack !== undefined
        ? !!targetTrack.isFavorite
        : this.favoriteIds.has(targetId);
      const newFavState = !currentlyFav;

      // 1. Update target track object directly (triggers Vue component reactivity)
      if (targetTrack) {
        targetTrack.isFavorite = newFavState;
      }

      // 2. Update currentTrack if matching
      if (this.currentTrack && this.currentTrack.id === targetId) {
        this.currentTrack.isFavorite = newFavState;
      }

      // 3. Update all instances in playback queue
      for (const t of this.queue) {
        if (t.id === targetId) {
          t.isFavorite = newFavState;
        }
      }

      // 4. Update in-memory Set & LocalStorage
      if (newFavState) {
        this.favoriteIds.add(targetId);
        const trackToCache = targetTrack || (this.currentTrack?.id === targetId ? this.currentTrack : null) || this.queue.find((t) => t.id === targetId);
        if (trackToCache) {
          const idx = this.favoriteCache.findIndex((t) => t.id === targetId);
          if (idx >= 0) {
            this.favoriteCache[idx] = { ...trackToCache, isFavorite: true };
          } else {
            this.favoriteCache.unshift({ ...trackToCache, isFavorite: true });
          }
        }
      } else {
        this.favoriteIds.delete(targetId);
        this.favoriteCache = this.favoriteCache.filter((t) => t.id !== targetId);
      }
      this.saveFavoritesToStorage();

      // 5. Update libraryStore if track exists in library
      try {
        const libraryStore = useLibraryStore();
        for (const t of libraryStore.tracks) {
          if (t.id === targetId) {
            t.isFavorite = newFavState;
          }
        }
      } catch (e) {
        // Ignored
      }

      // 6. Update playlistStore 'pl-favorites'
      try {
        const playlistStore = usePlaylistStore();
        const plFav = playlistStore.playlists.find((p) => p.id === 'pl-favorites');
        if (plFav) {
          if (newFavState && !plFav.trackIds.includes(targetId)) {
            plFav.trackIds.unshift(targetId);
          } else if (!newFavState) {
            plFav.trackIds = plFav.trackIds.filter((id) => id !== targetId);
          }
        }
      } catch (e) {
        // Ignored
      }

      // 7. Invoke Rust SQLite synchronization asynchronously
      invoke<boolean>('toggle_track_favorite', { trackId: targetId }).catch((err) => {
        console.warn('Native toggle favorite non-fatal sync notice:', err);
      });
    },

    updateActiveLyric() {
      if (this.lyrics.length === 0) return;
      for (let i = this.lyrics.length - 1; i >= 0; i--) {
        if (this.currentTime >= this.lyrics[i].time) {
          this.activeLyricIndex = i;
          return;
        }
      }
      this.activeLyricIndex = 0;
    },

    startSimulatedTicker() {
      this.stopSimulatedTicker();
      tickInterval = setInterval(() => {
        if (this.isPlaying) {
          this.currentTime += 1 * this.playbackRate;
          this.updateActiveLyric();
          if (this.currentTime >= this.duration) {
            this.handleTrackEnded();
          }
        }
      }, 1000);
    },

    stopSimulatedTicker() {
      if (tickInterval) {
        clearInterval(tickInterval);
        tickInterval = null;
      }
    },
  },
});

function formatTime(seconds: number): string {
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  return `${m}:${s.toString().padStart(2, '0')}`;
}
