import { ref, type App } from 'vue';
import zhCN from './locales/zh-CN';
import zhTW from './locales/zh-TW';
import enUS from './locales/en-US';

export type LocaleId = 'zh-CN' | 'zh-TW' | 'en-US';

export interface LocaleMeta {
  id: LocaleId;
  name: string;
  enName: string;
  flag: string;
  desc: string;
}

export const SUPPORTED_LOCALES: LocaleMeta[] = [
  {
    id: 'zh-CN',
    name: '简体中文',
    enName: 'Simplified Chinese',
    flag: '🇨🇳',
    desc: '中国大陆标准中文',
  },
  {
    id: 'zh-TW',
    name: '繁體中文',
    enName: 'Traditional Chinese',
    flag: '🇭🇰',
    desc: '港澳台標準繁體中文',
  },
  {
    id: 'en-US',
    name: 'English',
    enName: 'English (US)',
    flag: '🇺🇸',
    desc: 'International English',
  },
];

const dictionaries: Record<LocaleId, any> = {
  'zh-CN': zhCN,
  'zh-TW': zhTW,
  'en-US': enUS,
};

export const currentLocale = ref<LocaleId>('zh-CN');

export function initLocale() {
  const saved = localStorage.getItem('time_music_locale') as LocaleId;
  if (saved && dictionaries[saved]) {
    setLocale(saved);
    return;
  }

  // Auto-detect browser/OS language
  if (typeof navigator !== 'undefined') {
    const navLang = navigator.language || '';
    if (/zh-TW|zh-HK|zh-MO|zh-Hant/i.test(navLang)) {
      setLocale('zh-TW');
      return;
    }
    if (/zh/i.test(navLang)) {
      setLocale('zh-CN');
      return;
    }
    if (/en/i.test(navLang)) {
      setLocale('en-US');
      return;
    }
  }

  setLocale('zh-CN');
}

export function setLocale(id: LocaleId) {
  if (dictionaries[id]) {
    currentLocale.value = id;
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('time_music_locale', id);
    }
    if (typeof document !== 'undefined') {
      document.documentElement.lang = id;
    }
  }
}

export function t(path: string, params?: Record<string, string | number>): string {
  const lang = currentLocale.value;
  const dict = dictionaries[lang] || dictionaries['zh-CN'];
  const keys = path.split('.');

  let result: any = dict;
  for (const k of keys) {
    if (result && typeof result === 'object' && k in result) {
      result = result[k];
    } else {
      result = undefined;
      break;
    }
  }

  // Fallback to zh-CN if missing in current locale
  if (result === undefined && lang !== 'zh-CN') {
    let fallback: any = dictionaries['zh-CN'];
    for (const k of keys) {
      if (fallback && typeof fallback === 'object' && k in fallback) {
        fallback = fallback[k];
      } else {
        fallback = undefined;
        break;
      }
    }
    result = fallback;
  }

  if (typeof result !== 'string') {
    return path;
  }

  // Parameter interpolation: {key}
  if (params) {
    return result.replace(/\{(\w+)\}/g, (_, match) => {
      return params[match] !== undefined ? String(params[match]) : `{${match}}`;
    });
  }

  return result;
}

export function useI18n() {
  return {
    t,
    currentLocale,
    setLocale,
    SUPPORTED_LOCALES,
  };
}

export const i18nPlugin = {
  install(app: App) {
    app.config.globalProperties.$t = t;
    app.config.globalProperties.$locale = currentLocale;
    app.provide('i18n', {
      t,
      currentLocale,
      setLocale,
      SUPPORTED_LOCALES,
    });
  },
};

declare module 'vue' {
  interface ComponentCustomProperties {
    $t: typeof t;
    $locale: typeof currentLocale;
  }
}
