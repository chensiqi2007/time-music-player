<template>
  <div
    v-if="neteaseStore.showLoginModal"
    class="fixed inset-0 z-50 bg-black/75 backdrop-blur-xl flex items-center justify-center p-4 select-none transition-all duration-300"
  >
    <div
      class="glass-panel w-full max-w-sm rounded-3xl p-6 border border-white/15 space-y-4 shadow-2xl relative flex flex-col items-center text-center"
    >
      <!-- Close Button -->
      <button
        @click="neteaseStore.closeLoginModal"
        class="absolute top-4 right-4 p-1.5 rounded-full text-slate-400 hover:text-white hover:bg-white/10 transition-colors"
      >
        <X :size="16" />
      </button>

      <!-- App Header -->
      <div class="space-y-1 flex flex-col items-center">
        <div class="w-11 h-11 rounded-2xl bg-gradient-to-tr from-brand to-rose-600 flex items-center justify-center shadow-lg shadow-brand/30 mb-0.5">
          <Music2 :size="22" class="text-white" />
        </div>
        <h3 class="text-base font-bold text-white tracking-tight">登录网易云音乐</h3>
        <p class="text-xs text-slate-400">同步您的心动歌单、红心收藏与资产</p>
      </div>

      <!-- Mode Switcher Tabs -->
      <div class="flex items-center gap-1.5 p-1 rounded-xl bg-white/[0.06] border border-white/10 text-xs w-full max-w-[240px]">
        <button
          @click="neteaseStore.loginMode = 'qr'"
          class="flex-1 py-1 rounded-lg font-medium transition-all"
          :class="neteaseStore.loginMode === 'qr' ? 'bg-brand text-white font-semibold shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          扫码登录
        </button>
        <button
          @click="neteaseStore.loginMode = 'cookie'"
          class="flex-1 py-1 rounded-lg font-medium transition-all"
          :class="neteaseStore.loginMode === 'cookie' ? 'bg-brand text-white font-semibold shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Cookie 登录
        </button>
      </div>

      <!-- MODE 1: QR Code Login -->
      <template v-if="neteaseStore.loginMode === 'qr'">
        <div class="relative w-48 h-48 p-2.5 bg-white rounded-2xl shadow-xl flex items-center justify-center overflow-hidden">
          <img
            v-if="neteaseStore.qrDataUrl"
            :src="neteaseStore.qrDataUrl"
            alt="QR Code"
            class="w-full h-full object-contain"
          />
          <div v-else class="flex flex-col items-center justify-center space-y-2 text-slate-700">
            <RefreshCw :size="24" class="animate-spin text-brand" />
            <span class="text-xs font-medium">生成二维码中...</span>
          </div>

          <!-- Scanned status overlay with manual trigger button -->
          <div
            v-if="neteaseStore.qrStatus === 'scanned'"
            class="absolute inset-0 bg-white/95 flex flex-col items-center justify-center p-3 space-y-2 text-slate-800"
          >
            <CheckCircle2 :size="34" class="text-emerald-500 animate-bounce" />
            <span class="text-xs font-bold text-slate-900">手机已扫描成功</span>
            <span class="text-[11px] text-slate-600">请在手机上网易云音乐点击确认</span>

            <!-- Manual Confirm Button in case poll delay -->
            <button
              @click="neteaseStore.manualConfirmCheck()"
              :disabled="neteaseStore.isManualChecking"
              class="px-4 py-1.5 rounded-full bg-brand hover:bg-brand-light text-white text-xs font-semibold shadow-md active:scale-95 transition-all flex items-center gap-1.5 disabled:opacity-60 cursor-pointer"
            >
              <RefreshCw :size="12" :class="neteaseStore.isManualChecking ? 'animate-spin' : ''" />
              <span>{{ neteaseStore.isManualChecking ? '正在确认手机授权...' : '已点确认？点此完成' }}</span>
            </button>

            <!-- Feedback text -->
            <p v-if="neteaseStore.manualFeedback" class="text-[10px] text-amber-700 font-medium px-2 leading-tight">
              {{ neteaseStore.manualFeedback }}
            </p>

            <button
              @click="neteaseStore.generateQrCode()"
              class="text-[10px] text-slate-400 hover:text-brand underline pt-0.5 cursor-pointer"
            >
              重新生成二维码
            </button>
          </div>

          <!-- Expired overlay -->
          <div
            v-if="neteaseStore.qrStatus === 'expired'"
            @click="neteaseStore.generateQrCode"
            class="absolute inset-0 bg-black/80 flex flex-col items-center justify-center space-y-2 text-white cursor-pointer hover:bg-black/85 transition-colors"
          >
            <RotateCcw :size="26" class="text-brand" />
            <span class="text-xs font-medium">二维码已失效</span>
            <span class="text-[11px] text-slate-300 underline">点击重新生成</span>
          </div>
        </div>

        <!-- Status hint -->
        <div class="flex items-center gap-2 text-xs text-slate-300 font-medium">
          <Smartphone :size="14" class="text-brand" />
          <span>{{ neteaseStore.qrMessage }}</span>
        </div>
      </template>

      <!-- MODE 2: Direct Cookie Login -->
      <template v-else>
        <div class="w-full space-y-3 text-left">
          <!-- 1-Click Clipboard Paste & Login Button -->
          <button
            @click="pasteFromClipboardAndLogin"
            :disabled="isPasting"
            class="w-full py-2.5 px-3 rounded-xl bg-gradient-to-r from-brand to-rose-600 hover:from-brand-light hover:to-rose-500 text-white font-medium text-xs flex items-center justify-center gap-2 shadow-lg shadow-brand/20 active:scale-[0.98] transition-all cursor-pointer disabled:opacity-60"
          >
            <ClipboardPaste :size="15" />
            <span>{{ isPasting ? '正在读取剪贴板...' : '📋 从剪贴板一键读取 Cookie 登录' }}</span>
          </button>

          <p v-if="pasteFeedback" class="text-[11px] text-amber-400 font-medium px-1">
            {{ pasteFeedback }}
          </p>

          <div class="space-y-1">
            <label class="text-[11px] font-semibold text-slate-300">网易云 MUSIC_U 或完整 Cookie：</label>
            <textarea
              v-model="cookieInput"
              rows="2"
              placeholder="粘贴包含 MUSIC_U=xxxx 的 Cookie 字符串..."
              class="w-full p-2.5 text-xs bg-black/40 text-slate-200 placeholder-slate-500 rounded-xl border border-white/10 focus:border-brand focus:outline-none transition-all font-mono resize-none"
            ></textarea>
          </div>

          <div class="p-2.5 rounded-xl bg-white/[0.04] border border-white/[0.06] text-[10px] text-slate-400 space-y-1">
            <p class="font-semibold text-slate-300">💡 如何获取 Cookie：</p>
            <p>1. 在电脑浏览器打开 music.163.com 并登录账号</p>
            <p>2. 按 F12 打开开发者工具 $\rightarrow$ 应用程序 (Application) $\rightarrow$ 存储 $\rightarrow$ Cookie</p>
            <p>3. 复制 <code class="text-brand">MUSIC_U</code> 这一项的值，直接点上方一键读取即可</p>
          </div>

          <button
            @click="handleCookieLogin"
            :disabled="!cookieInput.trim()"
            class="w-full py-2 rounded-xl bg-white/10 hover:bg-white/15 text-white text-xs font-semibold border border-white/10 transition-all disabled:opacity-40"
          >
            确认登录
          </button>
        </div>
      </template>

      <!-- Security guarantee -->
      <div class="pt-2 border-t border-white/[0.06] w-full flex items-center justify-center gap-1.5 text-[10px] text-slate-400">
        <ShieldCheck :size="12" class="text-emerald-400" />
        <span>Time OS 官方协议桥接 • 本地加密存储 • 零隐私风险</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { X, Music2, RefreshCw, CheckCircle2, RotateCcw, Smartphone, ShieldCheck, ClipboardPaste } from 'lucide-vue-next';
import { useNeteaseStore } from '@/stores/netease';

const neteaseStore = useNeteaseStore();
const cookieInput = ref('');
const isPasting = ref(false);
const pasteFeedback = ref('');

const pasteFromClipboardAndLogin = async () => {
  isPasting.value = true;
  pasteFeedback.value = '';
  try {
    const text = await navigator.clipboard.readText();
    if (!text || !text.trim()) {
      pasteFeedback.value = '剪贴板为空，请先在浏览器复制 MUSIC_U 或 Cookie';
      isPasting.value = false;
      return;
    }
    let formatted = text.trim();
    if (!formatted.includes('=')) {
      formatted = `MUSIC_U=${formatted}`;
    }
    cookieInput.value = formatted;
    await neteaseStore.loginWithCookie(formatted);
  } catch (err) {
    pasteFeedback.value = '读取剪贴板受阻，请直接在文本框粘贴后点击确认';
  } finally {
    isPasting.value = false;
  }
};

const handleCookieLogin = async () => {
  if (!cookieInput.value.trim()) return;
  let formatted = cookieInput.value.trim();
  if (!formatted.includes('=')) {
    formatted = `MUSIC_U=${formatted}`;
  }
  await neteaseStore.loginWithCookie(formatted);
};
</script>
