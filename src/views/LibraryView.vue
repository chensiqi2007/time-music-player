<template>
  <div class="space-y-6 pb-12">
    <!-- Header with Action Buttons -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <h1 class="text-2xl font-bold text-white tracking-tight">{{ $t('library.title') }}</h1>
        <p class="text-xs text-slate-400 mt-1">
          共 {{ allTracks.length }} 首本地音乐 • {{ libraryStore.albums.length }} 张专辑 • {{ libraryStore.artists.length }} 位艺术家
        </p>
      </div>

      <div class="flex items-center gap-3">
        <button
          @click="startScan"
          :disabled="libraryStore.isScanning"
          class="px-4 py-2 rounded-xl bg-brand hover:bg-brand-light text-white text-xs font-semibold flex items-center gap-2 shadow-lg shadow-brand/20 active:scale-95 transition-all disabled:opacity-50"
        >
          <RefreshCw :size="14" :class="libraryStore.isScanning ? 'animate-spin' : ''" />
          <span>{{ libraryStore.isScanning ? $t('library.scanning') : $t('library.scan_btn') }}</span>
        </button>

        <button
          @click="showFolderPickerModal = true"
          class="px-4 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-white text-xs font-medium flex items-center gap-2 border border-white/10 transition-colors"
        >
          <FolderPlus :size="14" />
          <span>{{ $t('library.manage_folder') }}</span>
        </button>
      </div>
    </div>

    <!-- Scanned paths indicator -->
    <div class="glass-card rounded-2xl p-4 flex flex-wrap items-center justify-between gap-2 text-xs">
      <div class="flex flex-wrap items-center gap-2">
        <span class="text-slate-400 font-medium">{{ $t('library.monitored_dirs') }}：</span>
        <span
          v-for="p in libraryStore.scanPaths"
          :key="p"
          class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-white/10 text-slate-200 font-mono text-[11px] group"
        >
          <Folder :size="12" class="text-brand" />
          {{ p }}
          <button
            @click="libraryStore.removeScanPath(p)"
            class="text-slate-500 hover:text-red-400 opacity-60 group-hover:opacity-100 ml-1 transition-opacity"
            title="移除此目录"
          >
            ×
          </button>
        </span>
      </div>

      <span class="text-[11px] text-slate-400 font-mono">
        支持: MP3, FLAC, WAV, AAC, OGG, M4A, APE
      </span>
    </div>

    <!-- View Tabs -->
    <div class="flex items-center gap-2 border-b border-white/[0.08] pb-3">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        @click="activeTab = tab.id"
        class="px-3 py-1.5 rounded-lg text-xs font-medium transition-all"
        :class="activeTab === tab.id ? 'bg-white/15 text-white font-semibold' : 'text-slate-400 hover:text-white'"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- TAB 1: 全部歌曲列表 / 收藏歌曲列表 -->
    <div v-if="activeTab === 'all' || activeTab === 'favorites'" class="glass-panel rounded-2xl overflow-hidden border border-white/10">
      <!-- Table Header -->
      <div class="grid grid-cols-12 px-4 py-3 text-[11px] font-semibold text-slate-400 uppercase tracking-wider border-b border-white/[0.08] bg-white/[0.02]">
        <div class="col-span-1 text-center">#</div>
        <div class="col-span-5">歌曲标题</div>
        <div class="col-span-3">专辑</div>
        <div class="col-span-1">格式</div>
        <div class="col-span-2 text-right">时长 / 操作</div>
      </div>

      <!-- Empty State -->
      <div v-if="allTracks.length === 0" class="py-16 text-center text-xs text-slate-400 space-y-2">
        <Heart :size="28" class="mx-auto text-slate-600 mb-1" />
        <p>{{ activeTab === 'favorites' ? '暂无收藏的音乐，点击歌曲右侧的「心形」图标即可加入收藏' : '曲库暂无歌曲，请先扫描本地音乐目录' }}</p>
      </div>

      <!-- Table Body -->
      <div v-else class="divide-y divide-white/[0.04]">
        <div
          v-for="(track, index) in displayedTracks"
          :key="track.id"
          @click="playerStore.playTrack(track, allTracks)"
          @dblclick="playerStore.playTrack(track, allTracks)"
          @contextmenu.prevent.stop="contextMenuStore.openTrackMenu($event, track, allTracks)"
          class="grid grid-cols-12 px-4 py-3 items-center text-xs text-slate-300 hover:bg-white/[0.06] group cursor-pointer transition-colors"
          :class="playerStore.currentTrack?.id === track.id ? 'bg-brand/10 text-brand font-medium' : ''"
        >
          <!-- Index or Play Icon -->
          <div class="col-span-1 text-center text-slate-400 font-mono flex items-center justify-center">
            <div v-if="playerStore.currentTrack?.id === track.id && playerStore.isPlaying" class="flex items-end gap-0.5 h-3 shrink-0 px-1">
              <span class="w-0.5 bg-brand animate-bar-1 h-full rounded-full"></span>
              <span class="w-0.5 bg-brand animate-bar-2 h-1/2 rounded-full"></span>
              <span class="w-0.5 bg-brand animate-bar-3 h-3/4 rounded-full"></span>
            </div>
            <span v-else class="group-hover:hidden">{{ index + 1 }}</span>
            <button
              @click.stop="playerStore.playTrack(track, allTracks)"
              class="hidden group-hover:inline-block text-white hover:text-brand"
            >
              <Play :size="13" fill="currentColor" />
            </button>
          </div>

          <!-- Title & Artist & Cover -->
          <div class="col-span-5 flex items-center gap-3 min-w-0 pr-4">
            <img
              v-if="track.cover"
              :src="track.cover"
              loading="lazy"
              decoding="async"
              class="w-9 h-9 rounded-lg object-cover border border-white/10 shrink-0"
            />
            <div v-else class="w-9 h-9 rounded-lg bg-white/5 border border-white/10 flex items-center justify-center shrink-0">
              <Music :size="14" class="text-slate-400" />
            </div>
            <div class="min-w-0 flex-1">
              <p class="font-medium text-white truncate group-hover:text-brand transition-colors">
                {{ track.title }}
              </p>
              <p class="text-[11px] text-slate-400 truncate">
                {{ track.artist }}
              </p>
            </div>
          </div>

          <!-- Album -->
          <div class="col-span-3 truncate text-slate-400 text-xs pr-4">
            {{ track.album || '未知专辑' }}
          </div>

          <!-- Format / Quality -->
          <div class="col-span-1 font-mono text-[10px] text-slate-400">
            <span class="px-1.5 py-0.5 rounded bg-white/5 border border-white/10">
              {{ track.format ? track.format.split(' ')[0] : 'MP3' }}
            </span>
          </div>

          <!-- Duration & Actions -->
          <div class="col-span-2 flex items-center justify-end gap-2 text-slate-400 font-mono text-xs">
            <button
              @click.stop="playlistStore.openAddToPlaylistModal(track)"
              class="p-1 hover:text-white transition-colors opacity-0 group-hover:opacity-100"
              title="添加到歌单"
            >
              <PlusCircle :size="13" />
            </button>
            <button
              @click.stop="openTagEditor(track)"
              class="p-1 hover:text-white transition-colors opacity-0 group-hover:opacity-100"
              title="编辑 ID3 标签"
            >
              <Edit3 :size="13" />
            </button>
            <button
              @click.stop="playerStore.toggleFavorite(track)"
              class="p-1 hover:text-brand transition-colors"
              :class="(track.isFavorite || playerStore.isIdFavorite(track.id)) ? 'text-brand' : ''"
              title="收藏"
            >
              <Heart :size="14" :fill="(track.isFavorite || playerStore.isIdFavorite(track.id)) ? 'currentColor' : 'none'" />
            </button>
            <span class="tabular-nums w-10 text-right">{{ formatDuration(track.duration) }}</span>
          </div>
        </div>

        <!-- Progressive Load More Trigger -->
        <div v-if="hasMoreTracks" class="p-3.5 flex items-center justify-center border-t border-white/[0.06] bg-white/[0.02]">
          <button
            @click="loadMoreTracks"
            class="px-5 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-xs font-medium text-slate-300 hover:text-white transition-all flex items-center gap-2 active:scale-95"
          >
            <span>加载更多曲目 (已显示 {{ displayedTracks.length }} / 共 {{ allTracks.length }} 首)</span>
          </button>
        </div>
      </div>
    </div>

    <!-- TAB 2: 专辑视图 -->
    <div v-else-if="activeTab === 'albums'" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-5">
      <div
        v-for="alb in libraryStore.albums"
        :key="alb.id"
        @click="playAlbum(alb)"
        class="glass-card rounded-2xl p-3.5 space-y-3 cursor-pointer group hover:border-brand/40 transition-all"
      >
        <div class="relative aspect-square rounded-xl overflow-hidden border border-white/10">
          <img v-if="alb.cover" :src="alb.cover" loading="lazy" decoding="async" class="w-full h-full object-cover group-hover:scale-105 transition-transform" />
          <div v-else class="w-full h-full bg-white/5 flex items-center justify-center">
            <Disc :size="32" class="text-slate-500" />
          </div>
        </div>
        <div>
          <h4 class="text-xs font-semibold text-white truncate group-hover:text-brand transition-colors">
            {{ alb.title }}
          </h4>
          <p class="text-[11px] text-slate-400 truncate mt-0.5">
            {{ alb.artist }}
          </p>
          <span class="text-[10px] text-slate-500 font-mono mt-1 inline-block">
            {{ alb.tracksCount }} 首曲目
          </span>
        </div>
      </div>
    </div>

    <!-- TAB 3: 歌手视图 -->
    <div v-else-if="activeTab === 'artists'" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6">
      <div
        v-for="art in libraryStore.artists"
        :key="art.id"
        @click="playArtist(art)"
        class="glass-card rounded-2xl p-4 flex flex-col items-center text-center space-y-3 cursor-pointer group hover:border-brand/40 transition-all"
      >
        <div class="w-24 h-24 rounded-full overflow-hidden border border-white/15 shadow-lg shadow-black/40">
          <img v-if="art.avatar" :src="art.avatar" loading="lazy" decoding="async" class="w-full h-full object-cover group-hover:scale-105 transition-transform" />
          <div v-else class="w-full h-full bg-white/5 flex items-center justify-center">
            <User :size="28" class="text-slate-400" />
          </div>
        </div>
        <div>
          <h4 class="text-xs font-bold text-white truncate group-hover:text-brand transition-colors">
            {{ art.name }}
          </h4>
          <p class="text-[10px] text-slate-400 mt-0.5">
            {{ art.tracksCount }} 首歌曲
          </p>
        </div>
      </div>
    </div>

    <!-- ID3 Tag Editor Modal -->
    <div
      v-if="editingTrack"
      class="fixed inset-0 z-50 bg-black/70 backdrop-blur-md flex items-center justify-center p-4 select-none"
    >
      <div class="glass-panel w-full max-w-md rounded-2xl p-6 border border-white/15 space-y-5 shadow-2xl">
        <div class="flex items-center justify-between border-b border-white/[0.08] pb-3">
          <h3 class="text-sm font-bold text-white flex items-center gap-2">
            <Edit3 :size="16" class="text-brand" />
            编辑 ID3 音频标签
          </h3>
          <button @click="editingTrack = null" class="text-slate-400 hover:text-white">
            ×
          </button>
        </div>

        <div class="space-y-3.5 text-xs">
          <div>
            <label class="block text-slate-400 mb-1">歌曲标题</label>
            <input
              v-model="editForm.title"
              type="text"
              class="w-full h-8 px-3 rounded-lg bg-white/10 border border-white/15 text-white focus:outline-none focus:border-brand"
            />
          </div>

          <div>
            <label class="block text-slate-400 mb-1">艺术家</label>
            <input
              v-model="editForm.artist"
              type="text"
              class="w-full h-8 px-3 rounded-lg bg-white/10 border border-white/15 text-white focus:outline-none focus:border-brand"
            />
          </div>

          <div>
            <label class="block text-slate-400 mb-1">专辑名称</label>
            <input
              v-model="editForm.album"
              type="text"
              class="w-full h-8 px-3 rounded-lg bg-white/10 border border-white/15 text-white focus:outline-none focus:border-brand"
            />
          </div>

          <div class="pt-1 text-[10px] text-slate-500 font-mono break-all">
            文件路径: {{ editingTrack.path }}
          </div>
        </div>

        <div class="flex items-center justify-end gap-3 pt-2">
          <button
            @click="editingTrack = null"
            class="px-4 py-1.5 rounded-lg bg-white/10 text-white text-xs hover:bg-white/15 transition-colors"
          >
            取消
          </button>
          <button
            @click="saveTags"
            class="px-4 py-1.5 rounded-lg bg-brand hover:bg-brand-light text-white text-xs font-semibold shadow-md shadow-brand/20 transition-all"
          >
            保存并写入文件
          </button>
        </div>
      </div>
    </div>

    <!-- Visual Folder Picker & Monitored Directory Modal -->
    <div
      v-if="showFolderPickerModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md animate-fade-in"
      @click.self="showFolderPickerModal = false"
    >
      <div class="relative w-full max-w-lg bg-[#14141e]/95 border border-white/15 rounded-3xl p-6 md:p-7 shadow-2xl space-y-5 overflow-hidden">
        <!-- Ambient glow -->
        <div class="absolute -top-20 -right-20 w-48 h-48 rounded-full bg-brand/20 blur-3xl pointer-events-none"></div>

        <!-- Header -->
        <div class="flex items-center justify-between pb-3.5 border-b border-white/10 relative z-10">
          <div class="flex items-center gap-2.5">
            <div class="w-9 h-9 rounded-2xl bg-brand/20 border border-brand/30 flex items-center justify-center text-brand">
              <FolderPlus :size="18" />
            </div>
            <div>
              <h3 class="text-base font-bold text-white tracking-wide">{{ $t('library.modal_title') }}</h3>
              <p class="text-[11px] text-slate-400">{{ $t('library.modal_subtitle') }}</p>
            </div>
          </div>
          <button
            @click="showFolderPickerModal = false"
            class="p-1.5 rounded-full hover:bg-white/10 text-slate-400 hover:text-white transition-colors"
          >
            <X :size="18" />
          </button>
        </div>

        <!-- 1. Native Folder Picker Button (Prominent Card) -->
        <button
          @click="handleBrowseFolder"
          class="w-full p-4 rounded-2xl bg-gradient-to-r from-brand/20 via-brand/10 to-transparent hover:from-brand/30 hover:to-brand/10 border border-brand/40 text-white flex items-center gap-3.5 transition-all group active:scale-[0.99] text-left"
        >
          <div class="w-11 h-11 rounded-xl bg-brand text-white flex items-center justify-center shadow-lg shadow-brand/40 group-hover:scale-105 transition-transform shrink-0">
            <FolderOpen :size="20" />
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-xs font-bold text-white tracking-wide flex items-center gap-2">
              <span>{{ $t('library.native_browse') }}</span>
              <span class="text-[10px] px-2 py-0.5 rounded-full bg-brand/30 text-brand-light font-mono font-normal">{{ $t('library.recommend_tag') }}</span>
            </p>
            <p class="text-[11px] text-slate-300 mt-0.5 leading-snug">
              {{ $t('library.native_browse_desc') }}
            </p>
          </div>
        </button>

        <!-- 2. System Presets Quick-Add Chips -->
        <div class="space-y-2">
          <p class="text-xs font-semibold text-white flex items-center gap-1.5">
            <Sparkles :size="13" class="text-brand" />
            <span>{{ $t('library.preset_dirs') }}</span>
          </p>
          <div class="flex flex-wrap gap-2">
            <button
              v-for="p in settingsStore.systemPresets"
              :key="p.id"
              @click="addPresetPath(p.path)"
              :disabled="libraryStore.scanPaths.includes(p.path)"
              class="px-3 py-1.5 rounded-xl text-xs border transition-all flex items-center gap-1.5 disabled:opacity-40 disabled:cursor-not-allowed"
              :class="libraryStore.scanPaths.includes(p.path) ? 'bg-white/5 border-white/10 text-slate-500' : 'bg-white/10 hover:bg-white/15 border-white/15 text-slate-200 hover:text-white active:scale-95'"
            >
              <Music v-if="p.icon_type === 'music'" :size="12" class="text-brand" />
              <Download v-else-if="p.icon_type === 'download'" :size="12" class="text-blue-400" />
              <Laptop v-else-if="p.icon_type === 'desktop'" :size="12" class="text-purple-400" />
              <HardDrive v-else :size="12" class="text-cyan-400" />
              <span>{{ p.name }}</span>
              <Check v-if="libraryStore.scanPaths.includes(p.path)" :size="12" class="text-emerald-400 ml-0.5" />
            </button>
          </div>
        </div>

        <!-- 3. Current Monitored Paths List -->
        <div class="space-y-2 pt-1 border-t border-white/10">
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold text-white">
              {{ $t('library.current_monitored') }} ({{ libraryStore.scanPaths.length }})
            </span>
          </div>
          <div class="max-h-36 overflow-y-auto space-y-1.5 pr-1">
            <div
              v-for="path in libraryStore.scanPaths"
              :key="path"
              class="flex items-center justify-between p-2.5 rounded-xl bg-white/[0.04] border border-white/5 text-xs group"
            >
              <div class="flex items-center gap-2 min-w-0 pr-2">
                <Folder :size="14" class="text-brand shrink-0" />
                <span class="font-mono text-slate-300 truncate text-[11px]">{{ path }}</span>
              </div>
              <button
                @click="libraryStore.removeScanPath(path)"
                class="p-1 rounded-md text-slate-500 hover:text-red-400 hover:bg-red-500/10 transition-colors shrink-0"
                title="移除监控"
              >
                <X :size="14" />
              </button>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="flex items-center justify-between pt-2 border-t border-white/10">
          <button
            @click="showFolderPickerModal = false"
            class="px-4 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-slate-300 hover:text-white text-xs font-medium transition-colors"
          >
            {{ $t('common.finish') }}
          </button>
          <button
            @click="handleStartScanAndClose"
            :disabled="libraryStore.isScanning || libraryStore.scanPaths.length === 0"
            class="px-5 py-2 rounded-xl bg-gradient-to-r from-brand to-pink-500 hover:opacity-90 text-white text-xs font-semibold shadow-md shadow-brand/30 transition-all active:scale-95 disabled:opacity-50"
          >
            {{ $t('library.scan_now') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch } from 'vue';
import { useRoute } from 'vue-router';
import {
  RefreshCw,
  FolderPlus,
  Folder,
  Play,
  Heart,
  Music,
  Edit3,
  Disc,
  User,
  PlusCircle,
  FolderOpen,
  HardDrive,
  X,
  Laptop,
  Download,
  Check,
  Sparkles,
} from 'lucide-vue-next';
import { useLibraryStore } from '@/stores/library';
import { usePlayerStore } from '@/stores/player';
import { usePlaylistStore } from '@/stores/playlist';
import { useSettingsStore } from '@/stores/settings';
import type { Track } from '@/types/music';
import { useContextMenuStore } from '@/stores/contextMenu';

const libraryStore = useLibraryStore();
const playerStore = usePlayerStore();
const playlistStore = usePlaylistStore();
const settingsStore = useSettingsStore();
const contextMenuStore = useContextMenuStore();
const showFolderPickerModal = ref(false);

const handleBrowseFolder = async () => {
  await libraryStore.chooseFolderToAdd();
};

const addPresetPath = (path: string) => {
  libraryStore.addScanPath(path);
};

const handleStartScanAndClose = async () => {
  showFolderPickerModal.value = false;
  await libraryStore.scanDirectories();
};



const route = useRoute();
const activeTab = ref('all');
const editingTrack = ref<Track | null>(null);
const editForm = reactive({
  title: '',
  artist: '',
  album: '',
});

const tabs = [
  { id: 'all', label: '全部歌曲' },
  { id: 'favorites', label: '收藏歌曲' },
  { id: 'albums', label: '专辑分类' },
  { id: 'artists', label: '歌手分类' },
];

watch(
  () => route.query.tab,
  (newTab) => {
    if (newTab === 'favorites') {
      activeTab.value = 'favorites';
    } else if (newTab === 'albums' || newTab === 'artists' || newTab === 'all') {
      activeTab.value = newTab as string;
    }
  },
  { immediate: true }
);

const baseTracks = computed(() => {
  return libraryStore.tracks.length > 0 ? libraryStore.tracks : playerStore.queue;
});

const allTracks = computed(() => {
  if (activeTab.value === 'favorites') {
    const list: Track[] = [];
    const seen = new Set<string>();
    for (const t of [...playerStore.favoriteCache, ...baseTracks.value]) {
      if ((t.isFavorite || playerStore.isIdFavorite(t.id)) && !seen.has(t.id)) {
        list.push({ ...t, isFavorite: true });
        seen.add(t.id);
      }
    }
    return list;
  }
  return baseTracks.value;
});

// High-performance progressive rendering for large libraries
const displayLimit = ref(60);
const displayedTracks = computed(() => {
  return allTracks.value.slice(0, displayLimit.value);
});
const hasMoreTracks = computed(() => {
  return displayedTracks.value.length < allTracks.value.length;
});
const loadMoreTracks = () => {
  displayLimit.value += 60;
};

const startScan = async () => {
  await libraryStore.scanDirectories();
};

const openTagEditor = (track: Track) => {
  editingTrack.value = track;
  editForm.title = track.title;
  editForm.artist = track.artist;
  editForm.album = track.album;
};

const saveTags = async () => {
  if (!editingTrack.value) return;
  await libraryStore.updateTrackMetadata(
    editingTrack.value.id,
    editForm.title,
    editForm.artist,
    editForm.album
  );
  editingTrack.value = null;
};

const playAlbum = (alb: any) => {
  const albumTracks = libraryStore.tracks.filter((t) => t.album === alb.title);
  if (albumTracks.length > 0) {
    playerStore.playTrack(albumTracks[0], albumTracks);
    playerStore.showToast(`正在播放专辑《${alb.title}》`);
  } else {
    playerStore.showToast(`专辑《${alb.title}》暂无曲目`);
  }
};

const playArtist = (art: any) => {
  const artistTracks = libraryStore.tracks.filter((t) => t.artist === art.name);
  if (artistTracks.length > 0) {
    playerStore.playTrack(artistTracks[0], artistTracks);
    playerStore.showToast(`正在播放歌手「${art.name}」的精选曲目`);
  } else {
    playerStore.showToast(`歌手「${art.name}」暂无曲目`);
  }
};

const formatDuration = (seconds?: number): string => {
  if (!seconds) return '0:00';
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  return `${m}:${s.toString().padStart(2, '0')}`;
};
</script>
