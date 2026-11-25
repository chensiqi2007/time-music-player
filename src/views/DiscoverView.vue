<template>
  <div class="space-y-8 pb-12">
    <!-- Header with Online Multi-Platform Search Banner -->
    <div class="relative overflow-hidden rounded-3xl p-8 bg-gradient-to-r from-red-950/40 via-purple-950/30 to-slate-900/70 border border-white/10 shadow-2xl">
      <div class="relative z-10 max-w-3xl space-y-4">
        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-brand/20 border border-brand/30 text-brand text-xs font-semibold">
          <Layers :size="13" />
          <span>全网跨平台聚合曲库流媒体引擎</span>
        </div>
        <h1 class="text-3xl font-extrabold text-white tracking-tight">
          海量流媒体，一搜即听
        </h1>
        <p class="text-xs text-slate-300">
          支持网易云、QQ、酷我、酷狗、汽水、Apple Music 与 YouTube Music 聚合检索与高保真流媒体即点即播，无需安装庞杂客户端。
        </p>

        <!-- Search Input Box -->
        <div class="pt-2 flex items-center gap-2 max-w-xl">
          <div class="relative flex-1">
            <Search class="absolute left-3.5 top-1/2 -translate-y-1/2 text-slate-400 w-4 h-4" />
            <input
              v-model="keywordInput"
              @keydown.enter="handleSearch"
              type="text"
              :placeholder="$t('discover.search_placeholder')"
              class="w-full h-10 pl-10 pr-24 text-xs bg-black/40 hover:bg-black/60 focus:bg-black/80 text-white placeholder-slate-400 rounded-xl border border-white/15 focus:border-brand focus:outline-none transition-all shadow-inner"
            />
            <button
              v-if="keywordInput"
              @click="clearSearch"
              class="absolute right-14 top-1/2 -translate-y-1/2 text-slate-400 hover:text-white text-xs"
            >
              {{ $t('common.clear') }}
            </button>
            <button
              @click="handleSearch"
              :disabled="providerStore.isSearching"
              class="absolute right-1.5 top-1/2 -translate-y-1/2 px-3 py-1.5 rounded-lg bg-brand hover:bg-brand-light text-white text-xs font-semibold flex items-center gap-1.5 transition-all disabled:opacity-50"
            >
              <RefreshCw v-if="providerStore.isSearching" :size="12" class="animate-spin" />
              <span>{{ $t('common.search') }}</span>
            </button>
          </div>
        </div>

        <!-- Hot Search Chips -->
        <div class="flex flex-wrap items-center gap-2 pt-1">
          <span class="text-[11px] text-slate-400">热门搜索：</span>
          <button
            v-for="tag in hotTags"
            :key="tag"
            @click="quickSearch(tag)"
            class="px-2.5 py-1 rounded-lg bg-white/[0.06] hover:bg-white/[0.12] text-[11px] text-slate-300 hover:text-white transition-all border border-white/[0.06]"
          >
            {{ tag }}
          </button>
        </div>
      </div>
    </div>

    <!-- Multi-Platform Account Hub Strip -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
      <!-- 1. 网易云音乐 -->
      <div
        class="p-3 rounded-2xl border transition-all flex items-center justify-between gap-3"
        :class="neteaseStore.isLoggedIn ? 'bg-red-500/10 border-red-500/30' : 'bg-white/[0.02] border-white/5'"
      >
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="w-8 h-8 rounded-xl bg-red-500/20 text-red-400 flex items-center justify-center shrink-0">
            <Cloud :size="16" />
          </div>
          <div class="min-w-0">
            <div class="flex items-center gap-1.5">
              <span class="text-xs font-semibold text-white">网易云音乐</span>
              <span v-if="neteaseStore.isLoggedIn" class="text-[9px] px-1.5 py-0.2 rounded-full bg-red-500/20 text-red-300 font-semibold">已登录</span>
            </div>
            <p class="text-[10px] text-slate-400 truncate">
              {{ neteaseStore.isLoggedIn ? neteaseStore.user?.nickname : '扫码同步歌单与 VIP 资产' }}
            </p>
          </div>
        </div>
        <button
          v-if="!neteaseStore.isLoggedIn"
          @click="neteaseStore.openLoginModal()"
          class="px-2.5 py-1 rounded-lg bg-red-500/20 hover:bg-red-500 text-red-300 hover:text-white text-[11px] font-medium transition-colors shrink-0"
        >
          扫码登录
        </button>
        <button
          v-else
          @click="neteaseStore.logout()"
          class="text-slate-500 hover:text-red-400 p-1 transition-colors text-xs shrink-0"
          title="退出登录"
        >
          退出
        </button>
      </div>

      <!-- 2. QQ 音乐 -->
      <div
        class="p-3 rounded-2xl border transition-all flex items-center justify-between gap-3"
        :class="qqStore.isLoggedIn ? 'bg-emerald-500/10 border-emerald-500/30' : 'bg-white/[0.02] border-white/5'"
      >
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="w-8 h-8 rounded-xl bg-emerald-500/20 text-emerald-400 flex items-center justify-center shrink-0">
            <Music2 :size="16" />
          </div>
          <div class="min-w-0">
            <div class="flex items-center gap-1.5">
              <span class="text-xs font-semibold text-white">QQ 音乐</span>
              <span v-if="qqStore.isLoggedIn" class="text-[9px] px-1.5 py-0.2 rounded-full bg-emerald-500/20 text-emerald-300 font-semibold">已登录</span>
            </div>
            <p class="text-[10px] text-slate-400 truncate">
              {{ qqStore.isLoggedIn ? qqStore.user?.nickname : '微信/QQ 扫码同步红心资产' }}
            </p>
          </div>
        </div>
        <button
          v-if="!qqStore.isLoggedIn"
          @click="qqStore.openLoginModal()"
          class="px-2.5 py-1 rounded-lg bg-emerald-500/20 hover:bg-emerald-500 text-emerald-300 hover:text-white text-[11px] font-medium transition-colors shrink-0"
        >
          扫码登录
        </button>
        <button
          v-else
          @click="qqStore.logout()"
          class="text-slate-500 hover:text-red-400 p-1 transition-colors text-xs shrink-0"
          title="退出登录"
        >
          退出
        </button>
      </div>

      <!-- 3. YouTube Music -->
      <div
        class="p-3 rounded-2xl border transition-all flex items-center justify-between gap-3"
        :class="ytStore.isLoggedIn ? 'bg-rose-500/10 border-rose-500/30' : 'bg-white/[0.02] border-white/5'"
      >
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="w-8 h-8 rounded-xl bg-rose-500/20 text-rose-400 flex items-center justify-center shrink-0">
            <Youtube :size="16" />
          </div>
          <div class="min-w-0">
            <div class="flex items-center gap-1.5">
              <span class="text-xs font-semibold text-white">YouTube Music</span>
              <span v-if="ytStore.isLoggedIn" class="text-[9px] px-1.5 py-0.2 rounded-full bg-rose-500/20 text-rose-300 font-semibold">已登录</span>
            </div>
            <p class="text-[10px] text-slate-400 truncate">
              {{ ytStore.isLoggedIn ? ytStore.user?.name : 'Google 设备码 / Cookie 授权同步' }}
            </p>
          </div>
        </div>
        <button
          v-if="!ytStore.isLoggedIn"
          @click="ytStore.openLoginModal()"
          class="px-2.5 py-1 rounded-lg bg-rose-500/20 hover:bg-rose-500 text-rose-300 hover:text-white text-[11px] font-medium transition-colors shrink-0"
        >
          点击登录
        </button>
        <button
          v-else
          @click="ytStore.logout()"
          class="text-slate-500 hover:text-red-400 p-1 transition-colors text-xs shrink-0"
          title="退出登录"
        >
          退出
        </button>
      </div>

      <!-- 4. 酷我音乐 -->
      <div
        class="p-3 rounded-2xl border transition-all flex items-center justify-between gap-3"
        :class="extraStore.accounts.kuwo.isLoggedIn ? 'bg-amber-500/10 border-amber-500/30' : 'bg-white/[0.02] border-white/5'"
      >
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="w-8 h-8 rounded-xl bg-amber-500/20 text-amber-400 flex items-center justify-center shrink-0">
            <Disc :size="16" />
          </div>
          <div class="min-w-0">
            <div class="flex items-center gap-1.5">
              <span class="text-xs font-semibold text-white">酷我音乐</span>
              <span v-if="extraStore.accounts.kuwo.isLoggedIn" class="text-[9px] px-1.5 py-0.2 rounded-full bg-amber-500/20 text-amber-300 font-semibold">已登录</span>
            </div>
            <p class="text-[10px] text-slate-400 truncate">
              {{ extraStore.accounts.kuwo.isLoggedIn ? extraStore.accounts.kuwo.nickname : '登录解锁酷我 VIP 完整音质' }}
            </p>
          </div>
        </div>
        <button
          v-if="!extraStore.accounts.kuwo.isLoggedIn"
          @click="extraStore.openLoginModal('kuwo')"
          class="px-2.5 py-1 rounded-lg bg-amber-500/20 hover:bg-amber-500 text-amber-300 hover:text-white text-[11px] font-medium transition-colors shrink-0"
        >
          登录解锁
        </button>
        <button
          v-else
          @click="extraStore.logout('kuwo')"
          class="text-slate-500 hover:text-red-400 p-1 transition-colors text-xs shrink-0"
          title="退出登录"
        >
          退出
        </button>
      </div>

      <!-- 5. 酷狗音乐 -->
      <div
        class="p-3 rounded-2xl border transition-all flex items-center justify-between gap-3"
        :class="extraStore.accounts.kugou.isLoggedIn ? 'bg-sky-500/10 border-sky-500/30' : 'bg-white/[0.02] border-white/5'"
      >
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="w-8 h-8 rounded-xl bg-sky-500/20 text-sky-400 flex items-center justify-center shrink-0">
            <Headphones :size="16" />
          </div>
          <div class="min-w-0">
            <div class="flex items-center gap-1.5">
              <span class="text-xs font-semibold text-white">酷狗音乐</span>
              <span v-if="extraStore.accounts.kugou.isLoggedIn" class="text-[9px] px-1.5 py-0.2 rounded-full bg-sky-500/20 text-sky-300 font-semibold">已登录</span>
            </div>
            <p class="text-[10px] text-slate-400 truncate">
              {{ extraStore.accounts.kugou.isLoggedIn ? extraStore.accounts.kugou.nickname : '登录解锁酷狗 VIP 会员音频' }}
            </p>
          </div>
        </div>
        <button
          v-if="!extraStore.accounts.kugou.isLoggedIn"
          @click="extraStore.openLoginModal('kugou')"
          class="px-2.5 py-1 rounded-lg bg-sky-500/20 hover:bg-sky-500 text-sky-300 hover:text-white text-[11px] font-medium transition-colors shrink-0"
        >
          登录解锁
        </button>
        <button
          v-else
          @click="extraStore.logout('kugou')"
          class="text-slate-500 hover:text-red-400 p-1 transition-colors text-xs shrink-0"
          title="退出登录"
        >
          退出
        </button>
      </div>

      <!-- 6. 汽水音乐 -->
      <div
        class="p-3 rounded-2xl border transition-all flex items-center justify-between gap-3"
        :class="extraStore.accounts.qishui.isLoggedIn ? 'bg-teal-500/10 border-teal-500/30' : 'bg-white/[0.02] border-white/5'"
      >
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="w-8 h-8 rounded-xl bg-teal-500/20 text-teal-400 flex items-center justify-center shrink-0">
            <Waves :size="16" />
          </div>
          <div class="min-w-0">
            <div class="flex items-center gap-1.5">
              <span class="text-xs font-semibold text-white">汽水音乐</span>
              <span v-if="extraStore.accounts.qishui.isLoggedIn" class="text-[9px] px-1.5 py-0.2 rounded-full bg-teal-500/20 text-teal-300 font-semibold">已登录</span>
            </div>
            <p class="text-[10px] text-slate-400 truncate">
              {{ extraStore.accounts.qishui.isLoggedIn ? extraStore.accounts.qishui.nickname : '绑定抖音/汽水账号同步会员' }}
            </p>
          </div>
        </div>
        <button
          v-if="!extraStore.accounts.qishui.isLoggedIn"
          @click="extraStore.openLoginModal('qishui')"
          class="px-2.5 py-1 rounded-lg bg-teal-500/20 hover:bg-teal-500 text-teal-300 hover:text-white text-[11px] font-medium transition-colors shrink-0"
        >
          登录解锁
        </button>
        <button
          v-else
          @click="extraStore.logout('qishui')"
          class="text-slate-500 hover:text-red-400 p-1 transition-colors text-xs shrink-0"
          title="退出登录"
        >
          退出
        </button>
      </div>
    </div>

    <!-- Platform Selector Tabs & External Import Button -->
    <div class="flex flex-wrap items-center justify-between gap-3 border-b border-white/[0.08] pb-3">
      <div class="flex flex-wrap items-center gap-2">
        <button
          v-for="p in PLATFORMS"
          :key="p.id"
          @click="switchPlatform(p.id)"
          class="px-3.5 py-1.5 rounded-xl text-xs font-medium transition-all flex items-center gap-2 border"
          :class="providerStore.activePlatform === p.id ? 'bg-brand/20 border-brand/40 text-brand font-semibold shadow-sm shadow-brand/10' : 'bg-white/[0.03] border-white/[0.06] text-slate-400 hover:text-white hover:bg-white/[0.08]'"
        >
          <span>{{ p.shortName }}</span>
          <span
            v-if="hasSearched && getPlatformCount(p.id) > 0"
            class="px-1.5 py-0.2 rounded-full text-[10px] font-mono"
            :class="providerStore.activePlatform === p.id ? 'bg-brand text-white' : 'bg-white/10 text-slate-300'"
          >
            {{ getPlatformCount(p.id) }}
          </span>
        </button>
      </div>

      <button
        @click="importerStore.openModal()"
        class="px-3.5 py-1.5 rounded-xl bg-white/[0.06] hover:bg-white/[0.12] border border-white/10 text-white text-xs font-medium flex items-center gap-2 transition-all hover:border-brand/40 shadow-sm"
      >
        <Link2 :size="13" class="text-brand" />
        <span>{{ $t('discover.import_external') }}</span>
      </button>
    </div>

    <!-- Section: Search Results (if keyword search executed) -->
    <section v-if="hasSearched" class="space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-base font-semibold text-white flex items-center gap-2">
          <span>检索结果</span>
          <span class="text-xs text-slate-400 font-normal">({{ displayResults.length }} 首歌曲)</span>
        </h2>
        <button
          v-if="displayResults.length > 0"
          @click="playAllResults"
          class="px-3.5 py-1.5 rounded-full bg-brand/20 hover:bg-brand/30 border border-brand/30 text-brand text-xs font-semibold flex items-center gap-1.5 transition-colors"
        >
          <Play :size="12" fill="currentColor" />
          播放当前结果
        </button>
      </div>

      <!-- Loading skeleton -->
      <div v-if="providerStore.isSearching" class="py-16 flex flex-col items-center justify-center space-y-3">
        <RefreshCw :size="28" class="animate-spin text-brand" />
        <p class="text-xs text-slate-400">正在跨平台并发拉取音频流与元数据...</p>
      </div>

      <!-- No Results -->
      <div
        v-else-if="displayResults.length === 0"
        class="py-16 text-center text-xs text-slate-400 glass-card rounded-2xl p-6"
      >
        未找到与 “{{ keywordInput }}” 相关的歌曲，请尝试更换关键词或切换流媒体源
      </div>

      <!-- Results Table -->
      <div v-else class="glass-panel rounded-2xl overflow-hidden border border-white/10">
        <div class="grid grid-cols-12 px-4 py-3 text-[11px] font-semibold text-slate-400 uppercase tracking-wider border-b border-white/[0.08] bg-white/[0.02]">
          <div class="col-span-1 text-center">#</div>
          <div class="col-span-5">歌曲标题</div>
          <div class="col-span-2">平台来源</div>
          <div class="col-span-2">专辑</div>
          <div class="col-span-2 text-right">时长 / 操作</div>
        </div>

        <div class="divide-y divide-white/[0.04]">
          <div
            v-for="(track, index) in displayResults"
            :key="track.id"
            @click="playerStore.playTrack(track, displayResults)"
            @dblclick="playerStore.playTrack(track, displayResults)"
            @contextmenu.prevent.stop="contextMenuStore.openTrackMenu($event, track, displayResults)"
            class="grid grid-cols-12 px-4 py-3 items-center text-xs text-slate-300 hover:bg-white/[0.06] group cursor-pointer transition-colors"
            :class="playerStore.currentTrack?.id === track.id ? 'bg-brand/10 text-brand font-medium' : ''"
          >
            <!-- Track index / Play button -->
            <div class="col-span-1 text-center text-slate-400 font-mono flex items-center justify-center">
              <div v-if="playerStore.currentTrack?.id === track.id && playerStore.isPlaying" class="flex items-end gap-0.5 h-3 shrink-0 px-1">
                <span class="w-0.5 bg-brand animate-bar-1 h-full rounded-full"></span>
                <span class="w-0.5 bg-brand animate-bar-2 h-1/2 rounded-full"></span>
                <span class="w-0.5 bg-brand animate-bar-3 h-3/4 rounded-full"></span>
              </div>
              <span v-else class="group-hover:hidden">{{ index + 1 }}</span>
              <button
                @click.stop="playerStore.playTrack(track, displayResults)"
                class="hidden group-hover:inline-block text-white hover:text-brand"
              >
                <Play :size="13" fill="currentColor" />
              </button>
            </div>

            <!-- Title & Artist -->
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

            <!-- Platform Source Badge -->
            <div class="col-span-2 flex items-center gap-1.5">
              <span
                class="px-2 py-0.5 rounded-md text-[10px] font-semibold border flex items-center gap-1 shrink-0"
                :class="[getBadge(track).badgeBg, getBadge(track).badgeBorder, getBadge(track).badgeText]"
              >
                {{ getBadge(track).shortName }}
              </span>
              <span class="text-[10px] font-mono text-slate-400 truncate hidden sm:inline">
                {{ track.format?.includes(' ') ? track.format.split(' ')[1] : (track.format || '320k') }}
              </span>
            </div>

            <!-- Album -->
            <div class="col-span-2 truncate text-slate-400 text-xs pr-4">
              {{ track.album || '精选专辑' }}
            </div>

            <!-- Operations & Duration -->
            <div class="col-span-2 flex items-center justify-end gap-2.5 text-slate-400 font-mono text-xs">
              <button
                @click.stop="playlistStore.openAddToPlaylistModal(track)"
                class="p-1 hover:text-white transition-colors"
                title="添加到歌单"
              >
                <PlusCircle :size="15" />
              </button>
              <button
                @click.stop="addToQueue(track)"
                class="p-1 hover:text-white transition-colors"
                title="添加到播放队列"
              >
                <ListPlus :size="15" />
              </button>
              <button
                @click.stop="playerStore.toggleFavorite(track)"
                class="p-1 hover:text-brand transition-colors"
                :class="(track.isFavorite || playerStore.isIdFavorite(track.id)) ? 'text-brand' : ''"
                title="喜欢"
              >
                <Heart :size="14" :fill="(track.isFavorite || playerStore.isIdFavorite(track.id)) ? 'currentColor' : 'none'" />
              </button>
              <span class="tabular-nums w-10 text-right">{{ formatDuration(track.duration) }}</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Discover Playlists (发现歌单) -->
    <section ref="discoverPlaylistsSection" class="space-y-4 scroll-mt-4">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div>
          <h2 class="text-base font-semibold text-white">发现歌单</h2>
          <p class="text-[11px] text-slate-400">全网热门精选歌单，点击即刻收听</p>
        </div>

        <div class="flex items-center gap-1.5 flex-wrap">
          <button
            v-for="cat in playlistCategories"
            :key="cat"
            @click="selectPlaylistCategory(cat)"
            class="px-2.5 py-1 rounded-full text-[11px] border transition-all"
            :class="activePlaylistCat === cat ? 'bg-brand/20 border-brand/40 text-brand font-semibold' : 'bg-theme-surface border-theme-border text-slate-400 hover:text-white hover:bg-theme-surfaceElevated'"
          >
            {{ cat }}
          </button>
          <button
            @click="refreshDiscoverPlaylists"
            :disabled="isLoadingPlaylists"
            class="px-2.5 py-1 rounded-full text-[11px] bg-theme-surface hover:bg-theme-surfaceElevated border border-theme-border text-slate-300 hover:text-white flex items-center gap-1 transition-all disabled:opacity-50"
            title="换一批歌单"
          >
            <RefreshCw :size="11" :class="isLoadingPlaylists ? 'animate-spin' : ''" />
            <span>换一批</span>
          </button>
        </div>
      </div>

      <div v-if="isLoadingPlaylists && discoverPlaylists.length === 0" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
        <div
          v-for="n in 6"
          :key="n"
          class="glass-card rounded-2xl p-3 space-y-2.5 animate-pulse"
        >
          <div class="aspect-square rounded-xl bg-white/5"></div>
          <div class="h-3 rounded bg-white/10 w-3/4"></div>
          <div class="h-2.5 rounded bg-white/5 w-1/2"></div>
        </div>
      </div>

      <div v-else-if="discoverPlaylists.length > 0" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
        <div
          v-for="pl in discoverPlaylists"
          :key="pl.id"
          @click="openDiscoveredPlaylist(pl.id)"
          class="glass-card rounded-2xl p-3 space-y-2.5 cursor-pointer group"
        >
          <div class="relative aspect-square rounded-xl overflow-hidden border border-white/10 shadow-md">
            <img
              v-if="pl.cover"
              :src="pl.cover"
              loading="lazy"
              decoding="async"
              class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
            />
            <div v-else class="w-full h-full bg-white/5 flex items-center justify-center">
              <Disc :size="24" class="text-slate-400" />
            </div>
            <span
              v-if="pl.play_count > 0"
              class="absolute top-1.5 right-1.5 px-1.5 py-0.5 rounded-full bg-black/60 backdrop-blur-md text-[9px] text-white font-mono border border-white/10"
            >
              {{ formatPlayCount(pl.play_count) }}
            </span>
            <div class="absolute inset-0 bg-black/35 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
              <div class="w-9 h-9 rounded-full bg-brand text-white flex items-center justify-center shadow-lg shadow-brand/40">
                <Play :size="15" fill="white" class="ml-0.5" />
              </div>
            </div>
          </div>
          <div>
            <h4 class="text-xs font-semibold text-white line-clamp-2 leading-snug group-hover:text-brand transition-colors">
              {{ pl.name }}
            </h4>
            <p class="text-[10px] text-slate-400 mt-1 font-mono">
              {{ pl.track_count }} 首
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Discover Playlists (发现歌单) -->
    <section ref="discoverPlaylistsSection" class="space-y-4 scroll-mt-4">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div>
          <h2 class="text-base font-semibold text-white">发现歌单</h2>
          <p class="text-[11px] text-slate-400">全网热门精选歌单，点击即刻收听</p>
        </div>

        <div class="flex items-center gap-1.5 flex-wrap">
          <button
            v-for="cat in playlistCategories"
            :key="cat"
            @click="selectPlaylistCategory(cat)"
            class="px-2.5 py-1 rounded-full text-[11px] border transition-all"
            :class="activePlaylistCat === cat ? 'bg-brand/20 border-brand/40 text-brand font-semibold' : 'bg-theme-surface border-theme-border text-slate-400 hover:text-white hover:bg-theme-surfaceElevated'"
          >
            {{ cat }}
          </button>
          <button
            @click="refreshDiscoverPlaylists"
            :disabled="isLoadingPlaylists"
            class="px-2.5 py-1 rounded-full text-[11px] bg-theme-surface hover:bg-theme-surfaceElevated border border-theme-border text-slate-300 hover:text-white flex items-center gap-1 transition-all disabled:opacity-50"
            title="换一批歌单"
          >
            <RefreshCw :size="11" :class="isLoadingPlaylists ? 'animate-spin' : ''" />
            <span>换一批</span>
          </button>
        </div>
      </div>

      <div v-if="isLoadingPlaylists && discoverPlaylists.length === 0" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
        <div
          v-for="n in 6"
          :key="n"
          class="glass-card rounded-2xl p-3 space-y-2.5 animate-pulse"
        >
          <div class="aspect-square rounded-xl bg-white/5"></div>
          <div class="h-3 rounded bg-white/10 w-3/4"></div>
          <div class="h-2.5 rounded bg-white/5 w-1/2"></div>
        </div>
      </div>

      <div v-else-if="discoverPlaylists.length > 0" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
        <div
          v-for="pl in discoverPlaylists"
          :key="pl.id"
          @click="openDiscoveredPlaylist(pl.id)"
          class="glass-card rounded-2xl p-3 space-y-2.5 cursor-pointer group"
        >
          <div class="relative aspect-square rounded-xl overflow-hidden border border-white/10 shadow-md">
            <img
              v-if="pl.cover"
              :src="pl.cover"
              loading="lazy"
              decoding="async"
              class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
            />
            <div v-else class="w-full h-full bg-white/5 flex items-center justify-center">
              <Disc :size="24" class="text-slate-400" />
            </div>
            <span
              v-if="pl.play_count > 0"
              class="absolute top-1.5 right-1.5 px-1.5 py-0.5 rounded-full bg-black/60 backdrop-blur-md text-[9px] text-white font-mono border border-white/10"
            >
              {{ formatPlayCount(pl.play_count) }}
            </span>
            <div class="absolute inset-0 bg-black/35 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
              <div class="w-9 h-9 rounded-full bg-brand text-white flex items-center justify-center shadow-lg shadow-brand/40">
                <Play :size="15" fill="white" class="ml-0.5" />
              </div>
            </div>
          </div>
          <div>
            <h4 class="text-xs font-semibold text-white line-clamp-2 leading-snug group-hover:text-brand transition-colors">
              {{ pl.name }}
            </h4>
            <p class="text-[10px] text-slate-400 mt-1 font-mono">
              {{ pl.track_count }} 首
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Genre Categories Grid -->
    <section class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-base font-semibold text-white">风格与流派</h2>
          <p class="text-[11px] text-slate-400">按流派标签精准匹配精选歌单，而非关键字搜歌</p>
        </div>
      </div>
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
        <div
          v-for="genre in genres"
          :key="genre.name"
          @click="selectGenre(genre.cat)"
          class="relative h-28 rounded-2xl p-4 overflow-hidden border cursor-pointer group shadow-lg flex flex-col justify-between transition-all hover:-translate-y-0.5"
          :class="[
            genre.gradient,
            activePlaylistCat === genre.cat ? 'border-white/70 ring-2 ring-brand/60 shadow-brand/25' : 'border-white/10'
          ]"
        >
          <div class="z-10">
            <span class="text-sm font-bold text-white block">{{ genre.name }}</span>
            <span class="text-[10px] text-white/75 font-mono">{{ genre.subtitle }}</span>
          </div>
          <span class="text-[10px] text-white/85 z-10 font-medium flex items-center gap-1">
            <span>{{ activePlaylistCat === genre.cat ? '当前已匹配歌单' : '匹配流派歌单' }}</span>
          </span>

          <component
            :is="genre.icon"
            class="absolute -right-2 -bottom-2 w-16 h-16 text-white/15 group-hover:scale-110 group-hover:text-white/25 transition-all"
          />
        </div>
      </div>
    </section>

    <!-- Charts / Top Lists (精选榜单 & 全部榜单) -->
    <section class="space-y-4">
      <div class="flex items-center justify-between gap-3">
        <div>
          <h2 class="text-base font-semibold text-white">精选榜单</h2>
          <p class="text-[11px] text-slate-400">实时同步全网官方飙升、新歌、热歌与全球精选榜单</p>
        </div>
        <button
          v-if="allCharts.length > 0"
          @click="showAllCharts = !showAllCharts"
          class="px-3.5 py-1.5 rounded-full text-xs font-medium bg-theme-surface hover:bg-theme-surfaceElevated border border-theme-border text-slate-200 hover:text-white flex items-center gap-1.5 transition-all"
        >
          <Flame :size="13" class="text-brand" />
          <span>{{ showAllCharts ? '收起全部榜单' : `全部榜单 (${allCharts.length})` }}</span>
        </button>
      </div>

      <!-- Top Featured Charts (with Top 3 Tracks Preview) -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div
          v-for="chart in featuredCharts"
          :key="chart.id"
          @click="openDiscoveredPlaylist(chart.id)"
          class="glass-card rounded-2xl p-4 space-y-3 cursor-pointer group"
        >
          <div class="flex items-center gap-3">
            <img
              v-if="chart.cover"
              :src="chart.cover"
              loading="lazy"
              class="w-11 h-11 rounded-xl object-cover border border-white/10 shrink-0 shadow-sm"
            />
            <div class="min-w-0 flex-1">
              <div class="flex items-center justify-between gap-1">
                <span class="text-xs font-bold text-white truncate group-hover:text-brand transition-colors">{{ chart.name }}</span>
              </div>
              <span class="text-[10px] text-brand font-medium">{{ chart.update_frequency }}</span>
            </div>
          </div>

          <div class="space-y-1.5">
            <div
              v-for="(song, idx) in chart.tracks.slice(0, 3)"
              :key="`${chart.id}-${idx}`"
              @click.stop="quickPlay(song.title, song.artist)"
              class="flex items-center gap-2.5 p-1.5 rounded-lg hover:bg-white/10 transition-colors group/item"
            >
              <span class="w-3.5 text-xs font-mono font-bold" :class="idx === 0 ? 'text-amber-400' : idx === 1 ? 'text-slate-300' : 'text-amber-600'">
                {{ idx + 1 }}
              </span>
              <div class="flex-1 min-w-0">
                <p class="text-xs text-white truncate font-medium group-hover/item:text-brand transition-colors">{{ song.title }}</p>
                <p class="text-[10px] text-slate-400 truncate">{{ song.artist }}</p>
              </div>
              <button
                @click.stop="quickPlay(song.title, song.artist)"
                class="w-6 h-6 rounded-full bg-brand/20 hover:bg-brand text-brand hover:text-white flex items-center justify-center opacity-0 group-hover/item:opacity-100 transition-all shrink-0"
                title="立即试听"
              >
                <Play :size="11" fill="currentColor" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Expanded All Charts Grid (全部榜单) -->
      <Transition name="fade">
        <div v-if="showAllCharts && otherCharts.length > 0" class="pt-2 space-y-3">
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold text-slate-300">全部全球与特色榜单 ({{ otherCharts.length }})</span>
          </div>
          <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-3.5">
            <div
              v-for="chart in otherCharts"
              :key="chart.id"
              @click="openDiscoveredPlaylist(chart.id)"
              class="glass-card rounded-2xl p-2.5 space-y-2 cursor-pointer group"
            >
              <div class="relative aspect-square rounded-xl overflow-hidden border border-white/10">
                <img
                  v-if="chart.cover"
                  :src="chart.cover"
                  loading="lazy"
                  decoding="async"
                  class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                />
                <span class="absolute bottom-1.5 left-1.5 px-1.5 py-0.5 rounded-md bg-black/65 backdrop-blur-md text-[9px] text-white/90 border border-white/10">
                  {{ chart.update_frequency }}
                </span>
                <div class="absolute inset-0 bg-black/35 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
                  <div class="w-8 h-8 rounded-full bg-brand text-white flex items-center justify-center shadow-lg">
                    <Play :size="13" fill="white" class="ml-0.5" />
                  </div>
                </div>
              </div>
              <p class="text-xs font-semibold text-white truncate group-hover:text-brand transition-colors px-0.5">
                {{ chart.name }}
              </p>
            </div>
          </div>
        </div>
      </Transition>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import {
  Layers,
  Search,
  RefreshCw,
  Play,
  Heart,
  ListPlus,
  PlusCircle,
  Disc,
  Radio,
  Flame,
  Zap,
  Waves,
  Headphones,
  Link2,
  Cloud,
  Music2,
  Youtube,
} from 'lucide-vue-next';
import { useProviderStore } from '@/stores/provider';
import { usePlayerStore } from '@/stores/player';
import { usePlaylistStore } from '@/stores/playlist';
import { useImporterStore } from '@/stores/importer';
import { useNeteaseStore } from '@/stores/netease';
import { useQQMusicStore } from '@/stores/qqmusic';
import { useYTMusicStore } from '@/stores/ytmusic';
import { useExtraPlatformsStore } from '@/stores/extraPlatforms';
import { useContextMenuStore } from '@/stores/contextMenu';
import { PLATFORMS, getTrackPlatform, getPlatformBadge, type PlatformId } from '@/types/provider';
import type { Track } from '@/types/music';

const router = useRouter();
const providerStore = useProviderStore();
const playerStore = usePlayerStore();
const playlistStore = usePlaylistStore();
const importerStore = useImporterStore();
const neteaseStore = useNeteaseStore();
const qqStore = useQQMusicStore();
const ytStore = useYTMusicStore();
const extraStore = useExtraPlatformsStore();
const contextMenuStore = useContextMenuStore();

interface DiscoverPlaylistItem {
  id: string;
  name: string;
  description?: string | null;
  cover?: string | null;
  track_count: number;
  play_count: number;
  tags: string[];
}

const discoverPlaylistsSection = ref<HTMLElement | null>(null);
const playlistCategories = ['全部', '华语', '流行', '摇滚', '电子', '轻音乐', '古典', '爵士', '民谣', '说唱', 'ACG', '治愈'];
const activePlaylistCat = ref('全部');
const playlistOffset = ref(0);
const isLoadingPlaylists = ref(false);
const discoverPlaylists = ref<DiscoverPlaylistItem[]>([]);

const loadDiscoverPlaylists = async () => {
  isLoadingPlaylists.value = true;
  try {
    const res = await invoke<DiscoverPlaylistItem[]>('netease_get_discover_playlists', {
      cat: activePlaylistCat.value,
      limit: 12,
      offset: playlistOffset.value,
    });
    if (res && res.length > 0) {
      discoverPlaylists.value = res;
    }
  } catch (e) {
    console.warn('Failed to load discover playlists:', e);
  } finally {
    isLoadingPlaylists.value = false;
  }
};

const selectPlaylistCategory = (cat: string) => {
  activePlaylistCat.value = cat;
  playlistOffset.value = 0;
  loadDiscoverPlaylists();
};

const selectGenre = async (cat: string) => {
  activePlaylistCat.value = cat;
  playlistOffset.value = 0;
  await loadDiscoverPlaylists();
  discoverPlaylistsSection.value?.scrollIntoView({ behavior: 'smooth', block: 'start' });
};

const refreshDiscoverPlaylists = () => {
  playlistOffset.value += 12;
  loadDiscoverPlaylists();
};

const openDiscoveredPlaylist = (id: string) => {
  router.push(`/playlist/${id}`);
};

const formatPlayCount = (count: number): string => {
  if (count >= 100000000) return `${(count / 100000000).toFixed(1)}亿`;
  if (count >= 10000) return `${Math.floor(count / 10000)}万`;
  return String(count);
};

interface ToplistItem {
  id: string;
  name: string;
  update_frequency: string;
  cover?: string | null;
  description?: string | null;
  tracks: Array<{ title: string; artist: string }>;
}

const showAllCharts = ref(false);
const allCharts = ref<ToplistItem[]>([]);

const fallbackCharts: ToplistItem[] = [
  {
    id: 'netease-pl-19723756',
    name: '飙升榜',
    update_frequency: '每天更新',
    tracks: [
      { title: '晴天', artist: '周杰伦' },
      { title: '起风了', artist: '买辣椒也用券' },
      { title: 'Cruel Summer', artist: 'Taylor Swift' },
    ],
  },
  {
    id: 'netease-pl-3779629',
    name: '新歌榜',
    update_frequency: '每天更新',
    tracks: [
      { title: '交换余生', artist: '林俊杰' },
      { title: '孤勇者', artist: '陈奕迅' },
      { title: '悬溺', artist: '葛东琪' },
    ],
  },
  {
    id: 'netease-pl-3778678',
    name: '热歌榜',
    update_frequency: '每周四更新',
    tracks: [
      { title: '成都', artist: '赵雷' },
      { title: '稻香', artist: '周杰伦' },
      { title: '夜曲', artist: '周杰伦' },
    ],
  },
];

const featuredCharts = computed(() => {
  const source = allCharts.value.length > 0 ? allCharts.value : fallbackCharts;
  const withTracks = source.filter((c) => c.tracks && c.tracks.length > 0);
  return (withTracks.length > 0 ? withTracks : source).slice(0, 4);
});

const otherCharts = computed(() => {
  const featuredIds = new Set(featuredCharts.value.map((c) => c.id));
  return allCharts.value.filter((c) => !featuredIds.has(c.id));
});

const loadToplists = async () => {
  try {
    const res = await invoke<ToplistItem[]>('netease_get_toplists');
    if (res && res.length > 0) {
      allCharts.value = res;
    }
  } catch (e) {
    console.warn('Failed to load toplists:', e);
  }
};

onMounted(() => {
  loadDiscoverPlaylists();
  loadToplists();
});

const keywordInput = ref('');
const hasSearched = ref(false);

const hotTags = ['周杰伦', 'Taylor Swift', '林俊杰', '陈奕迅', '纯音乐', 'ACG 原声', 'Lo-Fi Chill', 'Synthwave'];

const genres = [
  { name: '电子 / 氛围', cat: '电子', subtitle: 'EDM • Synthwave', gradient: 'bg-gradient-to-br from-indigo-600 to-purple-800', icon: Zap },
  { name: 'Lo-Fi / 专注', cat: '轻音乐', subtitle: 'Study • Chill', gradient: 'bg-gradient-to-br from-teal-600 to-emerald-800', icon: Headphones },
  { name: '古典 / 纯音', cat: '古典', subtitle: 'Classical • Piano', gradient: 'bg-gradient-to-br from-amber-600 to-orange-800', icon: Disc },
  { name: '流行 / 原创', cat: '流行', subtitle: 'Pop • Hits', gradient: 'bg-gradient-to-br from-rose-600 to-brand', icon: Flame },
  { name: '摇滚 / 独立', cat: '摇滚', subtitle: 'Rock • Indie', gradient: 'bg-gradient-to-br from-blue-600 to-cyan-800', icon: Waves },
  { name: '爵士 / 蓝调', cat: '爵士', subtitle: 'Jazz • Blues', gradient: 'bg-gradient-to-br from-violet-700 to-fuchsia-900', icon: Radio },
  { name: '民谣 / 治愈', cat: '民谣', subtitle: 'Folk • Acoustic', gradient: 'bg-gradient-to-br from-emerald-600 to-teal-900', icon: Heart },
  { name: '说唱 / 嘻哈', cat: '说唱', subtitle: 'Hip-Hop • Rap', gradient: 'bg-gradient-to-br from-orange-600 to-red-900', icon: Layers },
];

const displayResults = computed(() => {
  if (providerStore.activePlatform === 'all') {
    return providerStore.results;
  }
  return providerStore.results.filter((t) => getTrackPlatform(t) === providerStore.activePlatform);
});

const getPlatformCount = (id: PlatformId): number => {
  if (id === 'all') {
    return (
    providerStore.counts.netease +
    providerStore.counts.qq +
    providerStore.counts.kuwo +
    providerStore.counts.kugou +
    providerStore.counts.qishui +
    providerStore.counts.apple +
    providerStore.counts.youtube
  );
  }
  return providerStore.counts[id] || 0;
};

const handleSearch = async () => {
  if (!keywordInput.value || !keywordInput.value.trim()) return;
  hasSearched.value = true;
  await providerStore.performSearch(keywordInput.value.trim());
};

const switchPlatform = (platform: PlatformId) => {
  providerStore.setActivePlatform(platform);
};

const quickSearch = async (tag: string) => {
  keywordInput.value = tag;
  await handleSearch();
  const mainEl = document.querySelector('main');
  if (mainEl) {
    mainEl.scrollTo({ top: 0, behavior: 'smooth' });
  }
};

const quickPlay = async (songTitle: string, artist?: string) => {
  const query = artist ? `${songTitle} ${artist}` : songTitle;
  keywordInput.value = query;
  hasSearched.value = true;
  await providerStore.performSearch(query);
  const mainEl = document.querySelector('main');
  if (mainEl) {
    mainEl.scrollTo({ top: 0, behavior: 'smooth' });
  }
  if (displayResults.value.length > 0) {
    playerStore.playTrack(displayResults.value[0], displayResults.value);
  }
};

const clearSearch = () => {
  keywordInput.value = '';
  hasSearched.value = false;
  providerStore.clearResults();
};

const playAllResults = () => {
  if (displayResults.value.length > 0) {
    playerStore.playTrack(displayResults.value[0], displayResults.value);
  }
};

const addToQueue = (track: Track) => {
  if (!playerStore.queue.some((t) => t.id === track.id)) {
    playerStore.queue.push(track);
  }
};

const getBadge = (track: Track) => {
  const p = getTrackPlatform(track);
  return getPlatformBadge(p);
};

const formatDuration = (seconds?: number): string => {
  if (!seconds) return '0:00';
  const m = Math.floor(seconds / 60);
  const s = Math.floor(seconds % 60);
  return `${m}:${s.toString().padStart(2, '0')}`;
};
</script>
