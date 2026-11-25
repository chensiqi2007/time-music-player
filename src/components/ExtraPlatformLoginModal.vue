<template>
  <Transition name="fade">
    <div
      v-if="extraStore.showLoginModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md"
      @click.self="extraStore.closeLoginModal()"
    >
      <div
        class="w-full max-w-md rounded-3xl liquid-glass-floating p-7 relative overflow-hidden flex flex-col items-center text-center animate-in fade-in zoom-in-95 duration-200"
      >
        <button
          @click="extraStore.closeLoginModal()"
          class="absolute top-5 right-5 text-slate-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors"
        >
          <X :size="18" />
        </button>

        <!-- Platform Switcher Tabs -->
        <div class="grid grid-cols-3 gap-1 p-1 rounded-2xl bg-black/30 border border-white/10 w-full mb-4 text-xs">
          <button
            v-for="id in (['kuwo', 'kugou', 'qishui'] as ExtraPlatformId[])"
            :key="id"
            @click="extraStore.activeTab = id"
            class="py-1.5 rounded-xl font-medium transition-all flex items-center justify-center gap-1.5"
            :class="extraStore.activeTab === id ? 'bg-brand text-white font-semibold shadow-sm' : 'text-slate-300 hover:text-white'"
          >
            <span>{{ EXTRA_PLATFORMS[id].name }}</span>
            <span
              v-if="extraStore.accounts[id].isLoggedIn"
              class="w-1.5 h-1.5 rounded-full bg-emerald-400"
            ></span>
          </button>
        </div>

        <h2 class="text-xl font-bold text-white tracking-tight">
          登录{{ currentMeta.name }}
        </h2>
        <p class="text-xs text-slate-400 mt-1 max-w-xs">
          支持扫码、手机号验证码与账号密码登录，自动同步会员权限
        </p>

        <!-- Login Mode Switcher -->
        <div class="grid grid-cols-3 gap-1 p-1 rounded-xl bg-white/[0.06] border border-white/10 w-full my-4 text-xs">
          <button
            @click="loginMode = 'window'"
            class="py-1.5 rounded-lg font-medium transition-all flex items-center justify-center gap-1"
            :class="loginMode === 'window' ? 'bg-white/15 text-white font-semibold' : 'text-slate-400 hover:text-white'"
          >
            <QrCode :size="13" />
            <span>扫码登录</span>
          </button>
          <button
            @click="loginMode = 'account'"
            class="py-1.5 rounded-lg font-medium transition-all flex items-center justify-center gap-1"
            :class="loginMode === 'account' ? 'bg-white/15 text-white font-semibold' : 'text-slate-400 hover:text-white'"
          >
            <Smartphone :size="13" />
            <span>手机 / 密码</span>
          </button>
          <button
            @click="loginMode = 'cookie'"
            class="py-1.5 rounded-lg font-medium transition-all flex items-center justify-center gap-1"
            :class="loginMode === 'cookie' ? 'bg-white/15 text-white font-semibold' : 'text-slate-400 hover:text-white'"
          >
            <KeyRound :size="13" />
            <span>Cookie</span>
          </button>
        </div>

        <!-- Logged-in Banner -->
        <div
          v-if="currentAccount.isLoggedIn"
          class="w-full mb-3 p-3 rounded-2xl bg-emerald-500/10 border border-emerald-500/30 flex items-center justify-between text-left"
        >
          <div>
            <p class="text-xs font-semibold text-emerald-300">{{ currentAccount.nickname }}</p>
            <p class="text-[10px] text-emerald-400/80">{{ currentMeta.badgeText }}</p>
          </div>
          <button
            @click="extraStore.logout(extraStore.activeTab)"
            class="px-3 py-1 rounded-lg bg-red-500/20 hover:bg-red-500 text-red-300 hover:text-white text-[11px] font-medium transition-colors"
          >
            退出
          </button>
        </div>

        <!-- MODE 1: Built-in Official Window QR / Scan Login -->
        <div v-if="loginMode === 'window'" class="w-full space-y-3 text-left">
          <div class="p-4 rounded-2xl bg-white/[0.04] border border-white/10 space-y-3 text-center">
            <div class="w-14 h-14 mx-auto rounded-2xl bg-brand/15 border border-brand/30 text-brand flex items-center justify-center">
              <QrCode :size="28" />
            </div>
            <div class="space-y-1">
              <p class="text-xs font-semibold text-white">内置官方窗口扫码 / 一键同步</p>
              <p class="text-[11px] text-slate-400 leading-relaxed">
                点击下方打开{{ currentMeta.name }}官方登录窗口，使用手机 App 扫码或账号登录，完成后点「同步登录状态」即可自动提取加密凭证，无需手动复制 Cookie。
              </p>
            </div>
          </div>

          <button
            @click="launchOfficialWindow"
            class="w-full py-2.5 px-4 rounded-xl bg-white/10 hover:bg-white/15 text-white text-xs font-semibold flex items-center justify-center gap-2 border border-white/15 transition-all"
          >
            <ExternalLink :size="14" />
            <span>{{ windowOpened ? `重新聚焦${currentMeta.name}登录窗口` : `1. 打开${currentMeta.name}官方登录窗口` }}</span>
          </button>

          <button
            @click="syncWindowSession"
            :disabled="isSyncing"
            class="w-full py-2.5 px-4 rounded-xl bg-gradient-to-r from-brand to-brand-light hover:brightness-110 text-white text-xs font-semibold flex items-center justify-center gap-2 shadow-lg shadow-brand/25 transition-all disabled:opacity-50"
          >
            <RefreshCw :size="14" :class="isSyncing ? 'animate-spin' : ''" />
            <span>{{ isSyncing ? '正在读取登录状态...' : '2. 已扫码/登录完成？点此自动同步' }}</span>
          </button>
        </div>

        <!-- MODE 2: Phone / Username & Password -->
        <div v-else-if="loginMode === 'account'" class="w-full space-y-3 text-left">
          <div class="space-y-2">
            <label class="text-[11px] font-medium text-slate-300 block">手机号 / 用户名</label>
            <input
              v-model="accountInput"
              type="text"
              placeholder="输入手机号或平台账号"
              class="w-full px-3 py-2 text-xs bg-black/40 text-slate-200 placeholder-slate-500 rounded-xl border border-white/10 focus:border-brand focus:outline-none"
            />
          </div>

          <div class="space-y-2">
            <label class="text-[11px] font-medium text-slate-300 block">登录密码（或留空使用短信验证码）</label>
            <input
              v-model="passwordInput"
              type="password"
              placeholder="输入密码（若使用短信验证可留空）"
              class="w-full px-3 py-2 text-xs bg-black/40 text-slate-200 placeholder-slate-500 rounded-xl border border-white/10 focus:border-brand focus:outline-none"
            />
          </div>

          <p class="text-[10px] text-slate-400 leading-relaxed">
            出于平台滑块与短信风控保护，点击登录将唤醒{{ currentMeta.name }}官方安全认证窗口完成验证，随后点此同步即可生效。
          </p>

          <div class="grid grid-cols-2 gap-2 pt-1">
            <button
              @click="launchOfficialWindow"
              :disabled="!accountInput.trim() && !windowOpened"
              class="py-2.5 px-3 rounded-xl bg-brand hover:bg-brand-light text-white text-xs font-semibold flex items-center justify-center gap-1.5 transition-all disabled:opacity-40"
            >
              <Smartphone :size="14" />
              <span>前往验证登录</span>
            </button>
            <button
              @click="syncWindowSession"
              :disabled="isSyncing"
              class="py-2.5 px-3 rounded-xl bg-white/10 hover:bg-white/15 text-white text-xs font-semibold flex items-center justify-center gap-1.5 border border-white/10 transition-all disabled:opacity-50"
            >
              <RefreshCw :size="13" :class="isSyncing ? 'animate-spin' : ''" />
              <span>同步登录状态</span>
            </button>
          </div>
        </div>

        <!-- MODE 3: Cookie Credential Fallback -->
        <div v-else class="w-full space-y-3 text-left">
          <div class="grid grid-cols-2 gap-2">
            <button
              @click="openOfficialSite"
              class="py-2 px-3 rounded-xl bg-white/10 hover:bg-white/15 text-white text-xs font-medium flex items-center justify-center gap-1.5 border border-white/10 transition-colors"
            >
              <ExternalLink :size="13" />
              <span>浏览器打开官网</span>
            </button>
            <button
              @click="pasteFromClipboardAndLogin"
              class="py-2 px-3 rounded-xl bg-gradient-to-r from-brand to-brand-light hover:brightness-110 text-white text-xs font-semibold flex items-center justify-center gap-1.5 shadow-md shadow-brand/25 transition-all"
            >
              <ClipboardPaste :size="13" />
              <span>剪贴板一键登录</span>
            </button>
          </div>

          <div class="space-y-1.5">
            <input
              v-model="nicknameInput"
              type="text"
              placeholder="账号备注昵称（选填）"
              class="w-full px-3 py-2 text-xs bg-black/40 text-slate-200 placeholder-slate-500 rounded-xl border border-white/10 focus:border-brand focus:outline-none"
            />
            <textarea
              v-model="cookieInput"
              rows="2"
              placeholder="粘贴从官网浏览器复制的完整 Cookie..."
              class="w-full p-2.5 text-xs bg-black/40 text-slate-200 placeholder-slate-500 rounded-xl border border-white/10 focus:border-brand focus:outline-none font-mono resize-none"
            ></textarea>
          </div>

          <div class="p-2.5 rounded-xl bg-white/[0.04] border border-white/[0.06] text-[10px] text-slate-400 leading-relaxed">
            💡 {{ currentMeta.cookieHint }}
          </div>

          <button
            @click="handleSubmit"
            :disabled="!cookieInput.trim()"
            class="w-full py-2 rounded-xl bg-brand hover:bg-brand-light text-white text-xs font-semibold transition-all disabled:opacity-40"
          >
            保存并启用 {{ currentMeta.name }} 会员凭证
          </button>
        </div>

        <!-- Feedback Alert -->
        <p v-if="feedback" class="w-full mt-3 text-[11px] text-amber-300 bg-amber-500/10 border border-amber-500/25 rounded-xl px-3 py-2 text-left">
          {{ feedback }}
        </p>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import {
  X,
  ExternalLink,
  ClipboardPaste,
  QrCode,
  Smartphone,
  KeyRound,
  RefreshCw,
} from 'lucide-vue-next';
import { openUrl } from '@tauri-apps/plugin-opener';
import { useExtraPlatformsStore, EXTRA_PLATFORMS, type ExtraPlatformId } from '@/stores/extraPlatforms';

const extraStore = useExtraPlatformsStore();
const loginMode = ref<'window' | 'account' | 'cookie'>('window');
const accountInput = ref('');
const passwordInput = ref('');
const cookieInput = ref('');
const nicknameInput = ref('');
const feedback = ref('');
const windowOpened = ref(false);
const isSyncing = ref(false);

const currentMeta = computed(() => EXTRA_PLATFORMS[extraStore.activeTab]);
const currentAccount = computed(() => extraStore.accounts[extraStore.activeTab]);

watch(
  () => [extraStore.showLoginModal, extraStore.activeTab],
  () => {
    feedback.value = '';
    windowOpened.value = false;
    cookieInput.value = extraStore.accounts[extraStore.activeTab].cookie || '';
    nicknameInput.value = extraStore.accounts[extraStore.activeTab].nickname || '';
  }
);

const launchOfficialWindow = async () => {
  feedback.value = '';
  try {
    await extraStore.openOfficialWindow(extraStore.activeTab);
    windowOpened.value = true;
    feedback.value = `已打开${currentMeta.value.name}登录窗口，请在窗口中扫码或输入账号密码登录后点击「同步」`;
  } catch (err: any) {
    feedback.value = typeof err === 'string' ? err : err?.message || '无法打开内置登录窗口';
  }
};

const syncWindowSession = async () => {
  feedback.value = '';
  isSyncing.value = true;
  try {
    const label = accountInput.value.trim() || nicknameInput.value.trim() || undefined;
    await extraStore.syncFromOfficialWindow(extraStore.activeTab, label);
    extraStore.closeLoginModal();
  } catch (err: any) {
    feedback.value = typeof err === 'string' ? err : err?.message || '同步失败，请确认已在登录窗口中完成登录';
  } finally {
    isSyncing.value = false;
  }
};

const openOfficialSite = async () => {
  const url = currentMeta.value.webUrl;
  try {
    await openUrl(url);
  } catch {
    window.open(url, '_blank');
  }
};

const pasteFromClipboardAndLogin = async () => {
  feedback.value = '';
  try {
    const text = await navigator.clipboard.readText();
    if (!text || !text.trim()) {
      feedback.value = '剪贴板为空，请先在浏览器复制 Cookie';
      return;
    }
    cookieInput.value = text.trim();
    await extraStore.loginWithCookie(extraStore.activeTab, text.trim(), nicknameInput.value);
    extraStore.closeLoginModal();
  } catch {
    feedback.value = '无法自动读取剪贴板，请手动粘贴到下方输入框';
  }
};

const handleSubmit = async () => {
  if (!cookieInput.value.trim()) return;
  await extraStore.loginWithCookie(extraStore.activeTab, cookieInput.value, nicknameInput.value);
  extraStore.closeLoginModal();
};
</script>
