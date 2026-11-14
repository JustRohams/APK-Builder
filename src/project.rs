use crate::config::BuildConfig;
use crate::error::Result;
use crate::fail;
use crate::util::{copy_dir_recursive, slugify, xml_escape};
use std::fs;
use std::path::{Path, PathBuf};

const SETTINGS_GRADLE: &str = include_str!("../templates/settings.gradle.kts");
const ROOT_GRADLE: &str = include_str!("../templates/build.gradle.kts");
const GRADLE_PROPS: &str = include_str!("../templates/gradle.properties");
const APP_GRADLE: &str = include_str!("../templates/app/build.gradle.kts");
const MANIFEST: &str = include_str!("../templates/app/AndroidManifest.xml");
const MAIN_ACTIVITY: &str = include_str!("../templates/app/MainActivity.kt");

fn render(template: &str, vars: &[(&str, &str)]) -> String {
    vars.iter().fold(template.to_string(), |acc, (k, v)| {
        acc.replace(&format!("{{{{{k}}}}}"), v)
    })
}

fn write(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

pub fn work_dir(cfg: &BuildConfig) -> PathBuf {
    std::env::temp_dir()
        .join("apk-builder")
        .join(slugify(&cfg.package))
}

pub fn generate(cfg: &BuildConfig) -> Result<PathBuf> {
    let root = work_dir(cfg);
    let src = root.join("app").join("src");

    if src.exists() {
        fs::remove_dir_all(&src)?;
    }

    let vars = [
        ("PACKAGE", cfg.package.as_str()),
        ("VERSION", cfg.version_name.as_str()),
    ];

    write(&root.join("settings.gradle.kts"), &render(SETTINGS_GRADLE, &vars))?;
    write(&root.join("build.gradle.kts"), ROOT_GRADLE)?;
    write(&root.join("gradle.properties"), GRADLE_PROPS)?;
    write(&root.join("app/build.gradle.kts"), &render(APP_GRADLE, &vars))?;

    let main = src.join("main");
    write(&main.join("AndroidManifest.xml"), MANIFEST)?;
    write(
        &main.join("res/values/strings.xml"),
        &format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<resources>\n    <string name=\"app_name\">{}</string>\n</resources>\n",
            xml_escape(&cfg.app_name)
        ),
    )?;

    let pkg_path = cfg.package.replace('.', "/");
    write(
        &main.join("java").join(pkg_path).join("MainActivity.kt"),
        &render(MAIN_ACTIVITY, &vars),
    )?;

    let assets = main.join("assets");
    if cfg.input.is_file() {
        fs::create_dir_all(&assets)?;
        fs::copy(&cfg.input, assets.join("index.html"))?;
    } else {
        let n = copy_dir_recursive(&cfg.input, &assets)?;
        if n == 0 {
            fail!("The selected folder is empty.");
        }
    }

    Ok(root)
}
