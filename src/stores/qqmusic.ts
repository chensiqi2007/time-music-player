import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { Playlist, Track } from '@/types/music';

export interface QQUser {
  uin: string;
  nickname: string;
  avatarUrl: string;
  vipType: number;
}

interface QQQrCodeResult {
  qrsig: string;
  qr_image: string;
}

interface QQQrCheckResult {
  code: number;
  message: string;
  uin?: string;
  nickname?: string;
  cookie?: string;
}

interface QQUserProfile {
  uin: string;
  nickname: string;
  avatar_url: string;
  vip_type: number;
}

interface QQPlaylistSummary {
  id: string;
  name: string;
  cover: string | null;
  count: number;
}

let pollTimer: any = null;

export const useQQMusicStore = defineStore('qqmusic', {
  state: () => ({
    isLoggedIn: false,
    user: null as QQUser | null,
    showLoginModal: false,
    loginMode: 'qr' as 'qr' | 'cookie',
    qrsig: '',
    qrDataUrl: '',
    qrStatus: 'idle' as 'idle' | 'waiting' | 'scanned' | 'confirmed' | 'expired',
    qrMessage: '请使用手机 QQ 或微信扫码授权登录',
    userPlaylists: [] as Playlist[],
    isSearching: false,
    searchResults: [] as Track[],
  }),

  actions: {
    async initQQMusic() {
      const savedUin = localStorage.getItem('time_music_qq_uin');
      const savedCookie = localStorage.getItem('time_music_qq_cookie');
      if (savedUin && savedCookie) {
        try {
          await invoke('qqmusic_set_cookie', { cookie: savedCookie });
          const profile = await invoke<QQUserProfile>('qqmusic_get_user_profile', { uin: savedUin });
          if (profile && profile.uin) {
            this.isLoggedIn = true;
            this.user = {
              uin: profile.uin,
              nickname: profile.nickname,
              avatarUrl: profile.avatar_url,
              vipType: profile.vip_type,
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
      this.loginMode = 'qr';
      this.qrStatus = 'idle';
      this.qrMessage = '正在生成二维码...';
      await this.generateQrCode();
    },

    closeLoginModal() {
      this.showLoginModal = false;
      this.stopPolling();
    },

    async generateQrCode() {
      this.stopPolling();
      try {
        const res = await invoke<QQQrCodeResult>('qqmusic_get_qr_key');
        this.qrsig = res.qrsig;
        this.qrDataUrl = res.qr_image;
        this.qrStatus = 'waiting';
        this.qrMessage = '请使用手机 QQ 或微信扫码授权';
        this.startPolling();
      } catch (err: any) {
        this.qrStatus = 'expired';
        this.qrMessage = '获取 QQ 登录二维码失败，请检查网络后重试';
      }
    },

    startPolling() {
      this.stopPolling();
      let pollCount = 0;
      pollTimer = setInterval(async () => {
        pollCount++;
        if (pollCount > 100) {
          this.qrStatus = 'expired';
          this.qrMessage = '二维码已失效，点击重新获取';
          this.stopPolling();
          return;
        }
        await this.checkStatus();
      }, 1800);
    },

    stopPolling() {
      if (pollTimer) {
        clearInterval(pollTimer);
        pollTimer = null;
      }
    },

    async checkStatus() {
      if (!this.qrsig) return;
      try {
        const res = await invoke<QQQrCheckResult>('qqmusic_check_qr', { qrsig: this.qrsig });
        if (res.code === 0) {
          // Success
          this.stopPolling();
          this.qrStatus = 'confirmed';
          this.qrMessage = '登录成功，正在同步资产...';
          const uin = res.uin || '888888';
          if (res.cookie) {
            localStorage.setItem('time_music_qq_cookie', res.cookie);
          }
          localStorage.setItem('time_music_qq_uin', uin);

          const profile = await invoke<QQUserProfile>('qqmusic_get_user_profile', { uin });
          this.isLoggedIn = true;
          this.user = {
            uin: profile.uin,
            nickname: res.nickname || profile.nickname,
            avatarUrl: profile.avatar_url,
            vipType: profile.vip_type,
          };

          await this.loadUserPlaylists();
          setTimeout(() => {
            this.closeLoginModal();
          }, 800);
        } else if (res.code === 67) {
          this.qrStatus = 'scanned';
          this.qrMessage = '已扫描，请在手机上确认授权登录';
        } else if (res.code === 65) {
          this.qrStatus = 'expired';
          this.qrMessage = '二维码已失效，点击刷新';
          this.stopPolling();
        } else if (res.code === 66) {
          this.qrStatus = 'waiting';
          this.qrMessage = '请使用手机 QQ 扫码授权登录';
        }
      } catch (err) {
        console.error('Check QQ QR status failed:', err);
      }
    },

    async manualCheck() {
      await this.checkStatus();
    },

    async loginWithCookie(cookieStr: string, manualUin?: string) {
      if (!cookieStr || !cookieStr.trim()) return;
      const c = cookieStr.trim();
      let uin = manualUin?.trim();
      if (!uin) {
        const match = c.match(/uin=(\d+)/);
        uin = match ? match[1] : '888888';
      }

      await invoke('qqmusic_set_cookie', { cookie: c });
      localStorage.setItem('time_music_qq_cookie', c);
      localStorage.setItem('time_music_qq_uin', uin);

      try {
        const profile = await invoke<QQUserProfile>('qqmusic_get_user_profile', { uin });
        this.isLoggedIn = true;
        this.user = {
          uin: profile.uin,
          nickname: profile.nickname,
          avatarUrl: profile.avatar_url,
          vipType: profile.vip_type,
        };
        await this.loadUserPlaylists();
        this.closeLoginModal();
      } catch (e) {
        console.error('Login with QQ cookie failed:', e);
      }
    },

    async loadUserPlaylists() {
      if (!this.user?.uin) return;
      try {
        const rows = await invoke<QQPlaylistSummary[]>('qqmusic_get_user_playlists', {
          uin: this.user.uin,
        });
        if (rows && rows.length > 0) {
          this.userPlaylists = rows.map((r) => ({
            id: r.id,
            name: r.name,
            description: `QQ 音乐同步歌单 • 共 ${r.count} 首`,
            cover: r.cover || 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=500&auto=format&fit=crop&q=80',
            trackIds: [],
            createdAt: Date.now(),
            updatedAt: Date.now(),
          }));
        }
      } catch (err) {
        console.warn('Load QQ user playlists failed:', err);
      }
    },

    async logout() {
      await invoke('qqmusic_logout');
      localStorage.removeItem('time_music_qq_cookie');
      localStorage.removeItem('time_music_qq_uin');
      this.isLoggedIn = false;
      this.user = null;
      this.userPlaylists = [];
    },
  },
});

