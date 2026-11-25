import { defineStore } from 'pinia';
import type { Playlist, Track } from '@/types/music';
import { invoke } from '@tauri-apps/api/core';
import { usePlayerStore } from './player';

interface BackendPlaylist {
  id: string;
  name: string;
  description: string | null;
  cover: string | null;
  track_ids: string[];
  created_at: number;
  updated_at: number;
}

const TRACK_CACHE_KEY = 'time_music_playlist_track_cache';

function loadTrackCache(): Track[] {
  try {
    const raw = localStorage.getItem(TRACK_CACHE_KEY);
    return raw ? JSON.parse(raw) : [];
  } catch {
    return [];
  }
}

function saveTrackCache(tracks: Track[]) {
  try {
    localStorage.setItem(TRACK_CACHE_KEY, JSON.stringify(tracks.slice(-500)));
  } catch {}
}

export const usePlaylistStore = defineStore('playlist', {
  state: () => ({
    playlists: [
      {
        id: 'pl-favorites',
        name: '我喜欢的音乐',
        description: '收藏的所有心动旋律',
        cover: 'https://images.unsplash.com/photo-1514525253161-7a46d19cd819?w=500&auto=format&fit=crop&q=80',
        trackIds: ['demo-1', 'demo-3'],
        createdAt: Date.now() - 86400000 * 10,
        updatedAt: Date.now(),
      },
      {
        id: 'pl-ambient',
        name: 'Time OS 极光氛围',
        description: '沉浸代码与专注时刻的太空氛围原声',
        cover: 'https://images.unsplash.com/photo-1518709268805-4e9042af9f23?w=500&auto=format&fit=crop&q=80',
        trackIds: ['demo-1', 'demo-2', 'demo-4'],
        createdAt: Date.now() - 86400000 * 3,
        updatedAt: Date.now(),
      },
    ] as Playlist[],
    activePlaylistId: null as string | null,
    isModalOpen: false,
    modalMode: 'create' as 'create' | 'edit',
    editingPlaylist: null as Playlist | null,
    isDeleteModalOpen: false,
    deletingPlaylist: null as Playlist | null,
    isAddModalOpen: false,
    trackToAdd: null as Track | null,
    trackCache: loadTrackCache(),
  }),

  getters: {
    activePlaylist: (state) => state.playlists.find((p) => p.id === state.activePlaylistId) || null,
  },

  actions: {
    async initPlaylists() {
      try {
        const rows = await invoke<BackendPlaylist[]>('get_playlists');
        if (rows && rows.length > 0) {
          this.playlists = rows.map((r) => ({
            id: r.id,
            name: r.name,
            description: r.description || '',
            cover: r.cover || 'https://images.unsplash.com/photo-1470225620780-dba8ba36b745?w=500&auto=format&fit=crop&q=80',
            trackIds: r.track_ids || [],
            createdAt: r.created_at * 1000,
            updatedAt: r.updated_at * 1000,
          }));
        }
      } catch (err) {
        console.warn('Could not load playlists from SQLite backend:', err);
      }
    },

    openCreateModal() {
      this.modalMode = 'create';
      this.editingPlaylist = null;
      this.isModalOpen = true;
    },

    openEditModal(playlist: Playlist) {
      this.modalMode = 'edit';
      this.editingPlaylist = playlist;
      this.isModalOpen = true;
    },

    closeModal() {
      this.isModalOpen = false;
      this.editingPlaylist = null;
    },

    async updatePlaylist(id: string, name: string, description?: string, cover?: string) {
      const coverUrl = cover || 'https://images.unsplash.com/photo-1470225620780-dba8ba36b745?w=500&auto=format&fit=crop&q=80';
      try {
        await invoke('update_playlist', {
          id,
          name,
          description: description || null,
          cover: coverUrl,
        });
      } catch (err) {
        console.warn('Native update playlist failed, updating memory store:', err);
      }

      const pl = this.playlists.find((p) => p.id === id);
      if (pl) {
        pl.name = name;
        pl.description = description || '';
        pl.cover = coverUrl;
        pl.updatedAt = Date.now();
      }
    },

    async createPlaylist(name: string, description?: string, cover?: string) {
      const coverUrl = cover || 'https://images.unsplash.com/photo-1470225620780-dba8ba36b745?w=500&auto=format&fit=crop&q=80';
      try {
        const created = await invoke<BackendPlaylist>('create_playlist', {
          name,
          description: description || null,
          cover: coverUrl,
        });

        const newPl: Playlist = {
          id: created.id,
          name: created.name,
          description: created.description || '',
          cover: created.cover || coverUrl,
          trackIds: [],
          createdAt: created.created_at * 1000,
          updatedAt: created.updated_at * 1000,
        };
        this.playlists.push(newPl);
        return newPl;
      } catch (err) {
        console.warn('Native create playlist failed, falling back to memory store:', err);
        const fallback: Playlist = {
          id: `pl-${Date.now()}`,
          name: name || '新建歌单',
          description: description || '',
          cover: coverUrl,
          trackIds: [],
          createdAt: Date.now(),
          updatedAt: Date.now(),
        };
        this.playlists.push(fallback);
        return fallback;
      }
    },

    async deletePlaylist(id: string) {
      try {
        await invoke('delete_playlist', { id });
      } catch (err) {
        console.warn('Native delete playlist failed:', err);
      }
      this.playlists = this.playlists.filter((p) => p.id !== id);
      if (this.activePlaylistId === id) {
        this.activePlaylistId = null;
      }
    },

    openDeleteModal(playlist: Playlist) {
      this.deletingPlaylist = playlist;
      this.isDeleteModalOpen = true;
    },

    closeDeleteModal() {
      this.isDeleteModalOpen = false;
      this.deletingPlaylist = null;
    },

    async confirmDeletePlaylist() {
      if (!this.deletingPlaylist) return;
      const target = this.deletingPlaylist;
      await this.deletePlaylist(target.id);
      this.closeDeleteModal();
      const playerStore = usePlayerStore();
      playerStore.showToast(`已删除歌单「${target.name}」`);
    },

    async addTrackToPlaylist(playlistId: string, trackId: string) {
      try {
        await invoke('add_track_to_playlist', { playlistId, trackId });
      } catch (err) {
        console.warn('Native add track to playlist failed:', err);
      }

      const pl = this.playlists.find((p) => p.id === playlistId);
      if (pl && !pl.trackIds.includes(trackId)) {
        pl.trackIds.push(trackId);
        pl.updatedAt = Date.now();
      }
    },

    async removeTrackFromPlaylist(playlistId: string, trackId: string) {
      try {
        await invoke('remove_track_from_playlist', { playlistId, trackId });
      } catch (err) {
        console.warn('Native remove track from playlist failed:', err);
      }

      const pl = this.playlists.find((p) => p.id === playlistId);
      if (pl) {
        pl.trackIds = pl.trackIds.filter((id) => id !== trackId);
        pl.updatedAt = Date.now();
      }
    },

    exportPlaylistAsJson(playlistId: string): string {
      const pl = this.playlists.find((p) => p.id === playlistId);
      return pl ? JSON.stringify(pl, null, 2) : '';
    },

    openAddToPlaylistModal(track: Track) {
      this.trackToAdd = track;
      this.isAddModalOpen = true;
    },

    closeAddToPlaylistModal() {
      this.isAddModalOpen = false;
      this.trackToAdd = null;
    },

    cacheTracks(tracks: Track[]) {
      let changed = false;
      for (const t of tracks) {
        if (!this.trackCache.some((c) => c.id === t.id)) {
          this.trackCache.push(t);
          changed = true;
        }
      }
      if (changed) {
        saveTrackCache(this.trackCache);
      }
    },

    async toggleTrackInPlaylist(playlistId: string, track: Track) {
      const pl = this.playlists.find((p) => p.id === playlistId);
      if (!pl) return false;
      this.cacheTracks([track]);
      const exists = pl.trackIds.includes(track.id);
      if (exists) {
        await this.removeTrackFromPlaylist(playlistId, track.id);
        return false;
      } else {
        await this.addTrackToPlaylist(playlistId, track.id);
        return true;
      }
    },

    async createPlaylistWithPreset(presetName: string, track?: Track | null) {
      const defaultCover = 'https://images.unsplash.com/photo-1511671782779-c97d3d27a1d4?w=500&auto=format&fit=crop&q=80';
      const newPl = await this.createPlaylist(presetName, `精选合集 • ${presetName}`, defaultCover);
      if (track && newPl) {
        this.cacheTracks([track]);
        await this.addTrackToPlaylist(newPl.id, track.id);
      }
      return newPl;
    },
  },
});
