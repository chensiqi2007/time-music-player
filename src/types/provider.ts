import type { Track } from './music';

export type PlatformId = 'all' | 'netease' | 'qq' | 'kuwo' | 'kugou' | 'qishui' | 'apple' | 'youtube';

export interface PlatformMeta {
  id: PlatformId;
  name: string;
  shortName: string;
  tagColor: string;
  badgeBg: string;
  badgeBorder: string;
  badgeText: string;
  icon?: string;
  description: string;
}

export const PLATFORMS: PlatformMeta[] = [
  {
    id: 'all',
    name: '全网聚合检索',
    shortName: '全部聚合',
    tagColor: 'text-brand',
    badgeBg: 'bg-brand/15',
    badgeBorder: 'border-brand/30',
    badgeText: 'text-brand',
    description: '同时检索网易云、QQ、酷我、酷狗、汽水、Apple 与 YouTube Music',
  },
  {
    id: 'netease',
    name: '网易云音乐',
    shortName: '网易云',
    tagColor: 'text-rose-400',
    badgeBg: 'bg-rose-500/15',
    badgeBorder: 'border-rose-500/25',
    badgeText: 'text-rose-400',
    description: '中国大陆主流高品质流行与独立音乐曲库',
  },
  {
    id: 'qq',
    name: 'QQ 音乐',
    shortName: 'QQ音乐',
    tagColor: 'text-emerald-400',
    badgeBg: 'bg-emerald-500/15',
    badgeBorder: 'border-emerald-500/25',
    badgeText: 'text-emerald-400',
    description: '海量华语流行、周杰伦全辑独家版权曲库',
  },
  {
    id: 'kuwo',
    name: '酷我音乐',
    shortName: '酷我',
    tagColor: 'text-amber-400',
    badgeBg: 'bg-amber-500/15',
    badgeBorder: 'border-amber-500/25',
    badgeText: 'text-amber-400',
    description: '高可用免登 MP3 直链与怀旧经典曲库',
  },
  {
    id: 'kugou',
    name: '酷狗音乐',
    shortName: '酷狗',
    tagColor: 'text-sky-400',
    badgeBg: 'bg-sky-500/15',
    badgeBorder: 'border-sky-500/25',
    badgeText: 'text-sky-400',
    description: '海量华语热歌、Hi-Fi 音质与精准歌词库',
  },
  {
    id: 'qishui',
    name: '汽水音乐',
    shortName: '汽水',
    tagColor: 'text-teal-400',
    badgeBg: 'bg-teal-500/15',
    badgeBorder: 'border-teal-500/25',
    badgeText: 'text-teal-400',
    description: '抖音热歌发现与个性潮流推荐曲库',
  },
  {
    id: 'apple',
    name: 'Apple Music',
    shortName: 'Apple',
    tagColor: 'text-pink-400',
    badgeBg: 'bg-pink-500/15',
    badgeBorder: 'border-pink-500/25',
    badgeText: 'text-pink-400',
    description: 'Apple Digital Master 母带级元数据与 256k AAC 试听',
  },
  {
    id: 'youtube',
    name: 'YouTube Music',
    shortName: 'YouTube',
    tagColor: 'text-red-400',
    badgeBg: 'bg-red-600/15',
    badgeBorder: 'border-red-600/25',
    badgeText: 'text-red-400',
    description: '全球海量 Live 现场、原创与全语种 Opus 音频流',
  },
];

export function getTrackPlatform(
  track?: Track | null
): 'netease' | 'qq' | 'kuwo' | 'kugou' | 'qishui' | 'apple' | 'youtube' | 'local' {
  if (!track) return 'local';
  if (track.path?.startsWith('netease://') || track.id.startsWith('netease-')) return 'netease';
  if (track.path?.startsWith('qq://') || track.id.startsWith('qq-')) return 'qq';
  if (track.path?.startsWith('kuwo://') || track.id.startsWith('kuwo-')) return 'kuwo';
  if (track.path?.startsWith('kugou://') || track.id.startsWith('kugou-')) return 'kugou';
  if (track.path?.startsWith('qishui://') || track.id.startsWith('qishui-')) return 'qishui';
  if (track.path?.startsWith('apple://') || track.id.startsWith('apple-') || track.path?.includes('apple.com')) return 'apple';
  if (track.path?.startsWith('yt://') || track.id.startsWith('yt-')) return 'youtube';
  return 'local';
}

export function getPlatformBadge(
  platform: 'netease' | 'qq' | 'kuwo' | 'kugou' | 'qishui' | 'apple' | 'youtube' | 'local'
): PlatformMeta {
  const match = PLATFORMS.find((p) => p.id === platform);
  if (match) return match;
  return {
    id: 'all',
    name: '本地文件',
    shortName: '本地',
    tagColor: 'text-slate-400',
    badgeBg: 'bg-white/10',
    badgeBorder: 'border-white/15',
    badgeText: 'text-slate-300',
    description: '存储在本地硬盘的音频文件',
  };
}

