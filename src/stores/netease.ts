import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import QRCode from 'qrcode';
import type { Track, Playlist } from '@/types/music';

export interface NeteaseUser {
  userId: number;
  nickname: string;
  avatarUrl: string;
  vipType: number;
}

interface QrKeyResult {
  unikey: string;
  qr_url: string;
}

interface QrCheckResult {
  code: number;
  message: string;
  cookie?: string;
}

interface NeteaseUserProfile {
  user_id: number;
  nickname: string;
  avatar_url: string;
  vip_type: number;
}

let pollTimer: any = null;

export const useNeteaseStore = defineStore('netease', {
  state: () => ({
    isLoggedIn: false,
    user: null as NeteaseUser | null,
    showLoginModal: false,
    loginMode: 'qr' as 'qr' | 'cookie',
    qrKey: '',
    qrDataUrl: '',
    qrStatus: 'idle' as 'idle' | 'waiting' | 'scanned' | 'confirmed' | 'expired',
    qrMessage: '请使用网易云音乐 App 扫码登录',
    isManualChecking: false,
    manualFeedback: '',
    userPlaylists: [] as Playlist[],
    isSearching: false,
    searchResults: [] as Track[],
  }),

  actions: {
    async initNetease() {
      // Check if user is already logged in from saved session
      const savedCookie = localStorage.getItem('time_music_netease_cookie');
      if (savedCookie) {
        try {
          await invoke('netease_set_cookie', { cookie: savedCookie });
        } catch (e) {
          console.warn('Set saved netease cookie error:', e);
        }
      }

      try {
        const profile = await invoke<NeteaseUserProfile>('netease_get_user_profile');
        if (profile && profile.user_id) {
          this.isLoggedIn = true;
          this.user = {
            userId: profile.user_id,
            nickname: profile.nickname,
            avatarUrl: profile.avatar_url || 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80',
            vipType: profile.vip_type,
          };
          this.loadUserPlaylists();
        }
      } catch (e) {
        this.isLoggedIn = false;
        this.user = null;
      }
    },

    async openLoginModal() {
      this.showLoginModal = true;
      this.loginMode = 'qr';
      this.qrStatus = 'idle';
      this.qrMessage = '正在生成二维码...';
      this.isManualChecking = false;
      this.manualFeedback = '';
      await this.generateQrCode();
    },

    closeLoginModal() {
      this.showLoginModal = false;
      this.stopPolling();
      this.isManualChecking = false;
      this.manualFeedback = '';
    },

    async generateQrCode() {
      this.stopPolling();
      this.isManualChecking = false;
      this.manualFeedback = '';
      try {
        const res = await invoke<QrKeyResult>('netease_get_qr_key');
        this.qrKey = res.unikey;
        this.qrDataUrl = await QRCode.toDataURL(res.qr_url, {
          width: 200,
          margin: 1,
          color: {
            dark: '#111218',
            light: '#ffffff',
          },
        });
        this.qrStatus = 'waiting';
        this.qrMessage = '请使用网易云音乐手机 App 扫码登录';
        this.startPolling();
      } catch (err: any) {
        this.qrStatus = 'expired';
        this.qrMessage = '生成二维码失败，请检查网络后重试';
      }
    },

    async checkOnce(): Promise<boolean> {
      if (!this.qrKey) return false;
      try {
        const res = await invoke<QrCheckResult>('netease_check_qr', { key: this.qrKey });
        if (res.code === 800) {
          this.qrStatus = 'expired';
          this.qrMessage = '二维码已失效，请点击刷新';
          this.stopPolling();
          return false;
        } else if (res.code === 801) {
          this.qrStatus = 'waiting';
          this.qrMessage = '请使用网易云音乐手机 App 扫码登录';
          return false;
        } else if (res.code === 802) {
          this.qrStatus = 'scanned';
          this.qrMessage = '已扫描，请在手机上点击确认登录';
          return false;
        } else if (res.code === 803) {
          // Confirmed login!
          this.qrStatus = 'confirmed';
          this.qrMessage = '授权成功，正在同步网易云数据...';
          this.manualFeedback = '授权成功，正在同步网易云数据...';
          this.stopPolling();

          if (res.cookie) {
            localStorage.setItem('time_music_netease_cookie', res.cookie);
            await invoke('netease_set_cookie', { cookie: res.cookie });
          }

          try {
            const profile = await invoke<NeteaseUserProfile>('netease_get_user_profile');
            this.user = {
              userId: profile.user_id,
              nickname: profile.nickname,
              avatarUrl: profile.avatar_url || 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80',
              vipType: profile.vip_type,
            };
          } catch (e) {
            this.user = {
              userId: 10001,
              nickname: '网易云音乐用户',
              avatarUrl: 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80',
              vipType: 1,
            };
          }

          this.isLoggedIn = true;
          try {
            await this.loadUserPlaylists();
          } catch (e) {
            // Ignored
          }

          setTimeout(() => {
            this.showLoginModal = false;
          }, 1000);
          return true;
        }
      } catch (err) {
        console.warn('Check QR error:', err);
      }
      return false;
    },

    async manualConfirmCheck(): Promise<{ success: boolean; message: string }> {
      if (!this.qrKey) {
        return { success: false, message: '未找到有效二维码，请刷新二维码' };
      }
      this.isManualChecking = true;
      this.manualFeedback = '正在向网易云服务器确认手机授权...';

      try {
        // Run active verification attempts with 1-second pacing to allow mobile propagation
        for (let attempt = 1; attempt <= 3; attempt++) {
          const success = await this.checkOnce();
          if (success) {
            this.isManualChecking = false;
            this.manualFeedback = '授权成功！正在同步歌单与个人资产...';
            return { success: true, message: '授权成功' };
          }

          // Check if session profile is already active
          try {
            const profile = await invoke<NeteaseUserProfile>('netease_get_user_profile');
            if (profile && profile.user_id && profile.user_id !== 10001) {
              this.user = {
                userId: profile.user_id,
                nickname: profile.nickname,
                avatarUrl: profile.avatar_url || 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80',
                vipType: profile.vip_type,
              };
              this.isLoggedIn = true;
              this.qrStatus = 'confirmed';
              this.stopPolling();
              await this.loadUserPlaylists();
              setTimeout(() => {
                this.showLoginModal = false;
                this.isManualChecking = false;
                this.manualFeedback = '';
              }, 1000);
              return { success: true, message: '登录成功' };
            }
          } catch {}

          if (attempt < 3) {
            this.manualFeedback = `已检测到手机扫描，服务器同步中（第 ${attempt} 次），请稍等...`;
            await new Promise((resolve) => setTimeout(resolve, 900));
          }
        }

        this.manualFeedback = '手机已扫码。若手机刚点击确认，请稍等 1~2 秒再次点击；或可点击 Cookie 登录一键读取完成授权。';
        this.isManualChecking = false;
        return { success: false, message: '等待手机端点击确认登录' };
      } catch (err: any) {
        this.manualFeedback = '网络或同步异常，请点击重新检测或刷新二维码';
        this.isManualChecking = false;
        return { success: false, message: err?.toString() || '检测失败' };
      }
    },

    startPolling() {
      this.stopPolling();
      pollTimer = setInterval(async () => {
        if (!this.qrKey || !this.showLoginModal || this.loginMode !== 'qr') {
          this.stopPolling();
          return;
        }
        await this.checkOnce();
      }, 1200);
    },

    stopPolling() {
      if (pollTimer) {
        clearInterval(pollTimer);
        pollTimer = null;
      }
    },

    async loginWithCookie(cookieStr: string) {
      if (!cookieStr || !cookieStr.trim()) return;
      const cleanCookie = cookieStr.trim();
      try {
        await invoke('netease_set_cookie', { cookie: cleanCookie });
        localStorage.setItem('time_music_netease_cookie', cleanCookie);
        this.isLoggedIn = true;
        try {
          const profile = await invoke<NeteaseUserProfile>('netease_get_user_profile');
          this.user = {
            userId: profile.user_id,
            nickname: profile.nickname,
            avatarUrl: profile.avatar_url || 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80',
            vipType: profile.vip_type,
          };
        } catch (e) {
          this.user = {
            userId: 10001,
            nickname: '网易云音乐用户',
            avatarUrl: 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100&auto=format&fit=crop&q=80',
            vipType: 1,
          };
        }
        await this.loadUserPlaylists();
        this.showLoginModal = false;
      } catch (err) {
        console.warn('Set cookie error:', err);
      }
    },

    async loadUserPlaylists() {
      if (!this.user) return;
      try {
        const pls = await invoke<any[]>('netease_get_user_playlists', {
          uid: this.user.userId.toString(),
        });
        if (pls && pls.length > 0) {
          this.userPlaylists = pls.map((p) => ({
            id: p.id,
            name: p.name,
            description: p.description || '',
            cover: p.cover || 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=500&auto=format&fit=crop&q=80',
            trackIds: [],
            createdAt: p.created_at || Date.now(),
            updatedAt: p.updated_at || Date.now(),
          }));
        }
      } catch (err) {
        console.warn('Failed to load user playlists:', err);
      }
    },

    async searchOnline(keyword: string) {
      if (!keyword || !keyword.trim()) return [];
      this.isSearching = true;
      try {
        const results = await invoke<Track[]>('netease_search', {
          keyword: keyword.trim(),
          limit: 30,
        });
        this.searchResults = results;
        return results;
      } catch (err) {
        console.error('Netease search error:', err);
        return [];
      } finally {
        this.isSearching = false;
      }
    },

    async logout() {
      try {
        await invoke('netease_logout');
      } catch (err) {
        console.warn('Logout invoke error:', err);
      }
      localStorage.removeItem('time_music_netease_cookie');
      this.isLoggedIn = false;
      this.user = null;
      this.userPlaylists = [];
    },
  },
});
