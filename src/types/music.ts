export interface Track {
  id: string;
  title: string;
  artist: string;
  album: string;
  duration: number; // in seconds
  path?: string; // local file path or stream url
  cover?: string; // data URI or URL
  genre?: string;
  year?: number;
  format?: string; // mp3, flac, wav, etc.
  bitrate?: number; // kbps
  isFavorite?: boolean;
  playCount?: number;
}

export interface Album {
  id: string;
  title: string;
  artist: string;
  cover?: string;
  year?: number;
  tracksCount: number;
}

export interface Artist {
  id: string;
  name: string;
  avatar?: string;
  tracksCount: number;
}

export interface Playlist {
  id: string;
  name: string;
  description?: string;
  cover?: string;
  trackIds: string[];
  trackCount?: number;
  tracks?: Track[];
  createdAt: number;
  updatedAt?: number;
  isCloud?: boolean;
  platform?: 'netease' | 'qq' | 'youtube' | 'apple' | 'local';
}

export type PlaybackMode = 'sequence' | 'loop' | 'single' | 'shuffle';

export interface LrcLine {
  time: number; // seconds
  text: string;
  translation?: string;
}

export interface PlayerState {
  currentTrack: Track | null;
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  volume: number; // 0 - 1
  isMuted: boolean;
  mode: PlaybackMode;
  playbackRate: number; // 0.5 - 2.0
  queue: Track[];
  queueIndex: number;
  lyrics: LrcLine[];
  activeLyricIndex: number;
  showFullscreenLyrics: boolean;
  showQueueDrawer: boolean;
}

