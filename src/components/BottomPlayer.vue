<template>
  <footer class="h-[74px] fixed bottom-2.5 left-3 right-3 rounded-2xl glass-player z-40 px-4 flex items-center justify-between select-none">
    <!-- Left: Track Cover & Meta -->
    <div class="flex items-center gap-3.5 w-1/4 min-w-[200px]">
      <div
        v-if="playerStore.currentTrack?.cover"
        @click="playerStore.toggleFullscreenLyrics"
        class="relative w-12 h-12 rounded-xl overflow-hidden shadow-lg shadow-black/40 group cursor-pointer shrink-0 border border-white/10 transition-all duration-300"
        :class="playerStore.isPlaying ? 'ring-2 ring-brand/50 shadow-brand/25' : ''"
      >
        <img
          :src="playerStore.currentTrack.cover"
          alt="Cover"
          decoding="async"
          class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
          :class="playerStore.isPlaying ? 'scale-[1.03]' : ''"
        />
        <div class="absolute inset-0 bg-black/30 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
          <Maximize2 :size="16" class="text-white" />
        </div>
      </div>
      <div v-else class="w-12 h-12 rounded-xl bg-white/5 border border-white/10 flex items-center justify-center shrink-0">
        <Music2 :size="20" class="text-slate-400" />
      </div>

      <div class="overflow-hidden">
        <div class="flex items-center gap-2">
          <h4
            @click="playerStore.toggleFullscreenLyrics"
            class="text-xs font-semibold text-white truncate hover:text-brand cursor-pointer transition-colors"
          >
            {{ playerStore.currentTrackTitle }}
          </h4>

          <!-- Live audio visualizer equalizer bars when playing -->
          <div v-if="playerStore.isPlaying" class="flex items-end gap-0.5 h-3 shrink-0 px-1.5 py-0.5 rounded bg-brand/15 border border-brand/25">
            <span class="w-0.5 bg-brand rounded-full animate-bar-1 h-full"></span>
            <span class="w-0.5 bg-brand rounded-full animate-bar-2 h-1/2"></span>
            <span class="w-0.5 bg-brand rounded-full animate-bar-3 h-3/4"></span>
            <span class="w-0.5 bg-brand rounded-full animate-bar-4 h-2/3"></span>
          </div>

          <span
            v-if="playerStore.currentTrack?.format"
            class="text-[9px] px-1.5 py-0.2 rounded-full bg-brand/15 text-brand font-medium tracking-tight shrink-0 font-mono border border-brand/20"
          >
            {{ playerStore.currentTrack.format.split(' ')[0] }}
          </span>
        </div>
        <p class="text-[11px] text-slate-400 truncate hover:text-slate-200 cursor-pointer mt-0.5">
          {{ playerStore.currentTrackArtist }}
        </p>
      </div>

      <!-- Favorite Button -->
      <button
        @click="playerStore.toggleFavorite()"
        class="ml-1 p-1.5 rounded-full hover:bg-white/10 transition-colors shrink-0"
        :class="(playerStore.currentTrack?.isFavorite || (playerStore.currentTrack && playerStore.isIdFavorite(playerStore.currentTrack.id))) ? 'text-brand' : 'text-slate-400 hover:text-white'"
        title="收藏"
      >
        <Heart :size="16" :fill="(playerStore.currentTrack?.isFavorite || (playerStore.currentTrack && playerStore.isIdFavorite(playerStore.currentTrack.id))) ? 'currentColor' : 'none'" />
      </button>
    </div>

    <!-- Center: Playback Controls & Progress Bar -->
    <div class="flex flex-col items-center justify-center w-2/4 max-w-xl px-4 pb-0.5">
      <!-- Buttons row -->
      <div class="flex items-center gap-4 mb-1.5">
        <!-- Shuffle Mode Button -->
        <button
          @click="playerStore.mode = playerStore.mode === 'shuffle' ? 'loop' : 'shuffle'"
          class="relative p-1.5 rounded-full hover:bg-white/10 transition-colors"
          :class="playerStore.mode === 'shuffle' ? 'text-brand' : 'text-slate-400 hover:text-white'"
          :title="$t('player.shuffle')"
        >
          <Shuffle :size="14" />
          <span v-if="playerStore.mode === 'shuffle'" class="absolute bottom-0.5 left-1/2 -translate-x-1/2 w-1 h-1 rounded-full bg-brand"></span>
        </button>

        <!-- Previous Button -->
        <button
          @click="playerStore.prevTrack"
          class="p-1.5 rounded-full text-slate-300 hover:text-white hover:bg-white/10 transition-colors active:scale-95"
          :title="$t('player.previous')"
        >
          <SkipBack :size="17" />
        </button>

        <!-- Play / Pause Button -->
        <button
          @click="playerStore.togglePlay"
          class="w-9 h-9 rounded-full bg-gradient-to-tr from-brand to-brand-light hover:brightness-110 text-white flex items-center justify-center shadow-lg shadow-brand/35 ring-1 ring-white/20 hover:scale-105 active:scale-95 transition-all"
          :title="playerStore.isPlaying ? $t('player.pause') : $t('player.play')"
        >
          <RefreshCw v-if="playerStore.isLoadingStream" :size="16" class="animate-spin" />
          <Pause v-else-if="playerStore.isPlaying" :size="17" fill="currentColor" />
          <Play v-else :size="17" fill="currentColor" class="ml-0.5" />
        </button>

        <!-- Next Button -->
        <button
          @click="playerStore.nextTrack"
          class="p-1.5 rounded-full text-slate-300 hover:text-white hover:bg-white/10 transition-colors active:scale-95"
          :title="$t('player.next')"
        >
          <SkipForward :size="17" />
        </button>

        <!-- Loop / Cycle Mode Button -->
        <button
          @click="playerStore.cycleMode"
          class="relative p-1.5 rounded-full hover:bg-white/10 transition-colors"
          :class="playerStore.mode !== 'sequence' ? 'text-brand' : 'text-slate-400 hover:text-white'"
          :title="modeTitle"
        >
          <Repeat1 v-if="playerStore.mode === 'single'" :size="15" />
          <Repeat v-else :size="15" />
          <span v-if="playerStore.mode !== 'sequence'" class="absolute bottom-0.5 left-1/2 -translate-x-1/2 w-1 h-1 rounded-full bg-brand"></span>
        </button>
      </div>

      <!-- Custom Interactive Progress Bar -->
      <div class="w-full flex items-center gap-2.5 text-[11px] text-slate-400 font-mono">
        <span class="w-9 text-right tabular-nums select-none">{{ playerStore.formattedCurrentTime }}</span>
        <div
          ref="progressBarRef"
          @mousedown="startProgressDrag"
          @mousemove="onProgressHover"
          @mouseleave="hoverTime = null"
          class="relative flex-1 h-4 flex items-center group cursor-pointer"
        >
          <!-- Hover Timestamp Tooltip -->
          <div
            v-if="hoverTime !== null"
            :style="{ left: `${hoverX}px` }"
            class="absolute -top-6 -translate-x-1/2 px-1.5 py-0.5 rounded-md bg-black/90 text-[10px] text-white border border-white/15 font-mono pointer-events-none shadow-lg z-30"
          >
            {{ formatHoverTime(hoverTime) }}
          </div>

          <div class="w-full h-[3px] group-hover:h-[5px] rounded-full bg-theme-sliderTrack overflow-visible relative transition-all duration-150">
            <div
              class="h-full rounded-full bg-gradient-to-r from-brand to-brand-light relative transition-[width] duration-75"
              :style="{ width: `${progressPercent}%` }"
            >
              <span
                class="absolute right-0 top-1/2 -translate-y-1/2 translate-x-1/2 w-2.5 h-2.5 rounded-full bg-white shadow-md shadow-black/40 ring-2 ring-brand/40 opacity-0 scale-75 group-hover:opacity-100 group-hover:scale-100 transition-all duration-150 pointer-events-none"
                :class="isDraggingProgress ? '!opacity-100 !scale-110' : ''"
              ></span>
            </div>
          </div>
        </div>
        <span class="w-9 tabular-nums select-none">{{ playerStore.formattedDuration }}</span>
      </div>
    </div>

    <!-- Right: Volume, Speed, Lyrics & Queue Controls -->
    <div class="flex items-center justify-end gap-2.5 w-1/4 min-w-[200px]">
      <!-- Playback Rate (Speed) -->
      <button
        @click="cyclePlaybackRate"
        class="px-1.5 py-0.5 rounded text-[10px] font-mono font-medium border border-white/10 hover:border-white/30 text-slate-300 hover:text-white transition-colors"
        title="倍速播放"
      >
        {{ playerStore.playbackRate }}x
      </button>

      <!-- Lyrics Toggle Button -->
      <button
        @click="playerStore.toggleFullscreenLyrics"
        class="p-2 rounded-xl transition-all"
        :class="playerStore.showFullscreenLyrics ? 'bg-brand/20 text-brand shadow-sm shadow-brand/10' : 'text-slate-400 hover:text-white hover:bg-white/10'"
        :title="$t('player.immersive_lyrics')"
      >
        <Mic2 :size="17" />
      </button>

      <!-- Queue Drawer Toggle -->
      <button
        @click="playerStore.toggleQueueDrawer"
        class="p-2 rounded-xl transition-all relative"
        :class="playerStore.showQueueDrawer ? 'bg-brand/20 text-brand shadow-sm shadow-brand/10' : 'text-slate-400 hover:text-white hover:bg-white/10'"
        :title="$t('player.queue_title')"
      >
        <ListMusic :size="17" />
        <span
          v-if="playerStore.queue.length > 0"
          class="absolute -top-0.5 -right-0.5 w-4 h-4 rounded-full bg-brand text-[9px] font-bold text-white flex items-center justify-center"
        >
          {{ playerStore.queue.length }}
        </span>
      </button>

      <!-- Volume Control -->
      <div class="flex items-center gap-1.5 ml-1.5 group">
        <button
          @click="playerStore.toggleMute"
          class="p-1 text-slate-400 hover:text-white transition-colors"
          :title="playerStore.isMuted ? $t('player.unmute') : $t('player.mute')"
        >
          <VolumeX v-if="playerStore.isMuted || playerStore.volume === 0" :size="15" />
          <Volume1 v-else-if="playerStore.volume < 0.5" :size="15" />
          <Volume2 v-else :size="15" />
        </button>
        <div
          ref="volumeBarRef"
          @mousedown="startVolumeDrag"
          class="relative w-20 h-4 flex items-center cursor-pointer"
        >
          <div class="w-full h-[3px] group-hover:h-[4px] rounded-full bg-theme-sliderTrack relative transition-all duration-150">
            <div
              class="h-full rounded-full bg-gradient-to-r from-brand to-brand-light relative"
              :style="{ width: `${volumePercent}%` }"
            >
              <span
                class="absolute right-0 top-1/2 -translate-y-1/2 translate-x-1/2 w-2.5 h-2.5 rounded-full bg-white shadow-md shadow-black/40 opacity-0 scale-75 group-hover:opacity-100 group-hover:scale-100 transition-all duration-150 pointer-events-none"
                :class="isDraggingVolume ? '!opacity-100 !scale-110' : ''"
              ></span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import {
  Play,
  Pause,
  SkipBack,
  SkipForward,
  Shuffle,
  Repeat,
  Repeat1,
  Heart,
  Volume2,
  Volume1,
  VolumeX,
  ListMusic,
  Mic2,
  Maximize2,
  Music2,
  RefreshCw,
} from 'lucide-vue-next';
import { usePlayerStore } from '@/stores/player';

const playerStore = usePlayerStore();
const progressBarRef = ref<HTMLElement | null>(null);
const volumeBarRef = ref<HTMLElement | null>(null);
const hoverTime = ref<number | null>(null);
const hoverX = ref<number>(0);
const isDraggingProgress = ref(false);
const isDraggingVolume = ref(false);

const progressPercent = computed(() => {
  const dur = playerStore.duration || 0;
  if (!dur) return 0;
  return Math.max(0, Math.min(100, (playerStore.currentTime / dur) * 100));
});

const volumePercent = computed(() => {
  if (playerStore.isMuted) return 0;
  return Math.max(0, Math.min(100, playerStore.volume * 100));
});

const modeTitle = computed(() => {
  switch (playerStore.mode) {
    case 'loop':
      return '列表循环';
    case 'single':
      return '单曲循环';
    case 'shuffle':
      return '随机播放';
    default:
      return '顺序播放';
  }
});

const seekFromClientX = (clientX: number) => {
  if (!progressBarRef.value || !playerStore.duration) return;
  const rect = progressBarRef.value.getBoundingClientRect();
  const ratio = Math.max(0, Math.min(1, (clientX - rect.left) / rect.width));
  playerStore.seek(Math.floor(ratio * playerStore.duration));
};

const startProgressDrag = (e: MouseEvent) => {
  isDraggingProgress.value = true;
  seekFromClientX(e.clientX);
  const onMove = (ev: MouseEvent) => seekFromClientX(ev.clientX);
  const onUp = () => {
    isDraggingProgress.value = false;
    window.removeEventListener('mousemove', onMove);
    window.removeEventListener('mouseup', onUp);
  };
  window.addEventListener('mousemove', onMove);
  window.addEventListener('mouseup', onUp);
};

const volumeFromClientX = (clientX: number) => {
  if (!volumeBarRef.value) return;
  const rect = volumeBarRef.value.getBoundingClientRect();
  const ratio = Math.max(0, Math.min(1, (clientX - rect.left) / rect.width));
  playerStore.setVolume(Number(ratio.toFixed(2)));
};

const startVolumeDrag = (e: MouseEvent) => {
  isDraggingVolume.value = true;
  volumeFromClientX(e.clientX);
  const onMove = (ev: MouseEvent) => volumeFromClientX(ev.clientX);
  const onUp = () => {
    isDraggingVolume.value = false;
    window.removeEventListener('mousemove', onMove);
    window.removeEventListener('mouseup', onUp);
  };
  window.addEventListener('mousemove', onMove);
  window.addEventListener('mouseup', onUp);
};

const cyclePlaybackRate = () => {
  const rates = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
  const idx = rates.indexOf(playerStore.playbackRate);
  const nextRate = rates[(idx + 1) % rates.length];
  playerStore.setPlaybackRate(nextRate);
};

const onProgressHover = (e: MouseEvent) => {
  if (!progressBarRef.value || !playerStore.duration) return;
  const rect = progressBarRef.value.getBoundingClientRect();
  const offsetX = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
  const ratio = offsetX / rect.width;
  hoverX.value = offsetX;
  hoverTime.value = Math.floor(ratio * playerStore.duration);
};

const formatHoverTime = (seconds: number): string => {
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  return `${m}:${s.toString().padStart(2, '0')}`;
};
</script>

<style scoped>
@keyframes bar1 {
  0%, 100% { height: 30%; }
  50% { height: 100%; }
}
@keyframes bar2 {
  0%, 100% { height: 90%; }
  50% { height: 40%; }
}
@keyframes bar3 {
  0%, 100% { height: 40%; }
  50% { height: 80%; }
}

.animate-bar-1 { animation: bar1 0.8s ease-in-out infinite; }
.animate-bar-2 { animation: bar2 0.7s ease-in-out infinite; }
.animate-bar-3 { animation: bar3 0.9s ease-in-out infinite; }
</style>
