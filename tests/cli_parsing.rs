use muxd::cli::{Cli, Commands, LaunchArgs, resolve_launch_request};
use muxd::config::{FileConfig, LaunchDefaults};
use muxd::model::{Backend, LaunchRequest, Target};
use std::path::PathBuf;

#[test]
fn parses_minimal_launch_command() {
    let cli = Cli::parse_from([
        "muxd",
        "launch",
        "--session",
        "work",
        "--target",
        "new-pane",
        "--",
        "echo",
        "hello",
    ]);

    match cli.command {
        Commands::Launch(args) => {
            assert_eq!(args.session.as_deref(), Some("work"));
            assert_eq!(args.tab, None);
            assert!(!args.ensure_session);
            assert!(!args.ensure_tab);
            assert_eq!(args.payload, vec!["echo".to_string(), "hello".to_string()]);
        }
    }
}

#[test]
fn parses_launch_command_with_optional_fields() {
    let cli = Cli::parse_from([
        "muxd",
        "launch",
        "--session",
        "work",
        "--tab",
        "triage",
        "--ensure-tab",
        "--target",
        "new-pane",
        "--cwd",
        "/repo",
        "--name",
        "nightly-report",
        "--",
        "make",
        "report",
    ]);

    match cli.command {
        Commands::Launch(args) => {
            assert_eq!(args.tab.as_deref(), Some("triage"));
            assert!(args.ensure_tab);
            assert_eq!(args.cwd, Some(PathBuf::from("/repo")));
            assert_eq!(args.name.as_deref(), Some("nightly-report"));
            assert_eq!(args.payload, vec!["make".to_string(), "report".to_string()]);
        }
    }
}

#[test]
fn converts_launch_args_into_request() {
    let args = LaunchArgs {
        backend: Some(muxd::cli::BackendArg::Zellij),
        session: Some("work".to_string()),
        tab: Some("triage".to_string()),
        ensure_session: true,
        ensure_tab: true,
        target: Some(muxd::cli::TargetArg::NewPane),
        cwd: Some(PathBuf::from("/repo")),
        name: Some("nightly-report".to_string()),
        payload: vec!["make".to_string(), "report".to_string()],
    };

    let request = LaunchRequest::try_from(args).expect("request should be valid");

    assert_eq!(request.backend, Backend::Zellij);
    assert_eq!(request.session, "work");
    assert_eq!(request.tab.as_deref(), Some("triage"));
    assert!(request.ensure_session);
    assert!(request.ensure_tab);
    assert_eq!(request.target, Target::NewPane);
    assert_eq!(request.cwd, Some(PathBuf::from("/repo")));
    assert_eq!(request.name.as_deref(), Some("nightly-report"));
    assert_eq!(request.command, "make");
    assert_eq!(request.args, vec!["report".to_string()]);
}

#[test]
fn rejects_invalid_target() {
    let result = Cli::try_parse_from([
        "muxd",
        "launch",
        "--session",
        "work",
        "--target",
        "floating-pane",
        "--",
        "echo",
        "hello",
    ]);

    assert!(result.is_err());
}

#[test]
fn rejects_missing_payload_command() {
    let result = Cli::try_parse_from([
        "muxd",
        "launch",
        "--session",
        "work",
        "--target",
        "new-pane",
    ]);

    assert!(result.is_err());
}

#[test]
fn resolves_missing_session_and_target_from_config() {
    let args = LaunchArgs {
        backend: None,
        session: None,
        tab: None,
        ensure_session: false,
        ensure_tab: false,
        target: None,
        cwd: None,
        name: Some("nightly-report".to_string()),
        payload: vec!["make".to_string(), "report".to_string()],
    };

    let config = FileConfig {
        defaults: LaunchDefaults {
            backend: Some(Backend::Zellij),
            session: Some("work".to_string()),
            tab: Some("triage".to_string()),
            target: Some(Target::NewPane),
            cwd: Some(PathBuf::from("/repo")),
        },
    };

    let request = resolve_launch_request(args, &config).expect("request should resolve");

    assert_eq!(request.backend, Backend::Zellij);
    assert_eq!(request.session, "work");
    assert_eq!(request.tab.as_deref(), Some("triage"));
    assert_eq!(request.target, Target::NewPane);
    assert_eq!(request.cwd, Some(PathBuf::from("/repo")));
    assert_eq!(request.name.as_deref(), Some("nightly-report"));
}

#[test]
fn cli_values_override_config_values() {
    let args = LaunchArgs {
        backend: Some(muxd::cli::BackendArg::Zellij),
        session: Some("cli-session".to_string()),
        tab: Some("cli-tab".to_string()),
        ensure_session: false,
        ensure_tab: false,
        target: Some(muxd::cli::TargetArg::NewPane),
        cwd: Some(PathBuf::from("/cli")),
        name: None,
        payload: vec!["echo".to_string(), "hello".to_string()],
    };

    let config = FileConfig {
        defaults: LaunchDefaults {
            backend: Some(Backend::Zellij),
            session: Some("config-session".to_string()),
            tab: Some("config-tab".to_string()),
            target: Some(Target::NewPane),
            cwd: Some(PathBuf::from("/config")),
        },
    };

    let request = resolve_launch_request(args, &config).expect("request should resolve");

    assert_eq!(request.session, "cli-session");
    assert_eq!(request.tab.as_deref(), Some("cli-tab"));
    assert_eq!(request.cwd, Some(PathBuf::from("/cli")));
}

#[test]
fn rejects_missing_session_after_merge() {
    let args = LaunchArgs {
        backend: None,
        session: None,
        tab: None,
        ensure_session: false,
        ensure_tab: false,
        target: Some(muxd::cli::TargetArg::NewPane),
        cwd: None,
        name: None,
        payload: vec!["echo".to_string(), "hello".to_string()],
    };

    let error = resolve_launch_request(args, &FileConfig::default())
        .expect_err("session should be required");
    assert_eq!(error, "session is required");
}

#[test]
fn rejects_ensure_tab_without_tab_name() {
    let args = LaunchArgs {
        backend: None,
        session: Some("work".to_string()),
        tab: None,
        ensure_session: false,
        ensure_tab: true,
        target: Some(muxd::cli::TargetArg::NewPane),
        cwd: None,
        name: None,
        payload: vec!["echo".to_string(), "hello".to_string()],
    };

    let error =
        resolve_launch_request(args, &FileConfig::default()).expect_err("tab should be required");
    assert_eq!(error, "tab is required when --ensure-tab is used");
}
