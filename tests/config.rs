use muxd::config::{FileConfig, default_config_path, load};
use muxd::model::{Backend, Target};
use std::fs;
use tempfile::TempDir;

#[test]
fn default_config_path_prefers_xdg_config_home() {
    let dir = TempDir::new().expect("temp dir");
    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", dir.path());
        std::env::remove_var("HOME");
    }

    let path = default_config_path().expect("path should resolve");
    assert_eq!(path, dir.path().join("muxd").join("config.toml"));
}

#[test]
fn load_returns_default_when_config_is_missing() {
    let dir = TempDir::new().expect("temp dir");
    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", dir.path());
    }

    let config = load().expect("missing config should be fine");
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
target = "new-pane"
cwd = "/repo"
"#,
    )
    .expect("config write");

    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", dir.path());
    }

    let config = load().expect("config should parse");

    assert_eq!(config.defaults.backend, Some(Backend::Zellij));
    assert_eq!(config.defaults.session.as_deref(), Some("work"));
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
        "[defaults]\ntarget = \"bad\"\n",
    )
    .expect("config write");

    unsafe {
        std::env::set_var("XDG_CONFIG_HOME", dir.path());
    }

    let error = load().expect_err("config should fail");
    assert!(
        error.to_string().contains("invalid config at"),
        "unexpected error: {error}"
    );
}
