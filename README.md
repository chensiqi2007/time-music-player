<div align="center">
  <img src="public/logo.svg" alt="Time Music Logo" width="88" height="88" />
  <h1>Time Music</h1>
  <p>基於 <strong>Tauri 2 + Vue 3 + TypeScript + Rust</strong> 打造的跨平台液態玻璃桌面音樂播放器</p>
  <p>
    <img src="https://img.shields.io/badge/version-0.1.5-fa233b?style=flat-square" alt="Version 0.1.5" />
    <img src="https://img.shields.io/badge/Tauri-2.x-24c8db?style=flat-square" alt="Tauri 2" />
    <img src="https://img.shields.io/badge/Vue-3.5-42b883?style=flat-square" alt="Vue 3.5" />
    <img src="https://img.shields.io/badge/Rust-2024-dea584?style=flat-square" alt="Rust" />
    <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="MIT License" />
  </p>
</div>

---

## ✨ 核心功能

### 1. 七大平台全網聚合檢索與串流
- **多音源並行搜尋**：同時檢索 **網易雲音樂**、**QQ 音樂**、**酷我音樂**、**酷狗音樂**、**汽水音樂**、**Apple Music** 與 **YouTube Music**，支援全部聚合檢索或依單一平台篩選。
- **多平台同步登入**：支援網易雲音樂（掃碼 / Cookie）、QQ 音樂（掃碼 / Cookie）、YouTube Music（Google 裝置授權碼 / Cookie），以及 **酷我音樂**、**酷狗音樂**、**汽水音樂** 網頁登入（掃碼 / 手機號 / 帳號密碼）與 Cookie 匯入，解鎖會員與完整音質。
- **智能音源回退與斷流自癒**：當曲目因版權或網路受限無法直接串流時，自動跨庫匹配酷我完整音訊流、Apple Music AAC 或網易雲音訊流無感接續播放。

### 2. URL Time Passport 統一帳號系統
- 整合 **URL Time Passport SSO**（`https://login.urltime.cn`），透過本機環回 HTTP 回調伺服器完成桌面端安全授權登入、權杖刷新與用戶資料同步。

### 3. 發現歌單與每日推薦
- **發現歌單與全部榜單**：支援依分類（`全部`、`華語`、`流行`、`搖滾`、`電子`、`輕音樂`、`ACG`、`治愈`）瀏覽全網熱門歌單與一鍵「換一批」，並提供「全部榜單」視圖，點擊即可進入歌單／榜單詳情試聽或備份至本地歌單。
- **每日推薦**：首頁結合線上每日精選與本地收聽紀錄即時生成推薦曲目，支援日期標記與一鍵換批刷新。

### 4. 本地無損曲庫與歌單管理
- **Rust 原生音訊引擎**：採用 `rodio` 進行本地音訊播放，搭配 `lofty` 讀取標籤與內嵌專輯封面、`walkdir` 掃描本地資料夾。
- **SQLite 本地持久化**：透過 `rusqlite` 儲存本地曲庫、播放歷史、收藏歌曲與自訂歌單。
- **零門檻點選互動**：支援點選加入歌單、編輯封面與資料、外部歌單連結匯入、JSON 匯出，以及專屬毛玻璃確認彈窗刪除歌單。

### 5. Liquid Glass 液態玻璃介面與音效設定
- **7 套內建主題**：Apple 深空黑、賽博霓虹、極光蒼穹、落日暮光、深海幽藍、櫻花迷霧與晨曦純白（淺色模式），支援頂部快捷切換與多語系（簡體中文 / 繁體中文 / English）。
- **細節打磨**：高飽和折射浮層（播放佇列、彈窗與自訂液態玻璃右鍵選單）、歌曲右鍵快速操作（立即播放、下一首播放、加入播放佇列、加入歌單、紅心收藏、搜尋同歌手）、全螢幕歌詞自訂背景、無邊框視窗拖曳控制，以及 `Ctrl + K` 全域搜尋快捷鍵。
- **完整播放控制**：交叉漸入漸出（Crossfade）、聲音增強器、音量平衡、無損／串流音質、杜比全景聲與靜音自動字幕。

---

## 🛠️ 技術架構

| 層級 | 技術選型 |
| :--- | :--- |
| **前端框架** | Vue 3 (`<script setup>`)、TypeScript、Vite 8、Pinia、Vue Router |
| **UI 與樣式** | Tailwind CSS、Lucide Icons (`lucide-vue-next`)、`@vueuse/core`、`qrcode` |
| **桌面執行時** | Tauri 2 (`tauri`、`tauri-plugin-opener`) |
| **音訊與資料** | Rust `rodio`、`lofty`、`walkdir`、`rusqlite` (bundled SQLite)、`rfd` |
| **網路與認證** | `reqwest`、`tokio`（含本機 SSO 回調服務）、`aes`、`cbc`、`base64` |

---

## 📁 專案結構

```text
time-music-player/
├── public/
│   ├── logo.svg                    # 向量應用圖示與網頁 Favicon
│   └── logo.png                    # 1024×1024 高解析度桌面應用圖示來源
├── src/                            # Vue 3 前端原始碼
│   ├── components/                 # 播放器列、標題列、側邊欄、全螢幕歌詞、播放佇列、登入與歌單彈窗
│   ├── i18n/                       # 多語系翻譯設定
│   ├── router/                     # 頁面路由
│   ├── stores/                     # Pinia 狀態庫（player / provider / passport / library / playlist / settings 等）
│   ├── types/                      # TypeScript 型別與平台定義
│   ├── utils/                      # 系統媒體控制 (SMTC / MPRIS) 與工具函式
│   └── views/                      # 首頁、發現音樂、資料庫、歌單詳情、系統設定
├── src-tauri/                      # Tauri / Rust 後端原始碼
│   ├── icons/                      # 全平台應用圖示集（.ico / .icns / PNG）
│   └── src/
│       ├── audio.rs                # 本地音訊播放引擎（rodio）
│       ├── db.rs                   # SQLite 本地資料庫
│       ├── scanner.rs              # 本地資料夾掃描與中繼資料解析
│       ├── importer.rs             # 外部歌單連結匯入
│       ├── passport.rs             # URL Time Passport SSO 認證用戶端
│       ├── netease.rs              # 網易雲音樂用戶端（含發現歌單與每日推薦）
│       ├── qqmusic.rs              # QQ 音樂用戶端
│       ├── kuwo.rs                 # 酷我音樂用戶端
│       ├── kugou.rs                # 酷狗音樂用戶端
│       ├── qishui.rs               # 汽水音樂用戶端
│       ├── applemusic.rs           # Apple Music / iTunes 檢索用戶端
│       ├── ytmusic.rs              # YouTube Music 用戶端
│       ├── ai.rs                   # AI 智能功能模組
│       ├── storage.rs              # 快取與儲存空間管理
│       └── lib.rs                  # Tauri 指令註冊與應用程式進入點
├── package.json
└── LICENSE
```

---

## 🚀 開發與建置

### 環境需求
- [Node.js](https://nodejs.org/) 與 [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/) 工具鏈
- Tauri 2 系統依賴（Windows 需具備 WebView2 與 MSVC Build Tools）

### 安裝與啟動

```bash
# 安裝前端依賴
pnpm install

# 啟動完整 Tauri 桌面開發環境（含前端熱重載）
pnpm tauri dev

# 或僅啟動前端 Vite 開發伺服器
pnpm dev
```

### 檢查與打包發布

```bash
# 前端型別檢查與建置
pnpm build

# Rust 編譯檢查
cargo check --manifest-path src-tauri/Cargo.toml

# 打包正式安裝檔（Windows 產出 NSIS .exe 與 .msi 安裝檔）
pnpm tauri build
```

打包產物預設輸出於 `src-tauri/target/release/bundle/` 目錄下。

---

## ⌨️ 快捷鍵

| 快捷鍵 | 功能 |
| :--- | :--- |
| `Ctrl + K` / `Cmd + K` | 聚焦頂部全域搜尋欄（再按 `Esc` 清空／失焦） |
| `Space` | 播放 / 暫停 |
| `Ctrl + ←` / `Ctrl + →` | 上一首 / 下一首 |
| `Ctrl + ↑` / `Ctrl + ↓` | 調高 / 調低音量 |
| `Esc` | 關閉全螢幕歌詞或播放佇列 |

---

## 📄 授權條款

本專案採用 [MIT License](LICENSE) 授權。
