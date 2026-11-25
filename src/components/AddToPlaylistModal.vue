<template>
  <div
    v-if="playlistStore.isAddModalOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md animate-fade-in select-none"
    @click.self="playlistStore.closeAddToPlaylistModal()"
  >
    <div
      class="relative w-full max-w-md bg-[#14141e]/95 border border-white/15 rounded-3xl p-6 shadow-2xl space-y-5 overflow-hidden"
    >
      <!-- Background Ambient Glow -->
      <div class="absolute -top-16 -right-16 w-40 h-40 rounded-full bg-brand/20 blur-3xl pointer-events-none"></div>

      <!-- Header -->
      <div class="flex items-center justify-between pb-3.5 border-b border-white/10 relative z-10">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-xl bg-brand/20 border border-brand/30 flex items-center justify-center text-brand">
            <PlusCircle :size="18" />
          </div>
          <div>
            <h3 class="text-sm font-bold text-white tracking-wide">添加到歌单</h3>
            <p class="text-[11px] text-slate-400">点击下方歌单即可一键点选添加</p>
          </div>
        </div>
        <button
          @click="playlistStore.closeAddToPlaylistModal()"
          class="p-1.5 rounded-full hover:bg-white/10 text-slate-400 hover:text-white transition-colors"
        >
          <X :size="18" />
        </button>
      </div>

      <!-- Track Preview Banner -->
      <div
        v-if="playlistStore.trackToAdd"
        class="flex items-center gap-3 p-3 rounded-2xl bg-white/[0.04] border border-white/10 relative z-10"
      >
        <img
          v-if="playlistStore.trackToAdd.cover"
          :src="playlistStore.trackToAdd.cover"
          class="w-11 h-11 rounded-xl object-cover border border-white/10 shrink-0"
        />
        <div v-else class="w-11 h-11 rounded-xl bg-white/5 border border-white/10 flex items-center justify-center shrink-0">
          <Music :size="18" class="text-slate-400" />
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-xs font-bold text-white truncate">
            {{ playlistStore.trackToAdd.title }}
          </p>
          <p class="text-[11px] text-slate-400 truncate mt-0.5">
            {{ playlistStore.trackToAdd.artist }}
          </p>
        </div>
        <span class="text-[10px] font-mono px-2 py-0.5 rounded-full bg-white/10 text-slate-300 shrink-0">
          {{ playlistStore.trackToAdd.format?.split(' ')[0] || 'MP3' }}
        </span>
      </div>

      <!-- Playlists Selection List -->
      <div class="space-y-2 relative z-10">
        <div class="flex items-center justify-between text-xs font-semibold text-slate-300">
          <span>选择目标歌单</span>
          <span class="text-[11px] text-slate-500 font-normal">({{ playlistStore.playlists.length }} 个歌单)</span>
        </div>

        <div class="max-h-60 overflow-y-auto space-y-2 pr-1">
          <div
            v-for="pl in playlistStore.playlists"
            :key="pl.id"
            @click="handleTogglePlaylist(pl)"
            class="flex items-center justify-between p-2.5 rounded-2xl border transition-all cursor-pointer group active:scale-[0.99]"
            :class="isTrackInPlaylist(pl) ? 'bg-brand/15 border-brand/40 shadow-sm shadow-brand/10' : 'bg-white/[0.03] hover:bg-white/[0.08] border-white/10'"
          >
            <div class="flex items-center gap-3 min-w-0 pr-2">
              <img
                v-if="pl.cover"
                :src="pl.cover"
                class="w-9 h-9 rounded-xl object-cover border border-white/10 shrink-0"
              />
              <div v-else class="w-9 h-9 rounded-xl bg-white/5 border border-white/10 flex items-center justify-center shrink-0">
                <ListMusic :size="16" class="text-slate-400" />
              </div>
              <div class="min-w-0">
                <p class="text-xs font-semibold text-white truncate group-hover:text-brand transition-colors">
                  {{ pl.name }}
                </p>
                <p class="text-[10px] text-slate-400 truncate">
                  {{ pl.trackIds?.length || 0 }} 首曲目
                </p>
              </div>
            </div>

            <!-- Action Status Pill -->
            <div class="shrink-0">
              <span
                v-if="isTrackInPlaylist(pl)"
                class="inline-flex items-center gap-1 px-2.5 py-1 rounded-xl text-[11px] font-semibold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30"
              >
                <Check :size="12" />
                <span>已加入</span>
              </span>
              <span
                v-else
                class="inline-flex items-center gap-1 px-2.5 py-1 rounded-xl text-[11px] font-medium bg-white/10 group-hover:bg-brand/20 text-slate-300 group-hover:text-brand border border-white/10 group-hover:border-brand/30 transition-colors"
              >
                <Plus :size="12" />
                <span>点选加入</span>
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Quick Preset Creator (Zero typing, 100% click to create) -->
      <div class="space-y-2 pt-2 border-t border-white/10 relative z-10">
        <div class="flex items-center justify-between text-xs font-semibold text-slate-300">
          <span class="flex items-center gap-1.5">
            <Sparkles :size="13" class="text-amber-400" />
            <span>一键点选新建歌单并收录</span>
          </span>
          <span class="text-[10px] text-slate-500">点选即建</span>
        </div>
        <div class="flex flex-wrap gap-1.5">
          <button
            v-for="preset in presetPlaylistNames"
            :key="preset"
            @click="handleCreatePreset(preset)"
            :disabled="isCreatingPreset"
            class="px-2.5 py-1 rounded-xl text-xs bg-white/[0.05] hover:bg-white/[0.12] hover:border-brand/40 text-slate-300 hover:text-white border border-white/10 transition-all flex items-center gap-1 active:scale-95 disabled:opacity-50"
          >
            <Plus :size="11" class="text-brand" />
            <span>{{ preset }}</span>
          </button>
        </div>
      </div>

      <!-- Footer -->
      <div class="pt-2 border-t border-white/10 flex items-center justify-end relative z-10">
        <button
          @click="playlistStore.closeAddToPlaylistModal()"
          class="px-5 py-2 rounded-xl bg-brand hover:bg-brand-light text-white text-xs font-semibold shadow-lg shadow-brand/25 transition-all active:scale-95"
        >
          完成
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { PlusCircle, X, Music, ListMusic, Check, Plus, Sparkles } from 'lucide-vue-next';
import { usePlaylistStore } from '@/stores/playlist';
import type { Playlist } from '@/types/music';

const playlistStore = usePlaylistStore();
const isCreatingPreset = ref(false);

const presetPlaylistNames = [
  '私人珍藏',
  '工作专注',
  '深夜听歌',
  '旅途漫游',
  '运动跑步',
  '纯音经典',
];

const isTrackInPlaylist = (playlist: Playlist): boolean => {
  if (!playlistStore.trackToAdd || !playlist.trackIds) return false;
  return playlist.trackIds.includes(playlistStore.trackToAdd.id);
};

const handleTogglePlaylist = async (playlist: Playlist) => {
  if (!playlistStore.trackToAdd) return;
  await playlistStore.toggleTrackInPlaylist(playlist.id, playlistStore.trackToAdd);
};

const handleCreatePreset = async (name: string) => {
  if (isCreatingPreset.value || !playlistStore.trackToAdd) return;
  isCreatingPreset.value = true;
  try {
    await playlistStore.createPlaylistWithPreset(name, playlistStore.trackToAdd);
  } finally {
    isCreatingPreset.value = false;
  }
};
</script>

