import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { Track } from '@/types/music';
import type { PlatformId } from '@/types/provider';
import { usePlayerStore } from './player';

export interface SearchCounts {
  netease: number;
  qq: number;
  kuwo: number;
  kugou: number;
  qishui: number;
  apple: number;
  youtube: number;
}

const searchCache = new Map<string, { results: Track[]; counts: SearchCounts; time: number }>();
const SEARCH_CACHE_TTL = 5 * 60 * 1000; // 5 min cache

export const useProviderStore = defineStore('provider', {
  state: () => ({
    activePlatform: 'all' as PlatformId,
    isSearching: false,
    keyword: '',
    results: [] as Track[],
    counts: {
      netease: 0,
      qq: 0,
      kuwo: 0,
      kugou: 0,
      qishui: 0,
      apple: 0,
      youtube: 0,
    } as SearchCounts,
    connectedPlatforms: {
      netease: true,
      qq: true,
      kuwo: true,
      kugou: true,
      qishui: true,
      apple: true,
      youtube: true,
    },
  }),

  actions: {
    setActivePlatform(platform: PlatformId) {
      this.activePlatform = platform;
      // If we don't have results yet and there is a keyword, perform search
      if (this.results.length === 0 && this.keyword.trim()) {
        this.performSearch(this.keyword);
      }
    },

    async performSearch(keyword: string) {
      const q = keyword.trim();
      if (!q) {
        this.results = [];
        this.keyword = '';
        return;
      }

      this.keyword = q;

      // 1. Check in-memory cache for instant zero-latency retrieval
      const cacheKey = `${this.activePlatform}::${q.toLowerCase()}`;
      const cached = searchCache.get(cacheKey);
      if (cached && Date.now() - cached.time < SEARCH_CACHE_TTL) {
        const playerStore = usePlayerStore();
        this.results = cached.results.map((t) => ({
          ...t,
          isFavorite: playerStore.isIdFavorite(t.id),
        }));
        this.counts = { ...cached.counts };
        this.isSearching = false;
        return;
      }

      this.isSearching = true;

      try {
        if (this.activePlatform === 'all') {
          await this.searchAggregated(q);
        } else {
          await this.searchSingle(this.activePlatform, q);
        }

        // Cache results
        if (searchCache.size > 50) {
          const oldestKey = searchCache.keys().next().value;
          if (oldestKey) searchCache.delete(oldestKey);
        }
        searchCache.set(cacheKey, {
          results: this.results,
          counts: { ...this.counts },
          time: Date.now(),
        });
      } catch (err) {
        console.error('Multi-platform search failed:', err);
      } finally {
        this.isSearching = false;
      }
    },

    async searchAggregated(q: string) {
      // Execute all platform searches in parallel
      const [neteaseRes, qqRes, kuwoRes, kugouRes, qishuiRes, appleRes, ytRes] = await Promise.allSettled([
        invoke<Track[]>('netease_search', { keyword: q, limit: 12 }),
        invoke<Track[]>('qqmusic_search', { keyword: q, limit: 12 }),
        invoke<Track[]>('kuwo_search', { keyword: q, limit: 12 }),
        invoke<Track[]>('kugou_search', { keyword: q, limit: 12 }),
        invoke<Track[]>('qishui_search', { keyword: q, limit: 10 }),
        invoke<Track[]>('applemusic_search', { keyword: q, limit: 10 }),
        invoke<Track[]>('ytmusic_search', { keyword: q, limit: 10 }),
      ]);

      const neteaseTracks = neteaseRes.status === 'fulfilled' ? neteaseRes.value : [];
      const qqTracks = qqRes.status === 'fulfilled' ? qqRes.value : [];
      const kuwoTracks = kuwoRes.status === 'fulfilled' ? kuwoRes.value : [];
      const kugouTracks = kugouRes.status === 'fulfilled' ? kugouRes.value : [];
      const qishuiTracks = qishuiRes.status === 'fulfilled' ? qishuiRes.value : [];
      const appleTracks = appleRes.status === 'fulfilled' ? appleRes.value : [];
      const ytTracks = ytRes.status === 'fulfilled' ? ytRes.value : [];

      this.counts = {
        netease: neteaseTracks.length,
        qq: qqTracks.length,
        kuwo: kuwoTracks.length,
        kugou: kugouTracks.length,
        qishui: qishuiTracks.length,
        apple: appleTracks.length,
        youtube: ytTracks.length,
      };

      // Interweave results for a rich, balanced aggregated stream
      const combined: Track[] = [];
      const maxLen = Math.max(
        neteaseTracks.length,
        qqTracks.length,
        kuwoTracks.length,
        kugouTracks.length,
        qishuiTracks.length,
        appleTracks.length,
        ytTracks.length
      );

      for (let i = 0; i < maxLen; i++) {
        if (i < qqTracks.length) combined.push(qqTracks[i]);
        if (i < neteaseTracks.length) combined.push(neteaseTracks[i]);
        if (i < kuwoTracks.length) combined.push(kuwoTracks[i]);
        if (i < kugouTracks.length) combined.push(kugouTracks[i]);
        if (i < qishuiTracks.length) combined.push(qishuiTracks[i]);
        if (i < appleTracks.length) combined.push(appleTracks[i]);
        if (i < ytTracks.length) combined.push(ytTracks[i]);
      }

      const playerStore = usePlayerStore();
      for (const t of combined) {
        t.isFavorite = playerStore.isIdFavorite(t.id);
      }

      this.results = combined;
    },

    async searchSingle(platform: PlatformId, q: string) {
      let tracks: Track[] = [];

      switch (platform) {
        case 'netease':
          tracks = await invoke<Track[]>('netease_search', { keyword: q, limit: 30 });
          this.counts.netease = tracks.length;
          break;
        case 'qq':
          tracks = await invoke<Track[]>('qqmusic_search', { keyword: q, limit: 30 });
          this.counts.qq = tracks.length;
          break;
        case 'kuwo':
          tracks = await invoke<Track[]>('kuwo_search', { keyword: q, limit: 30 });
          this.counts.kuwo = tracks.length;
          break;
        case 'kugou':
          tracks = await invoke<Track[]>('kugou_search', { keyword: q, limit: 30 });
          this.counts.kugou = tracks.length;
          break;
        case 'qishui':
          tracks = await invoke<Track[]>('qishui_search', { keyword: q, limit: 30 });
          this.counts.qishui = tracks.length;
          break;
        case 'apple':
          tracks = await invoke<Track[]>('applemusic_search', { keyword: q, limit: 30 });
          this.counts.apple = tracks.length;
          break;
        case 'youtube':
          tracks = await invoke<Track[]>('ytmusic_search', { keyword: q, limit: 30 });
          this.counts.youtube = tracks.length;
          break;
        default:
          break;
      }

      const playerStore = usePlayerStore();
      for (const t of tracks) {
        t.isFavorite = playerStore.isIdFavorite(t.id);
      }

      this.results = tracks;
    },

    clearResults() {
      this.keyword = '';
      this.results = [];
      this.counts = { netease: 0, qq: 0, kuwo: 0, kugou: 0, qishui: 0, apple: 0, youtube: 0 };
    },
  },
});

