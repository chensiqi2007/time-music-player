<template>
  <Transition name="fade">
    <div
      v-if="menuStore.visible"
      :style="{ left: `${menuStore.x}px`, top: `${menuStore.y}px` }"
      @click.stop
      @contextmenu.prevent
      class="fixed z-[100] w-52 rounded-2xl liquid-glass-floating p-1.5 text-xs select-none"
    >
      <!-- Mode 1: Track Context Menu -->
      <template v-if="menuStore.mode === 'track' && menuStore.track">
        <div class="px-2.5 py-1.5 border-b border-white/[0.08] mb-1">
          <p class="font-semibold text-white truncate text-[11px]">{{ menuStore.track.title }}</p>
          <p class="text-[10px] text-slate-400 truncate">{{ menuStore.track.artist }}</p>
        </div>

        <button
          @click="playTrackNow"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-brand/20 hover:text-white transition-colors text-left"
        >
          <Play :size="13" class="text-brand shrink-0" />
          <span>立即播放</span>
        </button>

        <button
          @click="playTrackNext"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <ListStart :size="13" class="text-slate-400 shrink-0" />
          <span>下一首播放</span>
        </button>

        <button
          @click="appendToQueue"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <ListPlus :size="13" class="text-slate-400 shrink-0" />
          <span>添加到播放队列</span>
        </button>

        <button
          @click="openAddPlaylist"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <PlusCircle :size="13" class="text-slate-400 shrink-0" />
          <span>添加到歌单</span>
        </button>

        <button
          @click="toggleFav"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <Heart
            :size="13"
            class="shrink-0"
            :class="isFav ? 'text-brand' : 'text-slate-400'"
            :fill="isFav ? 'currentColor' : 'none'"
          />
          <span>{{ isFav ? '取消喜欢' : '添加到喜欢' }}</span>
        </button>

        <div class="my-1 border-t border-white/[0.06]"></div>

        <button
          @click="searchSameArtist"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <Search :size="13" class="text-slate-400 shrink-0" />
          <span>搜索同歌手作品</span>
        </button>
      </template>

      <!-- Mode 2: Input Edit Menu -->
      <template v-else-if="menuStore.mode === 'input' && menuStore.inputEl">
        <button
          @click="copyFromInput"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <Copy :size="13" class="text-slate-400 shrink-0" />
          <span>复制</span>
        </button>
        <button
          @click="pasteToInput"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <ClipboardPaste :size="13" class="text-slate-400 shrink-0" />
          <span>粘贴</span>
        </button>
        <button
          @click="selectAllInput"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <CheckSquare :size="13" class="text-slate-400 shrink-0" />
          <span>全选</span>
        </button>
      </template>

      <!-- Mode 3: Global Player Context Menu -->
      <template v-else>
        <button
          @click="togglePlay"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-brand/20 hover:text-white transition-colors text-left"
        >
          <Pause v-if="playerStore.isPlaying" :size="13" class="text-brand shrink-0" />
          <Play v-else :size="13" class="text-brand shrink-0" />
          <span>{{ playerStore.isPlaying ? '暂停播放' : '继续播放' }}</span>
        </button>

        <button
          @click="prevTrack"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <SkipBack :size="13" class="text-slate-400 shrink-0" />
          <span>上一首</span>
        </button>

        <button
          @click="nextTrack"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <SkipForward :size="13" class="text-slate-400 shrink-0" />
          <span>下一首</span>
        </button>

        <div class="my-1 border-t border-white/[0.06]"></div>

        <button
          @click="toggleQueue"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <ListMusic :size="13" class="text-slate-400 shrink-0" />
          <span>{{ playerStore.showQueueDrawer ? '隐藏播放队列' : '打开播放队列' }}</span>
        </button>

        <button
          @click="toggleLyrics"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <Mic2 :size="13" class="text-slate-400 shrink-0" />
          <span>全屏沉浸歌词</span>
        </button>

        <button
          @click="goSettings"
          class="w-full px-2.5 py-1.5 rounded-xl flex items-center gap-2.5 text-slate-200 hover:bg-white/10 hover:text-white transition-colors text-left"
        >
          <Settings :size="13" class="text-slate-400 shrink-0" />
          <span>系统设置</span>
        </button>
      </template>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import {
  Play,
  Pause,
  SkipBack,
  SkipForward,
  ListStart,
  ListPlus,
  PlusCircle,
  Heart,
  Search,
  ListMusic,
  Mic2,
  Settings,
  Copy,
  ClipboardPaste,
  CheckSquare,
} from 'lucide-vue-next';
import { useContextMenuStore } from '@/stores/contextMenu';
import { usePlayerStore } from '@/stores/player';
import { usePlaylistStore } from '@/stores/playlist';
import { useProviderStore } from '@/stores/provider';

const router = useRouter();
const menuStore = useContextMenuStore();
const playerStore = usePlayerStore();
const playlistStore = usePlaylistStore();
const providerStore = useProviderStore();

const isFav = computed(() => {
  if (!menuStore.track) return false;
  return menuStore.track.isFavorite || playerStore.isIdFavorite(menuStore.track.id);
});

const playTrackNow = () => {
  if (menuStore.track) {
    playerStore.playTrack(menuStore.track, menuStore.queueList);
  }
  menuStore.close();
};

const playTrackNext = () => {
  if (menuStore.track) {
    playerStore.playNext(menuStore.track);
  }
  menuStore.close();
};

const appendToQueue = () => {
  if (menuStore.track) {
    playerStore.addToQueue(menuStore.track);
  }
  menuStore.close();
};

const openAddPlaylist = () => {
  if (menuStore.track) {
    playlistStore.openAddToPlaylistModal(menuStore.track);
  }
  menuStore.close();
};

const toggleFav = () => {
  if (menuStore.track) {
    playerStore.toggleFavorite(menuStore.track);
  }
  menuStore.close();
};

const searchSameArtist = async () => {
  if (menuStore.track) {
    const artist = menuStore.track.artist.split('/')[0].trim();
    menuStore.close();
    router.push('/discover');
    await providerStore.performSearch(artist);
  }
};

const copyFromInput = async () => {
  const el = menuStore.inputEl;
  if (el) {
    const start = el.selectionStart ?? 0;
    const end = el.selectionEnd ?? 0;
    const text = start !== end ? el.value.slice(start, end) : el.value;
    try {
      await navigator.clipboard.writeText(text);
    } catch {}
  }
  menuStore.close();
};

const pasteToInput = async () => {
  const el = menuStore.inputEl;
  if (el) {
    try {
      const clip = await navigator.clipboard.readText();
      const start = el.selectionStart ?? el.value.length;
      const end = el.selectionEnd ?? el.value.length;
      el.value = el.value.slice(0, start) + clip + el.value.slice(end);
      el.dispatchEvent(new Event('input', { bubbles: true }));
    } catch {}
  }
  menuStore.close();
};

const selectAllInput = () => {
  if (menuStore.inputEl) {
    menuStore.inputEl.focus();
    menuStore.inputEl.select();
  }
  menuStore.close();
};

const togglePlay = () => {
  playerStore.togglePlay();
  menuStore.close();
};

const prevTrack = () => {
  playerStore.prevTrack();
  menuStore.close();
};

const nextTrack = () => {
  playerStore.nextTrack();
  menuStore.close();
};

const toggleQueue = () => {
  playerStore.toggleQueueDrawer();
  menuStore.close();
};

const toggleLyrics = () => {
  playerStore.toggleFullscreenLyrics();
  menuStore.close();
};

const goSettings = () => {
  router.push('/settings');
  menuStore.close();
};

const handleDismiss = () => {
  if (menuStore.visible) {
    menuStore.close();
  }
};

onMounted(() => {
  window.addEventListener('click', handleDismiss);
  window.addEventListener('scroll', handleDismiss, true);
  window.addEventListener('resize', handleDismiss);
});

onUnmounted(() => {
  window.removeEventListener('click', handleDismiss);
  window.removeEventListener('scroll', handleDismiss, true);
  window.removeEventListener('resize', handleDismiss);
});
</script>
