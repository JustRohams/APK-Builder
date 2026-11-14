# APK-Builder launcher for PowerShell
$ErrorActionPreference = "Stop"
Set-Location -Path $PSScriptRoot
$Host.UI.RawUI.WindowTitle = "APK-Builder"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "[x] Rust was not found. Install it from https://rustup.rs" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host "[*] Preparing APK-Builder..."
cargo build --release --quiet
if ($LASTEXITCODE -ne 0) {
    Write-Host "[x] Failed to compile APK-Builder." -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
$exe = Join-Path $PSScriptRoot "target\release\apk-builder.exe"
& $exe @args

Write-Host ""
Read-Host "Press Enter to exit"
