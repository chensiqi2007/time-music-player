import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { usePlayerStore } from './player';

export type ExtraPlatformId = 'kuwo' | 'kugou' | 'qishui';

export interface ExtraPlatformMeta {
  id: ExtraPlatformId;
  name: string;
  webUrl: string;
  cookieHint: string;
  accentClass: string;
  badgeText: string;
}

export const EXTRA_PLATFORMS: Record<ExtraPlatformId, ExtraPlatformMeta> = {
  kuwo: {
    id: 'kuwo',
    name: '酷我音乐',
    webUrl: 'https://www.kuwo.cn/',
    cookieHint: '打开 www.kuwo.cn 并登录账号，按 F12 → Application → Cookies 复制完整 Cookie 或 kw_token',
    accentClass: 'amber',
    badgeText: 'VIP / 会员直链已解锁',
  },
  kugou: {
    id: 'kugou',
    name: '酷狗音乐',
    webUrl: 'https://www.kugou.com/',
    cookieHint: '打开 www.kugou.com 并登录账号，按 F12 → Application → Cookies 复制完整 Cookie（含 kg_mid / t / userid）',
    accentClass: 'sky',
    badgeText: '酷狗 VIP 会员已激活',
  },
  qishui: {
    id: 'qishui',
    name: '汽水音乐',
    webUrl: 'https://music.douyin.com/',
    cookieHint: '打开 music.douyin.com 并扫码登录抖音账号，按 F12 → Application → Cookies 复制完整 Cookie（含 sessionid / sid_tt）',
    accentClass: 'teal',
    badgeText: '汽水 / 抖音会员已同步',
  },
};

const STORAGE_KEYS: Record<ExtraPlatformId, string> = {
  kuwo: 'time_music_kuwo_cookie',
  kugou: 'time_music_kugou_cookie',
  qishui: 'time_music_qishui_cookie',
};

const NICKNAME_KEYS: Record<ExtraPlatformId, string> = {
  kuwo: 'time_music_kuwo_nick',
  kugou: 'time_music_kugou_nick',
  qishui: 'time_music_qishui_nick',
};

export const useExtraPlatformsStore = defineStore('extraPlatforms', {
  state: () => ({
    showLoginModal: false,
    activeTab: 'kuwo' as ExtraPlatformId,
    accounts: {
      kuwo: { isLoggedIn: false, nickname: '酷我会员用户', cookie: '' },
      kugou: { isLoggedIn: false, nickname: '酷狗会员用户', cookie: '' },
      qishui: { isLoggedIn: false, nickname: '汽水音乐用户', cookie: '' },
    } as Record<ExtraPlatformId, { isLoggedIn: boolean; nickname: string; cookie: string }>,
  }),

  actions: {
    async initExtraPlatforms() {
      for (const id of ['kuwo', 'kugou', 'qishui'] as ExtraPlatformId[]) {
        const savedCookie = localStorage.getItem(STORAGE_KEYS[id]) || '';
        const savedNick = localStorage.getItem(NICKNAME_KEYS[id]);
        if (savedNick) {
          this.accounts[id].nickname = savedNick;
        }
        if (savedCookie) {
          this.accounts[id].cookie = savedCookie;
          this.accounts[id].isLoggedIn = true;
          try {
            await invoke(`${id}_set_cookie`, { cookie: savedCookie });
          } catch {
            // Ignored outside Tauri
          }
        }
      }
    },

    openLoginModal(platform: ExtraPlatformId = 'kuwo') {
      this.activeTab = platform;
      this.showLoginModal = true;
    },

    closeLoginModal() {
      this.showLoginModal = false;
    },

    async loginWithCookie(platform: ExtraPlatformId, cookie: string, nickname?: string) {
      const trimmed = cookie.trim();
      if (!trimmed) return false;

      const nick = nickname?.trim() || this.accounts[platform].nickname;
      localStorage.setItem(STORAGE_KEYS[platform], trimmed);
      localStorage.setItem(NICKNAME_KEYS[platform], nick);

      this.accounts[platform] = {
        isLoggedIn: true,
        nickname: nick,
        cookie: trimmed,
      };

      try {
        await invoke(`${platform}_set_cookie`, { cookie: trimmed });
      } catch {
        // Ignored
      }

      const playerStore = usePlayerStore();
      playerStore.showToast(`已保存 ${EXTRA_PLATFORMS[platform].name} 登录凭证，会员曲库已启用`);
      return true;
    },

    async openOfficialWindow(platform: ExtraPlatformId) {
      await invoke('open_platform_login_window', { platform });
    },

    async syncFromOfficialWindow(platform: ExtraPlatformId, nickname?: string): Promise<string> {
      const cookie = await invoke<string>('sync_platform_window_cookies', { platform });
      await this.loginWithCookie(platform, cookie, nickname);
      return cookie;
    },

    async logout(platform: ExtraPlatformId) {
      localStorage.removeItem(STORAGE_KEYS[platform]);
      this.accounts[platform].isLoggedIn = false;
      this.accounts[platform].cookie = '';
      try {
        await invoke(`${platform}_logout`);
      } catch {
        // Ignored
      }
      const playerStore = usePlayerStore();
      playerStore.showToast(`已退出 ${EXTRA_PLATFORMS[platform].name} 账号`);
    },
  },
});
