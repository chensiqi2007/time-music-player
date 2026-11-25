<template>
  <div class="h-screen w-screen flex flex-col bg-theme-base text-theme-text overflow-hidden font-sans select-none relative transition-colors duration-300">
    <!-- Ambient Floating Glow Mesh -->
    <div v-if="settingsStore.enableAmbientGlow" class="ambient-glow-wrapper">
      <div class="ambient-orb ambient-orb-1"></div>
      <div class="ambient-orb ambient-orb-2"></div>
    </div>

    <!-- Window TitleBar -->
    <TitleBar />

    <!-- Main Workspace (Sidebar + Router Content) -->
    <div class="flex-1 flex overflow-hidden pt-11 pb-24 relative z-10">
      <!-- Left Sidebar Navigation -->
      <Sidebar />

      <!-- Center Main Content Area -->
      <main class="flex-1 overflow-y-auto px-8 py-6 relative">
        <router-view v-slot="{ Component }">
          <transition :name="settingsStore.enablePageAnimation ? 'page-slide' : ''" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </main>
    </div>

    <!-- Persistent Bottom Player Bar -->
    <BottomPlayer />

    <!-- Popups & Fullscreen Modals -->
    <FullscreenLyrics />
    <QueueDrawer />
    <PassportLoginModal />
    <NeteaseLoginModal />
    <QQLoginModal />
    <YouTubeLoginModal />
    <ExtraPlatformLoginModal />
    <PlaylistEditModal />
    <PlaylistDeleteModal />
    <ImportPlaylistModal />
    <AddToPlaylistModal />
    <AppContextMenu />

    <!-- Global Floating Toast Notification -->
    <Transition name="toast">
      <div
        v-if="playerStore.toastMessage"
        class="fixed bottom-24 left-1/2 -translate-x-1/2 z-50 px-5 py-2.5 rounded-2xl bg-[#1c1c24]/95 text-white border border-white/20 shadow-2xl backdrop-blur-2xl flex items-center gap-2.5 text-xs pointer-events-none select-none font-medium animate-in fade-in zoom-in-95 duration-200"
      >
        <Info :size="15" class="text-brand shrink-0" />
        <span>{{ playerStore.toastMessage }}</span>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, watch } from 'vue';
import { Info } from 'lucide-vue-next';
import TitleBar from '@/components/TitleBar.vue';
import Sidebar from '@/components/Sidebar.vue';
import BottomPlayer from '@/components/BottomPlayer.vue';
import FullscreenLyrics from '@/components/FullscreenLyrics.vue';
import QueueDrawer from '@/components/QueueDrawer.vue';
import PassportLoginModal from '@/components/PassportLoginModal.vue';
import NeteaseLoginModal from '@/components/NeteaseLoginModal.vue';
import QQLoginModal from '@/components/QQLoginModal.vue';
import YouTubeLoginModal from '@/components/YouTubeLoginModal.vue';
import ExtraPlatformLoginModal from '@/components/ExtraPlatformLoginModal.vue';
import PlaylistEditModal from '@/components/PlaylistEditModal.vue';
import PlaylistDeleteModal from '@/components/PlaylistDeleteModal.vue';
import ImportPlaylistModal from '@/components/ImportPlaylistModal.vue';
import AddToPlaylistModal from '@/components/AddToPlaylistModal.vue';
import AppContextMenu from '@/components/AppContextMenu.vue';
import { usePlayerStore } from '@/stores/player';
import { useLibraryStore } from '@/stores/library';
import { usePlaylistStore } from '@/stores/playlist';
import { useNeteaseStore } from '@/stores/netease';
import { useQQMusicStore } from '@/stores/qqmusic';
import { useYTMusicStore } from '@/stores/ytmusic';
import { useExtraPlatformsStore } from '@/stores/extraPlatforms';
import { useSettingsStore } from '@/stores/settings';
import { useAiStore } from '@/stores/ai';
import { usePassportStore } from '@/stores/passport';
import { useContextMenuStore } from '@/stores/contextMenu';
import { initMediaSession, syncMediaSessionState } from '@/utils/mediaSession';

const playerStore = usePlayerStore();
const libraryStore = useLibraryStore();
const playlistStore = usePlaylistStore();
const neteaseStore = useNeteaseStore();
const qqStore = useQQMusicStore();
const ytStore = useYTMusicStore();
const extraStore = useExtraPlatformsStore();
const settingsStore = useSettingsStore();
const aiStore = useAiStore();
const passportStore = usePassportStore();
const contextMenuStore = useContextMenuStore();

// Watch player state and sync with Windows SMTC / Linux MPRIS
watch(
  () => [playerStore.currentTrack, playerStore.isPlaying],
  () => {
    syncMediaSessionState(playerStore.currentTrack, playerStore.isPlaying);
  },
  { immediate: true }
);

// Global hotkeys listener
const handleKeyDown = (e: KeyboardEvent) => {
  // If typing in input, don't trigger global shortcuts
  const target = e.target as HTMLElement;
  if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA') {
    return;
  }

  // Space: Play / Pause
  if (e.code === 'Space') {
    e.preventDefault();
    playerStore.togglePlay();
  }
  // Ctrl + ArrowRight: Next Track
  else if (e.ctrlKey && e.code === 'ArrowRight') {
    e.preventDefault();
    playerStore.nextTrack();
  }
  // Ctrl + ArrowLeft: Prev Track
  else if (e.ctrlKey && e.code === 'ArrowLeft') {
    e.preventDefault();
    playerStore.prevTrack();
  }
  // Ctrl + ArrowUp: Vol Up
  else if (e.ctrlKey && e.code === 'ArrowUp') {
    e.preventDefault();
    playerStore.setVolume(playerStore.volume + 0.05);
  }
  // Ctrl + ArrowDown: Vol Down
  else if (e.ctrlKey && e.code === 'ArrowDown') {
    e.preventDefault();
    playerStore.setVolume(playerStore.volume - 0.05);
  }
  // Escape: Close Fullscreen Lyrics, Queue, or Context Menu
  else if (e.code === 'Escape') {
    if (contextMenuStore.visible) {
      contextMenuStore.close();
    }
    if (playerStore.showFullscreenLyrics) {
      playerStore.showFullscreenLyrics = false;
    }
    if (playerStore.showQueueDrawer) {
      playerStore.showQueueDrawer = false;
    }
  }
};

const handleGlobalContextMenu = (e: MouseEvent) => {
  e.preventDefault();
  const target = e.target as HTMLElement | null;
  if (target && (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA')) {
    contextMenuStore.openInputMenu(e, target as HTMLInputElement | HTMLTextAreaElement);
    return;
  }
  contextMenuStore.openGeneralMenu(e);
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
  window.addEventListener('contextmenu', handleGlobalContextMenu);
  settingsStore.initSettings();
  playerStore.initAudioEvents();
  libraryStore.initLibrary();
  playlistStore.initPlaylists();
  neteaseStore.initNetease();
  qqStore.initQQMusic();
  ytStore.initYTMusic();
  extraStore.initExtraPlatforms();
  aiStore.initAi();
  passportStore.initPassport();
  initMediaSession(playerStore);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
  window.removeEventListener('contextmenu', handleGlobalContextMenu);
});
</script>
