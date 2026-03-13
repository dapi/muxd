use muxd::cli::{Cli, Commands, LaunchArgs};
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
            assert_eq!(args.session, "work");
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
            assert_eq!(args.cwd, Some(PathBuf::from("/repo")));
            assert_eq!(args.name.as_deref(), Some("nightly-report"));
            assert_eq!(args.payload, vec!["make".to_string(), "report".to_string()]);
        }
    }
}

#[test]
fn converts_launch_args_into_request() {
    let args = LaunchArgs {
        backend: muxd::cli::BackendArg::Zellij,
        session: "work".to_string(),
        target: muxd::cli::TargetArg::NewPane,
        cwd: Some(PathBuf::from("/repo")),
        name: Some("nightly-report".to_string()),
        payload: vec!["make".to_string(), "report".to_string()],
    };

    let request = LaunchRequest::try_from(args).expect("request should be valid");

    assert_eq!(request.backend, Backend::Zellij);
    assert_eq!(request.session, "work");
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
