import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { currentLocale, setLocale, type LocaleId } from '@/i18n';

export type ThemeId = 'dark' | 'cyberpunk' | 'aurora' | 'sunset' | 'ocean' | 'sakura' | 'light';

export type StreamingAudioQuality = 'hi-res' | 'lossless' | 'high' | 'standard';
export type DownloadAudioQuality = 'lossless' | 'hi-res' | 'high';
export type DolbyAtmosMode = 'auto' | 'always' | 'off';
export type VideoStreamingQuality = '1080p' | '4k' | '720p';
export type VideoDownloadQuality = '1080p' | '4k' | 'sd';
export type LyricsBgMode = 'dynamic' | 'cover' | 'solid' | 'custom';

export interface ThemeMeta {
  id: ThemeId;
  name: string;
  enName: string;
  category: 'dark' | 'light';
  previewColor: string;
  accentColor: string;
  desc: string;
}

export interface CacheInfo {
  current_path: string;
  total_bytes: number;
  formatted_total: string;
  audio_bytes: number;
  formatted_audio: string;
  cover_bytes: number;
  formatted_cover: string;
  other_bytes: number;
  formatted_other: string;
  file_count: number;
}

export interface SystemPresetDir {
  id: string;
  name: string;
  path: string;
  icon_type: string;
  exists: boolean;
}

export const THEMES: ThemeMeta[] = [
  {
    id: 'dark',
    name: '深空黑 • 经典红',
    enName: 'Space Black',
    category: 'dark',
    previewColor: '#0c0d12',
    accentColor: '#fa233b',
    desc: 'Apple Music 标志性深色极简美学，纯粹深邃',
  },
  {
    id: 'cyberpunk',
    name: '赛博霓虹 • 幻夜紫',
    enName: 'Cyberpunk Neon',
    category: 'dark',
    previewColor: '#090813',
    accentColor: '#a855f7',
    desc: '迷离紫电与赛博朋克微光，充满未来科技感',
  },
  {
    id: 'aurora',
    name: '极光苍穹 • 翡翠绿',
    enName: 'Nordic Aurora',
    category: 'dark',
    previewColor: '#061210',
    accentColor: '#10b981',
    desc: '北欧苔原之夜，流动的翡翠极光与深谧静思',
  },
  {
    id: 'sunset',
    name: '落日暮光 • 琥珀暖阳',
    enName: 'Sunset Amber',
    category: 'dark',
    previewColor: '#130c0a',
    accentColor: '#f97316',
    desc: '温暖黄昏与老式黑胶咖啡厅的落日余晖',
  },
  {
    id: 'ocean',
    name: '深海幽蓝 • 湛蓝星河',
    enName: 'Deep Ocean',
    category: 'dark',
    previewColor: '#070e1c',
    accentColor: '#3b82f6',
    desc: '沉入静谧深海与广袤星河，专注而安详',
  },
  {
    id: 'sakura',
    name: '千禧粉晶 • 樱花迷雾',
    enName: 'Sakura Rose',
    category: 'dark',
    previewColor: '#140a10',
    accentColor: '#f43f5e',
    desc: '优雅柔和的玫瑰粉金，浪漫与现代轻奢',
  },
  {
    id: 'light',
    name: '晨曦纯白 • 极简透光',
    enName: 'Cupertino Day',
    category: 'light',
    previewColor: '#f1f4f9',
    accentColor: '#fa233b',
    desc: '清晨微光与通透白毛玻璃，高明度清爽质感',
  },
];

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    theme: 'dark' as ThemeId,
    enableAmbientGlow: true,
    enablePageAnimation: true,
    audioQuality: 'lossless' as 'standard' | 'high' | 'lossless',
    autoScanStartup: true,
    hardwareAcceleration: true,
    globalHotkeys: true,
    mprisIntegration: true,
    downloadPath: '',
    cachePath: '',
    cacheLimitGb: 5, // 0 = unlimited, 0.5, 1, 2, 5, 10, 20
    cacheInfo: null as CacheInfo | null,
    systemPresets: [] as SystemPresetDir[],
    isClearingCache: false,
    clearCacheMessage: '',
    locale: 'zh-CN' as LocaleId,

    // 播放设置 (Apple Music 播放设置项)
    enableCrossfade: true,
    crossfadeDuration: 4, // 1 - 12 秒钟
    enableSoundEnhancer: true,
    soundEnhancerLevel: 4, // 1 - 5 (低至高)
    enableSoundCheck: true,
    enableLossless: true,
    streamingAudioQuality: 'hi-res' as StreamingAudioQuality,
    downloadAudioQuality: 'lossless' as DownloadAudioQuality,
    dolbyAtmos: 'auto' as DolbyAtmosMode,
    videoStreamingQuality: '1080p' as VideoStreamingQuality,
    videoDownloadQuality: '1080p' as VideoDownloadQuality,
    autoCaptionsOnMute: true,

    // 歌词页自定义背景设置
    lyricsBgMode: 'dynamic' as LyricsBgMode,
    lyricsBgImage: '',
    lyricsBgBlur: 36,
    lyricsBgDim: 50,
    lyricsBgColor: '#08080c',
  }),

  actions: {
    async initSettings() {
      // 1. Theme
      const savedTheme = localStorage.getItem('time_music_theme') as ThemeId;
      if (savedTheme && THEMES.some((t) => t.id === savedTheme)) {
        this.setTheme(savedTheme);
      } else {
        this.setTheme('dark');
      }

      // 2. Ambient Glow
      const savedGlow = localStorage.getItem('time_music_ambient_glow');
      if (savedGlow !== null) {
        this.enableAmbientGlow = savedGlow === 'true';
      }

      // 3. Animation
      const savedAnim = localStorage.getItem('time_music_page_anim');
      if (savedAnim !== null) {
        this.enablePageAnimation = savedAnim === 'true';
      }

      // 4. Cache Path & Limit
      const savedCachePath = localStorage.getItem('time_music_cache_path');
      if (savedCachePath) {
        this.cachePath = savedCachePath;
      }

      const savedLimit = localStorage.getItem('time_music_cache_limit');
      if (savedLimit) {
        this.cacheLimitGb = Number(savedLimit);
      }

      // 5. Locale
      this.locale = currentLocale.value;

      // 6. Playback Settings
      const savedCrossfade = localStorage.getItem('time_music_crossfade');
      if (savedCrossfade !== null) this.enableCrossfade = savedCrossfade === 'true';

      const savedDuration = localStorage.getItem('time_music_crossfade_duration');
      if (savedDuration) this.crossfadeDuration = Number(savedDuration);

      const savedEnhancer = localStorage.getItem('time_music_sound_enhancer');
      if (savedEnhancer !== null) this.enableSoundEnhancer = savedEnhancer === 'true';

      const savedEnhancerLevel = localStorage.getItem('time_music_enhancer_level');
      if (savedEnhancerLevel) this.soundEnhancerLevel = Number(savedEnhancerLevel);

      const savedSoundCheck = localStorage.getItem('time_music_sound_check');
      if (savedSoundCheck !== null) this.enableSoundCheck = savedSoundCheck === 'true';

      const savedLossless = localStorage.getItem('time_music_lossless');
      if (savedLossless !== null) this.enableLossless = savedLossless === 'true';

      const savedStreamQuality = localStorage.getItem('time_music_stream_quality') as StreamingAudioQuality;
      if (savedStreamQuality) this.streamingAudioQuality = savedStreamQuality;

      const savedDownloadQuality = localStorage.getItem('time_music_download_quality') as DownloadAudioQuality;
      if (savedDownloadQuality) this.downloadAudioQuality = savedDownloadQuality;

      const savedDolby = localStorage.getItem('time_music_dolby') as DolbyAtmosMode;
      if (savedDolby) this.dolbyAtmos = savedDolby;

      const savedVidStream = localStorage.getItem('time_music_video_stream') as VideoStreamingQuality;
      if (savedVidStream) this.videoStreamingQuality = savedVidStream;

      const savedVidDown = localStorage.getItem('time_music_video_down') as VideoDownloadQuality;
      if (savedVidDown) this.videoDownloadQuality = savedVidDown;

      const savedAutoCaptions = localStorage.getItem('time_music_auto_captions');
      if (savedAutoCaptions !== null) this.autoCaptionsOnMute = savedAutoCaptions === 'true';

      // 7. Lyrics Custom Background
      const savedLyricsBgMode = localStorage.getItem('time_music_lyrics_bg_mode') as LyricsBgMode;
      if (savedLyricsBgMode && ['dynamic', 'cover', 'solid', 'custom'].includes(savedLyricsBgMode)) {
        this.lyricsBgMode = savedLyricsBgMode;
      }
      const savedLyricsBgImg = localStorage.getItem('time_music_lyrics_bg_image');
      if (savedLyricsBgImg) this.lyricsBgImage = savedLyricsBgImg;
      const savedLyricsBgBlur = localStorage.getItem('time_music_lyrics_bg_blur');
      if (savedLyricsBgBlur !== null) this.lyricsBgBlur = Number(savedLyricsBgBlur);
      const savedLyricsBgDim = localStorage.getItem('time_music_lyrics_bg_dim');
      if (savedLyricsBgDim !== null) this.lyricsBgDim = Number(savedLyricsBgDim);
      const savedLyricsBgColor = localStorage.getItem('time_music_lyrics_bg_color');
      if (savedLyricsBgColor) this.lyricsBgColor = savedLyricsBgColor;

      // Fetch cache info & system presets asynchronously
      this.fetchCacheInfo();
      this.fetchSystemPresets();
    },

    setTheme(themeId: ThemeId) {
      this.theme = themeId;
      localStorage.setItem('time_music_theme', themeId);
      if (typeof document !== 'undefined') {
        if (themeId === 'dark') {
          document.documentElement.removeAttribute('data-theme');
          document.documentElement.classList.add('dark');
        } else if (themeId === 'light') {
          document.documentElement.setAttribute('data-theme', 'light');
          document.documentElement.classList.remove('dark');
        } else {
          document.documentElement.setAttribute('data-theme', themeId);
          document.documentElement.classList.add('dark');
        }
      }
    },

    toggleAmbientGlow() {
      this.enableAmbientGlow = !this.enableAmbientGlow;
      localStorage.setItem('time_music_ambient_glow', String(this.enableAmbientGlow));
    },

    togglePageAnimation() {
      this.enablePageAnimation = !this.enablePageAnimation;
      localStorage.setItem('time_music_page_anim', String(this.enablePageAnimation));
    },

    setAudioQuality(quality: 'standard' | 'high' | 'lossless') {
      this.audioQuality = quality;
    },

    toggleGlobalHotkeys() {
      this.globalHotkeys = !this.globalHotkeys;
    },

    toggleCrossfade() {
      this.enableCrossfade = !this.enableCrossfade;
      localStorage.setItem('time_music_crossfade', String(this.enableCrossfade));
    },

    setCrossfadeDuration(sec: number) {
      this.crossfadeDuration = Math.max(1, Math.min(12, Math.round(sec)));
      localStorage.setItem('time_music_crossfade_duration', String(this.crossfadeDuration));
    },

    toggleSoundEnhancer() {
      this.enableSoundEnhancer = !this.enableSoundEnhancer;
      localStorage.setItem('time_music_sound_enhancer', String(this.enableSoundEnhancer));
    },

    setSoundEnhancerLevel(lvl: number) {
      this.soundEnhancerLevel = Math.max(1, Math.min(5, Math.round(lvl)));
      localStorage.setItem('time_music_enhancer_level', String(this.soundEnhancerLevel));
    },

    toggleSoundCheck() {
      this.enableSoundCheck = !this.enableSoundCheck;
      localStorage.setItem('time_music_sound_check', String(this.enableSoundCheck));
    },

    toggleLossless() {
      this.enableLossless = !this.enableLossless;
      localStorage.setItem('time_music_lossless', String(this.enableLossless));
    },

    setStreamingAudioQuality(q: StreamingAudioQuality) {
      this.streamingAudioQuality = q;
      localStorage.setItem('time_music_stream_quality', q);
    },

    setDownloadAudioQuality(q: DownloadAudioQuality) {
      this.downloadAudioQuality = q;
      localStorage.setItem('time_music_download_quality', q);
    },

    setDolbyAtmos(mode: DolbyAtmosMode) {
      this.dolbyAtmos = mode;
      localStorage.setItem('time_music_dolby', mode);
    },

    setVideoStreamingQuality(q: VideoStreamingQuality) {
      this.videoStreamingQuality = q;
      localStorage.setItem('time_music_video_stream', q);
    },

    setVideoDownloadQuality(q: VideoDownloadQuality) {
      this.videoDownloadQuality = q;
      localStorage.setItem('time_music_video_down', q);
    },

    toggleAutoCaptionsOnMute() {
      this.autoCaptionsOnMute = !this.autoCaptionsOnMute;
      localStorage.setItem('time_music_auto_captions', String(this.autoCaptionsOnMute));
    },

    setLanguage(loc: LocaleId) {
      setLocale(loc);
      this.locale = loc;
    },

    async fetchCacheInfo() {
      try {
        const info = await invoke<CacheInfo>('get_cache_info', {
          customPath: this.cachePath || null,
        });
        this.cacheInfo = info;
      } catch (e) {
        console.warn('Failed to get cache info:', e);
      }
    },

    async fetchSystemPresets() {
      try {
        const presets = await invoke<SystemPresetDir[]>('get_preset_directories');
        this.systemPresets = presets;
      } catch (e) {
        console.warn('Failed to get system preset dirs:', e);
      }
    },

    async chooseCacheFolder() {
      try {
        const selected = await invoke<string | null>('pick_folder', {
          defaultPath: this.cachePath || null,
        });
        if (selected && selected.trim()) {
          this.cachePath = selected.trim();
          localStorage.setItem('time_music_cache_path', this.cachePath);
          await this.fetchCacheInfo();
        }
      } catch (e) {
        console.warn('Folder selection failed:', e);
      }
    },

    async setCachePath(path: string) {
      this.cachePath = path.trim();
      localStorage.setItem('time_music_cache_path', this.cachePath);
      await this.fetchCacheInfo();
    },

    async resetCachePathToDefault() {
      this.cachePath = '';
      localStorage.removeItem('time_music_cache_path');
      await this.fetchCacheInfo();
    },

    setCacheLimit(gb: number) {
      this.cacheLimitGb = gb;
      localStorage.setItem('time_music_cache_limit', String(gb));
    },

    async clearCache(): Promise<number> {
      this.isClearingCache = true;
      try {
        const freed = await invoke<number>('clear_cache', {
          customPath: this.cachePath || null,
        });
        await this.fetchCacheInfo();
        this.clearCacheMessage = '缓存已成功清空！';
        setTimeout(() => {
          this.clearCacheMessage = '';
        }, 3000);
        return freed;
      } catch (e) {
        console.warn('Failed to clear cache:', e);
        return 0;
      } finally {
        this.isClearingCache = false;
      }
    },

    async openCacheInExplorer() {
      const target = this.cacheInfo?.current_path || this.cachePath;
      if (target) {
        await invoke('open_folder_in_explorer', { path: target });
      }
    },

    setLyricsBgMode(mode: LyricsBgMode) {
      this.lyricsBgMode = mode;
      localStorage.setItem('time_music_lyrics_bg_mode', mode);
    },

    setLyricsBgImage(imgUrl: string) {
      this.lyricsBgImage = imgUrl.trim();
      localStorage.setItem('time_music_lyrics_bg_image', this.lyricsBgImage);
      if (this.lyricsBgImage && this.lyricsBgMode !== 'custom') {
        this.setLyricsBgMode('custom');
      }
    },

    setLyricsBgBlur(blur: number) {
      this.lyricsBgBlur = Math.max(0, Math.min(80, blur));
      localStorage.setItem('time_music_lyrics_bg_blur', String(this.lyricsBgBlur));
    },

    setLyricsBgDim(dim: number) {
      this.lyricsBgDim = Math.max(10, Math.min(85, dim));
      localStorage.setItem('time_music_lyrics_bg_dim', String(this.lyricsBgDim));
    },

    setLyricsBgColor(color: string) {
      this.lyricsBgColor = color;
      localStorage.setItem('time_music_lyrics_bg_color', color);
    },

    resetLyricsBackground() {
      this.setLyricsBgMode('dynamic');
      this.lyricsBgImage = '';
      localStorage.removeItem('time_music_lyrics_bg_image');
      this.setLyricsBgBlur(36);
      this.setLyricsBgDim(50);
      this.setLyricsBgColor('#08080c');
    },
  },
});
