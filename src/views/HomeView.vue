<template>
  <div class="space-y-8 pb-12">
    <!-- Featured Hero Banner (Apple Music Style) -->
    <section class="relative rounded-3xl overflow-hidden p-8 border border-white/10 bg-gradient-to-r from-purple-900/60 via-brand/40 to-slate-900/80 shadow-2xl flex flex-col md:flex-row items-center justify-between gap-6">
      <div class="space-y-3 z-10 max-w-lg">
        <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[11px] font-semibold bg-white/10 text-white border border-white/10 backdrop-blur-md">
          <Sparkles :size="12" class="text-amber-300" />
          Time OS 原生推荐
        </span>
        <h1 class="text-3xl font-extrabold tracking-tight text-white leading-tight">
          极光空间 • 专注与纯粹的声音
        </h1>
        <p class="text-xs text-slate-300 leading-relaxed">
          专为 Time OS 打造的沉浸音频体验。采用 Rust 高保真解码核心，支持 24bit/96kHz Hi-Res FLAC 无损流，伴随每一次灵感涌现。
        </p>

        <div class="flex items-center gap-3 pt-2">
          <button
            @click="playFeatured"
            class="px-5 py-2.5 rounded-full bg-white text-black font-semibold text-xs flex items-center gap-2 hover:bg-slate-200 active:scale-95 transition-all shadow-lg shadow-white/10"
          >
            <Play :size="14" fill="currentColor" />
            立即播放
          </button>
          <button
            @click="router.push('/library')"
            class="px-5 py-2.5 rounded-full bg-white/10 text-white font-medium text-xs hover:bg-white/15 transition-all border border-white/10"
          >
            浏览资料库
          </button>
        </div>
      </div>

      <div class="relative w-48 h-48 md:w-56 md:h-56 shrink-0 rounded-2xl overflow-hidden shadow-2xl border border-white/20 group">
        <img
          src="https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=500&auto=format&fit=crop&q=80"
          class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
        />
      </div>
    </section>

    <!-- Daily Recommendation (今日推荐) -->
    <section class="space-y-4">
      <div class="flex items-center justify-between gap-4">
        <div>
          <div class="flex items-center gap-2">
            <h2 class="text-lg font-bold text-white tracking-tight">每日推荐歌曲</h2>
            <span class="px-2 py-0.5 rounded-full text-[10px] font-semibold bg-brand/15 text-brand border border-brand/25">
              {{ todayLabel }}
            </span>
          </div>
          <p class="text-xs text-slate-400">结合您的收听偏好与全网每日精选实时生成</p>
        </div>
        <div class="flex items-center gap-3">
          <button
            @click="refreshDaily"
            :disabled="isLoadingDaily"
            class="px-3 py-1.5 rounded-full bg-theme-surface hover:bg-theme-surfaceElevated border border-theme-border text-xs text-slate-300 hover:text-white flex items-center gap-1.5 transition-all disabled:opacity-50"
          >
            <RefreshCw :size="12" :class="isLoadingDaily ? 'animate-spin' : ''" />
            <span>换一批</span>
          </button>
          <button
            @click="playAllDaily"
            class="text-xs font-semibold text-brand hover:text-brand-light flex items-center gap-1 transition-colors"
          >
            <Play :size="12" fill="currentColor" />
            播放全部
          </button>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div
          v-for="track in displayDailyTracks"
          :key="track.id"
          @click="playerStore.playTrack(track, displayDailyTracks)"
          @contextmenu.prevent.stop="contextMenuStore.openTrackMenu($event, track, displayDailyTracks)"
          class="glass-card rounded-2xl p-3 flex items-center gap-3 cursor-pointer group transition-all"
          :class="playerStore.currentTrack?.id === track.id ? 'border-brand ring-1 ring-brand/30 bg-brand/10' : ''"
        >
          <div class="relative w-14 h-14 rounded-xl overflow-hidden shrink-0 border border-white/10 shadow-md">
            <img v-if="track.cover" :src="track.cover" class="w-full h-full object-cover group-hover:scale-105 transition-transform" />
            <div v-else class="w-full h-full bg-white/5 flex items-center justify-center">
              <Music :size="18" class="text-slate-400" />
            </div>
            <div class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
              <Play :size="16" fill="white" class="text-white ml-0.5" />
            </div>
          </div>

          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-1.5">
              <h4 class="text-xs font-semibold text-white truncate group-hover:text-brand transition-colors">
                {{ track.title }}
              </h4>
              <!-- Live Equalizer if playing this track -->
              <div v-if="playerStore.currentTrack?.id === track.id && playerStore.isPlaying" class="flex items-end gap-0.5 h-2.5 shrink-0 px-1 py-0.5 rounded bg-brand/20">
                <span class="w-0.5 bg-brand animate-bar-1 h-full"></span>
                <span class="w-0.5 bg-brand animate-bar-2 h-1/2"></span>
                <span class="w-0.5 bg-brand animate-bar-3 h-3/4"></span>
              </div>
            </div>
            <p class="text-[11px] text-slate-400 truncate mt-0.5">
              {{ track.artist }}
            </p>
            <span class="text-[9px] text-brand/80 font-mono mt-0.5 inline-block">
              {{ track.format || 'Hi-Res' }}
            </span>
          </div>

          <button
            @click.stop="playerStore.toggleFavorite(track)"
            class="p-2 text-slate-400 hover:text-brand transition-colors"
          >
            <Heart :size="14" :fill="(track.isFavorite || playerStore.isIdFavorite(track.id)) ? 'currentColor' : 'none'" :class="(track.isFavorite || playerStore.isIdFavorite(track.id)) ? 'text-brand' : ''" />
          </button>
        </div>
      </div>
    </section>

    <!-- Recent Plays (最近播放) -->
    <section v-if="recentTracks.length > 0" class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
            <Clock :size="16" class="text-brand" />
            最近播放
          </h2>
          <p class="text-xs text-slate-400">重温您最近聆听过的音乐</p>
        </div>
      </div>

      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-4">
        <div
          v-for="track in recentTracks"
          :key="track.id"
          @click="playerStore.playTrack(track)"
          @contextmenu.prevent.stop="contextMenuStore.openTrackMenu($event, track, recentTracks)"
          class="glass-card rounded-2xl p-3 space-y-2 cursor-pointer group"
        >
          <div class="relative aspect-square rounded-xl overflow-hidden border border-white/10">
            <img v-if="track.cover" :src="track.cover" class="w-full h-full object-cover group-hover:scale-105 transition-transform" />
            <div v-else class="w-full h-full bg-white/5 flex items-center justify-center">
              <Music :size="20" class="text-slate-400" />
            </div>
          </div>
          <div>
            <h4 class="text-xs font-semibold text-white truncate group-hover:text-brand transition-colors">
              {{ track.title }}
            </h4>
            <p class="text-[10px] text-slate-400 truncate">
              {{ track.artist }}
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Hot Playlists (热门歌单) -->
    <section class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-lg font-bold text-white tracking-tight">精选热门歌单</h2>
          <p class="text-xs text-slate-400">探索适合各种场景的声音氛围</p>
        </div>
      </div>

      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-5">
        <div
          v-for="pl in playlistStore.playlists"
          :key="pl.id"
          @click="router.push(`/playlist/${pl.id}`)"
          class="glass-card rounded-2xl p-3.5 space-y-3 cursor-pointer group"
        >
          <div class="relative aspect-square rounded-xl overflow-hidden border border-white/10">
            <img :src="pl.cover" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300" />
            <div class="absolute inset-0 bg-black/30 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
              <div class="w-10 h-10 rounded-full bg-brand text-white flex items-center justify-center shadow-lg shadow-brand/40">
                <Play :size="18" fill="white" class="ml-0.5" />
              </div>
            </div>
          </div>
          <div>
            <h4 class="text-xs font-semibold text-white truncate group-hover:text-brand transition-colors">
              {{ pl.name }}
            </h4>
            <p class="text-[11px] text-slate-400 line-clamp-1 mt-0.5">
              {{ pl.description || '精选曲目' }}
            </p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { Play, Sparkles, Heart, Clock, Music, RefreshCw } from 'lucide-vue-next';
import { usePlayerStore } from '@/stores/player';
import { usePlaylistStore } from '@/stores/playlist';
import { useContextMenuStore } from '@/stores/contextMenu';
import { invoke } from '@tauri-apps/api/core';
import type { Track } from '@/types/music';

const router = useRouter();
const playerStore = usePlayerStore();
const playlistStore = usePlaylistStore();
const contextMenuStore = useContextMenuStore();

const dynamicDaily = ref<Track[]>([]);
const recentTracks = ref<Track[]>([]);
const isLoadingDaily = ref(false);

const todayLabel = computed(() => {
  const now = new Date();
  return `${now.getMonth() + 1}月${now.getDate()}日`;
});

const displayDailyTracks = computed(() => {
  return dynamicDaily.value.length > 0 ? dynamicDaily.value : playerStore.queue;
});

const loadRecommendations = async () => {
  isLoadingDaily.value = true;
  try {
    const [onlineDaily, localDaily, recent] = await Promise.allSettled([
      invoke<Track[]>('netease_get_daily_songs', { limit: 12 }),
      invoke<Track[]>('get_daily_recommendations', { limit: 4 }),
      invoke<Track[]>('get_recent_played', { limit: 6 }),
    ]);

    const merged: Track[] = [];
    const seen = new Set<string>();
    if (onlineDaily.status === 'fulfilled' && onlineDaily.value?.length) {
      for (const t of onlineDaily.value) {
        if (!seen.has(t.id)) {
          seen.add(t.id);
          merged.push(t);
        }
      }
    }
    if (localDaily.status === 'fulfilled' && localDaily.value?.length) {
      for (const t of localDaily.value) {
        if (!seen.has(t.id)) {
          seen.add(t.id);
          merged.push(t);
        }
      }
    }
    if (merged.length > 0) {
      dynamicDaily.value = merged.slice(0, 8);
    }
    if (recent.status === 'fulfilled' && recent.value?.length) {
      recentTracks.value = recent.value;
    }
  } catch (e) {
    // Fallback in web mode
  } finally {
    isLoadingDaily.value = false;
  }
};

const refreshDaily = async () => {
  if (isLoadingDaily.value) return;
  if (dynamicDaily.value.length > 8) {
    dynamicDaily.value = [...dynamicDaily.value.slice(8), ...dynamicDaily.value.slice(0, 8)];
    return;
  }
  await loadRecommendations();
  if (dynamicDaily.value.length > 1) {
    dynamicDaily.value = [...dynamicDaily.value].sort(() => Math.random() - 0.5);
  }
};

const playFeatured = () => {
  if (playerStore.queue.length > 0) {
    playerStore.playTrack(playerStore.queue[0]);
  }
};

const playAllDaily = () => {
  const tracks = displayDailyTracks.value;
  if (tracks.length > 0) {
    playerStore.playTrack(tracks[0], tracks);
  }
};

onMounted(() => {
  loadRecommendations();
});
</script>
