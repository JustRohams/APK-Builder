use crate::config::BuildConfig;
use crate::error::Result;
use crate::fail;
use crate::util::slugify;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct Toolchain {
    gradle: &'static str,
    sdk: PathBuf,
}

#[cfg(windows)]
const GRADLE_BIN: &str = "gradle.bat";
#[cfg(not(windows))]
const GRADLE_BIN: &str = "gradle";

fn works(program: &str, arg: &str) -> bool {
    Command::new(program)
        .arg(arg)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn find_sdk() -> Option<PathBuf> {
    for var in ["ANDROID_HOME", "ANDROID_SDK_ROOT"] {
        if let Some(v) = std::env::var_os(var) {
            let p = PathBuf::from(v);
            if p.is_dir() {
                return Some(p);
            }
        }
    }
    let mut candidates = Vec::new();
    if let Some(local) = std::env::var_os("LOCALAPPDATA") {
        candidates.push(PathBuf::from(local).join("Android").join("Sdk"));
    }
    if let Some(home) = std::env::var_os("HOME") {
        let home = PathBuf::from(home);
        candidates.push(home.join("Android").join("Sdk"));
        candidates.push(home.join("Library").join("Android").join("sdk"));
    }
    candidates.into_iter().find(|p| p.is_dir())
}

pub fn detect() -> Result<Toolchain> {
    if !works("java", "-version") {
        fail!("Java (JDK 17+) was not found in PATH. Install it from https://adoptium.net");
    }
    if !works(GRADLE_BIN, "--version") {
        fail!("Gradle (8.7+) was not found in PATH. Install it from https://gradle.org/install");
    }
    let Some(sdk) = find_sdk() else {
        fail!("Android SDK not found. Install Android Studio or the command line tools and set ANDROID_HOME.");
    };
    Ok(Toolchain { gradle: GRADLE_BIN, sdk })
}

fn write_local_properties(root: &Path, sdk: &Path) -> Result<()> {
    let escaped = sdk.to_string_lossy().replace('\\', "\\\\");
    fs::write(root.join("local.properties"), format!("sdk.dir={escaped}\n"))?;
    Ok(())
}

pub fn build(tc: &Toolchain, root: &Path, cfg: &BuildConfig) -> Result<PathBuf> {
    write_local_properties(root, &tc.sdk)?;

    let status = Command::new(tc.gradle)
        .current_dir(root)
        .args([
            "assembleDebug",
            "--console=plain",
            "--parallel",
            "--build-cache",
            "--warning-mode=none",
        ])
        .status()?;
    if !status.success() {
        fail!("Gradle build failed (see the log above).");
    }

    let apk = root.join("app/build/outputs/apk/debug/app-debug.apk");
    if !apk.is_file() {
        fail!("Build finished but the APK was not found at {}", apk.display());
    }

    fs::create_dir_all(&cfg.output_dir)?;
    let dest = cfg.output_dir.join(format!("{}.apk", slugify(&cfg.app_name)));
    fs::copy(&apk, &dest)?;
    Ok(dest)
}
