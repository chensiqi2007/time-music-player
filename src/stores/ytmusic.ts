import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import QRCode from 'qrcode';
import type { Playlist, Track } from '@/types/music';

export interface YTUser {
  channelId: string;
  name: string;
  email?: string;
  avatarUrl: string;
  isPremium: boolean;
}

interface YTDeviceCodeResult {
  device_code: string;
  user_code: string;
  verification_url: string;
  interval: number;
  expires_in: number;
}

interface YTDeviceCheckResult {
  status: 'waiting' | 'success' | 'expired' | 'error';
  access_token?: string;
  refresh_token?: string;
  message: string;
}

interface YTUserProfile {
  channel_id: string;
  name: string;
  email?: string;
  avatar_url: string;
  is_premium: boolean;
}

interface YTPlaylistSummary {
  id: string;
  name: string;
  cover?: string;
  count: number;
  is_liked_songs: boolean;
}

let pollTimer: any = null;

export const useYTMusicStore = defineStore('ytmusic', {
  state: () => ({
    isLoggedIn: false,
    user: null as YTUser | null,
    showLoginModal: false,
    loginMode: 'device' as 'device' | 'cookie',
    deviceCode: '',
    userCode: '',
    verificationUrl: '',
    qrDataUrl: '',
    deviceStatus: 'idle' as 'idle' | 'waiting' | 'confirmed' | 'expired',
    deviceMessage: '正在请求 Google 授权设备码...',
    userPlaylists: [] as Playlist[],
    isSearching: false,
    searchResults: [] as Track[],
  }),

  actions: {
    async initYTMusic() {
      const savedToken = localStorage.getItem('time_music_yt_token');
      const savedCookie = localStorage.getItem('time_music_yt_cookie');

      if (savedToken || savedCookie) {
        try {
          if (savedToken) {
            await invoke('ytmusic_set_token', { token: savedToken });
          } else if (savedCookie) {
            await invoke('ytmusic_set_cookie', { cookie: savedCookie });
          }

          const profile = await invoke<YTUserProfile>('ytmusic_get_user_profile');
          if (profile && profile.name) {
            this.isLoggedIn = true;
            this.user = {
              channelId: profile.channel_id,
              name: profile.name,
              email: profile.email,
              avatarUrl: profile.avatar_url,
              isPremium: profile.is_premium,
            };
            this.loadUserPlaylists();
          }
        } catch {
          this.isLoggedIn = false;
          this.user = null;
        }
      }
    },

    async openLoginModal() {
      this.showLoginModal = true;
      this.loginMode = 'device';
      this.deviceStatus = 'idle';
      this.deviceMessage = '正在请求 Google 授权设备码...';
      await this.generateDeviceCode();
    },

    closeLoginModal() {
      this.showLoginModal = false;
      this.stopPolling();
    },

    async generateDeviceCode() {
      this.stopPolling();
      try {
        const res = await invoke<YTDeviceCodeResult>('ytmusic_get_device_code');
        this.deviceCode = res.device_code;
        this.userCode = res.user_code;
        this.verificationUrl = res.verification_url;

        // Generate QR code for phone scanning
        const qrUrl = `${res.verification_url}?user_code=${res.user_code}`;
        this.qrDataUrl = await QRCode.toDataURL(qrUrl, {
          width: 200,
          margin: 1,
          color: {
            dark: '#0f0f14',
            light: '#ffffff',
          },
        });

        this.deviceStatus = 'waiting';
        this.deviceMessage = `请在手机或浏览器打开 ${res.verification_url} 并输入授权码`;
        this.startPolling(res.device_code, res.interval || 5);
      } catch {
        this.deviceStatus = 'expired';
        this.deviceMessage = 'Google 授权服务直连受限，可切换至「Cookie / 凭据登录」或使用一键授权';
      }
    },

    startPolling(deviceCode: string, intervalSecs: number) {
      this.stopPolling();
      const intervalMs = Math.max(3000, intervalSecs * 1000);

      pollTimer = setInterval(async () => {
        try {
          const check = await invoke<YTDeviceCheckResult>('ytmusic_check_device_code', {
            deviceCode,
          });

          if (check.status === 'success' && check.access_token) {
            this.stopPolling();
            this.deviceStatus = 'confirmed';
            this.deviceMessage = 'Google 授权成功！正在载入 YouTube Music 用户资料...';

            localStorage.setItem('time_music_yt_token', check.access_token);
            if (check.refresh_token) {
              localStorage.setItem('time_music_yt_refresh_token', check.refresh_token);
            }

            const profile = await invoke<YTUserProfile>('ytmusic_get_user_profile');
            this.isLoggedIn = true;
            this.user = {
              channelId: profile.channel_id,
              name: profile.name,
              email: profile.email,
              avatarUrl: profile.avatar_url,
              isPremium: profile.is_premium,
            };

            await this.loadUserPlaylists();
            setTimeout(() => {
              this.closeLoginModal();
            }, 1000);
          } else if (check.status === 'expired') {
            this.stopPolling();
            this.deviceStatus = 'expired';
            this.deviceMessage = check.message;
          } else if (check.status === 'waiting') {
            this.deviceMessage = check.message;
          }
        } catch {
          // Keep polling until expire
        }
      }, intervalMs);
    },

    stopPolling() {
      if (pollTimer) {
        clearInterval(pollTimer);
        pollTimer = null;
      }
    },

    async loginWithCookie(cookieStr: string) {
      const clean = cookieStr.trim();
      if (!clean) return;

      try {
        await invoke('ytmusic_set_cookie', { cookie: clean });
        localStorage.setItem('time_music_yt_cookie', clean);

        const profile = await invoke<YTUserProfile>('ytmusic_get_user_profile');
        this.isLoggedIn = true;
        this.user = {
          channelId: profile.channel_id,
          name: profile.name,
          email: profile.email,
          avatarUrl: profile.avatar_url,
          isPremium: profile.is_premium,
        };

        await this.loadUserPlaylists();
        this.closeLoginModal();
      } catch (err: any) {
        this.deviceMessage = `Cookie 认证失败: ${err?.toString() || '请确认 Cookie 有效性'}`;
      }
    },

    async simulateQuickLogin() {
      // 1-Click instant authorization for testing / offline demo
      const mockProfile: YTUserProfile = {
        channel_id: 'yt-channel-simulated',
        name: 'YouTube Music Premium',
        email: 'user-connected@youtube.music',
        avatar_url: 'https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?w=100&auto=format&fit=crop&q=80',
        is_premium: true,
      };

      await invoke('ytmusic_set_token', { token: 'mock-youtube-token' });
      localStorage.setItem('time_music_yt_token', 'mock-youtube-token');

      this.isLoggedIn = true;
      this.user = {
        channelId: mockProfile.channel_id,
        name: mockProfile.name,
        email: mockProfile.email,
        avatarUrl: mockProfile.avatar_url,
        isPremium: mockProfile.is_premium,
      };

      await this.loadUserPlaylists();
      this.closeLoginModal();
    },

    async loadUserPlaylists() {
      try {
        const summaries = await invoke<YTPlaylistSummary[]>('ytmusic_get_user_playlists');
        this.userPlaylists = summaries.map((s) => ({
          id: `yt-${s.id}`,
          name: s.name,
          description: s.is_liked_songs ? 'YouTube Music 我喜欢的音乐同步' : 'YouTube Music 歌单',
          cover: s.cover || 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=300&auto=format&fit=crop&q=80',
          trackIds: [],
          trackCount: s.count,
          createdAt: Date.now(),
          tracks: [],
          isCloud: true,
          platform: 'youtube',
        }));
      } catch (e) {
        console.warn('Failed to load YouTube Music playlists:', e);
      }
    },

    logout() {
      invoke('ytmusic_logout').catch(() => {});
      localStorage.removeItem('time_music_yt_token');
      localStorage.removeItem('time_music_yt_refresh_token');
      localStorage.removeItem('time_music_yt_cookie');
      this.isLoggedIn = false;
      this.user = null;
      this.userPlaylists = [];
      this.stopPolling();
    },
  },
});
