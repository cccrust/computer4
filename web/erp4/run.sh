#!/usr/bin/env bash
# run.sh — 啟動 ERP4 開發環境（API 伺服器 + Vite 前端）
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
DB="${ERP4_DB:-erp4-dev.db}"
PORT="${ERP4_PORT:-8080}"

cleanup() {
    echo ""
    echo "正在停止服務..."
    kill $API_PID 2>/dev/null || true
    wait $API_PID 2>/dev/null || true
}
trap cleanup EXIT INT TERM

echo "=== ERP4 開發環境啟動 ==="
echo "資料庫: $DB"
echo "API 埠: $PORT"

# 確保 npm 依賴已安裝
if [ ! -d "$DIR/web/node_modules" ]; then
    echo "安裝前端依賴..."
    (cd "$DIR/web" && npm install)
fi

# 確認資料庫已初始化（若不存在則自動 init）
if [ ! -f "$DB" ]; then
    echo "初始化資料庫..."
    ERP4_DB="$DB" "$DIR/target/release/erp4" init 2>/dev/null \
        || ERP4_DB="$DB" cargo run -- init
fi

# 啟動 API 伺服器 (背景)
echo "啟動 API 伺服器 (port $PORT)..."
ERP4_DB="$DB" cargo run -- web --port "$PORT" --dev &
API_PID=$!
sleep 2

# 啟動 Vite 前端 (前景)
echo "啟動前端 dev server (port 5173)..."
echo ""
echo "  後端 API: http://127.0.0.1:$PORT"
echo "  前端:     http://127.0.0.1:5173"
echo "  登入:     admin / admin123"
echo ""
(cd "$DIR/web" && npm run dev)
