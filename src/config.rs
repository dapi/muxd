use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

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

    match fs::read_to_string(&path) {
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
    if let Some(xdg) = env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg).join("muxd").join("config.toml"));
    }

    env::var_os("HOME").map(|home| {
        PathBuf::from(home)
            .join(".config")
            .join("muxd")
            .join("config.toml")
    })
}
