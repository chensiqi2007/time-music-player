import type { Track } from '@/types/music';

export function initMediaSession(playerStore: any) {
  if (typeof window === 'undefined' || !('mediaSession' in navigator)) {
    return;
  }

  try {
    navigator.mediaSession.setActionHandler('play', () => {
      playerStore.togglePlay();
    });

    navigator.mediaSession.setActionHandler('pause', () => {
      playerStore.togglePlay();
    });

    navigator.mediaSession.setActionHandler('previoustrack', () => {
      playerStore.prevTrack();
    });

    navigator.mediaSession.setActionHandler('nexttrack', () => {
      playerStore.nextTrack();
    });

    navigator.mediaSession.setActionHandler('seekto', (details) => {
      if (details.seekTime !== undefined) {
        playerStore.seek(details.seekTime);
      }
    });

    navigator.mediaSession.setActionHandler('stop', () => {
      playerStore.isPlaying = false;
    });
  } catch (err) {
    console.warn('Error configuring MediaSession handlers:', err);
  }
}

export function syncMediaSessionState(track: Track | null, isPlaying: boolean) {
  if (typeof window === 'undefined' || !('mediaSession' in navigator)) {
    return;
  }

  if (!track) {
    navigator.mediaSession.playbackState = 'none';
    return;
  }

  try {
    navigator.mediaSession.metadata = new MediaMetadata({
      title: track.title,
      artist: track.artist,
      album: track.album || 'Time Music',
      artwork: track.cover
        ? [
            {
              src: track.cover,
              sizes: '512x512',
              type: 'image/jpeg',
            },
          ]
        : [],
    });

    navigator.mediaSession.playbackState = isPlaying ? 'playing' : 'paused';
  } catch (err) {
    console.warn('Error syncing MediaSession metadata:', err);
  }
}

