<template>
  <header
    data-tauri-drag-region
    @mousedown="onTitleBarMouseDown"
    @dblclick="onTitleBarDblClick"
    class="h-11 w-full flex items-center justify-between px-3 select-none border-b border-theme-border bg-theme-titlebar backdrop-blur-2xl z-50 fixed top-0 left-0 right-0 transition-colors duration-300"
  >
    <!-- Left: App Brand & Navigation Chevrons -->
    <div data-tauri-drag-region class="flex items-center gap-2.5">
      <!-- Time OS Music Logo -->
      <div data-tauri-drag-region class="flex items-center gap-2 pl-1 cursor-default">
        <img
          data-tauri-drag-region
          src="/logo.svg"
          alt="Time Music"
          class="w-6 h-6 rounded-lg shadow-md shadow-brand/25 shrink-0 pointer-events-none"
        />
        <span data-tauri-drag-region class="text-xs font-bold tracking-wider text-theme-text pointer-events-none">TIME MUSIC</span>
      </div>

      <!-- History Chevrons -->
      <div class="flex items-center gap-0.5 ml-2 text-slate-400">
        <button
          @click="router.back()"
          class="w-7 h-7 rounded-lg flex items-center justify-center hover:bg-theme-surfaceElevated hover:text-white transition-colors"
          title="后退"
        >
          <ChevronLeft :size="15" />
        </button>
        <button
          @click="router.forward()"
          class="w-7 h-7 rounded-lg flex items-center justify-center hover:bg-theme-surfaceElevated hover:text-white transition-colors"
          title="前进"
        >
          <ChevronRight :size="15" />
        </button>
      </div>
    </div>

    <!-- Center: Search Bar & Drag Area -->
    <div data-tauri-drag-region class="flex-1 flex justify-center max-w-md mx-4">
      <div class="relative w-full group">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 group-focus-within:text-brand w-3.5 h-3.5 transition-colors" />
        <input
          ref="searchInputRef"
          v-model="searchQuery"
          @input="onSearch"
          @keydown.enter="onEnter"
          @keydown.esc="onEscSearch"
          type="text"
          :placeholder="settingsStore.locale === 'en-US' ? 'Search library, Enter for online search...' : (settingsStore.locale === 'zh-TW' ? '搜尋本地資料庫，按 Enter 全網檢索...' : '搜索本地资料库，按 Enter 全网检索...')"
          class="w-full h-7 pl-8 pr-14 text-xs bg-theme-surface hover:bg-theme-surfaceElevated focus:bg-theme-surfaceElevated text-theme-text placeholder-slate-400 rounded-full border border-theme-border focus:border-brand/50 focus:ring-2 focus:ring-brand/15 focus:outline-none transition-all"
        />
        <kbd class="absolute right-2.5 top-1/2 -translate-y-1/2 text-[10px] text-slate-400 bg-theme-surfaceElevated border border-theme-subtle px-1.5 py-0.2 rounded font-mono pointer-events-none">
          Ctrl K
        </kbd>
      </div>
    </div>

    <!-- Right: Theme Picker & Window Controls -->
    <div class="flex items-center space-x-1.5">
      <!-- Quick Theme Switcher Dropdown Trigger -->
      <div class="relative">
        <button
          @click.stop="isThemeMenuOpen = !isThemeMenuOpen"
          class="w-7 h-7 rounded-lg flex items-center justify-center text-slate-400 hover:text-white hover:bg-white/10 transition-colors"
          :class="isThemeMenuOpen ? 'bg-white/15 text-brand' : ''"
          title="切换个性化主题样式"
        >
          <Palette :size="14" />
        </button>

        <!-- Theme Dropdown Popover -->
        <Transition name="fade">
          <div
            v-if="isThemeMenuOpen"
            class="absolute right-0 top-9 w-56 rounded-2xl liquid-glass-floating p-2 z-50 space-y-1 animate-in fade-in zoom-in-95 duration-150"
            @click.stop
          >
            <div class="px-2.5 py-1 text-[10px] font-bold text-slate-400 uppercase tracking-wider flex items-center justify-between border-b border-white/[0.06] pb-1.5 mb-1">
              <span>个性化主题</span>
              <span class="text-brand font-mono">{{ currentThemeName }}</span>
            </div>

            <div class="space-y-0.5 max-h-64 overflow-y-auto pr-0.5">
              <button
                v-for="t in THEMES"
                :key="t.id"
                @click="selectTheme(t.id)"
                class="w-full px-2.5 py-1.5 rounded-xl text-xs flex items-center justify-between transition-colors group text-left"
                :class="settingsStore.theme === t.id ? 'bg-brand/15 text-white font-semibold' : 'text-slate-300 hover:bg-white/10 hover:text-white'"
              >
                <div class="flex items-center gap-2.5 min-w-0">
                  <span
                    class="w-3.5 h-3.5 rounded-full border border-white/20 shrink-0 shadow-sm"
                    :style="{ backgroundColor: t.accentColor }"
                  ></span>
                  <div class="min-w-0">
                    <p class="truncate text-xs font-medium">{{ t.name }}</p>
                  </div>
                </div>
                <Check v-if="settingsStore.theme === t.id" :size="13" class="text-brand shrink-0 ml-1.5" stroke-width="2.5" />
              </button>
            </div>
          </div>
        </Transition>
      </div>

      <!-- Quick Language Switcher Dropdown Trigger -->
      <div class="relative">
        <button
          @click.stop="isLangMenuOpen = !isLangMenuOpen; isThemeMenuOpen = false"
          class="w-7 h-7 rounded-lg flex items-center justify-center text-slate-400 hover:text-white hover:bg-white/10 transition-colors"
          :class="isLangMenuOpen ? 'bg-white/15 text-brand' : ''"
          title="切换界面语言 (Language)"
        >
          <Languages :size="14" />
        </button>

        <!-- Language Dropdown Popover -->
        <Transition name="fade">
          <div
            v-if="isLangMenuOpen"
            class="absolute right-0 top-9 w-44 rounded-2xl liquid-glass-floating p-1.5 z-50 space-y-1 animate-in fade-in zoom-in-95 duration-150"
            @click.stop
          >
            <div class="px-2 py-1 text-[10px] font-bold text-slate-400 uppercase tracking-wider border-b border-white/[0.06] pb-1 mb-1 flex items-center justify-between">
              <span>语言 / Language</span>
            </div>

            <div class="space-y-0.5">
              <button
                v-for="loc in SUPPORTED_LOCALES"
                :key="loc.id"
                @click="selectLocale(loc.id)"
                class="w-full px-2.5 py-1.5 rounded-xl text-xs flex items-center justify-between transition-colors group text-left"
                :class="settingsStore.locale === loc.id ? 'bg-brand/15 text-brand font-semibold' : 'text-slate-300 hover:bg-white/10 hover:text-white'"
              >
                <div class="flex items-center gap-2">
                  <span class="text-sm">{{ loc.flag }}</span>
                  <span class="text-xs">{{ loc.name }}</span>
                </div>
                <Check v-if="settingsStore.locale === loc.id" :size="13" class="text-brand" />
              </button>
            </div>
          </div>
        </Transition>
      </div>

      <div class="h-3.5 w-[1px] bg-white/10 mx-1"></div>

      <button
        @click="minimizeWindow"
        class="w-8 h-8 rounded-md flex items-center justify-center text-slate-400 hover:text-slate-100 hover:bg-white/10 transition-colors"
        title="最小化"
      >
        <Minus :size="14" />
      </button>

      <button
        @click="toggleMaximize"
        class="w-8 h-8 rounded-md flex items-center justify-center text-slate-400 hover:text-slate-100 hover:bg-white/10 transition-colors"
        :title="isMaximized ? '还原' : '最大化'"
      >
        <Square :size="12" />
      </button>

      <button
        @click="closeWindow"
        class="w-8 h-8 rounded-md flex items-center justify-center text-slate-400 hover:text-white hover:bg-red-500/90 transition-colors"
        title="关闭"
      >
        <X :size="14" />
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import {
  ChevronLeft,
  ChevronRight,
  Search,
  Minus,
  Square,
  X,
  Palette,
  Check,
  Languages,
} from 'lucide-vue-next';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useLibraryStore } from '@/stores/library';
import { useProviderStore } from '@/stores/provider';
import { useSettingsStore, THEMES, type ThemeId } from '@/stores/settings';
import { SUPPORTED_LOCALES, type LocaleId } from '@/i18n';

const router = useRouter();
const libraryStore = useLibraryStore();
const providerStore = useProviderStore();
const settingsStore = useSettingsStore();
const searchInputRef = ref<HTMLInputElement | null>(null);
const searchQuery = ref('');
const isMaximized = ref(false);
const isThemeMenuOpen = ref(false);
const isLangMenuOpen = ref(false);

const currentThemeName = computed(() => {
  const current = THEMES.find((t) => t.id === settingsStore.theme);
  return current ? current.enName : 'Custom';
});

const selectTheme = (themeId: ThemeId) => {
  settingsStore.setTheme(themeId);
  isThemeMenuOpen.value = false;
};

const selectLocale = (id: LocaleId) => {
  settingsStore.setLanguage(id);
  isLangMenuOpen.value = false;
};

const handleWindowClick = () => {
  isThemeMenuOpen.value = false;
  isLangMenuOpen.value = false;
};

const handleGlobalKeyDown = (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault();
    searchInputRef.value?.focus();
    searchInputRef.value?.select();
  }
};

const onEscSearch = () => {
  if (searchQuery.value) {
    searchQuery.value = '';
    onSearch();
  } else {
    searchInputRef.value?.blur();
  }
};

const onSearch = () => {
  libraryStore.setSearchQuery(searchQuery.value);
};

const onEnter = async () => {
  if (searchQuery.value && searchQuery.value.trim()) {
    router.push('/discover');
    await providerStore.performSearch(searchQuery.value.trim());
  }
};

const isInteractiveTarget = (target: EventTarget | null): boolean => {
  const el = target as HTMLElement | null;
  if (!el) return false;
  return !!el.closest('button, input, textarea, select, a, [role="button"]');
};

const onTitleBarMouseDown = async (e: MouseEvent) => {
  if (e.button !== 0 || isInteractiveTarget(e.target)) return;
  try {
    await getCurrentWindow().startDragging();
  } catch {
    // Fallback to data-tauri-drag-region
  }
};

const onTitleBarDblClick = async (e: MouseEvent) => {
  if (isInteractiveTarget(e.target)) return;
  await toggleMaximize();
};

const minimizeWindow = async () => {
  try {
    await invoke('minimize_window');
  } catch (err) {
    console.error('Minimize error:', err);
  }
};

const toggleMaximize = async () => {
  try {
    const res = await invoke<boolean>('toggle_maximize');
    isMaximized.value = res;
  } catch (err) {
    console.error('Toggle maximize error:', err);
  }
};

const closeWindow = async () => {
  try {
    await invoke('close_window');
  } catch (err) {
    console.error('Close window error:', err);
  }
};

onMounted(async () => {
  window.addEventListener('click', handleWindowClick);
  window.addEventListener('keydown', handleGlobalKeyDown);
  try {
    isMaximized.value = await invoke<boolean>('is_maximized');
  } catch (err) {
    console.warn('Could not check window maximize state:', err);
  }
});

onUnmounted(() => {
  window.removeEventListener('click', handleWindowClick);
  window.removeEventListener('keydown', handleGlobalKeyDown);
});
</script>

