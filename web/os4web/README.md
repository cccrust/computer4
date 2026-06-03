# os4web 🖥️

透過瀏覽器存取 Server 端作業系統的輕量 Web 桌面，以手機優先設計。

## 功能

- 📁 **檔案總管** — 瀏覽、新建、重新命名、刪除，右鍵選單整合
- 💻 **終端機** — 即時 WebSocket shell，命令歷史，Ctrl+C/L
- 📝 **編輯器** — 多分頁，行號，語法提示，Ctrl+S 儲存
- 🖱️ **右鍵 / 長按選單** — 在任何檔案直接開啟編輯器；在任何資料夾開啟終端機
- 📱 **手機優先 UI** — 觸控友善，滑入動畫，狀態列 + 工作列

## 快速開始

### 1. 安裝 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. 編譯並執行

```bash
cd os4web
cargo run --release
```

伺服器啟動後開啟瀏覽器前往：**http://localhost:3000**

### 3. 手機存取

確保手機與 server 在同一網路：

```
http://<server-ip>:3000
```

## 專案結構

```
os4web/
├── Cargo.toml
├── src/
│   ├── main.rs         # Axum 伺服器、路由、WebSocket 升級
│   ├── api.rs          # REST API：檔案列表/讀取/寫入/mkdir/delete/rename
│   ├── terminal.rs     # WebSocket Terminal：spawn shell，stdin/stdout 橋接
│   └── fs_ops.rs       # 額外 FS 工具（保留擴展）
└── static/
    └── index.html      # 完整前端（單檔，無需打包工具）
```

## API 端點

| 方法 | 路徑 | 說明 |
|------|------|------|
| GET | `/api/fs/list?path=...` | 列出目錄內容 |
| GET | `/api/fs/read?path=...` | 讀取檔案內容 |
| POST | `/api/fs/write` | 寫入檔案 `{path, content}` |
| POST | `/api/fs/mkdir` | 建立目錄 `{path}` |
| POST | `/api/fs/delete` | 刪除檔案/目錄 `{path}` |
| POST | `/api/fs/rename` | 重新命名 `{from, to}` |
| WS | `/ws/terminal` | WebSocket 終端機 |

## WebSocket 終端機訊息格式

**Client → Server:**
```json
{ "type": "init",  "cwd": "/home/user" }
{ "type": "input", "data": "ls -la\n" }
{ "type": "resize","cols": 80, "rows": 24 }
```

**Server → Client:**
```json
{ "type": "output", "data": "..." }
```

## 依賴

- **axum** 0.7 — Web 框架 + WebSocket
- **tokio** 1 — 非同步執行環境
- **tower-http** — CORS
- **serde/serde_json** — JSON 序列化
- **dashmap** — 執行緒安全 HashMap（終端機 Session）
- **uuid** — Session ID

## 安全注意事項

⚠️ 本專案直接暴露 server 的 shell 與檔案系統，**僅供受信任的內網使用**。  
如需對外開放，請加上認證層（Basic Auth、JWT、反向代理等）。

## 擴展方向

- [ ] PTY 支援（需加入 `portable-pty` crate）以獲得完整終端機體驗
- [ ] 多終端機分頁
- [ ] 語法高亮（CodeMirror / Monaco）
- [ ] 檔案上傳 / 下載
- [ ] SSH 遠端連線支援
- [ ] 認證與 HTTPS
