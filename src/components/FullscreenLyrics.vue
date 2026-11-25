<template>
  <div
    v-if="playerStore.showFullscreenLyrics"
    class="fixed inset-0 z-50 overflow-hidden flex flex-col justify-between bg-[#08080c] text-white select-none transition-all duration-500"
    :style="settingsStore.lyricsBgMode === 'solid' ? { backgroundColor: settingsStore.lyricsBgColor } : undefined"
    @click="showBgPanel = false"
  >
    <!-- Mode A: Custom Image or Album Cover Backdrop -->
    <div
      v-if="(settingsStore.lyricsBgMode === 'custom' && settingsStore.lyricsBgImage) || (settingsStore.lyricsBgMode === 'cover' && playerStore.currentTrack?.cover)"
      class="absolute inset-0 overflow-hidden pointer-events-none z-0"
    >
      <img
        :src="settingsStore.lyricsBgMode === 'custom' ? settingsStore.lyricsBgImage : playerStore.currentTrack?.cover"
        alt="Lyrics Backdrop"
        class="w-full h-full object-cover scale-110 transition-all duration-700"
        :style="{ filter: `blur(${settingsStore.lyricsBgBlur}px) saturate(1.25)` }"
      />
      <div
        class="absolute inset-0 transition-colors duration-300"
        :style="{ backgroundColor: `rgba(0, 0, 0, ${settingsStore.lyricsBgDim / 100})` }"
      ></div>
    </div>

    <!-- Mode B: Apple Music Style Dynamic Liquid Fluid Mesh Backdrop (Default / Fallback) -->
    <div
      v-else-if="settingsStore.lyricsBgMode !== 'solid'"
      class="absolute inset-0 overflow-hidden pointer-events-none opacity-55 z-0"
    >
      <!-- Animated Fluid Orb 1 -->
      <div
        class="absolute -top-[15%] -left-[10%] w-[55vw] h-[55vw] rounded-full blur-[130px] bg-gradient-to-br from-brand/60 via-brand-light/40 to-transparent animate-fluid-1"
      ></div>
      <!-- Animated Fluid Orb 2 -->
      <div
        class="absolute -bottom-[20%] -right-[10%] w-[60vw] h-[60vw] rounded-full blur-[150px] bg-gradient-to-tl from-brand/50 via-brand-light/30 to-transparent animate-fluid-2"
      ></div>
      <!-- Animated Fluid Orb 3 -->
      <div
        class="absolute top-[30%] left-[25%] w-[45vw] h-[45vw] rounded-full blur-[140px] bg-gradient-to-tr from-brand-glow via-brand/20 to-transparent animate-fluid-3"
      ></div>
      <!-- Frosted Overlay -->
      <div class="absolute inset-0 bg-black/40 backdrop-blur-[60px]"></div>
    </div>

    <!-- Top Navigation Bar -->
    <header class="relative z-10 h-14 px-8 flex items-center justify-between border-b border-white/[0.06] bg-black/20 backdrop-blur-md">
      <div class="flex items-center gap-3">
        <button
          @click="playerStore.toggleFullscreenLyrics"
          class="p-2 rounded-full hover:bg-white/10 text-white/70 hover:text-white transition-all active:scale-95"
          title="收起全屏 (Esc)"
        >
          <ChevronDown :size="22" />
        </button>
        <span class="text-xs uppercase tracking-widest text-white/50 font-semibold flex items-center gap-2">
          <span>NOW PLAYING</span>
          <span class="w-1 h-1 rounded-full bg-brand"></span>
          <span>{{ $t('player.immersive_lyrics') }}</span>
        </span>
      </div>

      <!-- Format badge & visualizer pill & AI Tools -->
      <div class="flex items-center gap-2.5">
        <!-- AI Translation Button -->
        <button
          @click="handleAiTranslate"
          :disabled="aiStore.isTranslating || !playerStore.currentTrack"
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border transition-all duration-300 disabled:opacity-50"
          :class="[
            hasTranslation 
              ? 'bg-brand/20 border-brand/40 text-brand-light shadow-sm shadow-brand/20' 
              : 'bg-white/10 border-white/15 text-white/80 hover:bg-white/20 hover:text-white'
          ]"
          :title="hasTranslation ? '歌词已带有翻译，点击可重新通过 AI 逐句润色' : '使用 AI 逐句翻译并对齐歌词'"
        >
          <RefreshCw v-if="aiStore.isTranslating" :size="13" class="animate-spin text-brand" />
          <Sparkles v-else :size="13" class="text-brand" />
          <span>{{ aiStore.isTranslating ? $t('lyrics.ai_translating') : (hasTranslation ? $t('lyrics.ai_translated') : $t('lyrics.ai_translate')) }}</span>
        </button>

        <!-- AI Insight Button -->
        <button
          @click="openInsightModal"
          :disabled="!playerStore.currentTrack"
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium bg-white/10 border border-white/15 text-white/80 hover:bg-white/20 hover:text-white transition-all duration-300 disabled:opacity-50"
          :title="$t('lyrics.ai_insight_title')"
        >
          <Sparkles :size="13" class="text-amber-400" />
          <span>{{ $t('lyrics.ai_insight') }}</span>
        </button>

        <!-- Custom Lyrics Background Popover Trigger -->
        <div class="relative" @click.stop>
          <button
            @click="showBgPanel = !showBgPanel"
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border transition-all duration-300"
            :class="showBgPanel || settingsStore.lyricsBgMode !== 'dynamic' ? 'bg-brand/20 border-brand/40 text-brand-light' : 'bg-white/10 border-white/15 text-white/80 hover:bg-white/20 hover:text-white'"
            title="自定义歌词页背景"
          >
            <ImageIcon :size="13" />
            <span>背景</span>
          </button>

          <!-- Liquid Glass Background Customizer Popover -->
          <Transition name="fade">
            <div
              v-if="showBgPanel"
              class="absolute right-0 top-11 w-80 rounded-3xl liquid-glass-floating p-4 z-50 space-y-4 text-left animate-in fade-in zoom-in-95 duration-150"
            >
              <div class="flex items-center justify-between border-b border-white/10 pb-2.5">
                <div class="flex items-center gap-2">
                  <ImageIcon :size="14" class="text-brand" />
                  <span class="text-xs font-bold text-white">歌词页背景设置</span>
                </div>
                <button
                  @click="settingsStore.resetLyricsBackground()"
                  class="text-[11px] text-slate-400 hover:text-white transition-colors"
                >
                  恢复默认
                </button>
              </div>

              <!-- Background Mode Pills -->
              <div class="grid grid-cols-4 gap-1 p-1 rounded-xl bg-black/30 border border-white/10 text-[11px]">
                <button
                  v-for="m in bgModes"
                  :key="m.id"
                  @click="settingsStore.setLyricsBgMode(m.id)"
                  class="py-1.5 rounded-lg font-medium transition-all text-center"
                  :class="settingsStore.lyricsBgMode === m.id ? 'bg-brand text-white font-semibold shadow-sm' : 'text-slate-300 hover:text-white'"
                >
                  {{ m.label }}
                </button>
              </div>

              <!-- Solid Color Picker -->
              <div v-if="settingsStore.lyricsBgMode === 'solid'" class="space-y-2">
                <span class="text-[11px] text-slate-300 font-medium block">选择背景纯色</span>
                <div class="flex items-center gap-2 flex-wrap">
                  <button
                    v-for="c in solidColors"
                    :key="c"
                    @click="settingsStore.setLyricsBgColor(c)"
                    class="w-7 h-7 rounded-full border-2 transition-transform hover:scale-110"
                    :class="settingsStore.lyricsBgColor === c ? 'border-white scale-110' : 'border-white/20'"
                    :style="{ backgroundColor: c }"
                  ></button>
                  <input
                    type="color"
                    :value="settingsStore.lyricsBgColor"
                    @input="settingsStore.setLyricsBgColor(($event.target as HTMLInputElement).value)"
                    class="w-7 h-7 rounded-full bg-transparent border border-white/20 cursor-pointer"
                    title="自定义颜色"
                  />
                </div>
              </div>

              <!-- Custom Image Controls -->
              <div v-if="settingsStore.lyricsBgMode === 'custom'" class="space-y-3">
                <div class="flex items-center gap-2">
                  <input
                    ref="bgFileInput"
                    type="file"
                    accept="image/*"
                    class="hidden"
                    @change="onBgFileSelected"
                  />
                  <button
                    @click="bgFileInput?.click()"
                    class="flex-1 py-2 px-3 rounded-xl bg-brand/20 hover:bg-brand/30 border border-brand/35 text-brand-light text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors"
                  >
                    <Upload :size="13" />
                    <span>选择本地图片</span>
                  </button>
                </div>

                <!-- Built-in Wallpaper Presets -->
                <div class="space-y-1.5">
                  <span class="text-[11px] text-slate-400 block">精选预设壁纸</span>
                  <div class="grid grid-cols-4 gap-2">
                    <button
                      v-for="preset in presetWallpapers"
                      :key="preset.url"
                      @click="settingsStore.setLyricsBgImage(preset.url)"
                      class="aspect-video rounded-lg overflow-hidden border-2 transition-all"
                      :class="settingsStore.lyricsBgImage === preset.url ? 'border-brand scale-105' : 'border-white/10 hover:border-white/40'"
                      :title="preset.name"
                    >
                      <img :src="preset.url" class="w-full h-full object-cover" />
                    </button>
                  </div>
                </div>

                <!-- Image URL Input -->
                <input
                  :value="settingsStore.lyricsBgImage.startsWith('data:') ? '' : settingsStore.lyricsBgImage"
                  @change="settingsStore.setLyricsBgImage(($event.target as HTMLInputElement).value)"
                  type="text"
                  placeholder="或粘贴图片链接 https://..."
                  class="w-full px-3 py-1.5 text-[11px] bg-black/40 text-slate-200 placeholder-slate-500 rounded-xl border border-white/10 focus:border-brand focus:outline-none"
                />
              </div>

              <!-- Blur & Dim Sliders (for Cover or Custom Image) -->
              <div
                v-if="settingsStore.lyricsBgMode === 'cover' || settingsStore.lyricsBgMode === 'custom'"
                class="space-y-3 pt-1 border-t border-white/10"
              >
                <div class="space-y-1">
                  <div class="flex justify-between text-[11px] text-slate-300">
                    <span>背景模糊度</span>
                    <span class="font-mono text-slate-400">{{ settingsStore.lyricsBgBlur }}px</span>
                  </div>
                  <input
                    type="range"
                    min="0"
                    max="80"
                    :value="settingsStore.lyricsBgBlur"
                    @input="settingsStore.setLyricsBgBlur(Number(($event.target as HTMLInputElement).value))"
                    class="w-full h-1"
                  />
                </div>
                <div class="space-y-1">
                  <div class="flex justify-between text-[11px] text-slate-300">
                    <span>暗度遮罩（保证歌词可读性）</span>
                    <span class="font-mono text-slate-400">{{ settingsStore.lyricsBgDim }}%</span>
                  </div>
                  <input
                    type="range"
                    min="10"
                    max="85"
                    :value="settingsStore.lyricsBgDim"
                    @input="settingsStore.setLyricsBgDim(Number(($event.target as HTMLInputElement).value))"
                    class="w-full h-1"
                  />
                </div>
              </div>
            </div>
          </Transition>
        </div>

        <div v-if="playerStore.isPlaying" class="hidden sm:flex items-end gap-0.5 h-3.5 px-2 py-0.5 rounded-full bg-brand/20 border border-brand/30">
          <span class="w-0.5 bg-brand animate-bar-1 h-full"></span>
          <span class="w-0.5 bg-brand animate-bar-2 h-2/3"></span>
          <span class="w-0.5 bg-brand animate-bar-3 h-4/5"></span>
        </div>

        <span class="hidden md:inline-block px-3 py-1 rounded-full text-[11px] font-mono font-medium bg-white/10 text-white/90 border border-white/10 backdrop-blur-md">
          {{ playerStore.currentTrack?.format || 'Hi-Res 无损 24bit/96kHz' }}
        </span>
      </div>
    </header>

    <!-- Main Content: Left Album Art + Right Synced Lyrics -->
    <main class="relative z-10 flex-1 flex flex-col md:flex-row items-center justify-center max-w-6xl w-full mx-auto px-8 py-6 gap-12 overflow-hidden">
      <!-- Left Column: Album Art & Track Meta -->
      <div class="w-full md:w-5/12 flex flex-col items-center md:items-start text-center md:text-left space-y-6">
        <div class="relative w-64 h-64 md:w-84 md:h-84 rounded-3xl overflow-hidden shadow-2xl shadow-black/80 border border-white/15 group">
          <img
            v-if="playerStore.currentTrack?.cover"
            :src="playerStore.currentTrack.cover"
            alt="Album Cover"
            decoding="async"
            class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-700"
          />
          <div v-else class="w-full h-full bg-white/5 flex items-center justify-center">
            <Music :size="64" class="text-slate-500" />
          </div>
          <!-- Subtle Vinyl Glow -->
          <div class="absolute inset-0 ring-1 ring-inset ring-white/20 rounded-3xl pointer-events-none"></div>
        </div>

        <div class="space-y-2 max-w-xs md:max-w-md w-full">
          <div class="flex items-center justify-between">
            <h2 class="text-2xl md:text-3xl font-extrabold tracking-tight text-white line-clamp-1">
              {{ playerStore.currentTrackTitle }}
            </h2>
            <button
              @click="playerStore.toggleFavorite()"
              class="p-2 rounded-full hover:bg-white/10 transition-colors"
              :class="(playerStore.currentTrack?.isFavorite || (playerStore.currentTrack && playerStore.isIdFavorite(playerStore.currentTrack.id))) ? 'text-brand' : 'text-white/50 hover:text-white'"
              title="收藏"
            >
              <Heart :size="20" :fill="(playerStore.currentTrack?.isFavorite || (playerStore.currentTrack && playerStore.isIdFavorite(playerStore.currentTrack.id))) ? 'currentColor' : 'none'" />
            </button>
          </div>

          <p class="text-base text-white/80 font-medium line-clamp-1">
            {{ playerStore.currentTrackArtist }}
          </p>

          <p class="text-xs text-white/40 line-clamp-1 font-mono">
            {{ playerStore.currentTrack?.album || '未知专辑' }}
          </p>
        </div>
      </div>

      <!-- Right Column: Interactive Kinetic Lyrics Stream -->
      <div class="w-full md:w-7/12 h-full flex flex-col justify-center overflow-hidden relative">
        <div
          ref="lyricsContainer"
          class="h-[480px] overflow-y-auto space-y-9 pr-6 scroll-smooth mask-fade"
        >
          <!-- Empty Lyrics State -->
          <div
            v-if="!playerStore.lyrics || playerStore.lyrics.length === 0"
            class="h-full flex flex-col items-center justify-center text-center p-8 space-y-4"
          >
            <div class="w-16 h-16 rounded-3xl bg-white/5 border border-white/10 flex items-center justify-center shadow-lg shadow-black/40">
              <Music :size="28" class="text-white/40" />
            </div>
            <div class="space-y-1">
              <p class="text-lg font-medium text-white/80">{{ $t('lyrics.no_lyrics') }}</p>
              <p class="text-xs text-white/40 max-w-xs">{{ $t('lyrics.no_lyrics_tip') }}</p>
            </div>
            <button
              @click="openInsightModal"
              class="px-4 py-2 rounded-full bg-brand/20 border border-brand/30 text-brand-light text-xs font-medium hover:bg-brand/30 transition-all flex items-center gap-1.5"
            >
              <Sparkles :size="13" />
              <span>{{ $t('lyrics.ai_insight') }}</span>
            </button>
          </div>

          <!-- Synced Lyrics Lines -->
          <div
            v-else
            v-for="(line, index) in playerStore.lyrics"
            :key="index"
            :ref="el => setLyricRef(el, index)"
            @click="seekToLine(line.time)"
            class="cursor-pointer transition-all duration-500 group flex items-start gap-3"
            :class="[
              index === playerStore.activeLyricIndex
                ? 'scale-105 origin-left'
                : 'opacity-30 hover:opacity-75 blur-[0.3px]'
            ]"
          >
            <!-- Hover Play Quick Jump Icon -->
            <button
              class="w-6 h-6 rounded-full bg-white/10 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity mt-1 shrink-0 hover:bg-brand"
              title="跳转至此句"
            >
              <Play :size="10" fill="white" class="ml-0.5 text-white" />
            </button>

            <div class="flex-1">
              <p
                class="text-xl md:text-2xl font-bold transition-all duration-300 tracking-tight leading-snug"
                :class="index === playerStore.activeLyricIndex ? 'text-white text-glow' : 'text-slate-300'"
              >
                {{ line.text }}
              </p>
              <p
                v-if="line.translation"
                class="text-sm font-medium mt-1.5 transition-all duration-300 tracking-normal"
                :class="index === playerStore.activeLyricIndex ? 'text-white/85' : 'text-slate-400'"
              >
                {{ line.translation }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- Bottom Full Player Controller -->
    <footer class="relative z-10 h-24 px-8 flex flex-col justify-center border-t border-white/[0.08] bg-black/40 backdrop-blur-2xl">
      <!-- Timestamps & Progress Slider -->
      <div class="w-full max-w-4xl mx-auto flex items-center gap-4 text-xs font-mono text-white/60 mb-2">
        <span class="w-10 text-right tabular-nums">{{ playerStore.formattedCurrentTime }}</span>
        <div class="relative flex-1 flex items-center group py-1 cursor-pointer">
          <input
            type="range"
            min="0"
            :max="playerStore.duration || 100"
            :value="playerStore.currentTime"
            @input="onSeek"
            class="w-full h-1 bg-white/20 rounded-full accent-brand group-hover:h-1.5 transition-all"
          />
        </div>
        <span class="w-10 tabular-nums">{{ playerStore.formattedDuration }}</span>
      </div>

      <!-- Controls Row: Shuffle, Prev, Play/Pause, Next, Mode, Volume -->
      <div class="w-full max-w-xl mx-auto flex items-center justify-between px-4">
        <!-- Shuffle Mode -->
        <button
          @click="playerStore.mode = playerStore.mode === 'shuffle' ? 'loop' : 'shuffle'"
          class="p-2 rounded-full hover:bg-white/10 transition-colors"
          :class="playerStore.mode === 'shuffle' ? 'text-brand' : 'text-white/60 hover:text-white'"
          title="随机播放"
        >
          <Shuffle :size="16" />
        </button>

        <!-- Previous -->
        <button
          @click="playerStore.prevTrack"
          class="p-2 rounded-full text-white/80 hover:text-white hover:bg-white/10 transition-colors"
          title="上一首"
        >
          <SkipBack :size="20" />
        </button>

        <!-- Big Play / Pause Circle -->
        <button
          @click="playerStore.togglePlay"
          class="w-12 h-12 rounded-full bg-gradient-to-tr from-brand to-pink-500 text-white flex items-center justify-center shadow-lg shadow-brand/40 hover:scale-105 active:scale-95 transition-all"
          :title="playerStore.isPlaying ? '暂停' : '播放'"
        >
          <RefreshCw v-if="playerStore.isLoadingStream" :size="22" class="animate-spin" />
          <Pause v-else-if="playerStore.isPlaying" :size="24" fill="currentColor" />
          <Play v-else :size="24" fill="currentColor" class="ml-0.5" />
        </button>

        <!-- Next -->
        <button
          @click="playerStore.nextTrack"
          class="p-2 rounded-full text-white/80 hover:text-white hover:bg-white/10 transition-colors"
          title="下一首"
        >
          <SkipForward :size="20" />
        </button>

        <!-- Loop Mode -->
        <button
          @click="playerStore.cycleMode"
          class="p-2 rounded-full hover:bg-white/10 transition-colors"
          :class="playerStore.mode !== 'sequence' ? 'text-brand' : 'text-white/60 hover:text-white'"
          :title="modeTitle"
        >
          <Repeat1 v-if="playerStore.mode === 'single'" :size="17" />
          <Repeat v-else :size="17" />
        </button>

        <!-- Volume Slider -->
        <div class="flex items-center gap-2 group ml-4">
          <button
            @click="playerStore.toggleMute"
            class="p-1 text-white/60 hover:text-white transition-colors"
          >
            <VolumeX v-if="playerStore.isMuted || playerStore.volume === 0" :size="16" />
            <Volume2 v-else :size="16" />
          </button>
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            :value="playerStore.isMuted ? 0 : playerStore.volume"
            @input="onVolumeChange"
            class="w-20 h-1 bg-white/20 rounded-full accent-brand"
          />
        </div>
      </div>
    </footer>

    <!-- AI Insight Backdrop & Modal Dialog -->
    <div
      v-if="showInsightModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-xl animate-fade-in"
      @click.self="showInsightModal = false"
    >
      <div class="relative w-full max-w-lg bg-[#14141e]/95 border border-white/15 rounded-3xl p-6 md:p-8 shadow-2xl shadow-brand/20 overflow-hidden">
        <!-- Ambient Glow Blob in Modal -->
        <div class="absolute -top-24 -right-24 w-60 h-60 rounded-full bg-brand/25 blur-3xl pointer-events-none"></div>

        <!-- Modal Header -->
        <div class="flex items-center justify-between pb-4 border-b border-white/10 relative z-10">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-2xl bg-gradient-to-tr from-brand to-pink-500 flex items-center justify-center shadow-lg shadow-brand/30">
              <Sparkles :size="20" class="text-white" />
            </div>
            <div>
              <h3 class="text-base font-bold text-white tracking-tight flex items-center gap-2">
                <span>{{ $t('lyrics.ai_insight_title') }}</span>
                <span class="text-[10px] px-2 py-0.5 rounded-full bg-brand/20 text-brand-light border border-brand/30 uppercase font-mono font-normal">
                  {{ aiStore.engine === 'local_model' ? $t('lyrics.local_model_tag') : (aiStore.engine === 'ollama' ? 'Ollama' : aiStore.modelName) }}
                </span>
              </h3>
              <p class="text-xs text-white/50 line-clamp-1 mt-0.5">
                《{{ playerStore.currentTrackTitle }}》 · {{ playerStore.currentTrackArtist }}
              </p>
            </div>
          </div>
          <button
            @click="showInsightModal = false"
            class="p-2 rounded-full hover:bg-white/10 text-white/60 hover:text-white transition-colors"
          >
            <X :size="18" />
          </button>
        </div>

        <!-- Modal Body -->
        <div class="py-6 relative z-10">
          <!-- Loading State -->
          <div v-if="aiStore.isGeneratingInsight" class="space-y-4 py-8 flex flex-col items-center justify-center text-center">
            <div class="relative">
              <div class="w-14 h-14 rounded-full border-2 border-brand/20 border-t-brand animate-spin"></div>
              <Sparkles :size="20" class="text-brand absolute inset-0 m-auto animate-pulse" />
            </div>
            <div class="space-y-1">
              <p class="text-sm font-medium text-white/90">{{ $t('lyrics.ai_analyzing') }}</p>
              <p class="text-xs text-white/40">{{ $t('lyrics.ai_insight_desc') }}</p>
            </div>
          </div>

          <!-- Content State -->
          <div v-else class="space-y-4">
            <div class="relative p-5 rounded-2xl bg-white/[0.04] border border-white/10 leading-relaxed text-sm md:text-base text-slate-200 font-light">
              <div class="absolute -top-3 left-4 px-2 bg-[#14141e] text-[10px] font-mono text-brand uppercase tracking-wider flex items-center gap-1 border border-brand/30 rounded">
                <Quote :size="11" />
                <span>{{ $t('lyrics.interpretation') }}</span>
              </div>
              <p class="whitespace-pre-line leading-relaxed text-white/90">
                {{ aiStore.songInsight || '暂无分析内容' }}
              </p>
            </div>

            <!-- Atmosphere Tag Pills -->
            <div class="flex flex-wrap items-center gap-2 pt-1 text-xs">
              <span class="text-white/40 font-mono text-[11px]">{{ $t('lyrics.tags') }}:</span>
              <span class="px-2.5 py-1 rounded-full bg-brand/15 text-brand-light border border-brand/25 font-medium">沉浸放空</span>
              <span class="px-2.5 py-1 rounded-full bg-purple-500/15 text-purple-300 border border-purple-500/25 font-medium">深邃心境</span>
              <span class="px-2.5 py-1 rounded-full bg-cyan-500/15 text-cyan-300 border border-cyan-500/25 font-medium">声学留白</span>
            </div>
          </div>
        </div>

        <!-- Modal Footer -->
        <div class="flex items-center justify-between pt-4 border-t border-white/10 relative z-10 text-xs">
          <button
            @click="refreshInsight"
            :disabled="aiStore.isGeneratingInsight"
            class="flex items-center gap-1.5 px-3.5 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-white/80 hover:text-white transition-all disabled:opacity-50"
          >
            <RefreshCw :size="13" :class="{ 'animate-spin': aiStore.isGeneratingInsight }" />
            <span>{{ $t('lyrics.re_insight') }}</span>
          </button>

          <button
            @click="showInsightModal = false"
            class="px-5 py-2 rounded-xl bg-gradient-to-r from-brand to-pink-500 hover:opacity-90 text-white font-medium shadow-md shadow-brand/30 transition-all"
          >
            {{ $t('lyrics.taste_done') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue';
import {
  ChevronDown,
  Play,
  Pause,
  SkipBack,
  SkipForward,
  Shuffle,
  Repeat,
  Repeat1,
  Heart,
  Volume2,
  VolumeX,
  Music,
  RefreshCw,
  Sparkles,
  Quote,
  X,
  Image as ImageIcon,
  Upload,
} from 'lucide-vue-next';
import { usePlayerStore } from '@/stores/player';
import { useAiStore } from '@/stores/ai';
import { useSettingsStore, type LyricsBgMode } from '@/stores/settings';

const playerStore = usePlayerStore();
const aiStore = useAiStore();
const settingsStore = useSettingsStore();
const lyricsContainer = ref<HTMLElement | null>(null);
const lyricRefs = ref<(HTMLElement | null)[]>([]);
const showInsightModal = ref(false);
const showBgPanel = ref(false);
const bgFileInput = ref<HTMLInputElement | null>(null);

const bgModes: Array<{ id: LyricsBgMode; label: string }> = [
  { id: 'dynamic', label: '流体光晕' },
  { id: 'cover', label: '封面弥散' },
  { id: 'solid', label: '纯色' },
  { id: 'custom', label: '自定义图' },
];

const solidColors = ['#08080c', '#071124', '#160e0a', '#071814', '#1b0914', '#14141f'];

const presetWallpapers = [
  { name: '极光夜空', url: 'https://images.unsplash.com/photo-1531366936337-7c912a4589a7?w=1200&auto=format&fit=crop&q=80' },
  { name: '深空星云', url: 'https://images.unsplash.com/photo-1462331940025-496dfbfc7564?w=1200&auto=format&fit=crop&q=80' },
  { name: '霓虹雨夜', url: 'https://images.unsplash.com/photo-1519608487953-e999c86e7455?w=1200&auto=format&fit=crop&q=80' },
  { name: '流体抽象', url: 'https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=1200&auto=format&fit=crop&q=80' },
];

const onBgFileSelected = (e: Event) => {
  const input = e.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = () => {
    if (typeof reader.result === 'string') {
      settingsStore.setLyricsBgImage(reader.result);
    }
  };
  reader.readAsDataURL(file);
  input.value = '';
};

const setLyricRef = (el: any, index: number) => {
  if (el) {
    lyricRefs.value[index] = el;
  }
};

const seekToLine = (time: number) => {
  playerStore.seek(time);
};

const onSeek = (e: Event) => {
  const target = e.target as HTMLInputElement;
  playerStore.seek(Number(target.value));
};

const onVolumeChange = (e: Event) => {
  const target = e.target as HTMLInputElement;
  playerStore.setVolume(Number(target.value));
};

const hasTranslation = computed(() => {
  return playerStore.lyrics && playerStore.lyrics.some((l) => !!l.translation && l.translation.trim().length > 0);
});

const handleAiTranslate = async () => {
  if (!playerStore.currentTrack || !playerStore.lyrics.length) return;
  const translated = await aiStore.translateLyrics(playerStore.currentTrack, playerStore.lyrics);
  playerStore.lyrics = translated;
};

const openInsightModal = async () => {
  if (!playerStore.currentTrack) return;
  showInsightModal.value = true;
  if (aiStore.insightTrackId !== playerStore.currentTrack.id || !aiStore.songInsight) {
    await aiStore.generateSongInsight(playerStore.currentTrack, playerStore.lyrics);
  }
};

const refreshInsight = async () => {
  if (!playerStore.currentTrack) return;
  await aiStore.generateSongInsight(playerStore.currentTrack, playerStore.lyrics);
};

// Keyboard listener for Escape key
const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    if (showInsightModal.value) {
      showInsightModal.value = false;
    } else if (playerStore.showFullscreenLyrics) {
      playerStore.toggleFullscreenLyrics();
    }
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
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

// Auto scroll to active lyric line smoothly
watch(
  () => playerStore.activeLyricIndex,
  async (newIdx) => {
    await nextTick();
    const activeEl = lyricRefs.value[newIdx];
    const container = lyricsContainer.value;
    if (activeEl && container) {
      const topOffset = activeEl.offsetTop - container.offsetTop - container.clientHeight / 2 + activeEl.clientHeight / 2;
      container.scrollTo({
        top: Math.max(0, topOffset),
        behavior: 'smooth',
      });
    }
  }
);
</script>

<style scoped>
.text-glow {
  text-shadow: 0 0 25px rgba(255, 255, 255, 0.45);
}

.mask-fade {
  mask-image: linear-gradient(
    to bottom,
    transparent 0%,
    black 12%,
    black 88%,
    transparent 100%
  );
  -webkit-mask-image: linear-gradient(
    to bottom,
    transparent 0%,
    black 12%,
    black 88%,
    transparent 100%
  );
}

@keyframes fluid1 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50% { transform: translate(6vw, 4vh) scale(1.1); }
}

@keyframes fluid2 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50% { transform: translate(-6vw, -5vh) scale(1.15); }
}

@keyframes fluid3 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50% { transform: translate(4vw, -4vh) scale(1.08); }
}

.animate-fluid-1 { animation: fluid1 14s ease-in-out infinite; }
.animate-fluid-2 { animation: fluid2 18s ease-in-out infinite; }
.animate-fluid-3 { animation: fluid3 12s ease-in-out infinite; }

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
