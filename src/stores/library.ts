import { defineStore } from 'pinia';
import type { Track, Album, Artist } from '@/types/music';
import { invoke } from '@tauri-apps/api/core';
import { usePlayerStore } from './player';

export const useLibraryStore = defineStore('library', {
  state: () => ({
    tracks: [] as Track[],
    albums: [] as Album[],
    artists: [] as Artist[],
    favorites: [] as string[], // track IDs
    scanPaths: ['~/Music', 'D:/Music'] as string[],
    isScanning: false,
    searchQuery: '',
    selectedCategory: 'all' as 'all' | 'tracks' | 'albums' | 'artists' | 'favorites',
  }),

  getters: {
    totalTracksCount: (state) => state.tracks.length,
    favoriteTracks: (state) => state.tracks.filter((t) => t.isFavorite || state.favorites.includes(t.id)),
    filteredTracks: (state) => {
      const q = state.searchQuery.trim().toLowerCase();
      if (!q) return state.tracks;
      return state.tracks.filter(
        (t) =>
          t.title.toLowerCase().includes(q) ||
          t.artist.toLowerCase().includes(q) ||
          t.album.toLowerCase().includes(q)
      );
    },
  },

  actions: {
    async initLibrary() {
      try {
        const savedPaths = localStorage.getItem('time_music_scan_paths');
        if (savedPaths) {
          try {
            const parsed = JSON.parse(savedPaths);
            if (Array.isArray(parsed) && parsed.length > 0) {
              this.scanPaths = parsed;
            }
          } catch (e) {
            // Ignore parse error
          }
        } else {
          const defaultDirs = await invoke<string[]>('get_default_music_dirs');
          if (defaultDirs && defaultDirs.length > 0) {
            this.scanPaths = defaultDirs;
          }
        }

        await this.loadLocalTracks();
      } catch (err) {
        console.warn('Could not initialize library from backend:', err);
      }
    },

    async loadLocalTracks() {
      try {
        const localTracks = await invoke<Track[]>('get_local_tracks');
        if (localTracks && localTracks.length > 0) {
          const playerStore = usePlayerStore();
          this.tracks = localTracks.map((t) => ({
            ...t,
            isFavorite: t.isFavorite || playerStore.isIdFavorite(t.id),
          }));
          this.rebuildAlbumsAndArtists();
        }
      } catch (err) {
        console.warn('Could not load local tracks:', err);
      }
    },

    async scanDirectories() {
      if (this.isScanning) return;
      this.isScanning = true;

      try {
        await invoke('scan_music_directories', { dirs: this.scanPaths });
        await this.loadLocalTracks();
      } catch (err) {
        console.error('Scan directories failed:', err);
      } finally {
        this.isScanning = false;
      }
    },

    async toggleFavorite(trackOrId: Track | string) {
      const playerStore = usePlayerStore();
      playerStore.toggleFavorite(trackOrId);
    },

    async updateTrackMetadata(trackId: string, title: string, artist: string, album: string) {
      try {
        await invoke('update_track_tags', {
          trackId,
          title,
          artist,
          album,
        });

        const track = this.tracks.find((t) => t.id === trackId);
        if (track) {
          track.title = title;
          track.artist = artist;
          track.album = album;
        }
        this.rebuildAlbumsAndArtists();
      } catch (err) {
        console.error('Failed to update track metadata:', err);
      }
    },

    rebuildAlbumsAndArtists() {
      const albumMap = new Map<string, Album>();
      const artistMap = new Map<string, Artist>();

      for (const t of this.tracks) {
        // Albums
        const albKey = `${t.album}---${t.artist}`;
        if (!albumMap.has(albKey)) {
          albumMap.set(albKey, {
            id: `alb-${albKey}`,
            title: t.album,
            artist: t.artist,
            cover: t.cover,
            year: t.year,
            tracksCount: 1,
          });
        } else {
          albumMap.get(albKey)!.tracksCount += 1;
        }

        // Artists
        if (!artistMap.has(t.artist)) {
          artistMap.set(t.artist, {
            id: `art-${t.artist}`,
            name: t.artist,
            avatar: t.cover,
            tracksCount: 1,
          });
        } else {
          artistMap.get(t.artist)!.tracksCount += 1;
        }
      }

      this.albums = Array.from(albumMap.values());
      this.artists = Array.from(artistMap.values());
    },

    setSearchQuery(query: string) {
      this.searchQuery = query;
    },

    saveScanPaths() {
      localStorage.setItem('time_music_scan_paths', JSON.stringify(this.scanPaths));
    },

    addScanPath(path: string) {
      if (!this.scanPaths.includes(path)) {
        this.scanPaths.push(path);
        this.saveScanPaths();
      }
    },

    removeScanPath(path: string) {
      this.scanPaths = this.scanPaths.filter((p) => p !== path);
      this.saveScanPaths();
    },

    async chooseFolderToAdd(): Promise<string | null> {
      try {
        const folder = await invoke<string | null>('pick_folder');
        if (folder && folder.trim()) {
          this.addScanPath(folder.trim());
          return folder.trim();
        }
      } catch (e) {
        console.warn('Native folder selection failed:', e);
      }
      return null;
    },
  },
});
