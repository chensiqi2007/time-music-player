<template>
  <div class="space-y-6 max-w-4xl pb-16">
    <!-- Top Header & Navigation Tabs -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-2 border-b border-white/[0.08]">
      <div class="flex items-center gap-2.5">
        <h1 class="text-2xl font-bold tracking-tight text-white flex items-center gap-2">
          <span
            class="hover:text-brand cursor-pointer transition-colors"
            @click="setTab('all')"
          >
            {{ $t('playback.breadcrumb_settings') }}
          </span>
          <span class="text-slate-500 font-light text-xl">›</span>
          <span class="text-white">
            {{
              activeTab === 'playback'
                ? $t('playback.breadcrumb_playback')
                : activeTab === 'appearance'
                ? $t('settings.group_appearance')
                : activeTab === 'ai'
                ? 'AI 智能'
                : activeTab === 'storage'
                ? '缓存存储'
                : '全部设置'
            }}
          </span>
        </h1>
      </div>

      <!-- Apple Segmented Navigation Pills -->
      <div class="inline-flex items-center p-1 rounded-2xl bg-theme-surface border border-theme-border shrink-0 select-none overflow-x-auto max-w-full">
        <button
          @click="setTab('playback')"
          class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-xl text-xs font-medium transition-all"
          :class="activeTab === 'playback' ? 'bg-brand text-white shadow-md shadow-brand/25 font-semibold' : 'text-slate-400 hover:text-white hover:bg-white/5'"
        >
          <SlidersHorizontal :size="13" />
          <span>{{ $t('playback.title') }}</span>
        </button>

        <button
          @click="setTab('appearance')"
          class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-xl text-xs font-medium transition-all"
          :class="activeTab === 'appearance' ? 'bg-brand text-white shadow-md shadow-brand/25 font-semibold' : 'text-slate-400 hover:text-white hover:bg-white/5'"
        >
          <Palette :size="13" />
          <span>外观</span>
        </button>

        <button
          @click="setTab('ai')"
          class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-xl text-xs font-medium transition-all"
          :class="activeTab === 'ai' ? 'bg-brand text-white shadow-md shadow-brand/25 font-semibold' : 'text-slate-400 hover:text-white hover:bg-white/5'"
        >
          <Sparkles :size="13" />
          <span>AI 智能</span>
        </button>

        <button
          @click="setTab('storage')"
          class="flex items-center gap-1.5 px-3.5 py-1.5 rounded-xl text-xs font-medium transition-all"
          :class="activeTab === 'storage' ? 'bg-brand text-white shadow-md shadow-brand/25 font-semibold' : 'text-slate-400 hover:text-white hover:bg-white/5'"
        >
          <HardDrive :size="13" />
          <span>存储</span>
        </button>

        <button
          @click="setTab('all')"
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-medium transition-all"
          :class="activeTab === 'all' ? 'bg-brand text-white shadow-md shadow-brand/25 font-semibold' : 'text-slate-400 hover:text-white hover:bg-white/5'"
        >
          <span>全部</span>
        </button>
      </div>
    </div>

    <!-- ========================================== -->
    <!-- VIEW 1: 设置 › 播放 (Apple Music 1:1 Parity) -->
    <!-- ========================================== -->
    <div v-if="activeTab === 'playback' || activeTab === 'all'" class="space-y-3.5">
      <!-- 1. 交叉渐入渐出歌曲 Card -->
      <section class="bg-theme-surface border border-theme-border rounded-2xl p-4 sm:p-5 space-y-4 shadow-sm">
        <!-- Main Row: 交叉渐入渐出歌曲 -->
        <div class="flex items-center justify-between gap-4">
          <div class="min-w-0 pr-2">
            <h3 class="text-[13px] font-medium text-white tracking-wide">
              {{ $t('playback.crossfade_title') }}
            </h3>
            <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
              {{ $t('playback.crossfade_desc') }}
            </p>
          </div>
          <!-- Toggle on right -->
          <div class="flex items-center shrink-0">
            <span class="text-xs text-slate-300 mr-2.5 font-normal select-none">
              {{ settingsStore.enableCrossfade ? $t('playback.toggle_on') : $t('playback.toggle_off') }}
            </span>
            <button
              type="button"
              @click="settingsStore.toggleCrossfade()"
              class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full transition-colors duration-200 ease-in-out focus:outline-none"
              :class="settingsStore.enableCrossfade ? 'bg-brand' : 'bg-theme-sliderTrack'"
            >
              <span
                class="pointer-events-none inline-block h-5 w-5 m-0.5 transform rounded-full bg-white shadow-md transition duration-200 ease-in-out"
                :class="settingsStore.enableCrossfade ? 'translate-x-5' : 'translate-x-0'"
              />
            </button>
          </div>
        </div>

        <!-- Sub Row: 时长 (带分段刻度与秒数) -->
        <div class="border-t border-white/[0.05] pt-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
          <div class="min-w-0 pr-2">
            <h4 class="text-[13px] font-medium text-white tracking-wide">
              {{ $t('playback.duration_title') }}
            </h4>
            <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
              {{ $t('playback.duration_desc') }}
            </p>
          </div>
          <!-- Slider Component -->
          <div class="w-full sm:w-72 space-y-1.5 shrink-0">
            <div class="relative flex items-center h-4">
              <!-- Subtle tick marks on slider track (12 ticks) -->
              <div class="absolute inset-x-2 flex justify-between pointer-events-none z-0">
                <span
                  v-for="n in 12"
                  :key="n"
                  class="w-[1.5px] h-1.5 rounded-full"
                  :class="n <= settingsStore.crossfadeDuration ? 'bg-white/40' : 'bg-white/15'"
                ></span>
              </div>
              <input
                type="range"
                min="1"
                max="12"
                step="1"
                :value="settingsStore.crossfadeDuration"
                @input="settingsStore.setCrossfadeDuration(Number(($event.target as HTMLInputElement).value))"
                class="apple-slider w-full appearance-none cursor-pointer z-10"
                :style="{
                  background: `linear-gradient(to right, var(--brand-color) 0%, var(--brand-color) ${((settingsStore.crossfadeDuration - 1) / 11) * 100}%, var(--slider-track) ${((settingsStore.crossfadeDuration - 1) / 11) * 100}%, var(--slider-track) 100%)`
                }"
              />
            </div>
            <!-- Labels below slider -->
            <div class="flex justify-between items-center text-[11px] text-slate-400 select-none px-0.5">
              <span>1</span>
              <span class="text-slate-300 font-medium">
                {{ settingsStore.crossfadeDuration }} {{ $t('playback.seconds') }}
              </span>
              <span>12</span>
            </div>
          </div>
        </div>
      </section>

      <!-- 2. 声音增强器 Card -->
      <section class="bg-theme-surface border border-theme-border rounded-2xl p-4 sm:p-5 space-y-4 shadow-sm">
        <!-- Main Row: 声音增强器 -->
        <div class="flex items-center justify-between gap-4">
          <div class="min-w-0 pr-2">
            <h3 class="text-[13px] font-medium text-white tracking-wide">
              {{ $t('playback.sound_enhancer_title') }}
            </h3>
            <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
              {{ $t('playback.sound_enhancer_desc') }}
            </p>
          </div>
          <div class="flex items-center shrink-0">
            <span class="text-xs text-slate-300 mr-2.5 font-normal select-none">
              {{ settingsStore.enableSoundEnhancer ? $t('playback.toggle_on') : $t('playback.toggle_off') }}
            </span>
            <button
              type="button"
              @click="settingsStore.toggleSoundEnhancer()"
              class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full transition-colors duration-200 ease-in-out focus:outline-none"
              :class="settingsStore.enableSoundEnhancer ? 'bg-brand' : 'bg-theme-sliderTrack'"
            >
              <span
                class="pointer-events-none inline-block h-5 w-5 m-0.5 transform rounded-full bg-white shadow-md transition duration-200 ease-in-out"
                :class="settingsStore.enableSoundEnhancer ? 'translate-x-5' : 'translate-x-0'"
              />
            </button>
          </div>
        </div>

        <!-- Sub Row: 声音增强器级别 (低 - 高) -->
        <div class="border-t border-white/[0.05] pt-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
          <div class="min-w-0 pr-2">
            <h4 class="text-[13px] font-medium text-white tracking-wide">
              {{ $t('playback.enhancer_level_title') }}
            </h4>
            <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
              {{ $t('playback.enhancer_level_desc') }}
            </p>
          </div>
          <!-- Slider Component -->
          <div class="w-full sm:w-72 space-y-1.5 shrink-0">
            <div class="relative flex items-center h-4">
              <!-- Tick marks on slider track (5 ticks) -->
              <div class="absolute inset-x-2 flex justify-between pointer-events-none z-0">
                <span
                  v-for="n in 5"
                  :key="n"
                  class="w-[1.5px] h-1.5 rounded-full"
                  :class="n <= settingsStore.soundEnhancerLevel ? 'bg-white/40' : 'bg-white/15'"
                ></span>
              </div>
              <input
                type="range"
                min="1"
                max="5"
                step="1"
                :value="settingsStore.soundEnhancerLevel"
                @input="settingsStore.setSoundEnhancerLevel(Number(($event.target as HTMLInputElement).value))"
                class="apple-slider w-full appearance-none cursor-pointer z-10"
                :style="{
                  background: `linear-gradient(to right, var(--brand-color) 0%, var(--brand-color) ${((settingsStore.soundEnhancerLevel - 1) / 4) * 100}%, var(--slider-track) ${((settingsStore.soundEnhancerLevel - 1) / 4) * 100}%, var(--slider-track) 100%)`
                }"
              />
            </div>
            <!-- Labels below slider -->
            <div class="flex justify-between items-center text-[11px] text-slate-400 select-none px-0.5">
              <span>{{ $t('playback.low') }}</span>
              <span>{{ $t('playback.high') }}</span>
            </div>
          </div>
        </div>
      </section>

      <!-- 3. 音量平衡 Card -->
      <section class="bg-theme-surface border border-theme-border rounded-2xl p-4 sm:p-5 flex items-center justify-between gap-4 shadow-sm">
        <div class="min-w-0 pr-2">
          <h3 class="text-[13px] font-medium text-white tracking-wide">
            {{ $t('playback.sound_check_title') }}
          </h3>
          <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
            {{ $t('playback.sound_check_desc') }}
          </p>
        </div>
        <div class="flex items-center shrink-0">
          <span class="text-xs text-slate-300 mr-2.5 font-normal select-none">
            {{ settingsStore.enableSoundCheck ? $t('playback.toggle_on') : $t('playback.toggle_off') }}
          </span>
          <button
            type="button"
            @click="settingsStore.toggleSoundCheck()"
            class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full transition-colors duration-200 ease-in-out focus:outline-none"
            :class="settingsStore.enableSoundCheck ? 'bg-brand' : 'bg-theme-sliderTrack'"
          >
            <span
              class="pointer-events-none inline-block h-5 w-5 m-0.5 transform rounded-full bg-white shadow-md transition duration-200 ease-in-out"
              :class="settingsStore.enableSoundCheck ? 'translate-x-5' : 'translate-x-0'"
            />
          </button>
        </div>
      </section>

      <!-- 4. 无损音频 Card (包含 流播放 与 下载) -->
      <section class="bg-theme-surface border border-theme-border rounded-2xl p-4 sm:p-5 space-y-4 shadow-sm">
        <!-- Main Row: 无损音频 Toggle -->
        <div class="flex items-center justify-between gap-4">
          <div class="min-w-0 pr-2">
            <h3 class="text-[13px] font-medium text-white tracking-wide">
              {{ $t('playback.lossless_title') }}
            </h3>
            <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
              {{ $t('playback.lossless_desc') }}
            </p>
          </div>
          <div class="flex items-center shrink-0">
            <span class="text-xs text-slate-300 mr-2.5 font-normal select-none">
              {{ settingsStore.enableLossless ? $t('playback.toggle_on') : $t('playback.toggle_off') }}
            </span>
            <button
              type="button"
              @click="settingsStore.toggleLossless()"
              class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full transition-colors duration-200 ease-in-out focus:outline-none"
              :class="settingsStore.enableLossless ? 'bg-brand' : 'bg-theme-sliderTrack'"
            >
              <span
                class="pointer-events-none inline-block h-5 w-5 m-0.5 transform rounded-full bg-white shadow-md transition duration-200 ease-in-out"
                :class="settingsStore.enableLossless ? 'translate-x-5' : 'translate-x-0'"
              />
            </button>
          </div>
        </div>

        <!-- Sub Row: 流播放 Quality Dropdown -->
        <div class="border-t border-white/[0.05] pt-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
          <div class="min-w-0 pr-2">
            <h4 class="text-[13px] font-medium text-white tracking-wide">
              {{ $t('playback.streaming_quality_title') }}
            </h4>
            <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
              {{ $t('playback.streaming_quality_desc') }}
            </p>
          </div>
          <div class="relative shrink-0">
            <div class="px-3.5 py-1.5 rounded-xl bg-[#28282a] border border-white/10 text-xs text-slate-200 flex items-center justify-between gap-3 min-w-[270px] shadow-inner">
              <span class="truncate">{{ getStreamingQualityLabel(settingsStore.streamingAudioQuality) }}</span>
              <ChevronDown :size="13" class="text-slate-400 shrink-0" />
            </div>
            <select
              :value="settingsStore.streamingAudioQuality"
              @change="settingsStore.setStreamingAudioQuality(($event.target as HTMLSelectElement).value as any)"
              class="absolute inset-0 opacity-0 cursor-pointer w-full h-full text-xs"
            >
              <option value="hi-res" class="bg-[#1c1c1e] text-white">{{ $t('playback.quality_hi_res') }}</option>
              <option value="lossless" class="bg-[#1c1c1e] text-white">{{ $t('playback.quality_lossless') }}</option>
              <option value="high" class="bg-[#1c1c1e] text-white">{{ $t('playback.quality_high') }}</option>
              <option value="standard" class="bg-[#1c1c1e] text-white">{{ $t('playback.quality_standard') }}</option>
            </select>
          </div>
        </div>

        <!-- Sub Row: 下载 Quality Dropdown -->
        <div class="border-t border-white/[0.05] pt-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
          <div class="min-w-0 pr-2">
            <h4 class="text-[13px] font-medium text-white tracking-wide">
              {{ $t('playback.download_quality_title') }}
            </h4>
            <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
              {{ $t('playback.download_quality_desc') }}
            </p>
          </div>
          <div class="relative shrink-0">
            <div class="px-3.5 py-1.5 rounded-xl bg-[#28282a] border border-white/10 text-xs text-slate-200 flex items-center justify-between gap-3 min-w-[270px] shadow-inner">
              <span class="truncate">{{ getDownloadQualityLabel(settingsStore.downloadAudioQuality) }}</span>
              <ChevronDown :size="13" class="text-slate-400 shrink-0" />
            </div>
            <select
              :value="settingsStore.downloadAudioQuality"
              @change="settingsStore.setDownloadAudioQuality(($event.target as HTMLSelectElement).value as any)"
              class="absolute inset-0 opacity-0 cursor-pointer w-full h-full text-xs"
            >
              <option value="lossless" class="bg-[#1c1c1e] text-white">{{ $t('playback.quality_lossless') }}</option>
              <option value="hi-res" class="bg-[#1c1c1e] text-white">{{ $t('playback.quality_hi_res') }}</option>
              <option value="high" class="bg-[#1c1c1e] text-white">{{ $t('playback.quality_high') }}</option>
            </select>
          </div>
        </div>
      </section>

      <!-- 5. 杜比全景声 Card -->
      <section class="bg-theme-surface border border-theme-border rounded-2xl p-4 sm:p-5 flex flex-col sm:flex-row sm:items-center justify-between gap-3 shadow-sm">
        <div class="min-w-0 pr-2">
          <h3 class="text-[13px] font-medium text-white tracking-wide">
            {{ $t('playback.dolby_atmos_title') }}
          </h3>
          <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
            {{ $t('playback.dolby_atmos_desc') }}
          </p>
        </div>
        <div class="relative shrink-0">
          <div class="px-3.5 py-1.5 rounded-xl bg-[#28282a] border border-white/10 text-xs text-slate-200 flex items-center justify-between gap-3 min-w-[130px] shadow-inner">
            <span class="truncate">{{ getDolbyAtmosLabel(settingsStore.dolbyAtmos) }}</span>
            <ChevronDown :size="13" class="text-slate-400 shrink-0" />
          </div>
          <select
            :value="settingsStore.dolbyAtmos"
            @change="settingsStore.setDolbyAtmos(($event.target as HTMLSelectElement).value as any)"
            class="absolute inset-0 opacity-0 cursor-pointer w-full h-full text-xs"
          >
            <option value="auto" class="bg-[#1c1c1e] text-white">{{ $t('playback.auto') }}</option>
            <option value="always" class="bg-[#1c1c1e] text-white">{{ $t('playback.always_on') }}</option>
            <option value="off" class="bg-[#1c1c1e] text-white">{{ $t('playback.off') }}</option>
          </select>
        </div>
      </section>

      <!-- 6. 视频播放质量 Card -->
      <section class="bg-theme-surface border border-theme-border rounded-2xl p-4 sm:p-5 flex flex-col sm:flex-row sm:items-center justify-between gap-3 shadow-sm">
        <div class="min-w-0 pr-2">
          <h3 class="text-[13px] font-medium text-white tracking-wide">
            {{ $t('playback.video_streaming_title') }}
          </h3>
          <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
            {{ $t('playback.video_streaming_desc') }}
          </p>
        </div>
        <div class="relative shrink-0">
          <div class="px-3.5 py-1.5 rounded-xl bg-[#28282a] border border-white/10 text-xs text-slate-200 flex items-center justify-between gap-3 min-w-[180px] shadow-inner">
            <span class="truncate">{{ getVideoStreamingLabel(settingsStore.videoStreamingQuality) }}</span>
            <ChevronDown :size="13" class="text-slate-400 shrink-0" />
          </div>
          <select
            :value="settingsStore.videoStreamingQuality"
            @change="settingsStore.setVideoStreamingQuality(($event.target as HTMLSelectElement).value as any)"
            class="absolute inset-0 opacity-0 cursor-pointer w-full h-full text-xs"
          >
            <option value="1080p" class="bg-[#1c1c1e] text-white">{{ $t('playback.video_best_1080p') }}</option>
            <option value="4k" class="bg-[#1c1c1e] text-white">{{ $t('playback.video_4k') }}</option>
            <option value="720p" class="bg-[#1c1c1e] text-white">{{ $t('playback.video_720p') }}</option>
          </select>
        </div>
      </section>

      <!-- 7. 视频下载质量 Card -->
      <section class="bg-theme-surface border border-theme-border rounded-2xl p-4 sm:p-5 flex flex-col sm:flex-row sm:items-center justify-between gap-3 shadow-sm">
        <div class="min-w-0 pr-2">
          <h3 class="text-[13px] font-medium text-white tracking-wide">
            {{ $t('playback.video_download_title') }}
          </h3>
          <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
            {{ $t('playback.video_download_desc') }}
          </p>
        </div>
        <div class="relative shrink-0">
          <div class="px-3.5 py-1.5 rounded-xl bg-[#28282a] border border-white/10 text-xs text-slate-200 flex items-center justify-between gap-3 min-w-[180px] shadow-inner">
            <span class="truncate">{{ getVideoDownloadLabel(settingsStore.videoDownloadQuality) }}</span>
            <ChevronDown :size="13" class="text-slate-400 shrink-0" />
          </div>
          <select
            :value="settingsStore.videoDownloadQuality"
            @change="settingsStore.setVideoDownloadQuality(($event.target as HTMLSelectElement).value as any)"
            class="absolute inset-0 opacity-0 cursor-pointer w-full h-full text-xs"
          >
            <option value="1080p" class="bg-[#1c1c1e] text-white">{{ $t('playback.video_down_1080p') }}</option>
            <option value="4k" class="bg-[#1c1c1e] text-white">{{ $t('playback.video_down_4k') }}</option>
            <option value="sd" class="bg-[#1c1c1e] text-white">{{ $t('playback.video_down_sd') }}</option>
          </select>
        </div>
      </section>

      <!-- 8. 静音时显示自动字幕 Card -->
      <section class="bg-theme-surface border border-theme-border rounded-2xl p-4 sm:p-5 flex items-center justify-between gap-4 shadow-sm">
        <div class="min-w-0 pr-2">
          <h3 class="text-[13px] font-medium text-white tracking-wide">
            {{ $t('playback.auto_captions_title') }}
          </h3>
          <p class="text-[11px] text-slate-400 mt-0.5 leading-relaxed">
            {{ $t('playback.auto_captions_desc') }}
          </p>
        </div>
        <div class="flex items-center shrink-0">
          <span class="text-xs text-slate-300 mr-2.5 font-normal select-none">
            {{ settingsStore.autoCaptionsOnMute ? $t('playback.toggle_on') : $t('playback.toggle_off') }}
          </span>
          <button
            type="button"
            @click="settingsStore.toggleAutoCaptionsOnMute()"
            class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full transition-colors duration-200 ease-in-out focus:outline-none"
            :class="settingsStore.autoCaptionsOnMute ? 'bg-brand' : 'bg-theme-sliderTrack'"
          >
            <span
              class="pointer-events-none inline-block h-5 w-5 m-0.5 transform rounded-full bg-white shadow-md transition duration-200 ease-in-out"
              :class="settingsStore.autoCaptionsOnMute ? 'translate-x-5' : 'translate-x-0'"
            />
          </button>
        </div>
      </section>
    </div>

    <!-- ========================================== -->
    <!-- VIEW 2: 外观与语言 (Appearance & Language) -->
    <!-- ========================================== -->
    <div v-if="activeTab === 'appearance' || activeTab === 'all'" class="space-y-6">
      <!-- Group: 语言与国际化 -->
      <section class="glass-panel rounded-3xl p-6 border border-white/10 space-y-5">
        <div class="flex items-center justify-between">
          <h2 class="text-sm font-semibold text-white flex items-center gap-2">
            <Languages :size="16" class="text-brand" />
            <span>{{ $t('settings.group_language') }}</span>
          </h2>
          <span class="text-[11px] text-brand font-medium bg-brand/10 border border-brand/20 px-2.5 py-0.5 rounded-full font-mono">
            {{ currentLocale === 'zh-CN' ? '简体中文' : (currentLocale === 'zh-TW' ? '繁體中文' : 'English') }}
          </span>
        </div>

        <div class="space-y-3">
          <p class="text-xs text-slate-400">
            {{ $t('settings.language_desc') }}
          </p>

          <!-- Language Selector Cards -->
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <button
              v-for="loc in SUPPORTED_LOCALES"
              :key="loc.id"
              @click="settingsStore.setLanguage(loc.id)"
              class="p-4 rounded-2xl border text-left transition-all duration-200 relative overflow-hidden flex items-center gap-3.5 group active:scale-[0.99]"
              :class="[
                settingsStore.locale === loc.id
                  ? 'bg-brand/15 border-brand/60 shadow-lg shadow-brand/15'
                  : 'bg-white/[0.03] hover:bg-white/[0.07] border-white/10 hover:border-white/20'
              ]"
            >
              <div
                class="w-10 h-10 rounded-xl flex items-center justify-center text-xl shrink-0 shadow-sm border transition-transform group-hover:scale-105"
                :class="settingsStore.locale === loc.id ? 'bg-brand/20 border-brand/40' : 'bg-white/5 border-white/10'"
              >
                {{ loc.flag }}
              </div>

              <div class="min-w-0 flex-1">
                <div class="flex items-center justify-between">
                  <span class="text-xs font-bold text-white tracking-wide">
                    {{ loc.name }}
                  </span>
                  <Check v-if="settingsStore.locale === loc.id" :size="14" class="text-brand" />
                </div>
                <p class="text-[10px] text-slate-400 mt-0.5 font-mono">
                  {{ loc.enName }}
                </p>
                <p class="text-[9px] text-slate-400/80 mt-1 line-clamp-1">
                  {{ loc.desc }}
                </p>
              </div>
            </button>
          </div>
        </div>
      </section>

      <!-- Group: 主题外观 -->
      <section class="glass-panel rounded-3xl p-6 border border-white/10 space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-sm font-semibold text-white flex items-center gap-2">
            <Palette :size="16" class="text-brand" />
            {{ $t('settings.group_appearance') }}
          </h2>
          <span class="text-[11px] text-brand font-medium bg-brand/10 border border-brand/20 px-2.5 py-0.5 rounded-full">
            实时生效
          </span>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3.5">
          <div
            v-for="theme in THEMES"
            :key="theme.id"
            @click="settingsStore.setTheme(theme.id)"
            class="relative rounded-2xl p-3.5 border cursor-pointer transition-all duration-200 group flex flex-col justify-between"
            :class="settingsStore.theme === theme.id ? 'border-brand ring-2 ring-brand/30 bg-white/[0.08] shadow-lg shadow-black/30' : 'border-white/10 bg-white/[0.02] hover:bg-white/[0.06] hover:border-white/20'"
          >
            <div
              class="h-16 w-full rounded-xl p-2.5 flex items-end justify-between border border-white/10 mb-3 relative overflow-hidden shadow-inner"
              :style="{ backgroundColor: theme.previewColor }"
            >
              <div
                class="absolute -top-4 -right-4 w-16 h-16 rounded-full blur-lg opacity-60"
                :style="{ backgroundColor: theme.accentColor }"
              ></div>
              <div class="flex items-center gap-1.5 z-10">
                <span
                  class="w-3.5 h-3.5 rounded-full shadow-sm flex items-center justify-center text-white text-[8px] font-bold"
                  :style="{ backgroundColor: theme.accentColor }"
                >
                  <Music :size="9" />
                </span>
                <span class="text-[11px] font-semibold text-white drop-shadow">
                  {{ theme.name.split('•')[0] }}
                </span>
              </div>
              <span
                v-if="settingsStore.theme === theme.id"
                class="w-5 h-5 rounded-full bg-brand text-white flex items-center justify-center shadow-md z-10 scale-100 transition-transform"
              >
                <Check :size="12" stroke-width="3" />
              </span>
            </div>
            <div>
              <div class="flex items-center justify-between">
                <span class="text-xs font-bold text-white">{{ theme.name }}</span>
                <span class="text-[10px] text-slate-400 font-mono">{{ theme.enName }}</span>
              </div>
              <p class="text-[10px] text-slate-400 mt-1 line-clamp-2 leading-relaxed">
                {{ theme.desc }}
              </p>
            </div>
          </div>
        </div>

        <!-- Ambient Glow & Transitions -->
        <div class="border-t border-white/[0.06] pt-4 space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-xs font-medium text-white flex items-center gap-1.5">
                <Sparkles :size="13" class="text-amber-400" />
                <span>{{ $t('settings.ambient_glow') }}</span>
              </p>
              <p class="text-[11px] text-slate-400 mt-0.5">
                {{ $t('settings.ambient_glow_desc') }}
              </p>
            </div>
            <input
              type="checkbox"
              :checked="settingsStore.enableAmbientGlow"
              @change="settingsStore.toggleAmbientGlow()"
              class="w-4 h-4 rounded accent-brand cursor-pointer"
            />
          </div>

          <div class="border-t border-white/[0.06] pt-3 flex items-center justify-between">
            <div>
              <p class="text-xs font-medium text-white flex items-center gap-1.5">
                <Layers :size="13" class="text-cyan-400" />
                <span>{{ $t('settings.page_anim') }}</span>
              </p>
              <p class="text-[11px] text-slate-400 mt-0.5">
                {{ $t('settings.page_anim_desc') }}
              </p>
            </div>
            <input
              type="checkbox"
              :checked="settingsStore.enablePageAnimation"
              @change="settingsStore.togglePageAnimation()"
              class="w-4 h-4 rounded accent-brand cursor-pointer"
            />
          </div>
        </div>
      </section>
    </div>

    <!-- ========================================== -->
    <!-- VIEW 3: AI 智能与歌词翻译 (AI Translation) -->
    <!-- ========================================== -->
    <div v-if="activeTab === 'ai' || activeTab === 'all'" class="space-y-6">
      <section class="glass-panel rounded-3xl p-6 border border-white/10 space-y-6">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
          <div>
            <h2 class="text-sm font-semibold text-white flex items-center gap-2">
              <Sparkles :size="16" class="text-brand" />
              <span>{{ $t('settings.group_ai') }}</span>
            </h2>
            <p class="text-[11px] text-slate-400 mt-0.5">
              为外文曲目提供意境逐句精翻、歌词韵律对齐与深度背景赏析
            </p>
          </div>

          <!-- Mode Tabs -->
          <div class="flex items-center gap-1 p-1 rounded-xl bg-white/[0.06] border border-white/10 text-xs shrink-0">
            <button
              @click="aiStore.engine = 'cloud'"
              class="px-3 py-1 rounded-lg transition-all font-medium"
              :class="aiStore.engine === 'cloud' ? 'bg-brand text-white font-semibold shadow-sm' : 'text-slate-400 hover:text-white'"
            >
              云端大模型 API
            </button>
            <button
              @click="aiStore.engine = 'local_model'"
              class="px-3 py-1 rounded-lg transition-all font-medium"
              :class="aiStore.engine === 'local_model' ? 'bg-brand text-white font-semibold shadow-sm' : 'text-slate-400 hover:text-white'"
            >
              本地离线小模型
            </button>
            <button
              @click="aiStore.engine = 'ollama'"
              class="px-3 py-1 rounded-lg transition-all font-medium"
              :class="aiStore.engine === 'ollama' ? 'bg-brand text-white font-semibold shadow-sm' : 'text-slate-400 hover:text-white'"
            >
              本地 Ollama
            </button>
          </div>
        </div>

        <!-- Mode 1: Cloud API / Ollama -->
        <div v-if="aiStore.engine === 'cloud' || aiStore.engine === 'ollama'" class="space-y-4">
          <div>
            <label class="block text-[11px] font-semibold text-slate-300 mb-2">
              {{ $t('settings.preset_tip') }}
            </label>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="preset in AI_PRESETS"
                :key="preset.id"
                @click="aiStore.applyPreset(preset)"
                class="px-3 py-1.5 rounded-xl text-xs font-medium border transition-all"
                :class="aiStore.apiBaseUrl === preset.baseUrl ? 'bg-brand/20 border-brand/50 text-white shadow-sm' : 'bg-white/[0.04] border-white/10 text-slate-300 hover:bg-white/[0.08] hover:text-white'"
              >
                {{ preset.name }}
              </button>
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <label class="block text-xs font-semibold text-slate-300 mb-1">
                {{ $t('settings.base_url') }} <span class="text-brand">*</span>
              </label>
              <input
                v-model="aiStore.apiBaseUrl"
                @change="aiStore.saveSettings()"
                type="url"
                placeholder="https://api.deepseek.com/v1"
                class="w-full px-3.5 py-2 rounded-xl bg-white/[0.07] border border-white/10 text-white placeholder-slate-500 text-xs focus:outline-none focus:border-brand/60 font-mono transition-all"
              />
            </div>
            <div>
              <label class="block text-xs font-semibold text-slate-300 mb-1">
                {{ $t('settings.model_name') }} <span class="text-brand">*</span>
              </label>
              <input
                v-model="aiStore.modelName"
                @change="aiStore.saveSettings()"
                type="text"
                placeholder="deepseek-chat"
                class="w-full px-3.5 py-2 rounded-xl bg-white/[0.07] border border-white/10 text-white placeholder-slate-500 text-xs focus:outline-none focus:border-brand/60 font-mono transition-all"
              />
            </div>
          </div>

          <div v-if="aiStore.engine === 'cloud'">
            <label class="block text-xs font-semibold text-slate-300 mb-1">
              {{ $t('settings.api_key') }} <span class="text-brand">*</span>
            </label>
            <div class="relative">
              <input
                v-model="aiStore.apiKey"
                @change="aiStore.saveSettings()"
                :type="showApiKey ? 'text' : 'password'"
                placeholder="sk-..."
                class="w-full px-3.5 py-2 rounded-xl bg-white/[0.07] border border-white/10 text-white placeholder-slate-500 text-xs focus:outline-none focus:border-brand/60 font-mono transition-all pr-16"
              />
              <button
                @click="showApiKey = !showApiKey"
                type="button"
                class="absolute right-2.5 top-1/2 -translate-y-1/2 px-2 py-0.5 rounded text-[10px] text-slate-400 hover:text-white bg-white/5"
              >
                {{ showApiKey ? '隐藏' : '显示' }}
              </button>
            </div>
          </div>

          <div class="flex items-center justify-between pt-1">
            <div>
              <p class="text-xs font-medium text-white">{{ $t('settings.translation_style') }}</p>
              <p class="text-[11px] text-slate-400">选择歌词翻译的侧重点</p>
            </div>
            <div class="flex items-center gap-2">
              <button
                @click="aiStore.translationStyle = 'poetic'; aiStore.saveSettings()"
                class="px-3 py-1 rounded-lg text-xs font-medium border transition-colors"
                :class="aiStore.translationStyle === 'poetic' ? 'bg-brand/20 border-brand text-brand' : 'bg-white/5 border-white/10 text-slate-400'"
              >
                {{ $t('settings.style_poetic') }}
              </button>
              <button
                @click="aiStore.translationStyle = 'literal'; aiStore.saveSettings()"
                class="px-3 py-1 rounded-lg text-xs font-medium border transition-colors"
                :class="aiStore.translationStyle === 'literal' ? 'bg-brand/20 border-brand text-brand' : 'bg-white/5 border-white/10 text-slate-400'"
              >
                {{ $t('settings.style_literal') }}
              </button>
            </div>
          </div>

          <div class="pt-2 flex items-center justify-between gap-3">
            <div class="flex-1">
              <p
                v-if="aiStore.testResult"
                class="text-xs flex items-center gap-1.5"
                :class="aiStore.testResult.success ? 'text-emerald-400' : 'text-rose-400'"
              >
                <CheckCircle2 v-if="aiStore.testResult.success" :size="13" />
                <AlertCircle v-else :size="13" />
                <span>{{ aiStore.testResult.message }}</span>
              </p>
            </div>
            <button
              @click="aiStore.testConnection()"
              :disabled="aiStore.isTesting"
              class="px-4 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-white text-xs font-medium flex items-center gap-2 transition-all border border-white/10 disabled:opacity-50"
            >
              <RefreshCw v-if="aiStore.isTesting" :size="12" class="animate-spin" />
              <span>{{ aiStore.isTesting ? $t('settings.testing') : $t('settings.test_connection') }}</span>
            </button>
          </div>
        </div>

        <!-- Mode 2: Local Lightweight Model -->
        <div v-else class="space-y-4">
          <div class="p-4 rounded-2xl bg-white/[0.03] border border-white/10 space-y-3.5">
            <div class="flex items-start justify-between gap-4">
              <div class="space-y-1">
                <div class="flex items-center gap-2">
                  <h3 class="text-xs font-bold text-white">{{ $t('settings.local_model_title') }}</h3>
                  <span class="px-2 py-0.5 rounded-full text-[9px] font-bold uppercase tracking-wider bg-emerald-500/20 text-emerald-300 border border-emerald-500/30">
                    免 Key / 纯离线
                  </span>
                  <span class="text-[10px] text-slate-400 font-mono">{{ aiStore.localModelSize }}</span>
                </div>
                <p class="text-[11px] text-slate-400 leading-relaxed">
                  {{ $t('settings.local_model_desc') }}
                </p>
              </div>

              <div class="shrink-0">
                <span
                  v-if="aiStore.localModelStatus === 'ready'"
                  class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-emerald-500/20 text-emerald-400 text-xs font-semibold border border-emerald-500/30"
                >
                  <Check :size="12" stroke-width="3" /> 已就绪
                </span>
                <span
                  v-else-if="aiStore.localModelStatus === 'downloading'"
                  class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-amber-500/20 text-amber-400 text-xs font-semibold border border-amber-500/30 animate-pulse"
                >
                  下载中 {{ aiStore.localDownloadProgress }}%
                </span>
                <span
                  v-else
                  class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-white/10 text-slate-400 text-xs font-medium"
                >
                  未下载
                </span>
              </div>
            </div>

            <div v-if="aiStore.localModelStatus === 'downloading'" class="space-y-1.5 pt-1">
              <div class="w-full h-1.5 rounded-full bg-white/10 overflow-hidden">
                <div
                  class="h-full bg-brand transition-all duration-300 rounded-full"
                  :style="{ width: `${aiStore.localDownloadProgress}%` }"
                ></div>
              </div>
            </div>

            <div class="flex items-center justify-between pt-2 border-t border-white/[0.06]">
              <div class="text-[11px] text-slate-400">
                模型存储目录: <span class="font-mono text-slate-300">系统 AppData 缓存目录</span>
              </div>
              <div class="flex items-center gap-2">
                <button
                  v-if="aiStore.localModelStatus === 'ready'"
                  @click="aiStore.deleteLocalModel()"
                  class="px-3 py-1.5 rounded-xl bg-rose-500/10 hover:bg-rose-500/20 text-rose-300 text-xs font-medium border border-rose-500/30 transition-all flex items-center gap-1.5"
                >
                  <Trash2 :size="12" />
                  <span>{{ $t('settings.delete_model') }}</span>
                </button>
                <button
                  v-if="aiStore.localModelStatus === 'not_downloaded'"
                  @click="aiStore.downloadLocalModel()"
                  class="px-4 py-2 rounded-xl bg-brand hover:bg-brand-light text-white text-xs font-semibold shadow-md shadow-brand/20 transition-all flex items-center gap-1.5"
                >
                  <Download :size="13" />
                  <span>{{ $t('settings.download_model') }} (42.8 MB)</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </section>
    </div>

    <!-- ========================================== -->
    <!-- VIEW 4: 缓存位置与存储空间 (Cache & Storage) -->
    <!-- ========================================== -->
    <div v-if="activeTab === 'storage' || activeTab === 'all'" class="space-y-6">
      <section class="glass-panel rounded-3xl p-6 border border-white/10 space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-sm font-semibold text-white flex items-center gap-2">
            <HardDrive :size="16" class="text-brand" />
            <span>{{ $t('settings.group_cache') }}</span>
          </h2>
          <div class="flex items-center gap-2">
            <span v-if="settingsStore.clearCacheMessage" class="text-xs text-emerald-400 flex items-center gap-1 animate-pulse">
              <CheckCircle2 :size="13" />
              {{ settingsStore.clearCacheMessage }}
            </span>
            <button
              @click="settingsStore.fetchCacheInfo()"
              class="p-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white transition-colors"
              title="刷新缓存信息"
            >
              <RefreshCw :size="13" />
            </button>
          </div>
        </div>

        <!-- 缓存目录可视化卡片 -->
        <div class="p-4 rounded-2xl bg-white/[0.04] border border-white/10 space-y-3">
          <div class="flex items-start justify-between gap-3">
            <div class="space-y-1 min-w-0">
              <div class="flex items-center gap-2">
                <span class="text-xs font-semibold text-white">{{ $t('settings.cache_path_title') }}</span>
                <span
                  class="px-2 py-0.5 rounded-full text-[9px] font-semibold"
                  :class="settingsStore.cachePath ? 'bg-brand/20 text-brand border border-brand/30' : 'bg-white/10 text-slate-300'"
                >
                  {{ settingsStore.cachePath ? $t('settings.path_custom') : $t('settings.path_default') }}
                </span>
              </div>
              <p class="text-xs font-mono text-slate-300 break-all select-all bg-black/30 p-2 rounded-xl border border-white/5">
                {{ settingsStore.cacheInfo?.current_path || settingsStore.cachePath || '加载中...' }}
              </p>
            </div>
          </div>

          <div class="flex flex-wrap items-center gap-2 pt-1">
            <button
              @click="settingsStore.chooseCacheFolder()"
              class="px-3.5 py-1.5 rounded-xl bg-brand hover:bg-brand-light text-white text-xs font-medium flex items-center gap-1.5 shadow-md shadow-brand/20 transition-all active:scale-95"
            >
              <FolderPlus :size="14" />
              <span>{{ $t('settings.change_location') }}</span>
            </button>
            <button
              @click="settingsStore.openCacheInExplorer()"
              class="px-3 py-1.5 rounded-xl bg-white/10 hover:bg-white/15 text-white text-xs font-medium flex items-center gap-1.5 transition-all"
            >
              <FolderOpen :size="13" />
              <span>{{ $t('settings.open_folder') }}</span>
            </button>
            <button
              v-if="settingsStore.cachePath"
              @click="settingsStore.resetCachePathToDefault()"
              class="px-3 py-1.5 rounded-xl bg-white/5 hover:bg-white/10 text-slate-300 hover:text-white text-xs font-medium flex items-center gap-1.5 transition-all"
            >
              <RotateCcw :size="12" />
              <span>{{ $t('settings.reset_default') }}</span>
            </button>
          </div>
        </div>

        <!-- 缓存容量上限 -->
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-xs font-semibold text-white">{{ $t('settings.cache_limit') }}</p>
              <p class="text-[11px] text-slate-400">{{ $t('settings.cache_limit_desc') }}</p>
            </div>
            <span class="text-xs font-mono font-bold text-brand bg-brand/10 border border-brand/20 px-2.5 py-1 rounded-xl">
              {{ settingsStore.cacheLimitGb === 0 ? $t('settings.unlimited') : `${settingsStore.cacheLimitGb} GB` }}
            </span>
          </div>

          <div class="grid grid-cols-4 sm:grid-cols-7 gap-2">
            <button
              v-for="limit in [0.5, 1, 2, 5, 10, 20, 0]"
              :key="limit"
              @click="settingsStore.setCacheLimit(limit)"
              class="py-2 px-1 rounded-xl text-xs font-semibold border transition-all text-center"
              :class="settingsStore.cacheLimitGb === limit ? 'bg-brand/20 border-brand text-brand shadow-sm' : 'bg-white/[0.03] border-white/10 text-slate-400 hover:bg-white/[0.06] hover:text-white'"
            >
              {{ limit === 0 ? $t('settings.unlimited') : `${limit} GB` }}
            </button>
          </div>
        </div>

        <!-- 存储空间分析图表 -->
        <div class="space-y-3 pt-2 border-t border-white/[0.06]">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="text-xs font-semibold text-white">{{ $t('settings.storage_analysis') }}</h3>
              <p class="text-[11px] text-slate-400">
                {{ settingsStore.cacheInfo?.file_count || 0 }} {{ $t('settings.cache_files') }}，{{ $t('settings.cache_used') }}
                <span class="font-bold text-white">{{ settingsStore.cacheInfo?.formatted_total || '0 B' }}</span>
              </p>
            </div>
            <button
              @click="settingsStore.clearCache()"
              :disabled="settingsStore.isClearingCache"
              class="px-3.5 py-1.5 rounded-xl bg-rose-500/10 hover:bg-rose-500/20 text-rose-300 text-xs font-semibold border border-rose-500/30 flex items-center gap-1.5 transition-all disabled:opacity-50"
            >
              <Trash2 :size="13" />
              <span>{{ settingsStore.isClearingCache ? '正在清空...' : $t('settings.clear_all_cache') }}</span>
            </button>
          </div>

          <!-- Color Progress Bar -->
          <div class="w-full h-3 rounded-full bg-white/10 overflow-hidden flex shadow-inner">
            <div
              class="h-full bg-brand transition-all duration-500"
              :style="{ width: `${audioPercent}%` }"
              title="音频缓存"
            ></div>
            <div
              class="h-full bg-blue-500 transition-all duration-500"
              :style="{ width: `${coverPercent}%` }"
              title="封面缓存"
            ></div>
            <div
              class="h-full bg-purple-500 transition-all duration-500"
              :style="{ width: `${otherPercent}%` }"
              title="歌词与数据缓存"
            ></div>
          </div>

          <!-- Breakdown Categories -->
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-2 pt-1 text-[11px]">
            <div class="flex items-center gap-2 p-2 rounded-xl bg-white/[0.03] border border-white/5">
              <span class="w-2.5 h-2.5 rounded-full bg-brand shrink-0"></span>
              <div class="min-w-0 flex-1 flex items-center justify-between">
                <span class="text-slate-400 truncate">{{ $t('settings.audio_cache') }}</span>
                <span class="font-mono text-white">{{ settingsStore.cacheInfo?.formatted_audio || '0 B' }}</span>
              </div>
            </div>
            <div class="flex items-center gap-2 p-2 rounded-xl bg-white/[0.03] border border-white/5">
              <span class="w-2.5 h-2.5 rounded-full bg-blue-500 shrink-0"></span>
              <div class="min-w-0 flex-1 flex items-center justify-between">
                <span class="text-slate-400 truncate">{{ $t('settings.cover_cache') }}</span>
                <span class="font-mono text-white">{{ settingsStore.cacheInfo?.formatted_cover || '0 B' }}</span>
              </div>
            </div>
            <div class="flex items-center gap-2 p-2 rounded-xl bg-white/[0.03] border border-white/5">
              <span class="w-2.5 h-2.5 rounded-full bg-purple-500 shrink-0"></span>
              <div class="min-w-0 flex-1 flex items-center justify-between">
                <span class="text-slate-400 truncate">{{ $t('settings.other_cache') }}</span>
                <span class="font-mono text-white">{{ settingsStore.cacheInfo?.formatted_other || '0 B' }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 启动与扫描配置 -->
        <div class="flex items-center justify-between pt-1 border-t border-white/[0.06]">
          <div>
            <p class="text-xs font-medium text-white">{{ $t('settings.auto_scan_startup') }}</p>
            <p class="text-[11px] text-slate-400">{{ $t('settings.auto_scan_startup_desc') }}</p>
          </div>
          <input
            type="checkbox"
            v-model="settingsStore.autoScanStartup"
            class="w-4 h-4 rounded accent-brand cursor-pointer"
          />
        </div>
      </section>
    </div>

    <!-- About Section -->
    <div class="p-6 rounded-3xl glass-card border border-white/10 flex items-center justify-between">
      <div class="space-y-1">
        <h3 class="text-xs font-bold text-white tracking-wide">{{ $t('settings.about_title') }}</h3>
        <p class="text-[11px] text-slate-400">版本 0.1.0 • {{ $t('settings.about_desc') }}</p>
      </div>
      <div class="px-3.5 py-1.5 rounded-full bg-white/10 text-white text-[11px] font-mono border border-white/10">
        Tauri 2 + Vue 3 + Rust
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import {
  Palette,
  Check,
  Sparkles,
  Layers,
  CheckCircle2,
  AlertCircle,
  Download,
  RefreshCw,
  HardDrive,
  FolderOpen,
  FolderPlus,
  RotateCcw,
  Trash2,
  Music,
  Languages,
  SlidersHorizontal,
  ChevronDown,
} from 'lucide-vue-next';
import { useSettingsStore, THEMES } from '@/stores/settings';
import { useAiStore, AI_PRESETS } from '@/stores/ai';
import { SUPPORTED_LOCALES, currentLocale, t } from '@/i18n';

const route = useRoute();
const router = useRouter();
const settingsStore = useSettingsStore();
const aiStore = useAiStore();

type SettingsTab = 'playback' | 'appearance' | 'ai' | 'storage' | 'all';
const activeTab = ref<SettingsTab>((route.query.tab as SettingsTab) || 'playback');

const setTab = (tab: SettingsTab) => {
  activeTab.value = tab;
  router.replace({ query: { ...route.query, tab } }).catch(() => {});
};

const showApiKey = ref(false);

const totalBytes = computed(() => settingsStore.cacheInfo?.total_bytes || 0);
const maxBytes = computed(() => {
  if (settingsStore.cacheLimitGb === 0) return Math.max(totalBytes.value, 1024 * 1024 * 1024);
  return settingsStore.cacheLimitGb * 1024 * 1024 * 1024;
});

const audioPercent = computed(() => {
  if (!maxBytes.value) return 0;
  return Math.min(100, Math.max(2, ((settingsStore.cacheInfo?.audio_bytes || 0) / maxBytes.value) * 100));
});

const coverPercent = computed(() => {
  if (!maxBytes.value) return 0;
  return Math.min(100, Math.max(2, ((settingsStore.cacheInfo?.cover_bytes || 0) / maxBytes.value) * 100));
});

const otherPercent = computed(() => {
  if (!maxBytes.value) return 0;
  return Math.min(100, Math.max(2, ((settingsStore.cacheInfo?.other_bytes || 0) / maxBytes.value) * 100));
});

// Dropdown Display Labels
const getStreamingQualityLabel = (val: string) => {
  switch (val) {
    case 'hi-res':
      return t('playback.quality_hi_res');
    case 'lossless':
      return t('playback.quality_lossless');
    case 'high':
      return t('playback.quality_high');
    case 'standard':
      return t('playback.quality_standard');
    default:
      return val;
  }
};

const getDownloadQualityLabel = (val: string) => {
  switch (val) {
    case 'lossless':
      return t('playback.quality_lossless');
    case 'hi-res':
      return t('playback.quality_hi_res');
    case 'high':
      return t('playback.quality_high');
    default:
      return val;
  }
};

const getDolbyAtmosLabel = (val: string) => {
  switch (val) {
    case 'auto':
      return t('playback.auto');
    case 'always':
      return t('playback.always_on');
    case 'off':
      return t('playback.off');
    default:
      return val;
  }
};

const getVideoStreamingLabel = (val: string) => {
  switch (val) {
    case '1080p':
      return t('playback.video_best_1080p');
    case '4k':
      return t('playback.video_4k');
    case '720p':
      return t('playback.video_720p');
    default:
      return val;
  }
};

const getVideoDownloadLabel = (val: string) => {
  switch (val) {
    case '1080p':
      return t('playback.video_down_1080p');
    case '4k':
      return t('playback.video_down_4k');
    case 'sd':
      return t('playback.video_down_sd');
    default:
      return val;
  }
};
</script>

<style scoped>
.apple-slider {
  -webkit-appearance: none;
  appearance: none;
  outline: none;
  height: 4px;
  border-radius: 9999px;
}

.apple-slider::-webkit-slider-runnable-track {
  height: 4px;
  border-radius: 9999px;
}

.apple-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 16px;
  height: 16px;
  margin-top: -6px;
  border-radius: 50%;
  background: var(--brand-color);
  border: 2.5px solid #ffffff;
  box-shadow: 0 1px 6px rgba(0, 0, 0, 0.45), 0 0 8px var(--brand-glow);
  cursor: pointer;
  transition: transform 0.12s ease;
}

.apple-slider::-webkit-slider-thumb:hover {
  transform: scale(1.15);
}

.apple-slider::-webkit-slider-thumb:active {
  transform: scale(1.22);
}

.apple-slider::-moz-range-thumb {
  width: 17px;
  height: 17px;
  border-radius: 50%;
  background: var(--brand-color);
  border: 2.5px solid #ffffff;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.45);
  cursor: pointer;
}
</style>
