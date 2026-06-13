#!/usr/bin/env bash
# run.sh — 啟動 SMS4（可選模式）
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
DB="${SMS4_DB:-sms4-dev.db}"
PORT="${SMS4_PORT:-8080}"
MODE="${1:-dev}"

usage() {
    echo "用法: $0 [dev|prod|api|build]"
    echo ""
    echo "  dev     開發模式 (預設) — API 伺服器 + Vite 前端 dev server"
    echo "  prod    Production 模式 — 建置前端後，一體伺服 API + 靜態檔案"
    echo "  api     僅啟動 API 伺服器 (dev 模式)"
    echo "  build   僅建置前端"
    exit 1
}

cleanup() {
    echo ""
    echo "正在停止服務..."
    kill $API_PID 2>/dev/null || true
    wait $API_PID 2>/dev/null || true
}
trap cleanup EXIT INT TERM

case "$MODE" in
    dev)
        echo "=== SMS4 開發模式 ==="
        echo "資料庫: $DB"
        echo "API 埠: $PORT"

        if [ ! -d "$DIR/web/node_modules" ]; then
            echo "安裝前端依賴..."
            (cd "$DIR/web" && npm install)
        fi
        if [ ! -f "$DB" ]; then
            echo "初始化資料庫..."
            SMS4_DB="$DB" cargo run -- init
        fi

        echo "啟動 API 伺服器 (port $PORT)..."
        SMS4_DB="$DB" cargo run -- web --port "$PORT" --dev &
        API_PID=$!
        sleep 2

        echo "啟動前端 dev server (port 5173)..."
        echo ""
        echo "  後端 API: http://127.0.0.1:$PORT"
        echo "  前端:     http://127.0.0.1:5173"
        echo ""
        (cd "$DIR/web" && npm run dev)
        ;;

    prod)
        echo "=== SMS4 Production 模式 ==="
        echo "資料庫: $DB"
        echo "埠號:   $PORT"

        if [ ! -d "$DIR/web/node_modules" ]; then
            echo "安裝前端依賴..."
            (cd "$DIR/web" && npm install)
        fi
        if [ ! -d "$DIR/web/dist" ]; then
            echo "建置前端..."
            (cd "$DIR/web" && npm run build)
        fi
        if [ ! -f "$DB" ]; then
            echo "初始化資料庫..."
            SMS4_DB="$DB" cargo run -- init
        fi

        echo "啟動伺服器 (API + 靜態檔案)..."
        echo ""
        echo "  http://127.0.0.1:$PORT"
        echo ""
        SMS4_DB="$DB" cargo run -- web --port "$PORT"
        ;;

    api)
        echo "=== SMS4 API 模式 ==="
        echo "資料庫: $DB"
        echo "埠號:   $PORT"

        if [ ! -f "$DB" ]; then
            echo "初始化資料庫..."
            SMS4_DB="$DB" cargo run -- init
        fi

        echo "啟動 API 伺服器 (dev 模式)..."
        echo ""
        echo "  API: http://127.0.0.1:$PORT"
        echo ""
        SMS4_DB="$DB" cargo run -- web --port "$PORT" --dev
        ;;

    build)
        echo "=== 建置前端 ==="
        if [ ! -d "$DIR/web/node_modules" ]; then
            echo "安裝前端依賴..."
            (cd "$DIR/web" && npm install)
        fi
        (cd "$DIR/web" && npm run build)
        echo "前端已建置至 web/dist/"
        ;;

    *)
        usage
        ;;
esac
