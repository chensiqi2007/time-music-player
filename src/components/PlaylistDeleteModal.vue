<template>
  <Transition name="fade">
    <div
      v-if="playlistStore.isDeleteModalOpen && playlistStore.deletingPlaylist"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md"
      @click.self="playlistStore.closeDeleteModal()"
    >
      <div
        class="w-full max-w-sm rounded-3xl glass-card border border-white/15 p-6 shadow-2xl relative overflow-hidden space-y-5 animate-in fade-in zoom-in-95 duration-150"
      >
        <div class="flex items-start justify-between gap-3">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-2xl bg-red-500/15 border border-red-500/30 text-red-400 flex items-center justify-center shrink-0">
              <Trash2 :size="18" />
            </div>
            <div>
              <h3 class="text-base font-bold text-white">删除歌单</h3>
              <p class="text-[11px] text-slate-400">确认移除此本地歌单？</p>
            </div>
          </div>
          <button
            @click="playlistStore.closeDeleteModal()"
            class="text-slate-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors"
          >
            <X :size="16" />
          </button>
        </div>

        <div class="p-3 rounded-2xl bg-theme-surface border border-theme-border flex items-center gap-3">
          <img
            v-if="playlistStore.deletingPlaylist.cover"
            :src="playlistStore.deletingPlaylist.cover"
            class="w-11 h-11 rounded-xl object-cover border border-white/10 shrink-0"
          />
          <div class="min-w-0 flex-1">
            <p class="text-xs font-semibold text-white truncate">
              {{ playlistStore.deletingPlaylist.name }}
            </p>
            <p class="text-[11px] text-slate-400 mt-0.5">
              包含 {{ playlistStore.deletingPlaylist.trackIds.length }} 首歌曲
            </p>
          </div>
        </div>

        <p class="text-[11px] text-slate-400 leading-relaxed">
          删除后歌单将从本地数据库移除，但不会删除硬盘上的本地音频文件。
        </p>

        <div class="flex items-center justify-end gap-2.5 pt-1">
          <button
            @click="playlistStore.closeDeleteModal()"
            class="px-4 py-2 rounded-xl bg-theme-surface hover:bg-theme-surfaceElevated text-slate-300 hover:text-white text-xs font-medium border border-theme-border transition-colors"
          >
            取消
          </button>
          <button
            @click="handleConfirmDelete"
            class="px-4 py-2 rounded-xl bg-red-500 hover:bg-red-600 text-white text-xs font-semibold shadow-lg shadow-red-500/25 active:scale-95 transition-all"
          >
            确认删除
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';
import { Trash2, X } from 'lucide-vue-next';
import { usePlaylistStore } from '@/stores/playlist';

const route = useRoute();
const router = useRouter();
const playlistStore = usePlaylistStore();

const handleConfirmDelete = async () => {
  const targetId = playlistStore.deletingPlaylist?.id;
  await playlistStore.confirmDeletePlaylist();
  if (targetId && route.path === `/playlist/${targetId}`) {
    router.push('/');
  }
};
</script>
