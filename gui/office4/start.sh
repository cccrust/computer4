#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
echo "🅾️  Building Office 4 server..."
cd "$SCRIPT_DIR/backend"
cargo build --release 2>&1
echo ""
echo "✅ Build complete!"
echo "🚀 Starting Office 4 server on ws://localhost:9001"
echo "📂 Open: http://localhost:8080 (after running python3 -m http.server 8080 -d frontend)"
echo ""
exec "$SCRIPT_DIR/backend/target/release/office4-server"
