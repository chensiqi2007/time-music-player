import { defineStore } from 'pinia';
import type { Track } from '@/types/music';

export const useContextMenuStore = defineStore('contextMenu', {
  state: () => ({
    visible: false,
    x: 0,
    y: 0,
    mode: 'general' as 'track' | 'general' | 'input',
    track: null as Track | null,
    queueList: undefined as Track[] | undefined,
    inputEl: null as HTMLInputElement | HTMLTextAreaElement | null,
  }),

  actions: {
    clampPosition(clientX: number, clientY: number, menuWidth = 200, menuHeight = 230) {
      const maxX = Math.max(8, window.innerWidth - menuWidth - 8);
      const maxY = Math.max(8, window.innerHeight - menuHeight - 8);
      this.x = Math.min(Math.max(8, clientX), maxX);
      this.y = Math.min(Math.max(8, clientY), maxY);
    },

    openTrackMenu(e: MouseEvent, track: Track, queueList?: Track[]) {
      e.preventDefault();
      e.stopPropagation();
      this.mode = 'track';
      this.track = track;
      this.queueList = queueList;
      this.inputEl = null;
      this.clampPosition(e.clientX, e.clientY, 208, 240);
      this.visible = true;
    },

    openInputMenu(e: MouseEvent, el: HTMLInputElement | HTMLTextAreaElement) {
      e.preventDefault();
      e.stopPropagation();
      this.mode = 'input';
      this.track = null;
      this.queueList = undefined;
      this.inputEl = el;
      this.clampPosition(e.clientX, e.clientY, 172, 136);
      this.visible = true;
    },

    openGeneralMenu(e: MouseEvent) {
      e.preventDefault();
      this.mode = 'general';
      this.track = null;
      this.queueList = undefined;
      this.inputEl = null;
      this.clampPosition(e.clientX, e.clientY, 196, 220);
      this.visible = true;
    },

    close() {
      this.visible = false;
    },
  },
});
