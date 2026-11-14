@echo off
setlocal
title APK-Builder
cd /d "%~dp0"

where cargo >nul 2>nul
if errorlevel 1 (
    echo [x] Rust was not found. Install it from https://rustup.rs
    pause
    exit /b 1
)

echo [*] Preparing APK-Builder...
cargo build --release --quiet
if errorlevel 1 (
    echo [x] Failed to compile APK-Builder.
    pause
    exit /b 1
)

echo.
"target\release\apk-builder.exe" %*

echo.
pause
