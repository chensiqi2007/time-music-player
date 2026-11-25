import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { Playlist } from '@/types/music';

export interface ImportedTrackItem {
  title: string;
  artist: string;
  album?: string;
  duration: number;
}

export interface ImportedPlaylistPreview {
  platform: string;
  platform_name: string;
  name: string;
  description?: string;
  cover?: string;
  tracks_count: number;
  tracks: ImportedTrackItem[];
}

export const useImporterStore = defineStore('importer', {
  state: () => ({
    isModalOpen: false,
    inputUrl: '',
    isParsing: false,
    parseError: '',
    preview: null as ImportedPlaylistPreview | null,
    isImporting: false,
  }),

  actions: {
    openModal() {
      this.isModalOpen = true;
      this.inputUrl = '';
      this.parseError = '';
      this.preview = null;
    },

    closeModal() {
      this.isModalOpen = false;
      this.parseError = '';
      this.preview = null;
    },

    async parseLink(url: string) {
      const target = url.trim();
      if (!target) return;
      this.inputUrl = target;
      this.isParsing = true;
      this.parseError = '';
      this.preview = null;

      try {
        const res = await invoke<ImportedPlaylistPreview>('parse_external_playlist_link', {
          url: target,
        });
        this.preview = res;
      } catch (err: any) {
        this.parseError = typeof err === 'string' ? err : '解析链接失败，请确认链接公开有效';
      } finally {
        this.isParsing = false;
      }
    },

    async confirmImport(): Promise<Playlist | null> {
      if (!this.preview || this.isImporting) return null;
      this.isImporting = true;
      try {
        const created = await invoke<any>('confirm_import_external_playlist', {
          preview: this.preview,
        });

        this.closeModal();
        return {
          id: created.id,
          name: created.name,
          description: created.description || '',
          cover: created.cover || '',
          trackIds: created.track_ids || [],
          createdAt: created.created_at * 1000,
          updatedAt: created.updated_at * 1000,
        };
      } catch (err: any) {
        this.parseError = typeof err === 'string' ? err : '导入歌单失败';
        return null;
      } finally {
        this.isImporting = false;
      }
    },
  },
});

