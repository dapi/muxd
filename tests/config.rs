use muxd::config::{FileConfig, default_config_path_from, load_from_path};
use muxd::model::{Backend, Target};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn default_config_path_prefers_xdg_config_home() {
    let dir = TempDir::new().expect("temp dir");
    let path = default_config_path_from(Some(PathBuf::from(dir.path())), None)
        .expect("path should resolve");
    assert_eq!(path, dir.path().join("muxd").join("config.toml"));
}

#[test]
fn load_returns_default_when_config_is_missing() {
    let dir = TempDir::new().expect("temp dir");
    let config = load_from_path(&dir.path().join("muxd").join("config.toml"))
        .expect("missing config should be fine");
    assert_eq!(config, FileConfig::default());
}

#[test]
fn load_parses_defaults_from_config_file() {
    let dir = TempDir::new().expect("temp dir");
    let config_dir = dir.path().join("muxd");
    fs::create_dir_all(&config_dir).expect("config dir");
    fs::write(
        config_dir.join("config.toml"),
        r#"[defaults]
backend = "zellij"
session = "work"
tab = "triage"
target = "new-pane"
cwd = "/repo"
"#,
    )
    .expect("config write");

    let config = load_from_path(&config_dir.join("config.toml")).expect("config should parse");

    assert_eq!(config.defaults.backend, Some(Backend::Zellij));
    assert_eq!(config.defaults.session.as_deref(), Some("work"));
    assert_eq!(config.defaults.tab.as_deref(), Some("triage"));
    assert_eq!(config.defaults.target, Some(Target::NewPane));
    assert_eq!(
        config.defaults.cwd.as_deref(),
        Some(std::path::Path::new("/repo"))
    );
}

#[test]
fn invalid_config_returns_error() {
    let dir = TempDir::new().expect("temp dir");
    let config_dir = dir.path().join("muxd");
    fs::create_dir_all(&config_dir).expect("config dir");
    fs::write(
        config_dir.join("config.toml"),
        "[defaults]\nsession = \"work\"\ntarget = \"bad\"\n",
    )
    .expect("config write");

    let error = load_from_path(&config_dir.join("config.toml")).expect_err("config should fail");
    assert!(
        error.to_string().contains("invalid config at"),
        "unexpected error: {error}"
    );
}
