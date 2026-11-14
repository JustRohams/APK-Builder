<div align="center">

# 📦 APK-Builder

**Turn any HTML website into an installable Android APK — in one command.**

[![CI](https://github.com/JustRohams/APK-Builder/actions/workflows/ci.yml/badge.svg)](https://github.com/JustRohams/APK-Builder/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
![Rust](https://img.shields.io/badge/built%20with-Rust-orange?logo=rust)
![Platform](https://img.shields.io/badge/target-Android%207.0%2B-3DDC84?logo=android&logoColor=white)

</div>

---

## ✨ Features

- 🚀 **Fast** – a dependency-free Rust binary; rebuilds reuse the Gradle cache.
- 🖱️ **Simple** – double-click a launcher, answer two questions, get an APK.
- 🌐 **Full web support** – JavaScript, DOM storage, local assets (CSS, images, fonts, JS).
- 🧩 **Kotlin under the hood** – a clean native shell hosts your site in a `WebView`.
- 🔁 **Scriptable** – every prompt has a CLI flag for CI and automation.

## 🧠 How it works

```text
 your site (HTML/CSS/JS)
          │
          ▼
 ┌───────────────────┐   1. validate input
 │    APK-Builder    │   2. generate a Kotlin + WebView Android project
 │      (Rust)       │   3. copy your site into app/src/main/assets
 └─────────┬─────────┘   4. run Gradle (assembleDebug)
           ▼             5. copy the APK to your output folder
      your-app.apk
```

> Why a WebView instead of converting HTML to Kotlin code? A website is not
> source code that can be translated; it has to be *rendered*. A WebView shell
> keeps your site pixel-identical and is the fastest approach.

## 📋 Requirements

| Tool | Version | Notes |
| --- | --- | --- |
| [Rust](https://rustup.rs) | stable | Only needed to compile APK-Builder |
| [JDK](https://adoptium.net) | 17+ | Required by Android Gradle Plugin |
| [Gradle](https://gradle.org/install) | 8.7+ | Must be available in `PATH` |
| [Android SDK](https://developer.android.com/studio) | API 34 | Set `ANDROID_HOME` (Android Studio sets it up) |

> 💡 Accept the SDK licenses once: `sdkmanager --licenses`

## 🚀 Quick start

```bash
git clone https://github.com/JustRohams/APK-Builder.git
cd APK-Builder
```

Then run the launcher for your system:

| System | Command |
| --- | --- |
| Windows (CMD) | double-click `run.cmd` |
| Windows (PowerShell) | `./run.ps1` |
| Linux / macOS | `./run.sh` |

You will be asked:

```text
Path of the HTML file (or site folder) to build: C:\sites\portfolio
Folder where the APK should be saved: C:\apks
App name [portfolio]:
Package id [com.apkbuilder.portfolio]:
Start build? [Y/n]:
```

> 💡 You can drag & drop a file or folder into the terminal to paste its path.

### Input rules

- **Single `.html` file** → it becomes `index.html` of the app.
- **Folder** → the whole folder is bundled and must contain `index.html`.
  Use relative links (`./css/style.css`) for local assets.

## ⚙️ Command line options

```text
-i, --input <PATH>     HTML file or folder containing index.html
-o, --output <DIR>     Folder for the final APK
-n, --name <NAME>      App name (default: derived from input)
-p, --package <ID>     Android package id (default: com.apkbuilder.<name>)
-y, --yes              Skip the confirmation prompt
-h, --help             Show help
```

Example:

```bash
apk-builder -i ./my-site -o ./dist -n "My Site" -y
```

## 🗂️ Project structure

```text
APK-Builder/
├── src/
│   ├── main.rs        # entry point and build pipeline
│   ├── cli.rs         # prompts and argument parsing
│   ├── config.rs      # build configuration
│   ├── project.rs     # Android project generator
│   ├── builder.rs     # toolchain detection + Gradle runner
│   ├── util.rs        # helpers (copy, slug, validation)
│   └── error.rs       # shared error type
├── templates/         # Kotlin / Gradle / manifest templates
├── run.cmd · run.ps1 · run.sh
└── .github/workflows/ci.yml
```

## 🛠️ Troubleshooting

| Problem | Fix |
| --- | --- |
| `Java ... was not found` | Install JDK 17+ and reopen the terminal |
| `Gradle ... was not found` | Install Gradle 8.7+ and add it to `PATH` |
| `Android SDK not found` | Install Android Studio or set `ANDROID_HOME` |
| License errors | Run `sdkmanager --licenses` and accept all |
| Blank screen in the app | Make sure `index.html` uses relative paths |

## 📱 About the generated APK

The APK is **debug-signed**, so it installs directly on any device with
"Install unknown apps" enabled. For Play Store publishing you need a release
build signed with your own key (see the roadmap).

## 🗺️ Roadmap

- [ ] Custom app icon (`--icon`)
- [ ] Release signing with a keystore
- [ ] Splash screen and orientation options
- [ ] Automatic Gradle wrapper download
- [ ] Built-in offline cache / PWA support

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

## 📄 License

Released under the [MIT License](LICENSE) © 2026 [JustRohams](https://github.com/JustRohams).
