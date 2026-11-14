use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub input: PathBuf,
    pub output_dir: PathBuf,
    pub app_name: String,
    pub package: String,
    pub version_name: String,
}
