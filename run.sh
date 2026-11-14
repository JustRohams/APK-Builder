#!/usr/bin/env bash
# APK-Builder launcher for Linux / macOS
set -e
cd "$(dirname "$0")"
command -v cargo >/dev/null || { echo "[x] Rust was not found. Install it from https://rustup.rs"; exit 1; }
echo "[*] Preparing APK-Builder..."
cargo build --release --quiet
echo
./target/release/apk-builder "$@"
