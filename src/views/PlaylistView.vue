<template>
  <div class="space-y-6 pb-12">
    <!-- Loading State -->
    <div v-if="isLoading" class="py-20 flex flex-col items-center justify-center space-y-3">
      <RefreshCw :size="32" class="animate-spin text-brand" />
      <p class="text-xs text-slate-400">正在从网易云音乐同步歌单曲目与高保真音源...</p>
    </div>

    <template v-else>
      <!-- Playlist Header Banner -->
      <div class="flex flex-col sm:flex-row items-center sm:items-end gap-6 p-6 rounded-3xl glass-card border border-white/10">
        <div
          class="relative w-44 h-44 rounded-2xl overflow-hidden shadow-2xl shrink-0 border border-white/15 group"
          :class="!isCloud ? 'cursor-pointer' : ''"
          @click="!isCloud && openEdit()"
        >
          <img :src="currentPlaylist?.cover || defaultCover" loading="lazy" decoding="async" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300" />
          <div
            v-if="!isCloud"
            class="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity flex flex-col items-center justify-center gap-1.5 text-white"
          >
            <Edit3 :size="20" class="text-brand" />
            <span class="text-xs font-medium">更换封面与资料</span>
          </div>
        </div>

        <div class="space-y-3 flex-1 text-center sm:text-left">
          <div class="flex items-center justify-center sm:justify-start gap-2">
            <span
              class="text-[11px] font-bold uppercase tracking-wider px-2 py-0.5 rounded-full"
              :class="isNetease ? 'bg-red-500/20 text-red-400 border border-red-500/30' : (isQQ ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30' : (isYT ? 'bg-rose-500/20 text-rose-400 border border-rose-500/30' : 'bg-white/10 text-slate-300'))"
            >
              {{ isNetease ? '网易云同步歌单' : (isQQ ? 'QQ 音乐同步歌单' : (isYT ? 'YouTube Music 同步歌单' : '本地歌单')) }}
            </span>
            <span v-if="!isCloud && canDelete" class="text-[10px] text-slate-500">自定义</span>
          </div>

          <h1 class="text-3xl font-extrabold text-white tracking-tight">
            {{ currentPlaylist?.name || '歌单详情' }}
          </h1>
          <p class="text-xs text-slate-300 max-w-xl line-clamp-2">
            {{ currentPlaylist?.description || '暂无描述' }}
          </p>

          <div class="flex flex-wrap items-center justify-center sm:justify-start gap-4 text-xs text-slate-400 font-medium pt-1">
            <span>共 {{ playlistTracks.length }} 首歌曲</span>
            <span>•</span>
            <button
              @click="playAll"
              :disabled="playlistTracks.length === 0"
              class="px-5 py-2 rounded-full bg-brand hover:bg-brand-light text-white font-semibold flex items-center gap-2 shadow-lg shadow-brand/25 transition-all disabled:opacity-50"
            >
              <Play :size="14" fill="white" />
              播放全部
            </button>

            <button
              v-if="isCloud"
              @click="saveToLocalPlaylist"
              class="px-4 py-2 rounded-full bg-white/10 hover:bg-white/15 text-white flex items-center gap-2 border border-white/10 transition-colors"
              title="保存一份副本到本地歌单"
            >
              <Save :size="14" />
              导入本地
            </button>

            <button
              v-if="!isCloud"
              @click="openEdit"
              class="px-4 py-2 rounded-full bg-white/10 hover:bg-white/15 text-white flex items-center gap-2 border border-white/10 transition-colors"
              title="编辑歌单信息与封面"
            >
              <Edit3 :size="14" />
              编辑资料
            </button>

            <button
              @click="exportJson"
              class="px-4 py-2 rounded-full bg-white/10 hover:bg-white/15 text-white flex items-center gap-2 border border-white/10 transition-colors"
            >
              <Download :size="14" />
              导出 JSON
            </button>

            <button
              v-if="!isCloud && canDelete"
              @click="deleteCurrentPlaylist"
              class="px-3 py-2 rounded-full hover:bg-red-500/20 text-slate-400 hover:text-red-400 flex items-center gap-1.5 transition-colors"
              title="删除此歌单"
            >
              <Trash2 :size="14" />
              删除
            </button>
          </div>
        </div>
      </div>

      <!-- Track List -->
      <div class="glass-panel rounded-2xl overflow-hidden border border-white/10">
        <div class="grid grid-cols-12 px-4 py-3 text-[11px] font-semibold text-slate-400 uppercase tracking-wider border-b border-white/[0.08] bg-white/[0.02]">
          <div class="col-span-1 text-center">#</div>
          <div class="col-span-5">歌曲标题</div>
          <div class="col-span-3">专辑</div>
          <div class="col-span-1">音质</div>
          <div class="col-span-2 text-right">时长 / 操作</div>
        </div>

        <div v-if="playlistTracks.length === 0" class="py-12 text-center text-xs text-slate-500">
          此歌单暂无歌曲
        </div>

        <div v-else class="divide-y divide-white/[0.04]">
          <div
            v-for="(track, index) in playlistTracks"
            :key="track.id"
            @click="playerStore.playTrack(track, playlistTracks)"
            @dblclick="playerStore.playTrack(track, playlistTracks)"
            @contextmenu.prevent.stop="contextMenuStore.openTrackMenu($event, track, playlistTracks)"
            class="grid grid-cols-12 px-4 py-3 items-center text-xs text-slate-300 hover:bg-white/[0.06] group cursor-pointer transition-colors"
            :class="playerStore.currentTrack?.id === track.id ? 'bg-brand/10 text-brand font-medium' : ''"
          >
            <div class="col-span-1 text-center text-slate-400 font-mono flex items-center justify-center">
              <div v-if="playerStore.currentTrack?.id === track.id && playerStore.isPlaying" class="flex items-end gap-0.5 h-3 shrink-0 px-1">
                <span class="w-0.5 bg-brand animate-bar-1 h-full rounded-full"></span>
                <span class="w-0.5 bg-brand animate-bar-2 h-1/2 rounded-full"></span>
                <span class="w-0.5 bg-brand animate-bar-3 h-3/4 rounded-full"></span>
              </div>
              <span v-else class="group-hover:hidden">{{ index + 1 }}</span>
              <button
                @click.stop="playerStore.playTrack(track, playlistTracks)"
                class="hidden group-hover:inline-block text-white hover:text-brand"
              >
                <Play :size="13" fill="currentColor" />
              </button>
            </div>

            <div class="col-span-5 flex items-center gap-3 min-w-0 pr-4">
              <img
                v-if="track.cover"
                :src="track.cover"
                loading="lazy"
                decoding="async"
                class="w-9 h-9 rounded-lg object-cover border border-white/10 shrink-0"
              />
              <div class="min-w-0 flex-1">
                <p class="font-medium text-white truncate group-hover:text-brand transition-colors">
                  {{ track.title }}
                </p>
                <p class="text-[11px] text-slate-400 truncate">
                  {{ track.artist }}
                </p>
              </div>
            </div>

            <div class="col-span-3 truncate text-slate-400 text-xs pr-4">
              {{ track.album || '网易云精选' }}
            </div>

            <div class="col-span-1 truncate text-slate-400 font-mono text-[10px]">
              <span class="px-1.5 py-0.5 rounded bg-white/[0.06] border border-white/10 text-slate-300">
                {{ track.format || '320k' }}
              </span>
            </div>

            <div class="col-span-2 flex items-center justify-end gap-2.5 text-slate-400 font-mono text-xs">
              <button
                @click.stop="playlistStore.openAddToPlaylistModal(track)"
                class="p-1 hover:text-white opacity-0 group-hover:opacity-100 transition-opacity"
                title="添加到其他歌单"
              >
                <PlusCircle :size="14" />
              </button>
              <button
                v-if="!isNetease"
                @click.stop="removeTrack(track.id)"
                class="p-1 hover:text-red-400 opacity-0 group-hover:opacity-100 transition-opacity"
                title="从歌单移除"
              >
                <Trash2 :size="13" />
              </button>
              <button
                @click.stop="playerStore.toggleFavorite(track)"
                class="p-1 hover:text-brand transition-colors"
                :class="(track.isFavorite || playerStore.isIdFavorite(track.id)) ? 'text-brand' : ''"
              >
                <Heart :size="14" :fill="(track.isFavorite || playerStore.isIdFavorite(track.id)) ? 'currentColor' : 'none'" />
              </button>
              <span class="tabular-nums w-10 text-right">{{ formatDuration(track.duration) }}</span>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useRoute } from 'vue-router';
import { Play, Download, Heart, Trash2, RefreshCw, Save, Edit3, PlusCircle } from 'lucide-vue-next';
import { invoke } from '@tauri-apps/api/core';
import { usePlaylistStore } from '@/stores/playlist';
import { usePlayerStore } from '@/stores/player';
import { useLibraryStore } from '@/stores/library';
import { useContextMenuStore } from '@/stores/contextMenu';
import type { Track, Playlist } from '@/types/music';

const route = useRoute();
const playlistStore = usePlaylistStore();
const playerStore = usePlayerStore();
const libraryStore = useLibraryStore();
const contextMenuStore = useContextMenuStore();

const defaultCover = 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=500&auto=format&fit=crop&q=80';
const isLoading = ref(false);
const remotePlaylist = ref<{
  id: string;
  name: string;
  description: string;
  cover: string;
  tracks: Track[];
} | null>(null);

const isNetease = computed(() => {
  const id = route.params.id as string;
  return !!id && (id.startsWith('netease-pl-') || id.startsWith('netease-'));
});

const isQQ = computed(() => {
  const id = route.params.id as string;
  return !!id && id.startsWith('qq-pl-');
});

const isYT = computed(() => {
  const id = route.params.id as string;
  return !!id && (id.startsWith('yt-') || id.startsWith('yt-pl-'));
});

const isCloud = computed(() => isNetease.value || isQQ.value || isYT.value);

const currentPlaylist = computed(() => {
  const id = route.params.id as string;
  if (isCloud.value) {
    if (remotePlaylist.value && remotePlaylist.value.id === id) {
      return {
        id: remotePlaylist.value.id,
        name: remotePlaylist.value.name,
        description: remotePlaylist.value.description,
        cover: remotePlaylist.value.cover || defaultCover,
        trackIds: remotePlaylist.value.tracks.map((t) => t.id),
        createdAt: Date.now(),
        updatedAt: Date.now(),
      } as Playlist;
    }
    return {
      id: id || 'cloud',
      name: '正在同步歌单...',
      description: '',
      cover: defaultCover,
      trackIds: [],
      createdAt: Date.now(),
      updatedAt: Date.now(),
    } as Playlist;
  }
  return playlistStore.playlists.find((p) => p.id === id) || playlistStore.playlists[0];
});

const canDelete = computed(() => {
  return currentPlaylist.value && currentPlaylist.value.id !== 'pl-favorites';
});

const playlistTracks = computed<Track[]>(() => {
  if (isCloud.value && remotePlaylist.value) {
    return remotePlaylist.value.tracks.map((t) => ({
      ...t,
      isFavorite: t.isFavorite || playerStore.isIdFavorite(t.id),
    }));
  }

  if (!currentPlaylist.value) return [];
  const allAvailable = [
    ...playlistStore.trackCache,
    ...playerStore.favoriteCache,
    ...libraryStore.tracks,
    ...playerStore.queue,
  ];
  const uniqueMap = new Map<string, Track>();
  for (const t of allAvailable) {
    uniqueMap.set(t.id, t);
  }

  if (currentPlaylist.value.id === 'pl-favorites') {
    const list: Track[] = [];
    const seen = new Set<string>();

    for (const id of currentPlaylist.value.trackIds || []) {
      const tr = uniqueMap.get(id);
      if (tr && !seen.has(id)) {
        list.push({ ...tr, isFavorite: true });
        seen.add(id);
      }
    }

    for (const tr of [...playerStore.favoriteCache, ...libraryStore.tracks, ...playerStore.queue]) {
      if ((tr.isFavorite || playerStore.isIdFavorite(tr.id)) && !seen.has(tr.id)) {
        list.push({ ...tr, isFavorite: true });
        seen.add(tr.id);
      }
    }
    return list;
  }

  if (currentPlaylist.value.trackIds && currentPlaylist.value.trackIds.length > 0) {
    return currentPlaylist.value.trackIds
      .map((id) => uniqueMap.get(id))
      .filter((t): t is Track => !!t)
      .map((t) => ({
        ...t,
        isFavorite: t.isFavorite || playerStore.isIdFavorite(t.id),
      }));
  }

  return [];
});

// Watch route parameter changes to fetch NetEase or QQ Music playlist details dynamically
watch(
  () => route.params.id,
  async (newId) => {
    if (!newId || typeof newId !== 'string') return;
    if (newId.startsWith('netease-pl-') || newId.startsWith('netease-')) {
      const neteaseId = newId.replace('netease-pl-', '').replace('netease-', '');
      isLoading.value = true;
      try {
        const detail = await invoke<{
          id: string;
          name: string;
          description: string | null;
          cover: string | null;
          tracks: Track[];
        }>('netease_get_playlist', { playlistId: neteaseId });

        remotePlaylist.value = {
          id: newId,
          name: detail.name,
          description: detail.description || '',
          cover: detail.cover || defaultCover,
          tracks: detail.tracks.map((t) => ({
            ...t,
            cover: t.cover || detail.cover || defaultCover,
          })),
        };
      } catch (err) {
        console.error('Failed to load NetEase playlist:', err);
      } finally {
        isLoading.value = false;
      }
    } else if (newId.startsWith('qq-pl-')) {
      const disstid = newId.replace('qq-pl-', '');
      isLoading.value = true;
      try {
        const detail = await invoke<{
          id: string;
          name: string;
          description: string | null;
          cover: string | null;
          tracks: Track[];
        }>('qqmusic_get_playlist', { disstid });

        remotePlaylist.value = {
          id: newId,
          name: detail.name,
          description: detail.description || '',
          cover: detail.cover || defaultCover,
          tracks: detail.tracks.map((t) => ({
            ...t,
            cover: t.cover || detail.cover || defaultCover,
          })),
        };
      } catch (err) {
        console.error('Failed to load QQ Music playlist:', err);
      } finally {
        isLoading.value = false;
      }
    } else if (newId.startsWith('yt-')) {
      const ytId = newId.replace('yt-pl-', '').replace('yt-', '');
      isLoading.value = true;
      try {
        const detail = await invoke<{
          id: string;
          name: string;
          description: string | null;
          cover: string | null;
          tracks: Track[];
        }>('ytmusic_get_playlist', { playlistId: ytId });

        remotePlaylist.value = {
          id: newId,
          name: detail.name,
          description: detail.description || 'YouTube Music 在线歌单',
          cover: detail.cover || defaultCover,
          tracks: detail.tracks.map((t) => ({
            ...t,
            cover: t.cover || detail.cover || defaultCover,
          })),
        };
      } catch (err) {
        console.error('Failed to load YouTube Music playlist:', err);
      } finally {
        isLoading.value = false;
      }
    } else {
      remotePlaylist.value = null;
    }
  },
  { immediate: true }
);

const playAll = () => {
  if (playlistTracks.value.length > 0) {
    playerStore.playTrack(playlistTracks.value[0], playlistTracks.value);
  }
};

const openEdit = () => {
  if (currentPlaylist.value) {
    playlistStore.openEditModal(currentPlaylist.value);
  }
};

const saveToLocalPlaylist = async () => {
  if (!currentPlaylist.value) return;
  const created = await playlistStore.createPlaylist(
    `${currentPlaylist.value.name} (云同步备份)`,
    currentPlaylist.value.description,
    currentPlaylist.value.cover
  );
  const tracksToSave = playlistTracks.value;
  if (tracksToSave.length > 0 && created) {
    playlistStore.cacheTracks(tracksToSave);
    for (const t of tracksToSave) {
      await playlistStore.addTrackToPlaylist(created.id, t.id);
    }
  }
  playerStore.showToast(`已成功创建本地备份歌单「${created.name}」（含 ${tracksToSave.length} 首歌曲）`);
};

const removeTrack = async (trackId: string) => {
  if (currentPlaylist.value && !isNetease.value) {
    await playlistStore.removeTrackFromPlaylist(currentPlaylist.value.id, trackId);
  }
};

const deleteCurrentPlaylist = () => {
  if (currentPlaylist.value) {
    playlistStore.openDeleteModal(currentPlaylist.value);
  }
};

const exportJson = () => {
  if (!currentPlaylist.value) return;
  const exportData = {
    ...currentPlaylist.value,
    tracks: playlistTracks.value,
  };
  const json = JSON.stringify(exportData, null, 2);
  const blob = new Blob([json], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `${currentPlaylist.value.name}.json`;
  a.click();
  URL.revokeObjectURL(url);
};

const formatDuration = (seconds?: number): string => {
  if (!seconds) return '0:00';
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  return `${m}:${s.toString().padStart(2, '0')}`;
};
</script>
