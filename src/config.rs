use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::MuxdError;
use crate::model::{Backend, Target};

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize)]
pub struct FileConfig {
    #[serde(default)]
    pub defaults: LaunchDefaults,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize)]
pub struct LaunchDefaults {
    pub backend: Option<Backend>,
    pub session: Option<String>,
    pub target: Option<Target>,
    pub cwd: Option<PathBuf>,
}

pub fn load() -> Result<FileConfig, MuxdError> {
    let Some(path) = default_config_path() else {
        return Ok(FileConfig::default());
    };

    load_from_path(&path)
}

pub fn load_from_path(path: &Path) -> Result<FileConfig, MuxdError> {
    match fs::read_to_string(path) {
        Ok(contents) => toml::from_str(&contents).map_err(|error| {
            MuxdError::InvalidConfig(format!("invalid config at {}: {}", path.display(), error))
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(FileConfig::default()),
        Err(error) => Err(MuxdError::InvalidConfig(format!(
            "unable to read config at {}: {}",
            path.display(),
            error
        ))),
    }
}

pub fn default_config_path() -> Option<PathBuf> {
    default_config_path_from(
        std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from),
        std::env::var_os("HOME").map(PathBuf::from),
    )
}

pub fn default_config_path_from(
    xdg_config_home: Option<PathBuf>,
    home: Option<PathBuf>,
) -> Option<PathBuf> {
    if let Some(xdg) = xdg_config_home {
        return Some(xdg.join("muxd").join("config.toml"));
    }

    home.map(|home| home.join(".config").join("muxd").join("config.toml"))
}
