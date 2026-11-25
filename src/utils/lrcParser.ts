import type { LrcLine } from '@/types/music';

/**
 * Robust LRC Parser with Bilingual Translation Support
 */
export function parseLrc(rawLrc: string, rawTlyric?: string): LrcLine[] {
  if (!rawLrc || !rawLrc.trim()) return [];

  const parseLines = (text: string): { time: number; text: string }[] => {
    const result: { time: number; text: string }[] = [];
    const textLines = text.split('\n');

    for (const rawLine of textLines) {
      const line = rawLine.trim();
      if (!line) continue;

      // Ignore common metadata tags like [ti:], [ar:], [al:], [by:], [offset:]
      if (/^\[(ti|ar|al|by|offset|length):.*\]$/i.test(line)) {
        continue;
      }

      // Match all [mm:ss.xx] or [mm:ss.xxx] or [mm:ss]
      const timestampRegex = /\[(\d{1,2}):(\d{2})(?:\.(\d{1,3}))?\]/g;
      const matches = Array.from(line.matchAll(timestampRegex));
      if (!matches.length) continue;

      // Extract text content by stripping out all timestamps
      const cleanText = line.replace(timestampRegex, '').trim();
      if (!cleanText) continue;

      for (const match of matches) {
        const mins = parseInt(match[1], 10);
        const secs = parseInt(match[2], 10);
        const msStr = match[3] || '0';
        let ms = 0;
        if (msStr.length === 1) ms = parseInt(msStr, 10) * 100;
        else if (msStr.length === 2) ms = parseInt(msStr, 10) * 10;
        else ms = parseInt(msStr.slice(0, 3), 10);

        const time = mins * 60 + secs + ms / 1000;
        result.push({ time, text: cleanText });
      }
    }
    return result;
  };

  const parsedMain = parseLines(rawLrc);
  parsedMain.sort((a, b) => a.time - b.time);

  if (!rawTlyric || !rawTlyric.trim()) {
    return parsedMain;
  }

  const parsedTrans = parseLines(rawTlyric);

  // Match translations to the closest main lyric line (within 0.8 seconds tolerance)
  return parsedMain.map((item) => {
    const match = parsedTrans.find(
      (t) => Math.abs(t.time - item.time) <= 0.8 && t.text.length > 0
    );
    return {
      time: item.time,
      text: item.text,
      translation: match ? match.text : undefined,
    };
  });
}

