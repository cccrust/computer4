# WebOS4Tauri — 開發計畫

## 目標
以 Rust + Tauri v2 打造一個類似 iOS/Android 的 Web 型作業系統介面，適合手機/平板操作，並允許第三方開發者編寫獨立 App 安裝執行。

---

## 一、技術選型

| 層級 | 技術 | 說明 |
|------|------|------|
| 框架 | Tauri v2 | Rust 後端 + Web 前端 |
| 前端 | React + TypeScript + Vite | Shell UI |
| 樣式 | Tailwind CSS | 快速響應式 UI |
| 狀態 | Zustand | 輕量狀態管理 |
| 圖標 | Lucide React | 統一圖標集 |
| 終端 | xterm.js | Terminal App |
| 編輯器 | CodeMirror 6 | Editor App |
| 瀏覽器 | Tauri WebviewWindow | 獨立 webview |

---

## 二、階段規劃

### Phase 1 — 專案初始化與 Shell 核心 (Week 1)

- [ ] 1.1 使用 Tauri v2 初始化專案
- [ ] 1.2 建立 React + Vite + TypeScript 前端
- [ ] 1.3 實作 Shell 核心元件
  - [ ] StatusBar (時間、電量、訊號)
  - [ ] HomeScreen (桌面網格 + App Icon)
  - [ ] Dock (常用 App 快捷列)
  - [ ] AppContainer (iframe 容器／App 執行環境)
- [ ] 1.4 定義 App Manifest 規格 (JSON schema)
- [ ] 1.5 實作 App Manager (Rust commands)
  - `list_apps` — 列出已安裝 App
  - `get_app_manifest` — 讀取 App manifest
  - `install_app` — 安裝 App (zip/tar)
  - `uninstall_app` — 移除 App

### Phase 2 — App 規格與通訊協定 (Week 2)

- [ ] 2.1 定義 Shell-App 通訊協定 (postMessage)
- [ ] 2.2 實作 App API (提供給 App 呼叫的服務)
  - `app.fs.readFile(path)`
  - `app.fs.writeFile(path, data)`
  - `app.fs.listDir(path)`
  - `app.notification.show(title, body)`
  - `app.window.resize(width, height)`
  - `app.window.close()`
- [ ] 2.3 實作權限系統 (Permissions)
- [ ] 2.4 撰寫 Developer Guide 與範例 App

### Phase 3 — 內建 App (Week 3)

- [ ] 3.1 **File Explorer**
  - 檔案列表 (grid/list)
  - 目錄導航 (麵包屑)
  - 新增/重新命名/刪除/複製/貼上
  - 檔案類型圖標
- [ ] 3.2 **Terminal**
  - xterm.js 整合
  - Tauri shell plugin 串接
  - 支援 ANSI 色彩
  - 歷史命令
- [ ] 3.3 **Editor**
  - CodeMirror 6 整合
  - 語法高亮 (多語言)
  - 開啟/儲存檔案
  - 行號、搜尋
- [ ] 3.4 **Browser**
  - Tauri multiwebview 開啟新視窗
  - 網址列、前進、後退、重新整理
  - 書籤功能

### Phase 4 — 系統功能強化 (Week 4)

- [ ] 4.1 App 安裝流程 UI (可從 File Explorer 安裝)
- [ ] 4.2 多視窗／多工切換 (App Switcher)
- [ ] 4.3 通知中心 (Notification Center)
- [ ] 4.4 設定 App (Settings)
  - Wi-Fi 設定 (placeholder)
  - 顯示與亮度
  - 關於本機
- [ ] 4.5 螢幕鎖定 / 解鎖畫面

### Phase 5 — 第三方開發者工具 (Week 5)

- [ ] 5.1 CLI 工具 `webos-cli` (Rust binary)
  - `webos-cli init my-app` — 建立 App 骨架
  - `webos-cli build my-app` — 打包為 `.wapp` 格式
  - `webos-cli install my-app.wapp` — 安裝到模擬器
- [ ] 5.2 App 模板 (template)
- [ ] 5.3 撰寫完整開發者文件

---

## 三、App Manifest 規格

```json
{
  "appId": "com.example.myapp",
  "name": "My App",
  "version": "1.0.0",
  "icon": "icon.png",
  "entry": "index.html",
  "permissions": [
    "fs:read",
    "fs:write",
    "notification"
  ],
  "orientation": "portrait",
  "description": "A sample app",
  "author": "Developer Name"
}
```

- **appId**: 唯一識別碼 (反向域名)
- **entry**: App 入口 HTML (相對於 App 目錄)
- **permissions**: 請求的權限列表
- **orientation**: portrait / landscape / both
- App 安裝目錄: `~/.webos/apps/<appId>/`

---

## 四、Shell-App 通訊協定

使用 `window.postMessage` 雙向通訊：

**App → Shell (Request)**
```json
{
  "type": "request",
  "id": "uuid",
  "method": "fs.readFile",
  "params": { "path": "/home/file.txt" }
}
```

**Shell → App (Response)**
```json
{
  "type": "response",
  "id": "uuid",
  "result": { "data": "..." },
  "error": null
}
```

**Shell → App (Event)**
```json
{
  "type": "event",
  "event": "app:focus",
  "data": {}
}
```

---

## 五、目錄結構

```
webos4tauri/
├── _doc/                    # 文件
├── src-tauri/               # Rust 後端
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── app_manager.rs
│   │   │   ├── file_system.rs
│   │   │   ├── terminal.rs
│   │   │   └── system.rs
│   │   └── models/
│   │       ├── mod.rs
│   │       └── app.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                     # 前端
│   ├── main.tsx
│   ├── App.tsx
│   ├── components/
│   │   ├── Shell/
│   │   │   ├── HomeScreen.tsx
│   │   │   ├── AppIcon.tsx
│   │   │   ├── StatusBar.tsx
│   │   │   ├── Dock.tsx
│   │   │   └── AppContainer.tsx
│   │   └── common/
│   ├── apps/                # 內建 App 前端
│   │   ├── file-explorer/
│   │   ├── terminal/
│   │   ├── editor/
│   │   └── browser/
│   ├── store/
│   ├── api/
│   └── types/
├── public/
│   └── apps/                # 外部 App 存放目錄
├── package.json
├── tsconfig.json
├── vite.config.ts
└── tailwind.config.js
```

---

## 六、Rust 後端 commands 大綱

| Command | 說明 |
|---------|------|
| `list_installed_apps` | 列出所有已安裝 App |
| `read_app_manifest(app_id)` | 讀取指定 App 的 manifest |
| `install_app(source_path)` | 從 .wapp 檔案安裝 App |
| `uninstall_app(app_id)` | 移除指定 App |
| `read_file(path)` | 讀取檔案內容 |
| `write_file(path, content)` | 寫入檔案 |
| `delete_file(path)` | 刪除檔案 |
| `create_directory(path)` | 建立目錄 |
| `list_directory(path)` | 列出目錄內容 |
| `rename_item(old, new)` | 重新命名 |
| `copy_item(src, dest)` | 複製 |
| `execute_command(command)` | 執行 shell 命令（Terminal） |
| `open_url_in_webview(url)` | 用新 webview 開啟 URL |
| `get_system_info()` | 系統資訊 (OS, CPU, RAM) |

---

## 七、完成標準 (Definition of Done)

- [ ] `cargo tauri dev` 可正常啟動
- [ ] Shell UI 在手機 viewport 下操作順暢
- [ ] App Manager 可列出/安裝/移除 App
- [ ] 4 個內建 App 可正常使用
- [ ] 外部 App 可透過 postMessage API 正常運作
- [ ] 權限系統有作用
