<template>
  <Transition name="fade">
    <div
      v-if="playerStore.showQueueDrawer"
      class="fixed right-3 bottom-[92px] w-80 sm:w-96 max-h-[500px] flex flex-col liquid-glass-floating rounded-3xl z-40 overflow-hidden"
    >
      <!-- Header -->
      <div class="h-12 px-4 flex items-center justify-between border-b border-white/[0.08] bg-white/[0.03]">
        <div class="flex items-center gap-2">
          <ListMusic :size="16" class="text-brand" />
          <span class="text-xs font-semibold text-white">播放队列 ({{ playerStore.queue.length }})</span>
        </div>
        <div class="flex items-center gap-2">
          <button
            @click="playerStore.queue = []"
            class="px-2 py-1 rounded-lg text-[11px] text-slate-400 hover:text-white hover:bg-white/10 transition-colors"
          >
            清空
          </button>
          <button
            @click="playerStore.showQueueDrawer = false"
            class="p-1 text-slate-400 hover:text-white rounded-lg hover:bg-white/10 transition-colors"
          >
            <X :size="14" />
          </button>
        </div>
      </div>

      <!-- Queue List -->
      <div class="flex-1 overflow-y-auto p-2 space-y-1">
        <div
          v-for="(track, index) in playerStore.queue"
          :key="track.id"
          @click="playFromQueue(track)"
          @contextmenu.prevent.stop="contextMenuStore.openTrackMenu($event, track, playerStore.queue)"
          class="flex items-center gap-3 p-2 rounded-2xl cursor-pointer group transition-all"
          :class="track.id === playerStore.currentTrack?.id ? 'bg-brand/15 text-brand font-medium ring-1 ring-brand/30' : 'hover:bg-white/[0.07] text-slate-300'"
        >
          <span class="w-4 text-[11px] text-center text-slate-400 group-hover:text-white font-mono">
            <span v-if="track.id === playerStore.currentTrack?.id && playerStore.isPlaying" class="text-brand">▶</span>
            <span v-else>{{ index + 1 }}</span>
          </span>

          <img
            v-if="track.cover"
            :src="track.cover"
            loading="lazy"
            decoding="async"
            class="w-9 h-9 rounded-xl object-cover border border-white/15 shrink-0 shadow-sm"
          />

          <div class="flex-1 min-w-0">
            <p class="text-xs truncate font-medium" :class="track.id === playerStore.currentTrack?.id ? 'text-brand' : 'text-white'">
              {{ track.title }}
            </p>
            <p class="text-[10px] text-slate-400 truncate">
              {{ track.artist }}
            </p>
          </div>

          <button
            @click.stop="removeFromQueue(track.id)"
            class="opacity-0 group-hover:opacity-100 p-1.5 rounded-lg text-slate-400 hover:text-red-400 hover:bg-red-500/10 transition-all"
          >
            <Trash2 :size="13" />
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ListMusic, X, Trash2 } from 'lucide-vue-next';
import { usePlayerStore } from '@/stores/player';
import { useContextMenuStore } from '@/stores/contextMenu';
import type { Track } from '@/types/music';

const playerStore = usePlayerStore();
const contextMenuStore = useContextMenuStore();

const playFromQueue = (track: Track) => {
  playerStore.playTrack(track);
};

const removeFromQueue = (trackId: string) => {
  playerStore.queue = playerStore.queue.filter((t) => t.id !== trackId);
};
</script>

