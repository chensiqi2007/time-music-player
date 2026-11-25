<template>
  <Transition name="fade">
    <div
      v-if="passportStore.showLoginModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md"
      @click.self="passportStore.closeLoginModal()"
    >
      <div
        class="w-full max-w-md rounded-3xl glass-card border border-white/15 p-7 shadow-2xl relative overflow-hidden flex flex-col items-center text-center animate-in fade-in zoom-in-95 duration-200"
      >
        <!-- Close Button -->
        <button
          @click="passportStore.closeLoginModal()"
          class="absolute top-5 right-5 text-slate-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors"
        >
          <X :size="18" />
        </button>

        <!-- Brand Icon -->
        <div class="w-14 h-14 rounded-2xl bg-gradient-to-tr from-sky-500 to-indigo-600 p-0.5 shadow-xl shadow-sky-500/20 mb-3 flex items-center justify-center">
          <div class="w-full h-full rounded-[14px] bg-[#121318] flex items-center justify-center text-sky-400">
            <ShieldCheck :size="28" />
          </div>
        </div>

        <h2 class="text-xl font-bold text-white tracking-tight">URL Time Passport</h2>
        <p class="text-xs text-slate-400 mt-1 max-w-xs">
          使用 Time OS 统一认证中心登录 Music 桌面客户端
        </p>

        <!-- Primary Action Area -->
        <div class="w-full mt-5 space-y-3">
          <button
            @click="passportStore.startSsoLogin()"
            :disabled="passportStore.loginStatus === 'exchanging'"
            class="w-full py-3 px-4 rounded-2xl bg-gradient-to-r from-sky-500 to-indigo-600 hover:from-sky-400 hover:to-indigo-500 text-white font-semibold text-sm flex items-center justify-center gap-2 shadow-lg shadow-sky-500/25 active:scale-[0.98] transition-all cursor-pointer disabled:opacity-60"
          >
            <RefreshCw
              v-if="passportStore.loginStatus === 'waiting' || passportStore.loginStatus === 'exchanging'"
              :size="16"
              class="animate-spin"
            />
            <ExternalLink v-else :size="16" />
            <span>
              {{
                passportStore.loginStatus === 'exchanging'
                  ? '正在验证登录凭证...'
                  : passportStore.loginStatus === 'waiting'
                    ? '等待浏览器完成授权...'
                    : '前往 URL Time Passport 登录'
              }}
            </span>
          </button>

          <!-- Status Banner -->
          <div
            v-if="passportStore.statusMessage"
            class="p-3 rounded-2xl text-xs leading-relaxed border text-left flex items-start gap-2"
            :class="
              passportStore.loginStatus === 'error'
                ? 'bg-red-500/10 border-red-500/30 text-red-300'
                : passportStore.loginStatus === 'success'
                  ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-300'
                  : 'bg-sky-500/10 border-sky-500/25 text-sky-200'
            "
          >
            <CheckCircle2 v-if="passportStore.loginStatus === 'success'" :size="15" class="shrink-0 mt-0.5 text-emerald-400" />
            <AlertCircle v-else-if="passportStore.loginStatus === 'error'" :size="15" class="shrink-0 mt-0.5 text-red-400" />
            <RefreshCw v-else :size="14" class="shrink-0 mt-0.5 animate-spin text-sky-400" />
            <span>{{ passportStore.statusMessage }}</span>
          </div>

          <!-- Waiting helpers -->
          <div v-if="passportStore.authUrl" class="flex items-center justify-center gap-2 pt-1">
            <button
              @click="passportStore.reopenBrowser()"
              class="px-3 py-1.5 rounded-xl bg-white/[0.06] hover:bg-white/10 text-slate-200 text-xs flex items-center gap-1.5 border border-white/10 transition-colors"
            >
              <ExternalLink :size="12" />
              <span>重新打开浏览器</span>
            </button>
            <button
              @click="copyAuthUrl"
              class="px-3 py-1.5 rounded-xl bg-white/[0.06] hover:bg-white/10 text-slate-200 text-xs flex items-center gap-1.5 border border-white/10 transition-colors"
            >
              <Copy :size="12" />
              <span>{{ isCopied ? '已复制链接' : '复制登录链接' }}</span>
            </button>
          </div>
        </div>

        <!-- Security footer -->
        <div class="mt-5 pt-3 border-t border-white/[0.06] w-full flex items-center justify-center gap-1.5 text-[10px] text-slate-400">
          <ShieldCheck :size="12" class="text-sky-400" />
          <span>URL Time Passport SSO • 本地环回回调 • 安全加密存储</span>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  X,
  ShieldCheck,
  ExternalLink,
  RefreshCw,
  Copy,
  AlertCircle,
  CheckCircle2,
} from 'lucide-vue-next';
import { usePassportStore } from '@/stores/passport';

const passportStore = usePassportStore();
const isCopied = ref(false);

const copyAuthUrl = async () => {
  if (!passportStore.authUrl) return;
  try {
    await navigator.clipboard.writeText(passportStore.authUrl);
    isCopied.value = true;
    setTimeout(() => {
      isCopied.value = false;
    }, 1800);
  } catch {
    // Ignored
  }
};
</script>
