import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { openUrl } from '@tauri-apps/plugin-opener';
import { usePlayerStore } from './player';

export interface PassportUser {
  id: string;
  username: string;
  nickname: string;
  email?: string;
  avatarUrl?: string;
  role?: string;
}

interface PassportUserApi {
  id: string;
  username: string;
  nickname: string;
  email?: string;
  avatar_url?: string;
  role?: string;
}

interface PassportAuthResultApi {
  access_token: string;
  refresh_token?: string;
  user: PassportUserApi;
}

interface PassportLoginStartApi {
  auth_url: string;
  redirect_uri: string;
  state: string;
}

interface PassportCallbackPayload {
  code?: string;
  state?: string;
  redirect_uri: string;
  error?: string;
}

const TOKEN_KEY = 'time_music_passport_token';
const REFRESH_KEY = 'time_music_passport_refresh';
const USER_KEY = 'time_music_passport_user';
const SECRET_KEY = 'time_music_passport_client_secret';

export const PASSPORT_CLIENT_ID = 'app-xpnsfgs9';

function mapUser(u: PassportUserApi): PassportUser {
  return {
    id: u.id,
    username: u.username,
    nickname: u.nickname || u.username,
    email: u.email,
    avatarUrl: u.avatar_url,
    role: u.role,
  };
}

export const usePassportStore = defineStore('passport', {
  state: () => ({
    isLoggedIn: false,
    user: null as PassportUser | null,
    accessToken: localStorage.getItem(TOKEN_KEY) || '',
    refreshToken: localStorage.getItem(REFRESH_KEY) || '',
    clientSecret: localStorage.getItem(SECRET_KEY) || '',
    showLoginModal: false,
    loginStatus: 'idle' as 'idle' | 'waiting' | 'exchanging' | 'success' | 'error',
    statusMessage: '',
    authUrl: '',
    pendingRedirectUri: '',
    pendingState: '',
    listenerRegistered: false,
  }),

  actions: {
    async initPassport() {
      if (this.clientSecret) {
        try {
          await invoke('passport_set_client_secret', { secret: this.clientSecret });
        } catch {
          // Ignored
        }
      }

      if (!this.listenerRegistered) {
        this.listenerRegistered = true;
        try {
          await listen<PassportCallbackPayload>('passport:callback', async (event) => {
            const payload = event.payload;
            if (payload.error) {
              this.loginStatus = 'error';
              this.statusMessage = payload.error;
              return;
            }
            if (payload.code) {
              await this.exchangeCode(payload.code, payload.redirect_uri);
            }
          });
        } catch {
          // Running outside Tauri webview
        }
      }

      const savedUser = localStorage.getItem(USER_KEY);
      if (this.accessToken && savedUser) {
        try {
          this.user = JSON.parse(savedUser);
          this.isLoggedIn = true;
          const fresh = await invoke<PassportUserApi>('passport_get_user_info', {
            token: this.accessToken,
          });
          this.user = mapUser(fresh);
          localStorage.setItem(USER_KEY, JSON.stringify(this.user));
        } catch {
          if (this.refreshToken) {
            try {
              const refreshed = await invoke<PassportAuthResultApi>('passport_refresh_token', {
                refreshToken: this.refreshToken,
              });
              this.applyAuthResult(refreshed);
              return;
            } catch {
              this.clearSession();
            }
          } else {
            this.clearSession();
          }
        }
      }
    },

    openLoginModal() {
      this.showLoginModal = true;
      if (this.loginStatus === 'success') {
        this.loginStatus = 'idle';
        this.statusMessage = '';
      }
    },

    closeLoginModal() {
      this.showLoginModal = false;
    },

    async saveClientSecret(secret: string) {
      this.clientSecret = secret.trim();
      if (this.clientSecret) {
        localStorage.setItem(SECRET_KEY, this.clientSecret);
      } else {
        localStorage.removeItem(SECRET_KEY);
      }
      try {
        await invoke('passport_set_client_secret', {
          secret: this.clientSecret || null,
        });
      } catch {
        // Ignored
      }
    },

    async startSsoLogin() {
      this.loginStatus = 'waiting';
      this.statusMessage = '正在打开 URL Time Passport 认证页面...';

      try {
        const res = await invoke<PassportLoginStartApi>('passport_start_login');
        this.authUrl = res.auth_url;
        this.pendingRedirectUri = res.redirect_uri;
        this.pendingState = res.state;
        this.statusMessage = '请在浏览器中完成登录并授权，授权后将自动返回客户端';

        try {
          await openUrl(res.auth_url);
        } catch {
          window.open(res.auth_url, '_blank');
        }
      } catch (err: any) {
        this.loginStatus = 'error';
        this.statusMessage = typeof err === 'string' ? err : err?.message || '启动 SSO 登录失败';
      }
    },

    async reopenBrowser() {
      if (!this.authUrl) {
        await this.startSsoLogin();
        return;
      }
      try {
        await openUrl(this.authUrl);
      } catch {
        window.open(this.authUrl, '_blank');
      }
    },

    async completeFromManualInput(input: string) {
      const trimmed = input.trim();
      if (!trimmed) {
        this.loginStatus = 'error';
        this.statusMessage = '请输入回调 URL 或授权码';
        return;
      }

      let code = trimmed;
      let redirectUri = this.pendingRedirectUri || undefined;

      if (trimmed.startsWith('http://') || trimmed.startsWith('https://')) {
        try {
          const url = new URL(trimmed);
          const urlCode = url.searchParams.get('code');
          if (!urlCode) {
            this.loginStatus = 'error';
            this.statusMessage = '回调 URL 中未找到 code 参数';
            return;
          }
          code = urlCode;
          redirectUri = `${url.origin}${url.pathname}`;
        } catch {
          this.loginStatus = 'error';
          this.statusMessage = '回调 URL 格式无效';
          return;
        }
      }

      await this.exchangeCode(code, redirectUri);
    },

    async exchangeCode(code: string, redirectUri?: string) {
      this.loginStatus = 'exchanging';
      this.statusMessage = '正在验证授权凭证并同步用户资料...';

      try {
        const res = await invoke<PassportAuthResultApi>('passport_exchange_code', {
          code,
          redirectUri: redirectUri || this.pendingRedirectUri || null,
          clientSecret: this.clientSecret || null,
        });
        this.applyAuthResult(res);
        this.loginStatus = 'success';
        this.statusMessage = `欢迎回来，${this.user?.nickname || 'Time 用户'}！`;

        const playerStore = usePlayerStore();
        playerStore.showToast(`已登录 URL Time Passport：${this.user?.nickname}`);

        setTimeout(() => {
          this.showLoginModal = false;
        }, 900);
      } catch (err: any) {
        this.loginStatus = 'error';
        this.statusMessage = typeof err === 'string' ? err : err?.message || '登录凭证验证失败';
      }
    },

    applyAuthResult(res: PassportAuthResultApi) {
      this.accessToken = res.access_token;
      this.refreshToken = res.refresh_token || '';
      this.user = mapUser(res.user);
      this.isLoggedIn = true;

      localStorage.setItem(TOKEN_KEY, this.accessToken);
      if (this.refreshToken) {
        localStorage.setItem(REFRESH_KEY, this.refreshToken);
      } else {
        localStorage.removeItem(REFRESH_KEY);
      }
      localStorage.setItem(USER_KEY, JSON.stringify(this.user));
    },

    clearSession() {
      this.isLoggedIn = false;
      this.user = null;
      this.accessToken = '';
      this.refreshToken = '';
      localStorage.removeItem(TOKEN_KEY);
      localStorage.removeItem(REFRESH_KEY);
      localStorage.removeItem(USER_KEY);
    },

    async logout() {
      const token = this.accessToken;
      this.clearSession();
      try {
        await invoke('passport_logout', { token: token || null });
      } catch {
        // Ignored
      }
      const playerStore = usePlayerStore();
      playerStore.showToast('已退出 URL Time Passport 账号');
    },
  },
});
