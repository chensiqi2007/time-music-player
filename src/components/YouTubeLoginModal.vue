<template>
  <Transition name="fade">
    <div
      v-if="ytStore.showLoginModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md"
      @click.self="ytStore.closeLoginModal()"
    >
      <div
        class="w-full max-w-md rounded-3xl glass-card border border-white/15 p-7 shadow-2xl relative overflow-hidden flex flex-col items-center text-center animate-in fade-in zoom-in-95 duration-200"
      >
        <!-- Close Button -->
        <button
          @click="ytStore.closeLoginModal()"
          class="absolute top-5 right-5 text-slate-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors"
        >
          <X :size="18" />
        </button>

        <!-- Brand Icon -->
        <div class="w-14 h-14 rounded-2xl bg-gradient-to-tr from-red-600 to-rose-500 p-0.5 shadow-xl shadow-red-500/20 mb-3 flex items-center justify-center">
          <div class="w-full h-full rounded-[14px] bg-[#121318] flex items-center justify-center text-red-500">
            <Youtube :size="28" />
          </div>
        </div>

        <h2 class="text-xl font-bold text-white tracking-tight">登录 YouTube Music</h2>
        <p class="text-xs text-slate-400 mt-1 max-w-xs">
          同步您的 YouTube Music 喜欢歌曲、私人歌单与全球流媒体高保真资产
        </p>

        <!-- Tab Switcher -->
        <div class="flex items-center gap-1.5 p-1 rounded-xl bg-white/[0.06] border border-white/10 my-4 text-xs">
          <button
            @click="ytStore.loginMode = 'device'"
            class="px-3.5 py-1.5 rounded-lg transition-all font-medium"
            :class="ytStore.loginMode === 'device' ? 'bg-red-600 text-white font-semibold shadow-sm' : 'text-slate-400 hover:text-white'"
          >
            Google 扫码与设备码
          </button>
          <button
            @click="ytStore.loginMode = 'cookie'"
            class="px-3.5 py-1.5 rounded-lg transition-all font-medium"
            :class="ytStore.loginMode === 'cookie' ? 'bg-red-600 text-white font-semibold shadow-sm' : 'text-slate-400 hover:text-white'"
          >
            Cookie 凭据
          </button>
        </div>

        <!-- Mode 1: Google OAuth Device Code & QR -->
        <div v-if="ytStore.loginMode === 'device'" class="flex flex-col items-center w-full">
          <!-- QR Code Frame -->
          <div class="relative w-48 h-48 bg-white rounded-2xl p-3 shadow-2xl flex items-center justify-center my-1 border-2 border-red-500/20">
            <img
              v-if="ytStore.qrDataUrl"
              :src="ytStore.qrDataUrl"
              alt="Google Auth QR"
              class="w-full h-full object-contain rounded-lg"
            />

            <!-- Loading Spinner -->
            <div
              v-if="ytStore.deviceStatus === 'idle'"
              class="absolute inset-0 flex flex-col items-center justify-center bg-white/90 rounded-2xl gap-2"
            >
              <RefreshCw class="animate-spin text-red-500" :size="28" />
              <span class="text-xs text-slate-600 font-medium">获取授权设备码...</span>
            </div>

            <!-- Expired State Overlay -->
            <div
              v-if="ytStore.deviceStatus === 'expired'"
              class="absolute inset-0 flex flex-col items-center justify-center bg-black/85 rounded-2xl text-white gap-2 p-3 text-center cursor-pointer backdrop-blur-sm"
              @click="ytStore.generateDeviceCode()"
            >
              <AlertCircle :size="28" class="text-amber-400" />
              <p class="text-xs font-semibold">连接受限或超时</p>
              <span class="text-[11px] text-red-400 underline underline-offset-4">
                点击重新获取
              </span>
            </div>
          </div>

          <!-- User Code Display Pill -->
          <div v-if="ytStore.userCode" class="w-full max-w-xs mt-3 p-3 rounded-2xl bg-white/[0.04] border border-white/10 flex items-center justify-between">
            <div class="text-left">
              <span class="text-[10px] text-slate-400 block uppercase font-mono tracking-wider">Google 授权设备码</span>
              <span class="text-base font-extrabold text-white font-mono tracking-widest">{{ ytStore.userCode }}</span>
            </div>
            <button
              @click="copyCode"
              class="px-2.5 py-1.5 rounded-xl bg-white/10 hover:bg-white/20 text-white text-xs font-medium flex items-center gap-1 transition-colors"
            >
              <Copy :size="12" />
              <span>{{ isCopied ? '已复制' : '复制' }}</span>
            </button>
          </div>

          <!-- Status Message -->
          <p class="text-[11px] text-slate-400 mt-2 px-4 leading-relaxed">
            {{ ytStore.deviceMessage }}
          </p>

          <!-- External Browser Link -->
          <div class="flex items-center gap-2 mt-3">
            <a
              v-if="ytStore.verificationUrl"
              :href="`${ytStore.verificationUrl}?user_code=${ytStore.userCode}`"
              target="_blank"
              class="px-4 py-1.5 rounded-xl bg-white/10 hover:bg-white/15 text-white text-xs font-medium flex items-center gap-1.5 transition-colors border border-white/10"
            >
              <ExternalLink :size="12" />
              <span>打开 Google 网页授权</span>
            </a>
            <button
              @click="ytStore.simulateQuickLogin()"
              class="px-3.5 py-1.5 rounded-xl bg-red-600/20 hover:bg-red-600/30 text-red-300 text-xs font-medium border border-red-500/30 transition-colors"
              title="网络受限时一键直接体验"
            >
              一键快速授权
            </button>
          </div>
        </div>

        <!-- Mode 2: Cookie Login -->
        <div v-else class="w-full space-y-3.5 my-1">
          <!-- 1-Click Clipboard Paste & Login Button -->
          <button
            @click="pasteAndLoginYT"
            :disabled="isPastingYT"
            class="w-full py-2.5 px-3 rounded-xl bg-gradient-to-r from-red-600 to-rose-600 hover:from-red-500 hover:to-rose-500 text-white font-medium text-xs flex items-center justify-center gap-2 shadow-lg shadow-red-600/20 active:scale-[0.98] transition-all cursor-pointer disabled:opacity-60"
          >
            <ClipboardPaste :size="15" />
            <span>{{ isPastingYT ? '正在读取剪贴板...' : '📋 从剪贴板一键读取 Cookie 登录' }}</span>
          </button>

          <div class="text-left">
            <label class="block text-xs font-semibold text-slate-300 mb-1">
              YouTube Music Cookie / SAPISID
            </label>
            <textarea
              v-model="cookieInput"
              rows="3"
              placeholder="在此粘贴包含 SAPISID 或完整的 Cookie 字符串..."
              class="w-full px-3 py-2 rounded-xl bg-white/[0.05] border border-white/10 text-xs text-white placeholder-slate-500 focus:outline-none focus:border-red-500 font-mono resize-none transition-colors"
            ></textarea>
            <p class="text-[10px] text-slate-400 mt-1">
              💡 提示：在网页端访问 music.youtube.com，打开浏览器控制台 F12 复制 Cookie 标头即可完成登录。
            </p>
          </div>

          <div class="flex items-center justify-end gap-2 pt-1">
            <button
              @click="ytStore.simulateQuickLogin()"
              class="px-3.5 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-slate-300 text-xs font-medium transition-colors"
            >
              一键快速登录
            </button>
            <button
              @click="handleCookieSubmit"
              :disabled="!cookieInput.trim()"
              class="px-5 py-2 rounded-xl bg-red-600 hover:bg-red-500 text-white text-xs font-semibold shadow-lg shadow-red-600/20 disabled:opacity-50 transition-all flex items-center gap-1.5"
            >
              <Check :size="14" stroke-width="2.5" />
              <span>登录并同步歌单</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  X,
  RefreshCw,
  AlertCircle,
  Copy,
  ExternalLink,
  Check,
  Youtube,
  ClipboardPaste,
} from 'lucide-vue-next';
import { useYTMusicStore } from '@/stores/ytmusic';

const ytStore = useYTMusicStore();
const cookieInput = ref('');
const isCopied = ref(false);
const isPastingYT = ref(false);

const pasteAndLoginYT = async () => {
  isPastingYT.value = true;
  try {
    const text = await navigator.clipboard.readText();
    if (text && text.trim()) {
      cookieInput.value = text.trim();
      await ytStore.loginWithCookie(cookieInput.value);
    }
  } catch (e) {
    console.warn('Paste failed:', e);
  } finally {
    isPastingYT.value = false;
  }
};

const copyCode = async () => {
  if (!ytStore.userCode) return;
  try {
    await navigator.clipboard.writeText(ytStore.userCode);
    isCopied.value = true;
    setTimeout(() => {
      isCopied.value = false;
    }, 2000);
  } catch (e) {
    console.warn('Copy failed:', e);
  }
};

const handleCookieSubmit = async () => {
  if (!cookieInput.value.trim()) return;
  await ytStore.loginWithCookie(cookieInput.value);
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

