set -x
# 開發模式
RUST_BACKTRACE=1 npx tauri dev

# 或直接執行已編譯的 app
# open src-tauri/target/release/bundle/macos/webOS.app