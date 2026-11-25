<template>
  <Transition name="fade">
    <div
      v-if="qqStore.showLoginModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md"
      @click.self="qqStore.closeLoginModal()"
    >
      <div
        class="w-full max-w-md rounded-3xl glass-card border border-white/15 p-7 shadow-2xl relative overflow-hidden flex flex-col items-center text-center animate-in fade-in zoom-in-95 duration-200"
      >
        <!-- Close Button -->
        <button
          @click="qqStore.closeLoginModal()"
          class="absolute top-5 right-5 text-slate-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors"
        >
          <X :size="18" />
        </button>

        <!-- Brand Icon -->
        <div class="w-14 h-14 rounded-2xl bg-gradient-to-tr from-emerald-600 to-teal-400 p-0.5 shadow-xl shadow-emerald-500/20 mb-4 flex items-center justify-center">
          <div class="w-full h-full rounded-[14px] bg-[#121318] flex items-center justify-center text-emerald-400">
            <Music2 :size="28" />
          </div>
        </div>

        <h2 class="text-xl font-bold text-white tracking-tight">扫码登录 QQ 音乐</h2>
        <p class="text-xs text-slate-400 mt-1 max-w-xs">
          同步您的 QQ 音乐红心歌单、自建收藏与高保真资产
        </p>

        <!-- Tab Switcher: QR Code vs Cookie -->
        <div class="flex items-center gap-1.5 p-1 rounded-xl bg-white/[0.06] border border-white/10 my-4 text-xs">
          <button
            @click="qqStore.loginMode = 'qr'"
            class="px-4 py-1.5 rounded-lg transition-all font-medium"
            :class="qqStore.loginMode === 'qr' ? 'bg-emerald-500 text-white font-semibold shadow-sm' : 'text-slate-400 hover:text-white'"
          >
            手机扫码登录
          </button>
          <button
            @click="qqStore.loginMode = 'cookie'"
            class="px-4 py-1.5 rounded-lg transition-all font-medium"
            :class="qqStore.loginMode === 'cookie' ? 'bg-emerald-500 text-white font-semibold shadow-sm' : 'text-slate-400 hover:text-white'"
          >
            Cookie 凭证登录
          </button>
        </div>

        <!-- Mode 1: QR Code Scanner -->
        <div v-if="qqStore.loginMode === 'qr'" class="flex flex-col items-center">
          <!-- QR Code Frame -->
          <div class="relative w-52 h-52 bg-white rounded-2xl p-3 shadow-2xl flex items-center justify-center my-2 border-2 border-emerald-500/20">
            <img
              v-if="qqStore.qrDataUrl"
              :src="qqStore.qrDataUrl"
              alt="QQ Login QR"
              class="w-full h-full object-contain rounded-lg"
              :class="qqStore.qrStatus === 'scanned' ? 'opacity-25 blur-[1px]' : ''"
            />

            <!-- Loading Spinner -->
            <div
              v-if="qqStore.qrStatus === 'idle'"
              class="absolute inset-0 flex flex-col items-center justify-center bg-white/90 rounded-2xl gap-2"
            >
              <RefreshCw class="animate-spin text-emerald-500" :size="28" />
              <span class="text-xs text-slate-600 font-medium">生成授权二维码...</span>
            </div>

            <!-- Scanned State Overlay -->
            <div
              v-if="qqStore.qrStatus === 'scanned'"
              class="absolute inset-0 flex flex-col items-center justify-center bg-emerald-950/80 rounded-2xl text-emerald-300 gap-2 p-4 text-center backdrop-blur-sm"
            >
              <div class="w-12 h-12 rounded-full bg-emerald-500/20 flex items-center justify-center border border-emerald-400/40">
                <CheckCircle2 :size="26" class="text-emerald-400" />
              </div>
              <p class="text-sm font-bold text-white">已扫描</p>
              <p class="text-[11px] text-slate-300">请在手机上点击确认授权登录</p>
            </div>

            <!-- Expired State Overlay -->
            <div
              v-if="qqStore.qrStatus === 'expired'"
              class="absolute inset-0 flex flex-col items-center justify-center bg-black/85 rounded-2xl text-white gap-2 p-4 cursor-pointer backdrop-blur-sm"
              @click="qqStore.generateQrCode()"
            >
              <AlertCircle :size="30" class="text-amber-400" />
              <p class="text-xs font-semibold">二维码已失效</p>
              <span class="text-[11px] text-emerald-400 flex items-center gap-1 underline underline-offset-4">
                点击刷新
              </span>
            </div>
          </div>

          <!-- Status Message & Actions -->
          <div class="flex items-center gap-2 mt-2 text-xs">
            <Smartphone :size="14" class="text-emerald-400" />
            <span class="text-slate-300">{{ qqStore.qrMessage }}</span>
          </div>

          <div class="flex items-center gap-2 mt-3">
            <button
              v-if="qqStore.qrStatus === 'scanned'"
              @click="qqStore.manualCheck()"
              class="px-4 py-1.5 rounded-full bg-emerald-500 hover:bg-emerald-600 text-white text-xs font-semibold shadow-md shadow-emerald-500/30 transition-all flex items-center gap-1.5 animate-pulse"
            >
              <Check :size="13" />
              手机已点确认？点此完成
            </button>
            <button
              @click="qqStore.generateQrCode()"
              class="text-xs text-slate-400 hover:text-emerald-400 flex items-center gap-1 transition-colors px-2 py-1"
            >
              <RefreshCw :size="12" />
              刷新二维码
            </button>
          </div>
        </div>

        <!-- Mode 2: Cookie Login -->
        <div v-else class="w-full space-y-3 my-2 text-left">
          <!-- 1-Click Clipboard Paste & Login Button -->
          <button
            @click="pasteAndLoginQQ"
            :disabled="isPastingQQ"
            class="w-full py-2.5 px-3 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 text-white font-medium text-xs flex items-center justify-center gap-2 shadow-lg shadow-emerald-500/20 active:scale-[0.98] transition-all cursor-pointer disabled:opacity-60"
          >
            <ClipboardPaste :size="15" />
            <span>{{ isPastingQQ ? '正在读取剪贴板...' : '📋 从剪贴板一键读取 Cookie 登录' }}</span>
          </button>

          <div>
            <label class="block text-xs font-semibold text-slate-300 mb-1">
              QQ 号码 (UIN) <span class="text-slate-500 text-[10px]">(可选，自动解析)</span>
            </label>
            <input
              v-model="manualUin"
              type="text"
              placeholder="例如：10001"
              class="w-full px-3 py-2 rounded-xl bg-white/[0.07] border border-white/10 text-white placeholder-slate-500 text-xs focus:outline-none focus:border-emerald-500 font-mono"
            />
          </div>

          <div>
            <label class="block text-xs font-semibold text-slate-300 mb-1">
              QQ 音乐 Cookie / Key <span class="text-emerald-400">*</span>
            </label>
            <textarea
              v-model="manualCookie"
              rows="3"
              placeholder="请粘贴浏览器提取的 uin=...; qm_keyst=... 或完整 Cookie"
              class="w-full px-3 py-2 rounded-xl bg-white/[0.07] border border-white/10 text-white placeholder-slate-500 text-xs focus:outline-none focus:border-emerald-500 font-mono resize-none leading-relaxed"
            ></textarea>
          </div>

          <button
            @click="handleCookieSubmit"
            :disabled="!manualCookie.trim() || isSubmittingCookie"
            class="w-full py-2 rounded-xl bg-white/10 hover:bg-white/15 text-white text-xs font-semibold border border-white/10 transition-all disabled:opacity-40 flex items-center justify-center gap-2"
          >
            <KeyRound :size="14" />
            <span>{{ isSubmittingCookie ? '验证登录中...' : '确认使用 Cookie 登录' }}</span>
          </button>
        </div>

        <!-- Security Disclaimer -->
        <div class="mt-4 pt-3 border-t border-white/10 w-full flex items-center justify-center gap-1.5 text-[10px] text-slate-500">
          <ShieldCheck :size="13" class="text-emerald-400" />
          <span>官方协议桥接 • 不保存用户密码 • 零安全风险</span>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  X,
  Music2,
  RefreshCw,
  Smartphone,
  CheckCircle2,
  AlertCircle,
  KeyRound,
  ShieldCheck,
  Check,
  ClipboardPaste,
} from 'lucide-vue-next';
import { useQQMusicStore } from '@/stores/qqmusic';

const qqStore = useQQMusicStore();
const manualUin = ref('');
const manualCookie = ref('');
const isSubmittingCookie = ref(false);
const isPastingQQ = ref(false);

const pasteAndLoginQQ = async () => {
  isPastingQQ.value = true;
  try {
    const text = await navigator.clipboard.readText();
    if (text && text.trim()) {
      manualCookie.value = text.trim();
      await qqStore.loginWithCookie(manualCookie.value.trim(), manualUin.value.trim());
    }
  } catch (err) {
    console.warn('Paste failed', err);
  } finally {
    isPastingQQ.value = false;
  }
};

const handleCookieSubmit = async () => {
  if (!manualCookie.value.trim() || isSubmittingCookie.value) return;
  isSubmittingCookie.value = true;
  try {
    await qqStore.loginWithCookie(manualCookie.value.trim(), manualUin.value.trim());
    manualCookie.value = '';
    manualUin.value = '';
  } finally {
    isSubmittingCookie.value = false;
  }
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

