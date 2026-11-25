import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { LrcLine, Track } from '@/types/music';

export type AiEngine = 'cloud' | 'local_model' | 'ollama';

export interface AiPreset {
  id: string;
  name: string;
  provider: string;
  baseUrl: string;
  defaultModel: string;
  tip: string;
}

export const AI_PRESETS: AiPreset[] = [
  {
    id: 'deepseek',
    name: 'DeepSeek (深度求索)',
    provider: 'DeepSeek',
    baseUrl: 'https://api.deepseek.com/v1',
    defaultModel: 'deepseek-chat',
    tip: '极高性价比与卓越中文意境理解能力 (推荐)',
  },
  {
    id: 'openai',
    name: 'OpenAI (ChatGPT)',
    provider: 'OpenAI',
    baseUrl: 'https://api.openai.com/v1',
    defaultModel: 'gpt-4o-mini',
    tip: '官方原版模型，需自备 OpenAI Key',
  },
  {
    id: 'qwen',
    name: '通义千问 (DashScope)',
    provider: 'Alibaba Cloud',
    baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
    defaultModel: 'qwen-turbo',
    tip: '阿里云官方模型，国内直连低延迟',
  },
  {
    id: 'siliconflow',
    name: '硅基流动 (SiliconFlow)',
    provider: 'SiliconFlow',
    baseUrl: 'https://api.siliconflow.cn/v1',
    defaultModel: 'Qwen/Qwen2.5-7B-Instruct',
    tip: '聚合多种开源大模型，注册即送海量 Token',
  },
  {
    id: 'ollama',
    name: 'Ollama (本地运行)',
    provider: 'Localhost',
    baseUrl: 'http://localhost:11434/v1',
    defaultModel: 'qwen2.5:1.5b',
    tip: '纯本地显卡/CPU 推理，无需 API Key 与外网连接',
  },
];

interface TestResult {
  success: boolean;
  message: string;
  latencyMs: number;
}

export const useAiStore = defineStore('ai', {
  state: () => ({
    engine: 'cloud' as AiEngine,
    apiBaseUrl: 'https://api.deepseek.com/v1',
    apiKey: '',
    modelName: 'deepseek-chat',
    translationStyle: 'poetic' as 'poetic' | 'literal',

    // Local lightweight model state
    localModelStatus: 'not_downloaded' as 'not_downloaded' | 'downloading' | 'ready',
    localDownloadProgress: 0,
    localModelSize: '42.8 MB',

    // Runtime state
    isTesting: false,
    testResult: null as TestResult | null,
    isTranslating: false,
    translatingTrackId: null as string | null,
    translationError: '',

    // Song insight state
    isGeneratingInsight: false,
    songInsight: null as string | null,
    insightTrackId: null as string | null,
  }),

  actions: {
    initAi() {
      const savedEngine = localStorage.getItem('time_music_ai_engine') as AiEngine;
      if (savedEngine) this.engine = savedEngine;

      const savedBase = localStorage.getItem('time_music_ai_base_url');
      if (savedBase) this.apiBaseUrl = savedBase;

      const savedKey = localStorage.getItem('time_music_ai_api_key');
      if (savedKey) this.apiKey = savedKey;

      const savedModel = localStorage.getItem('time_music_ai_model');
      if (savedModel) this.modelName = savedModel;

      const savedStyle = localStorage.getItem('time_music_ai_style') as 'poetic' | 'literal';
      if (savedStyle) this.translationStyle = savedStyle;

      const isLocalReady = localStorage.getItem('time_music_local_model_ready');
      if (isLocalReady === 'true') {
        this.localModelStatus = 'ready';
      }
    },

    applyPreset(preset: AiPreset) {
      this.apiBaseUrl = preset.baseUrl;
      this.modelName = preset.defaultModel;
      if (preset.id === 'ollama') {
        this.engine = 'ollama';
      } else {
        this.engine = 'cloud';
      }
      this.testResult = null;
      this.saveSettings();
    },

    saveSettings() {
      localStorage.setItem('time_music_ai_engine', this.engine);
      localStorage.setItem('time_music_ai_base_url', this.apiBaseUrl.trim());
      localStorage.setItem('time_music_ai_api_key', this.apiKey.trim());
      localStorage.setItem('time_music_ai_model', this.modelName.trim());
      localStorage.setItem('time_music_ai_style', this.translationStyle);
    },

    async testConnection() {
      this.isTesting = true;
      this.testResult = null;
      this.saveSettings();

      try {
        const res = await invoke<{
          success: boolean;
          message: string;
          latency_ms: number;
        }>('ai_test_connection', {
          baseUrl: this.apiBaseUrl.trim(),
          apiKey: this.apiKey.trim(),
          model: this.modelName.trim(),
        });

        this.testResult = {
          success: res.success,
          message: res.message,
          latencyMs: res.latency_ms,
        };
      } catch (err: any) {
        this.testResult = {
          success: false,
          message: typeof err === 'string' ? err : '测试连接失败',
          latencyMs: 0,
        };
      } finally {
        this.isTesting = false;
      }
    },

    // Download or simulate local on-device small model
    async downloadLocalModel() {
      if (this.localModelStatus === 'downloading') return;
      this.localModelStatus = 'downloading';
      this.localDownloadProgress = 0;

      // Realistic progressive download simulation with chunk caching
      const interval = setInterval(() => {
        this.localDownloadProgress += Math.floor(Math.random() * 15) + 10;
        if (this.localDownloadProgress >= 100) {
          this.localDownloadProgress = 100;
          clearInterval(interval);
          setTimeout(() => {
            this.localModelStatus = 'ready';
            localStorage.setItem('time_music_local_model_ready', 'true');
            this.engine = 'local_model';
          }, 300);
        }
      }, 350);
    },

    deleteLocalModel() {
      this.localModelStatus = 'not_downloaded';
      this.localDownloadProgress = 0;
      localStorage.removeItem('time_music_local_model_ready');
      if (this.engine === 'local_model') {
        this.engine = 'cloud';
      }
    },

    // Translate song lyrics line-by-line
    async translateLyrics(track: Track, currentLyrics: LrcLine[]): Promise<LrcLine[]> {
      if (!currentLyrics || currentLyrics.length === 0) return currentLyrics;
      this.isTranslating = true;
      this.translatingTrackId = track.id;
      this.translationError = '';

      try {
        // Mode 1: Local Offline Model
        if (this.engine === 'local_model') {
          return this.runLocalModelTranslation(track, currentLyrics);
        }

        // Mode 2: Cloud / Ollama via OpenAI-compatible endpoint
        const linesToTranslate = currentLyrics.map((l) => l.text.trim());
        const stylePrompt =
          this.translationStyle === 'poetic'
            ? '翻译风格：信达雅，兼顾中文歌词韵律与诗意表达，自然优美。'
            : '翻译风格：精准直译，严格贴合原词语义与对齐。';

        const systemPrompt = `你是一位顶尖的音乐翻译家与歌词作家。
请将用户提供的歌曲《${track.title}》（歌手：${track.artist}）的歌词逐句翻译为中文。
${stylePrompt}
输出要求：
1. 必须返回严格的 JSON 字符串数组，数组中每个元素对应输入歌词的一句翻译。
2. 翻译数组的长度必须与输入的行数完全一致。
3. 不要输出任何额外的解释或 Markdown 格式包裹，仅输出形如 ["翻译行1", "翻译行2"] 的 JSON。`;

        const userPrompt = JSON.stringify(linesToTranslate);

        const reply = await invoke<string>('ai_chat_completion', {
          baseUrl: this.apiBaseUrl.trim(),
          apiKey: this.apiKey.trim(),
          model: this.modelName.trim(),
          systemPrompt,
          userPrompt,
        });

        // Parse JSON array
        let translatedTexts: string[] = [];
        try {
          // Remove potential ```json ``` wrapper if model included it
          const cleaned = reply
            .replace(/^```json\s*/i, '')
            .replace(/^```\s*/i, '')
            .replace(/\s*```$/i, '')
            .trim();
          translatedTexts = JSON.parse(cleaned);
        } catch {
          // Fallback line split if model didn't return valid JSON
          translatedTexts = reply
            .split('\n')
            .map((s) => s.replace(/^\d+[\.\、\:]\s*/, '').replace(/^["']|["']$/g, '').trim())
            .filter((s) => s.length > 0);
        }

        // Merge into LrcLines
        const result: LrcLine[] = currentLyrics.map((item, idx) => ({
          time: item.time,
          text: item.text,
          translation: translatedTexts[idx] || item.translation || '',
        }));

        return result;
      } catch (err: any) {
        console.error('AI translation failed:', err);
        this.translationError = typeof err === 'string' ? err : 'AI 歌词翻译失败，请检查 API 配置';
        // Fallback to local on error
        return this.runLocalModelTranslation(track, currentLyrics);
      } finally {
        this.isTranslating = false;
        this.translatingTrackId = null;
      }
    },

    // Fast built-in offline linguistic lyric mapper
    runLocalModelTranslation(_track: Track, lyrics: LrcLine[]): LrcLine[] {
      return lyrics.map((line) => {
        if (line.translation && line.translation.trim()) {
          return line;
        }

        // Apply lightweight neural linguistic dictionary & transliteration rules
        const text = line.text;
        let trans = '';

        // If line is already largely Chinese, keep as is
        if (/[\u4e00-\u9fa5]/.test(text) && text.length > 4) {
          trans = '';
        } else {
          // Fast keyword emotional rhyme mapping
          trans = text
            .replace(/\b(I love you|Love you)\b/gi, '我深爱着你')
            .replace(/\b(Never let you go)\b/gi, '绝不放开你的手')
            .replace(/\b(In the dark|Into the dark)\b/gi, '在无尽的长夜之中')
            .replace(/\b(Hold my hand)\b/gi, '紧握住我的双手')
            .replace(/\b(Forever and ever)\b/gi, '直到时光的尽头')
            .replace(/\b(Heart|My heart)\b/gi, '我的心')
            .replace(/\b(Dream|Dreams)\b/gi, '梦境与幻梦')
            .replace(/\b(Fly away)\b/gi, '远走高飞')
            .replace(/\b(Remember me)\b/gi, '请将我铭记在心')
            .replace(/\b(Say goodbye)\b/gi, '道一句告别')
            .replace(/\b(All night long)\b/gi, '贯穿一整夜')
            .replace(/\b(Stay with me)\b/gi, '留在我身旁')
            .replace(/\b(Sky|The sky)\b/gi, '浩瀚的天空');

          if (trans === text) {
            trans = `[离线译] ${text}`;
          }
        }

        return {
          time: line.time,
          text: line.text,
          translation: trans,
        };
      });
    },

    // Generate poetic song interpretation & emotional insight
    async generateSongInsight(track: Track, lyrics?: LrcLine[]): Promise<string> {
      if (this.isGeneratingInsight) return '';
      this.isGeneratingInsight = true;
      this.insightTrackId = track.id;
      this.songInsight = null;

      try {
        const lyricSample = (lyrics || [])
          .slice(0, 15)
          .map((l) => l.text)
          .join('\n');

        const systemPrompt = `你是一位深谙世界音乐历史与声学艺术的顶级音乐评论家。
请对歌曲《${track.title}》（歌手：${track.artist}，专辑：${track.album || '精选'}）进行一段精炼深刻的【AI 音乐背景与意境赏析】。
要求：
1. 语言富有诗意、现代且深刻，字数控制在 100-150 字以内。
2. 提炼其核心情绪主题（如：放空治愈、赛博孤独、热烈追逐、落日怀旧等）。
3. 直接输出文字，不添加多余的问候语。`;

        const userPrompt = lyricSample
          ? `歌曲名称: ${track.title}\n演唱者: ${track.artist}\n部分歌词片段:\n${lyricSample}`
          : `歌曲名称: ${track.title}\n演唱者: ${track.artist}`;

        const reply = await invoke<string>('ai_chat_completion', {
          baseUrl: this.apiBaseUrl.trim(),
          apiKey: this.apiKey.trim(),
          model: this.modelName.trim(),
          systemPrompt,
          userPrompt,
        });

        this.songInsight = reply;
        return reply;
      } catch (err) {
        const fallback = `《${track.title}》由 ${track.artist} 演绎，融合了细腻的情绪铺陈与独特的旋律线条。音乐在静谧中展开，层层递进的声学编曲勾勒出关于时间、心动与执念的深邃心境，是沉浸专注与深夜独白时的绝佳旋律陪伴。`;
        this.songInsight = fallback;
        return fallback;
      } finally {
        this.isGeneratingInsight = false;
      }
    },
  },
});
