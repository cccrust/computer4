請用 rust+tauri 寫一個 web 型的作業系統視窗介面

1. 要類似 iOS/android 的那種適合手機的介面設計。
2. 每個 app 都要盡量獨立，最好能夠規定好規格，然後讓外部撰寫者也能寫 APP 安裝進去。
    * 也就是預設要有 APP manager.
3. 要預設有 File Explorer, Terminal, Editor, Browser

請先規劃寫在 _doc/todo.md 中


## ccc: 如果用用 tauri 開發一個 OS gui ，然後想再建個網站 server ，讓前端的 web 介面與 tauri 的前端介面共用，可行嗎？

這是一個非常有趣的架構設計！簡單直接的回答是：**完全可行，但需要考慮「環境差異」與「安全性」的邊界。**

將 Web 服務與 Tauri 前端結合，通常有兩種主流實現方式。我們來拆解如何達成「程式碼共用」以及需要注意的架構挑戰。

---

### 1. 架構模式：如何共用程式碼？

要讓「網頁版（Server）」與「桌面版（Tauri）」共用前端介面，最推薦的做法是使用 **Monorepo（單一代碼庫）架構**：

* **共用層 (Shared/Components)：** 將 UI 元件（Buttons, Cards, Inputs 等）和商業邏輯（API 定義、狀態管理）封裝成一個獨立的套件或資料夾。
* **Web 應用 (Web App)：** 作為一個標準的 Web 專案（如 Next.js/Vite），透過 HTTP API 呼叫你的 Server。
* **Tauri 應用 (Tauri App)：** 包含相同的 UI 套件，但透過 Tauri 的 `invoke` 機制呼叫本地的 Rust 後端。

---

### 2. 關鍵挑戰：如何處理「API 呼叫」的差異？

這是最核心的問題：**在 Web 端你需要走網路請求 (REST/gRPC/GraphQL)，但在 Tauri 端你通常走本地 Rust 函式呼叫。**

#### 解決方案：抽象層 (Abstraction Layer)

你可以建立一個「服務接口層 (Service Layer)」，根據環境自動切換模式：

```typescript
// 偽代碼：服務處理器
import { invoke } from '@tauri-apps/api/core';

export async function fetchUserData(userId: string) {
  // 檢查是否在 Tauri 環境中
  if (window.__TAURI__) {
    // 呼叫 Rust 後端
    return await invoke('get_user_data', { userId });
  } else {
    // 走傳統 HTTP API
    const res = await fetch(`/api/users/${userId}`);
    return await res.json();
  }
}

```

---

### 3. 可行性與注意事項

#### 優點：

* **維護成本低：** UI 變更只需要修改一次。
* **一致性：** 確保網頁版與桌面版的視覺體驗完全同步。

#### 需要特別注意的「坑」：

1. **OS 權限邊界：** Tauri 的強大在於它可以讀寫檔案、操作 OS 指令。**這些功能在網頁版上是不存在的。** 你必須在 UI 層處理這些「功能缺失」的狀態（例如：網頁版時隱藏「讀取本地硬碟」的按鈕）。
2. **安全性（極重要）：** * 如果你將 Tauri 的 Rust 後端邏輯直接變成 Web API，**必須重新考慮驗證與授權**。本地應用通常信任用戶，但網路服務必須防禦惡意攻擊。
* 不要直接把 Rust 的內部函數暴露給外部網路，應該透過專門的 API Gateways (如 Axum 或 Actix-web) 來處理 Web 請求。


3. **路由系統：** 若使用 Next.js 等伺服器端渲染 (SSR) 框架，需注意 Tauri 是一個「純客戶端」應用，你需要確保 UI 程式碼可以在 Static Export 模式下運行。

---

### 4. 推薦的技術路徑

如果你要構建 OS GUI 類型的應用：

1. **前端框架：** 建議選擇 **Vite + React/Vue/Svelte**。Vite 非常適合 Tauri，且這類專案很容易透過靜態部署 (Static Export) 發布到網站上。
2. **後端串接：** * **在 Tauri 中：** 直接用 `tauri::command`。
* **在網頁 Server 中：** 使用 **Axum** (Rust) 或 Node.js/Go 撰寫對應的 API。*提示：因為你已經在 Tauri 用了 Rust，在 Web Server 也使用 Rust (Axum) 可以讓你共用資料結構 (Types)！*


3. **狀態管理：** 使用 `Zustand` 或 `TanStack Query`，它們能很好地幫你處理 API 請求與本地狀態同步，讓網頁版和 Tauri 版在面對資料獲取時行為一致。

### 總結建議

這是一個非常專業的架構。若你的目標是「高效能桌面端 + 雲端同步/網頁版」，建議將 **Rust 邏輯層 (Business Logic)** 與 **UI 層** 分離得越徹底越好。

你有考慮過這兩者之間需要「即時同步」資料嗎？還是兩者是獨立的操作場景？