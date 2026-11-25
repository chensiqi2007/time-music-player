<template>
  <Transition name="fade">
    <div
      v-if="playlistStore.isModalOpen"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md"
      @click.self="playlistStore.closeModal()"
    >
      <div
        class="w-full max-w-lg rounded-3xl glass-card border border-white/15 p-6 shadow-2xl relative overflow-hidden flex flex-col max-h-[90vh] animate-in fade-in zoom-in-95 duration-200"
      >
        <!-- Modal Header -->
        <div class="flex items-center justify-between pb-4 border-b border-white/10">
          <div class="flex items-center gap-2.5">
            <div class="w-8 h-8 rounded-xl bg-brand/20 border border-brand/30 flex items-center justify-center text-brand">
              <FolderEdit v-if="isEdit" :size="18" />
              <FolderPlus v-else :size="18" />
            </div>
            <div>
              <h2 class="text-base font-bold text-white tracking-wide">
                {{ isEdit ? '编辑歌单资料' : '创建新歌单' }}
              </h2>
              <p class="text-[11px] text-slate-400">
                {{ isEdit ? '修改歌单名称、详细介绍或个性化封面' : '定制属于您的音乐合辑与封面艺术' }}
              </p>
            </div>
          </div>
          <button
            @click="playlistStore.closeModal()"
            class="text-slate-400 hover:text-white p-1.5 rounded-full hover:bg-white/10 transition-colors"
          >
            <X :size="18" />
          </button>
        </div>

        <!-- Modal Form Body -->
        <form @submit.prevent="handleSubmit" class="space-y-4 py-4 overflow-y-auto pr-1">
          <!-- Cover & Basic Info Row -->
          <div class="flex gap-4 items-start">
            <!-- Cover Preview & Trigger -->
            <div class="space-y-1.5 shrink-0">
              <label class="text-xs font-semibold text-slate-300">歌单封面</label>
              <div
                @click="triggerFileInput"
                class="relative w-28 h-28 rounded-2xl overflow-hidden border border-white/20 group cursor-pointer shadow-lg bg-black/40 flex items-center justify-center"
                title="点击上传本地图片"
              >
                <img
                  :src="coverUrl || defaultCover"
                  alt="Playlist Cover"
                  class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                />
                <!-- Hover Overlay -->
                <div
                  class="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity flex flex-col items-center justify-center gap-1 text-white p-2 text-center"
                >
                  <Camera :size="20" class="text-brand" />
                  <span class="text-[10px] font-medium leading-tight">更换本地图片</span>
                </div>
              </div>
              <input
                ref="fileInputRef"
                type="file"
                accept="image/*"
                class="hidden"
                @change="handleFileUpload"
              />
              <button
                type="button"
                @click="triggerFileInput"
                class="w-full text-center py-1 text-[11px] text-slate-400 hover:text-brand flex items-center justify-center gap-1 transition-colors"
              >
                <Upload :size="12" />
                上传本地图片
              </button>
            </div>

            <!-- Name & Description Inputs -->
            <div class="flex-1 space-y-3 min-w-0">
              <div>
                <label class="block text-xs font-semibold text-slate-300 mb-1 flex items-center justify-between">
                  <span>歌单名称 <span class="text-brand">*</span></span>
                  <span class="text-[10px] text-brand-light font-normal">点击即可快速点选</span>
                </label>
                <!-- Click-to-Select Preset Name Chips -->
                <div class="flex flex-wrap gap-1.5 pb-2">
                  <button
                    v-for="pName in ['私人精选', '深夜放空', '华语经典', '太空漫步', '工作专注', '慢调民谣', '摇滚现场']"
                    :key="pName"
                    type="button"
                    @click="name = pName"
                    class="px-2.5 py-1 rounded-xl text-[11px] font-medium border transition-all active:scale-95"
                    :class="name === pName ? 'bg-brand/20 border-brand/50 text-white font-semibold' : 'bg-white/[0.04] hover:bg-white/[0.08] border-white/10 text-slate-300'"
                  >
                    {{ pName }}
                  </button>
                </div>
                <input
                  v-model="name"
                  type="text"
                  required
                  maxlength="60"
                  placeholder="点选上方推荐，或自定义歌单名"
                  class="w-full px-3.5 py-2 rounded-xl bg-white/[0.07] border border-white/10 text-white placeholder-slate-500 text-xs focus:outline-none focus:border-brand/60 focus:bg-white/[0.1] transition-all"
                />
              </div>

              <div>
                <label class="block text-xs font-semibold text-slate-300 mb-1 flex items-center justify-between">
                  <span>歌单描述 <span class="text-slate-500 font-normal text-[10px]">(可选)</span></span>
                  <span class="text-[10px] text-slate-400 font-normal">点击快速点选</span>
                </label>
                <div class="flex flex-wrap gap-1.5 pb-2">
                  <button
                    v-for="pDesc in ['深夜安静放空，享受一人时光', '沉浸代码与专注时刻的极光原声', '华语流行黄金年代的感动旋律', '自驾公路巡礼与海风节奏']"
                    :key="pDesc"
                    type="button"
                    @click="description = pDesc"
                    class="px-2 py-0.5 rounded-lg text-[10px] bg-white/[0.04] hover:bg-white/[0.08] border border-white/10 text-slate-400 hover:text-slate-200 transition-all"
                  >
                    {{ pDesc.slice(0, 10) }}...
                  </button>
                </div>
                <textarea
                  v-model="description"
                  rows="2"
                  maxlength="200"
                  placeholder="点选上方推荐填充，或自定义描述..."
                  class="w-full px-3.5 py-2 rounded-xl bg-white/[0.07] border border-white/10 text-white placeholder-slate-500 text-xs focus:outline-none focus:border-brand/60 focus:bg-white/[0.1] transition-all resize-none"
                ></textarea>
              </div>
            </div>
          </div>

          <!-- Cover Image URL input -->
          <div>
            <label class="block text-xs font-semibold text-slate-300 mb-1">
              或输入网络图片 URL
            </label>
            <div class="relative flex items-center">
              <input
                v-model="coverUrl"
                type="url"
                placeholder="https://images.unsplash.com/..."
                class="w-full pl-8 pr-3.5 py-2 rounded-xl bg-white/[0.07] border border-white/10 text-white placeholder-slate-500 text-xs focus:outline-none focus:border-brand/60 focus:bg-white/[0.1] transition-all font-mono"
              />
              <Link :size="13" class="absolute left-2.5 text-slate-500" />
            </div>
          </div>

          <!-- Preset Covers Grid -->
          <div>
            <label class="block text-xs font-semibold text-slate-300 mb-2 flex items-center justify-between">
              <span>精选封面风格预设</span>
              <span class="text-[10px] text-slate-500 font-normal">点击快速应用</span>
            </label>
            <div class="grid grid-cols-4 gap-2">
              <div
                v-for="preset in presetCovers"
                :key="preset.id"
                @click="coverUrl = preset.url"
                class="group relative aspect-square rounded-xl overflow-hidden border cursor-pointer transition-all"
                :class="coverUrl === preset.url ? 'border-brand ring-2 ring-brand/40 scale-[0.98]' : 'border-white/10 hover:border-white/30'"
              >
                <img :src="preset.url" :alt="preset.label" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300" />
                <div class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/80 via-black/30 to-transparent p-1 text-center">
                  <span class="text-[9px] font-medium text-slate-200 block truncate">{{ preset.label }}</span>
                </div>
                <div
                  v-if="coverUrl === preset.url"
                  class="absolute top-1 right-1 w-4 h-4 rounded-full bg-brand flex items-center justify-center text-white shadow-md"
                >
                  <Check :size="10" stroke-width="3" />
                </div>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex items-center justify-end gap-3 pt-3 border-t border-white/10">
            <button
              type="button"
              @click="playlistStore.closeModal()"
              class="px-4 py-2 rounded-xl bg-white/10 hover:bg-white/15 text-slate-300 hover:text-white text-xs font-medium transition-colors"
            >
              取消
            </button>
            <button
              type="submit"
              :disabled="!name.trim() || isSubmitting"
              class="px-6 py-2 rounded-xl bg-brand hover:bg-brand-light text-white text-xs font-semibold shadow-lg shadow-brand/25 transition-all disabled:opacity-50 flex items-center gap-1.5"
            >
              <Save :size="13" />
              <span>{{ isSubmitting ? '保存中...' : (isEdit ? '保存更改' : '立即创建') }}</span>
            </button>
          </div>
        </form>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import {
  X,
  Camera,
  FolderPlus,
  FolderEdit,
  Save,
  Upload,
  Link,
  Check,
} from 'lucide-vue-next';
import { usePlaylistStore } from '@/stores/playlist';

const playlistStore = usePlaylistStore();
const router = useRouter();

const defaultCover = 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=500&auto=format&fit=crop&q=80';

const presetCovers = [
  {
    id: 'ambient',
    label: '极光太空',
    url: 'https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'cyberpunk',
    label: '赛博霓虹',
    url: 'https://images.unsplash.com/photo-1508700115892-45ecd05ae2ad?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'vinyl',
    label: '经典黑胶',
    url: 'https://images.unsplash.com/photo-1539375665275-f9de415ef9ac?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'night',
    label: '深夜独白',
    url: 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'cassette',
    label: '复古磁带',
    url: 'https://images.unsplash.com/photo-1507838153414-b4b713384a76?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'festival',
    label: '电子现场',
    url: 'https://images.unsplash.com/photo-1470225620780-dba8ba36b745?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'chill',
    label: '城市晨光',
    url: 'https://images.unsplash.com/photo-1511671782779-c97d3d27a1d4?w=500&auto=format&fit=crop&q=80',
  },
  {
    id: 'rain',
    label: '静谧雨幕',
    url: 'https://images.unsplash.com/photo-1519681393784-d120267933ba?w=500&auto=format&fit=crop&q=80',
  },
];

const name = ref('');
const description = ref('');
const coverUrl = ref(defaultCover);
const isSubmitting = ref(false);
const fileInputRef = ref<HTMLInputElement | null>(null);

const isEdit = computed(() => playlistStore.modalMode === 'edit');

// Initialize form when modal opens
watch(
  () => [playlistStore.isModalOpen, playlistStore.editingPlaylist],
  ([isOpen]) => {
    if (isOpen) {
      if (playlistStore.modalMode === 'edit' && playlistStore.editingPlaylist) {
        name.value = playlistStore.editingPlaylist.name;
        description.value = playlistStore.editingPlaylist.description || '';
        coverUrl.value = playlistStore.editingPlaylist.cover || defaultCover;
      } else {
        name.value = '';
        description.value = '';
        // Pick a nice random preset for new playlist
        const randomPreset = presetCovers[Math.floor(Math.random() * presetCovers.length)];
        coverUrl.value = randomPreset.url;
      }
    }
  }
);

const triggerFileInput = () => {
  fileInputRef.value?.click();
};

const handleFileUpload = (e: Event) => {
  const target = e.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  // Convert to base64 Data URL
  const reader = new FileReader();
  reader.onload = (event) => {
    if (event.target?.result) {
      coverUrl.value = event.target.result as string;
    }
  };
  reader.readAsDataURL(file);
};

const handleSubmit = async () => {
  if (!name.value.trim() || isSubmitting.value) return;

  isSubmitting.value = true;
  try {
    const finalCover = coverUrl.value.trim() || defaultCover;
    const finalDesc = description.value.trim();
    const finalName = name.value.trim();

    if (isEdit.value && playlistStore.editingPlaylist) {
      await playlistStore.updatePlaylist(
        playlistStore.editingPlaylist.id,
        finalName,
        finalDesc,
        finalCover
      );
    } else {
      const created = await playlistStore.createPlaylist(
        finalName,
        finalDesc,
        finalCover
      );
      if (created) {
        router.push(`/playlist/${created.id}`);
      }
    }
    playlistStore.closeModal();
  } catch (err) {
    console.error('Save playlist failed:', err);
  } finally {
    isSubmitting.value = false;
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

