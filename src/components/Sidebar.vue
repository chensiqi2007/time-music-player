<template>
  <aside class="w-60 h-full flex flex-col justify-between py-3 px-2.5 select-none glass-nav text-sm text-slate-300 shrink-0">
    <!-- Top Section: Main Navigation & Playlists -->
    <div class="space-y-4 overflow-y-auto pr-1">
      <!-- Group 1: 探索 -->
      <div>
        <div class="px-3 pb-1.5 text-[10px] font-semibold text-slate-400/80 uppercase tracking-wider">
          {{ $t('sidebar.explore') }}
        </div>
        <nav class="space-y-0.5">
          <RouterLink
            to="/"
            class="relative flex items-center gap-2.5 px-3 py-1.5 rounded-xl text-xs font-medium transition-all"
            :class="route.path === '/' ? 'bg-brand/15 text-brand font-semibold' : 'hover:bg-white/[0.06] text-slate-300 hover:text-white'"
          >
            <span v-if="route.path === '/'" class="absolute left-0 top-1.5 bottom-1.5 w-[3px] rounded-r-full bg-brand"></span>
            <Home :size="15" />
            <span>{{ $t('sidebar.home') }}</span>
          </RouterLink>

          <RouterLink
            to="/discover"
            class="relative flex items-center gap-2.5 px-3 py-1.5 rounded-xl text-xs font-medium transition-all"
            :class="route.path === '/discover' ? 'bg-brand/15 text-brand font-semibold' : 'hover:bg-white/[0.06] text-slate-300 hover:text-white'"
          >
            <span v-if="route.path === '/discover'" class="absolute left-0 top-1.5 bottom-1.5 w-[3px] rounded-r-full bg-brand"></span>
            <Compass :size="15" />
            <span>{{ $t('sidebar.discover') }}</span>
          </RouterLink>
        </nav>
      </div>

      <!-- Group 2: 我的资料库 -->
      <div>
        <div class="px-3 pb-1.5 text-[10px] font-semibold text-slate-400/80 uppercase tracking-wider">
          {{ $t('sidebar.local_library') }}
        </div>
        <nav class="space-y-0.5">
          <RouterLink
            to="/library"
            class="relative flex items-center gap-2.5 px-3 py-1.5 rounded-xl text-xs font-medium transition-all"
            :class="route.path === '/library' && !route.query.tab ? 'bg-brand/15 text-brand font-semibold' : 'hover:bg-white/[0.06] text-slate-300 hover:text-white'"
          >
            <span v-if="route.path === '/library' && !route.query.tab" class="absolute left-0 top-1.5 bottom-1.5 w-[3px] rounded-r-full bg-brand"></span>
            <Music :size="15" />
            <span>{{ $t('sidebar.local_music') }}</span>
          </RouterLink>

          <RouterLink
            to="/library?tab=favorites"
            class="relative flex items-center gap-2.5 px-3 py-1.5 rounded-xl text-xs font-medium transition-all"
            :class="route.path === '/library' && route.query.tab === 'favorites' ? 'bg-brand/15 text-brand font-semibold' : 'hover:bg-white/[0.06] text-slate-300 hover:text-white'"
          >
            <span v-if="route.path === '/library' && route.query.tab === 'favorites'" class="absolute left-0 top-1.5 bottom-1.5 w-[3px] rounded-r-full bg-brand"></span>
            <Heart :size="15" />
            <span>{{ $t('sidebar.favorites') }}</span>
          </RouterLink>

          <RouterLink
            to="/library?tab=history"
            class="relative flex items-center gap-2.5 px-3 py-1.5 rounded-xl text-xs font-medium transition-all"
            :class="route.path === '/library' && route.query.tab === 'history' ? 'bg-brand/15 text-brand font-semibold' : 'hover:bg-white/[0.06] text-slate-300 hover:text-white'"
          >
            <span v-if="route.path === '/library' && route.query.tab === 'history'" class="absolute left-0 top-1.5 bottom-1.5 w-[3px] rounded-r-full bg-brand"></span>
            <Clock :size="15" />
            <span>{{ $t('sidebar.recent') }}</span>
          </RouterLink>
        </nav>
      </div>

      <!-- Group 3: 聚合云端音乐服务 (Unified Cloud Music Services) -->
      <div>
        <div class="flex items-center justify-between px-3 pb-1.5">
          <span class="text-[10px] font-semibold text-slate-400/80 uppercase tracking-wider flex items-center gap-1.5">
            <Radio :size="11" class="text-brand" />
            <span>{{ $t('sidebar.cloud_services') }}</span>
          </span>
          <div class="flex items-center gap-1">
            <button
              v-if="neteaseStore.isLoggedIn || qqStore.isLoggedIn || ytStore.isLoggedIn"
              @click="refreshAllCloudPlaylists"
              class="w-5 h-5 rounded-md flex items-center justify-center text-slate-400 hover:text-white hover:bg-white/10 transition-colors"
              :title="$t('sidebar.refresh_cloud')"
            >
              <RefreshCw :size="11" :class="(isRefreshing || isRefreshingQQ || isRefreshingYT) ? 'animate-spin' : ''" />
            </button>
          </div>
        </div>

        <!-- 紧凑型多平台账户胶囊条 (Compact 3-Column Cloud Account Pills) -->
        <div class="grid grid-cols-3 gap-1.5 px-1 mb-2">
          <!-- 1. 网易云音乐 -->
          <div
            v-if="neteaseStore.isLoggedIn && neteaseStore.user"
            class="relative group p-1.5 rounded-xl bg-theme-surface border border-red-500/30 flex flex-col items-center gap-1 text-center cursor-default"
            :title="`${neteaseStore.user.nickname} (网易云音乐)`"
          >
            <img
              :src="neteaseStore.user.avatarUrl"
              alt="NetEase"
              class="w-6 h-6 rounded-full object-cover border border-red-400/40"
            />
            <span class="text-[10px] font-medium text-white truncate w-full leading-none">
              {{ neteaseStore.user.nickname }}
            </span>
            <button
              @click="neteaseStore.logout()"
              class="absolute -top-1 -right-1 w-4 h-4 rounded-full bg-slate-900/90 border border-white/15 text-slate-300 hover:text-red-400 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity shadow-sm"
              title="退出网易云音乐"
            >
              <LogOut :size="9" />
            </button>
          </div>
          <button
            v-else
            @click="neteaseStore.openLoginModal()"
            class="p-1.5 rounded-xl bg-theme-surface hover:bg-red-500/10 border border-theme-subtle hover:border-red-500/30 flex flex-col items-center justify-center gap-1 group transition-all"
            title="登录网易云音乐"
          >
            <div class="w-6 h-6 rounded-full bg-red-500/15 text-red-400 flex items-center justify-center group-hover:bg-red-500 group-hover:text-white transition-colors">
              <Cloud :size="12" />
            </div>
            <span class="text-[10px] text-slate-300 group-hover:text-white truncate w-full text-center leading-none">网易云</span>
          </button>

          <!-- 2. QQ 音乐 -->
          <div
            v-if="qqStore.isLoggedIn && qqStore.user"
            class="relative group p-1.5 rounded-xl bg-theme-surface border border-emerald-500/30 flex flex-col items-center gap-1 text-center cursor-default"
            :title="`${qqStore.user.nickname} (QQ 音乐)`"
          >
            <img
              :src="qqStore.user.avatarUrl"
              alt="QQ"
              class="w-6 h-6 rounded-full object-cover border border-emerald-400/40"
            />
            <span class="text-[10px] font-medium text-white truncate w-full leading-none">
              {{ qqStore.user.nickname }}
            </span>
            <button
              @click="qqStore.logout()"
              class="absolute -top-1 -right-1 w-4 h-4 rounded-full bg-slate-900/90 border border-white/15 text-slate-300 hover:text-red-400 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity shadow-sm"
              title="退出 QQ 音乐"
            >
              <LogOut :size="9" />
            </button>
          </div>
          <button
            v-else
            @click="qqStore.openLoginModal()"
            class="p-1.5 rounded-xl bg-theme-surface hover:bg-emerald-500/10 border border-theme-subtle hover:border-emerald-500/30 flex flex-col items-center justify-center gap-1 group transition-all"
            title="登录 QQ 音乐"
          >
            <div class="w-6 h-6 rounded-full bg-emerald-500/15 text-emerald-400 flex items-center justify-center group-hover:bg-emerald-500 group-hover:text-white transition-colors">
              <Music2 :size="12" />
            </div>
            <span class="text-[10px] text-slate-300 group-hover:text-white truncate w-full text-center leading-none">QQ 音乐</span>
          </button>

          <!-- 3. YouTube Music -->
          <div
            v-if="ytStore.isLoggedIn && ytStore.user"
            class="relative group p-1.5 rounded-xl bg-theme-surface border border-rose-500/30 flex flex-col items-center gap-1 text-center cursor-default"
            :title="`${ytStore.user.name} (YouTube Music)`"
          >
            <img
              :src="ytStore.user.avatarUrl"
              alt="YouTube"
              class="w-6 h-6 rounded-full object-cover border border-rose-400/40"
            />
            <span class="text-[10px] font-medium text-white truncate w-full leading-none">
              {{ ytStore.user.name }}
            </span>
            <button
              @click="ytStore.logout()"
              class="absolute -top-1 -right-1 w-4 h-4 rounded-full bg-slate-900/90 border border-white/15 text-slate-300 hover:text-red-400 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity shadow-sm"
              title="退出 YouTube Music"
            >
              <LogOut :size="9" />
            </button>
          </div>
          <button
            v-else
            @click="ytStore.openLoginModal()"
            class="p-1.5 rounded-xl bg-theme-surface hover:bg-rose-500/10 border border-theme-subtle hover:border-rose-500/30 flex flex-col items-center justify-center gap-1 group transition-all"
            title="登录 YouTube Music"
          >
            <div class="w-6 h-6 rounded-full bg-rose-500/15 text-rose-400 flex items-center justify-center group-hover:bg-rose-500 group-hover:text-white transition-colors">
              <Youtube :size="12" />
            </div>
            <span class="text-[10px] text-slate-300 group-hover:text-white truncate w-full text-center leading-none">YouTube</span>
          </button>
        </div>

        <!-- 聚合云歌单列表 (Filter tabs + Unified list) -->
        <div v-if="hasAnyCloudPlaylists">
          <!-- Quick Filter Pills -->
          <div
            v-if="hasMultipleCloudPlatforms"
            class="flex items-center gap-1 px-1 mb-1.5 text-[10px] overflow-x-auto pb-0.5"
          >
            <button
              @click="cloudFilter = 'all'"
              class="px-2 py-0.5 rounded-md transition-colors whitespace-nowrap"
              :class="cloudFilter === 'all' ? 'bg-white/15 text-white font-semibold' : 'text-slate-400 hover:text-white'"
            >
              全部 ({{ totalCloudCount }})
            </button>
            <button
              v-if="neteaseStore.isLoggedIn && neteaseStore.userPlaylists.length > 0"
              @click="cloudFilter = 'netease'"
              class="px-2 py-0.5 rounded-md flex items-center gap-1 transition-colors whitespace-nowrap"
              :class="cloudFilter === 'netease' ? 'bg-red-500/20 text-red-300 font-semibold' : 'text-slate-400 hover:text-white'"
            >
              <span class="w-1.5 h-1.5 rounded-full bg-red-400"></span>
              网易 ({{ neteaseStore.userPlaylists.length }})
            </button>
            <button
              v-if="qqStore.isLoggedIn && qqStore.userPlaylists.length > 0"
              @click="cloudFilter = 'qq'"
              class="px-2 py-0.5 rounded-md flex items-center gap-1 transition-colors whitespace-nowrap"
              :class="cloudFilter === 'qq' ? 'bg-emerald-500/20 text-emerald-300 font-semibold' : 'text-slate-400 hover:text-white'"
            >
              <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
              QQ ({{ qqStore.userPlaylists.length }})
            </button>
            <button
              v-if="ytStore.isLoggedIn && ytStore.userPlaylists.length > 0"
              @click="cloudFilter = 'youtube'"
              class="px-2 py-0.5 rounded-md flex items-center gap-1 transition-colors whitespace-nowrap"
              :class="cloudFilter === 'youtube' ? 'bg-rose-500/20 text-rose-300 font-semibold' : 'text-slate-400 hover:text-white'"
            >
              <span class="w-1.5 h-1.5 rounded-full bg-rose-400"></span>
              YouTube ({{ ytStore.userPlaylists.length }})
            </button>
          </div>

          <!-- The List -->
          <nav class="space-y-0.5">
            <RouterLink
              v-for="playlist in filteredCloudPlaylists"
              :key="playlist.id"
              :to="`/playlist/${playlist.id}`"
              class="flex items-center justify-between px-3 py-1.5 rounded-xl text-xs font-medium truncate transition-all group"
              :class="route.params.id === playlist.id ? 'bg-brand/15 text-brand font-semibold shadow-sm' : 'hover:bg-white/[0.06] text-slate-300 hover:text-white'"
            >
              <div class="flex items-center gap-2.5 min-w-0 pr-1">
                <img
                  v-if="playlist.cover"
                  :src="playlist.cover"
                  class="w-4 h-4 rounded object-cover shrink-0 border border-white/10"
                />
                <ListMusic v-else :size="14" class="shrink-0 text-slate-400" />
                <span class="truncate">{{ playlist.name }}</span>
              </div>
              <span
                class="w-1.5 h-1.5 rounded-full shrink-0"
                :class="playlist.platform === 'netease' ? 'bg-red-400' : (playlist.platform === 'qq' ? 'bg-emerald-400' : 'bg-rose-500')"
                :title="playlist.platform === 'netease' ? '网易云歌单' : (playlist.platform === 'qq' ? 'QQ 音乐歌单' : 'YouTube Music 歌单')"
              ></span>
            </RouterLink>
          </nav>
        </div>
      </div>

      <!-- Group 4: 本地自定义歌单 -->
      <div>
        <div class="flex items-center justify-between px-3 pb-2">
          <span class="text-[11px] font-semibold text-slate-400 uppercase tracking-wider">{{ $t('sidebar.local_playlists') }}</span>
          <div class="flex items-center gap-1">
            <button
              @click="importerStore.openModal()"
              class="w-5 h-5 rounded-md flex items-center justify-center text-slate-400 hover:text-brand hover:bg-white/10 transition-colors"
              :title="$t('sidebar.import_playlist')"
            >
              <Link2 :size="13" />
            </button>
            <button
              @click="createNewPlaylist"
              class="w-5 h-5 rounded-md flex items-center justify-center text-slate-400 hover:text-white hover:bg-white/10 transition-colors"
              :title="$t('sidebar.new_playlist')"
            >
              <Plus :size="14" />
            </button>
          </div>
        </div>
        <nav class="space-y-1">
          <RouterLink
            v-for="playlist in playlistStore.playlists"
            :key="playlist.id"
            :to="`/playlist/${playlist.id}`"
            class="group flex items-center justify-between gap-2 px-3 py-2 rounded-xl text-xs font-medium truncate transition-all"
            :class="route.params.id === playlist.id ? 'bg-brand/15 text-brand font-semibold' : 'hover:bg-white/[0.06] text-slate-300 hover:text-white'"
          >
            <div class="flex items-center gap-3 min-w-0">
              <img
                v-if="playlist.cover"
                :src="playlist.cover"
                class="w-4 h-4 rounded object-cover shrink-0 border border-white/10"
              />
              <ListMusic v-else :size="15" class="shrink-0 text-slate-400" />
              <span class="truncate">{{ playlist.name }}</span>
            </div>
            <button
              v-if="playlist.id !== 'pl-favorites'"
              @click.prevent.stop="playlistStore.openDeleteModal(playlist)"
              class="p-1 rounded-md text-slate-500 hover:text-red-400 hover:bg-red-500/10 opacity-0 group-hover:opacity-100 transition-all shrink-0"
              title="删除歌单"
            >
              <Trash2 :size="12" />
            </button>
          </RouterLink>
        </nav>
      </div>
    </div>

    <!-- Bottom Section: User Account & Settings -->
    <div class="pt-2.5 border-t border-theme-border space-y-1">
      <!-- URL Time Passport User Card (Restrained Glass Style) -->
      <div
        v-if="passportStore.isLoggedIn && passportStore.user"
        class="p-1.5 px-2 rounded-xl bg-theme-surface border border-theme-border flex items-center justify-between gap-2 group"
      >
        <div class="flex items-center gap-2 min-w-0">
          <img
            v-if="passportStore.user.avatarUrl"
            :src="passportStore.user.avatarUrl"
            alt="Passport Avatar"
            class="w-6 h-6 rounded-full object-cover border border-white/15 shrink-0"
          />
          <div
            v-else
            class="w-6 h-6 rounded-full bg-brand/20 text-brand text-[11px] font-bold flex items-center justify-center shrink-0"
          >
            {{ passportStore.user.nickname.charAt(0).toUpperCase() }}
          </div>
          <div class="min-w-0">
            <p class="text-[11px] font-semibold text-white truncate leading-tight">
              {{ passportStore.user.nickname }}
            </p>
            <p class="text-[9px] text-slate-400 truncate leading-tight">
              {{ passportStore.user.email || 'Time Passport' }}
            </p>
          </div>
        </div>
        <button
          @click="passportStore.logout()"
          class="text-slate-500 hover:text-red-400 p-1 rounded opacity-0 group-hover:opacity-100 transition-opacity shrink-0"
          title="退出 URL Time Passport"
        >
          <LogOut :size="11" />
        </button>
      </div>

      <button
        v-else
        @click="passportStore.openLoginModal()"
        class="w-full p-1.5 px-2.5 rounded-xl bg-theme-surface hover:bg-theme-surfaceElevated border border-theme-subtle hover:border-brand/35 flex items-center justify-between group transition-all text-left"
      >
        <div class="flex items-center gap-2 min-w-0">
          <div class="w-6 h-6 rounded-lg bg-brand/15 text-brand flex items-center justify-center shrink-0 group-hover:bg-brand group-hover:text-white transition-colors">
            <ShieldCheck :size="12" />
          </div>
          <div class="min-w-0">
            <p class="text-[11px] font-semibold text-white truncate leading-tight">Time Passport</p>
            <p class="text-[9px] text-slate-400 truncate leading-tight">统一账号登录</p>
          </div>
        </div>
        <span class="text-[10px] text-brand font-medium shrink-0">登录</span>
      </button>

      <RouterLink
        to="/settings"
        class="relative flex items-center justify-between px-3 py-1.5 rounded-xl text-xs font-medium transition-all"
        :class="route.path === '/settings' ? 'bg-brand/15 text-brand font-semibold' : 'hover:bg-white/[0.06] text-slate-300 hover:text-white'"
      >
        <span v-if="route.path === '/settings'" class="absolute left-0 top-1.5 bottom-1.5 w-[3px] rounded-r-full bg-brand"></span>
        <div class="flex items-center gap-2.5">
          <Settings :size="15" />
          <span>{{ $t('sidebar.settings') }}</span>
        </div>
        <span class="inline-flex items-center gap-1 text-[9px] text-slate-400 font-mono">
          <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
          <span>0.1.4</span>
        </span>
      </RouterLink>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute } from 'vue-router';
import {
  Home,
  Compass,
  Music,
  Heart,
  Clock,
  ListMusic,
  Plus,
  Settings,
  Cloud,
  LogOut,
  RefreshCw,
  Music2,
  Link2,
  Radio,
  Youtube,
  ShieldCheck,
  Trash2,
} from 'lucide-vue-next';
import { usePlaylistStore } from '@/stores/playlist';
import { useNeteaseStore } from '@/stores/netease';
import { useQQMusicStore } from '@/stores/qqmusic';
import { useYTMusicStore } from '@/stores/ytmusic';
import { useImporterStore } from '@/stores/importer';
import { usePassportStore } from '@/stores/passport';

const route = useRoute();
const playlistStore = usePlaylistStore();
const neteaseStore = useNeteaseStore();
const qqStore = useQQMusicStore();
const ytStore = useYTMusicStore();
const importerStore = useImporterStore();
const passportStore = usePassportStore();
const isRefreshing = ref(false);
const isRefreshingQQ = ref(false);
const isRefreshingYT = ref(false);
const cloudFilter = ref<'all' | 'netease' | 'qq' | 'youtube'>('all');

const hasAnyCloudPlaylists = computed(() => {
  return (
    (neteaseStore.isLoggedIn && neteaseStore.userPlaylists.length > 0) ||
    (qqStore.isLoggedIn && qqStore.userPlaylists.length > 0) ||
    (ytStore.isLoggedIn && ytStore.userPlaylists.length > 0)
  );
});

const totalCloudCount = computed(() => {
  let count = 0;
  if (neteaseStore.isLoggedIn) count += neteaseStore.userPlaylists.length;
  if (qqStore.isLoggedIn) count += qqStore.userPlaylists.length;
  if (ytStore.isLoggedIn) count += ytStore.userPlaylists.length;
  return count;
});

const hasMultipleCloudPlatforms = computed(() => {
  let activeSources = 0;
  if (neteaseStore.isLoggedIn && neteaseStore.userPlaylists.length > 0) activeSources++;
  if (qqStore.isLoggedIn && qqStore.userPlaylists.length > 0) activeSources++;
  if (ytStore.isLoggedIn && ytStore.userPlaylists.length > 0) activeSources++;
  return activeSources > 1;
});

const filteredCloudPlaylists = computed(() => {
  const list: Array<{ id: string; name: string; cover?: string; platform: 'netease' | 'qq' | 'youtube' }> = [];
  if (neteaseStore.isLoggedIn && (cloudFilter.value === 'all' || cloudFilter.value === 'netease')) {
    for (const pl of neteaseStore.userPlaylists) {
      list.push({ id: pl.id, name: pl.name, cover: pl.cover, platform: 'netease' });
    }
  }
  if (qqStore.isLoggedIn && (cloudFilter.value === 'all' || cloudFilter.value === 'qq')) {
    for (const pl of qqStore.userPlaylists) {
      list.push({ id: pl.id, name: pl.name, cover: pl.cover, platform: 'qq' });
    }
  }
  if (ytStore.isLoggedIn && (cloudFilter.value === 'all' || cloudFilter.value === 'youtube')) {
    for (const pl of ytStore.userPlaylists) {
      list.push({ id: pl.id, name: pl.name, cover: pl.cover, platform: 'youtube' });
    }
  }
  return list;
});

const createNewPlaylist = () => {
  playlistStore.openCreateModal();
};

const refreshNeteasePlaylists = async () => {
  if (isRefreshing.value) return;
  isRefreshing.value = true;
  await neteaseStore.loadUserPlaylists();
  setTimeout(() => {
    isRefreshing.value = false;
  }, 600);
};

const refreshQQPlaylists = async () => {
  if (isRefreshingQQ.value) return;
  isRefreshingQQ.value = true;
  await qqStore.loadUserPlaylists();
  setTimeout(() => {
    isRefreshingQQ.value = false;
  }, 600);
};

const refreshYTPlaylists = async () => {
  if (isRefreshingYT.value) return;
  isRefreshingYT.value = true;
  await ytStore.loadUserPlaylists();
  setTimeout(() => {
    isRefreshingYT.value = false;
  }, 600);
};

const refreshAllCloudPlaylists = async () => {
  if (neteaseStore.isLoggedIn) {
    refreshNeteasePlaylists();
  }
  if (qqStore.isLoggedIn) {
    refreshQQPlaylists();
  }
  if (ytStore.isLoggedIn) {
    refreshYTPlaylists();
  }
};
</script>
