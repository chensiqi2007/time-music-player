<template>
  <Transition name="fade">
    <div
      v-if="importerStore.isModalOpen"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md"
      @click.self="importerStore.closeModal()"
    >
      <div
        class="w-full max-w-lg rounded-3xl glass-card border border-white/15 p-6 shadow-2xl relative overflow-hidden flex flex-col max-h-[90vh] animate-in fade-in zoom-in-95 duration-200"
      >
        <!-- Modal Header -->
        <div class="flex items-center justify-between pb-4 border-b border-white/10">
          <div class="flex items-center gap-2.5">
            <div class="w-9 h-9 rounded-xl bg-gradient-to-tr from-brand to-purple-600 flex items-center justify-center text-white shadow-md">
              <Link2 :size="18" />
            </div>
            <div>
              <h2 class="text-base font-bold text-white tracking-wide">
                一键导入外部流媒体歌单
              </h2>
              <p class="text-[11px] text-slate-400">
                支持 Spotify、Apple Music、网易云音乐、QQ 音乐公开分享链接
              </p>
            </div>
          </div>
          <button
            @click="importerStore.closeModal()"
            class="text-slate-400 hover:text-white p-1.5 rounded-full hover:bg-white/10 transition-colors"
          >
            <X :size="18" />
          </button>
        </div>

        <!-- Body -->
        <div class="space-y-4 py-4 overflow-y-auto pr-1">
          <!-- Supported Platforms Badges -->
          <div class="flex flex-wrap items-center gap-2">
            <span class="text-[11px] text-slate-400">支持平台：</span>
            <span class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-[10px] font-medium">
              <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span> Spotify
            </span>
            <span class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-rose-500/10 border border-rose-500/20 text-rose-400 text-[10px] font-medium">
              <span class="w-1.5 h-1.5 rounded-full bg-rose-400"></span> Apple Music
            </span>
            <span class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-red-500/10 border border-red-500/20 text-red-400 text-[10px] font-medium">
              <span class="w-1.5 h-1.5 rounded-full bg-red-400"></span> 网易云音乐
            </span>
            <span class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-teal-500/10 border border-teal-500/20 text-teal-400 text-[10px] font-medium">
              <span class="w-1.5 h-1.5 rounded-full bg-teal-400"></span> QQ 音乐
            </span>
          </div>

          <!-- URL Input Box -->
          <div class="space-y-1.5">
            <div class="flex items-center justify-between">
              <label class="block text-xs font-semibold text-slate-300">
                歌单分享链接 <span class="text-brand">*</span>
              </label>
              <button
                type="button"
                @click="pasteClipboard"
                class="px-2.5 py-0.5 rounded-lg bg-white/10 hover:bg-white/15 text-[11px] text-brand-light flex items-center gap-1 transition-colors active:scale-95"
              >
                <Clipboard :size="11" />
                <span>一键读取剪贴板</span>
              </button>
            </div>
            <div class="relative flex items-center">
              <input
                v-model="inputUrl"
                @keydown.enter="handleParse"
                type="url"
                placeholder="例如：https://open.spotify.com/playlist/... 或 Apple Music 歌单"
                class="w-full pl-3.5 pr-24 py-2.5 rounded-xl bg-white/[0.07] border border-white/10 text-white placeholder-slate-500 text-xs focus:outline-none focus:border-brand/60 focus:bg-white/[0.1] transition-all font-mono"
              />
              <button
                @click="handleParse"
                :disabled="!inputUrl.trim() || importerStore.isParsing"
                class="absolute right-1.5 top-1/2 -translate-y-1/2 px-3.5 py-1.5 rounded-lg bg-brand hover:bg-brand-light text-white text-xs font-semibold flex items-center gap-1.5 shadow-md shadow-brand/20 transition-all disabled:opacity-50"
              >
                <RefreshCw v-if="importerStore.isParsing" :size="12" class="animate-spin" />
                <span>{{ importerStore.isParsing ? '解析中...' : '解析歌单' }}</span>
              </button>
            </div>
            <p v-if="importerStore.parseError" class="text-[11px] text-red-400 flex items-center gap-1 mt-1">
              <AlertCircle :size="12" />
              <span>{{ importerStore.parseError }}</span>
            </p>
          </div>

          <!-- Preview Card (When Parsed) -->
          <div
            v-if="importerStore.preview"
            class="rounded-2xl bg-white/[0.04] border border-white/10 p-4 space-y-3.5 animate-in fade-in duration-200"
          >
            <div class="flex gap-3.5 items-start">
              <img
                :src="importerStore.preview.cover || defaultCover"
                alt="Cover"
                class="w-20 h-20 rounded-xl object-cover border border-white/15 shrink-0 shadow-lg"
              />
              <div class="min-w-0 flex-1 space-y-1">
                <div class="flex items-center gap-1.5">
                  <span class="px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider bg-brand/20 text-brand border border-brand/30">
                    {{ importerStore.preview.platform_name }}
                  </span>
                  <span class="text-[10px] text-slate-400">共 {{ importerStore.preview.tracks_count }} 首歌曲</span>
                </div>
                <h3 class="text-sm font-bold text-white truncate">
                  {{ importerStore.preview.name }}
                </h3>
                <p class="text-[11px] text-slate-400 line-clamp-2 leading-relaxed">
                  {{ importerStore.preview.description || '暂无描述' }}
                </p>
              </div>
            </div>

            <!-- Preview Tracks List -->
            <div class="border-t border-white/[0.06] pt-2 space-y-1">
              <p class="text-[10px] font-semibold text-slate-400 uppercase tracking-wider mb-1">
                曲目预览 (前 {{ Math.min(importerStore.preview.tracks.length, 6) }} 首)
              </p>
              <div class="max-h-36 overflow-y-auto divide-y divide-white/[0.04] pr-1">
                <div
                  v-for="(t, idx) in importerStore.preview.tracks.slice(0, 15)"
                  :key="idx"
                  class="flex items-center justify-between py-1.5 text-xs text-slate-300"
                >
                  <div class="flex items-center gap-2 min-w-0 pr-2">
                    <span class="text-[10px] font-mono text-slate-500 w-4 text-center">{{ idx + 1 }}</span>
                    <span class="font-medium text-white truncate">{{ t.title }}</span>
                    <span class="text-slate-500 truncate">- {{ t.artist }}</span>
                  </div>
                  <span class="text-[10px] text-slate-400 font-mono tabular-nums">
                    {{ formatDuration(t.duration) }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Footer Actions -->
        <div class="flex items-center justify-between pt-3 border-t border-white/10">
          <span class="text-[10px] text-slate-500">导入后可离线播放或双击自动匹配高保真在线流</span>
          <div class="flex items-center gap-2.5">
            <button
              type="button"
              @click="importerStore.closeModal()"
              class="px-4 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-slate-300 hover:text-white text-xs font-medium transition-colors"
            >
              取消
            </button>
            <button
              @click="handleConfirmImport"
              :disabled="!importerStore.preview || importerStore.isImporting"
              class="px-5 py-2 rounded-xl bg-brand hover:bg-brand-light text-white text-xs font-semibold shadow-lg shadow-brand/25 transition-all disabled:opacity-50 flex items-center gap-1.5"
            >
              <Download :size="13" />
              <span>{{ importerStore.isImporting ? '正在生成歌单...' : '确认导入到 Time Music' }}</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import {
  X,
  Link2,
  RefreshCw,
  AlertCircle,
  Download,
  Clipboard,
} from 'lucide-vue-next';
import { useImporterStore } from '@/stores/importer';
import { usePlaylistStore } from '@/stores/playlist';

const importerStore = useImporterStore();
const playlistStore = usePlaylistStore();
const router = useRouter();

const inputUrl = ref('');
const defaultCover = 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=500&auto=format&fit=crop&q=80';

const pasteClipboard = async () => {
  try {
    const text = await navigator.clipboard.readText();
    if (text && text.trim()) {
      inputUrl.value = text.trim();
      importerStore.parseLink(inputUrl.value);
    }
  } catch (err) {
    console.warn('Clipboard read permission notice:', err);
  }
};

const handleParse = () => {
  if (inputUrl.value.trim()) {
    importerStore.parseLink(inputUrl.value.trim());
  }
};

const handleConfirmImport = async () => {
  const created = await importerStore.confirmImport();
  if (created) {
    await playlistStore.initPlaylists();
    router.push(`/playlist/${created.id}`);
  }
};

const formatDuration = (seconds?: number): string => {
  if (!seconds) return '0:00';
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  return `${m}:${s.toString().padStart(2, '0')}`;
};
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

