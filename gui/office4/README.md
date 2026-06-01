# Office 4

全端辦公套件，Rust WebSocket 後端 + 原生 HTML/JS 前端，支援多人即時協作。

## 模組

| 模組 | 說明 |
|------|------|
| 📊 試算表 | 50×26 格子、公式引擎、多人協作 |
| 🖥️ 投影片 | 文字 / 形狀 / 拖曳 / 播放 / 多人協作 |

## 啟動

```bash
# 終端 1 — 後端
./start.sh

# 終端 2 — 前端靜態伺服器
python3 -m http.server 8080 -d frontend

# 瀏覽器開啟
http://localhost:8080
```

## 結構

```
office4/
├── README.md
├── start.sh
├── frontend/
│   └── index.html          # 完整前端（無需建構工具）
└── backend/
    ├── Cargo.toml
    ├── Cargo.lock
    └── src/
        └── main.rs         # Rust WebSocket 伺服器（612 行）
```

## WebSocket 協定

### 試算表訊息
| 訊息 | 說明 |
|------|------|
| `update_cell` | 更新儲存格（值或公式） |
| `clear_cell` | 清除單格 |
| `clear_all` | 清除全部 |
| `add_rows` | 新增列 |
| `rename_sheet` | 重新命名 |

### 投影片訊息
| 訊息 | 說明 |
|------|------|
| `add_slide` / `delete_slide` | 新增 / 刪除投影片 |
| `set_background` | 設定背景色 / 漸層 |
| `add_text` / `update_text` / `delete_text` | 文字元素 CRUD |
| `add_shape` / `update_shape` / `delete_shape` | 形狀元素 CRUD |
| `add_image` / `update_image` / `delete_image` | 圖片元素 CRUD |
| `set_current_slide` | 同步當前頁 |
| `update_notes` | 備忘稿 |

### 支援公式
`=SUM` `=AVG` `=MAX` `=MIN` `=COUNT` 以及 `+` `-` `*` `/` 運算式

## 快捷鍵（試算表）
方向鍵移動、Enter/F2 編輯、Tab 橫移、Esc 取消、Delete 清除

## 快捷鍵（投影片播放）
← → 換頁、Space 下一頁、Esc 結束播放
