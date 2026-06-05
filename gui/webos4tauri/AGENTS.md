# webos4tauri

Tauri + React 型 web OS shell，運行於 macOS/Windows。

## 開發

```bash
./run.sh              # 開發模式（自動启动 npm dev + tauri dev）
npm run build         # 前端建置
npx tauri build       # 完整建置（含原生後端）
npm run lint          # ESLint
npm test              # Vitest（src/test/）
```

## 架構

- **前端**: React 19 + TypeScript + Vite + TailwindCSS + Zustand
- **後端**: Tauri 2 (Rust) — `src-tauri/src/lib.rs` 是進入點
- **前端入口**: `src/main.tsx`，根元件 `src/App.tsx`
- **Tauri commands**: `src-tauri/src/commands/` 下，註冊於 `lib.rs:9-14`
  - `app_manager::list_installed_apps`
  - `app_manager::get_app_manifest`
  - `app_manager::get_app_entry_url`
  - `app_manager::execute_shell`

## 前端結構

```
src/
├── App.tsx              # 根元件
├── api/                 # 與 Tauri 後端溝通的 API 層
├── apps/                # 安裝的 web apps
├── components/Shell/    # StatusBar, HomeScreen, Dock, AppContainer
├── store/appStore.ts     # Zustand 狀態管理
├── test/                # Vitest 測試（setup 在 src/test/setup.ts）
└── types/
```

## 測試

```bash
npm test              # 跑 Vitest（所有測試）
vitest run src/test/editor.test.tsx  # 跑單一測試檔
vitest run --watch   # 監聽模式
```

### 測試架構

- **框架**: Vitest + jsdom + @testing-library/react
- **設定檔**: `vite.config.ts:16-21`
- **Setup**: `src/test/setup.ts` — 模擬 Tauri API 呼叫
- **命名**: `src/test/**/*.test.ts` 或 `.test.tsx`

### Mock 規則（setup.ts）

所有 Tauri 相關模組皆已 mock，無需實際呼叫後端：

| 模組 | Mock 內容 |
|------|-----------|
| `@tauri-apps/api/core` | `invoke: vi.fn()` |
| `@tauri-apps/plugin-fs` | `readTextFile`, `writeTextFile`, `readDir`, `mkdir`, `remove`, `rename`, `exists` |
| `@tauri-apps/api/path` | `documentDir: mockResolvedValue('/Users/test/Documents')`, `join` |
| `lucide-react` | 回傳 icon 元件名稱（而非實際元件） |

### 測試範例

```tsx
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { vi } from 'vitest'
import * as fs from '@tauri-apps/plugin-fs'

const mockFs = vi.mocked(fs)

describe('Component', () => {
  beforeEach(() => {
    vi.clearAllMocks()  // 每個測試前重置 mock
  })

  it('renders and handles interaction', async () => {
    mockFs.readTextFile.mockResolvedValue('file content')

    render(<MyComponent />)

    fireEvent.click(screen.getByText('Open'))

    await waitFor(() => {
      expect(screen.getByText('expected')).toBeInTheDocument()
    })
  })
})
```

### 常見陷阱

- 用 `waitFor()` 包 async 斷言（避免 flaky test）
- `vi.clearAllMocks()` 而非 `vi.resetAllMocks()`（保留 mock 設定）
- Lucide icon 在 mock 後是純文字，斷言用 `getByText('IconName')` 而非 `getByRole`

## 注意

- Tauri 2，需要 `tauri.conf.json` 設定 dev port 1420、build target `dist/`
- 前端嚴格使用 `src-tauri/target/` 外的 `dist/` 目錄
- 原始碼註解使用繁體中文