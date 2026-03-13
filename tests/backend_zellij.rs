use muxd::backend::zellij::{build_launch_command, session_exists_in_output};
use muxd::model::{Backend, LaunchRequest, Target};
use std::path::PathBuf;

#[test]
fn session_exists_parser_matches_first_column() {
    let output = "work [Created 1h ago]\nother [Created 2h ago]\n";
    assert!(session_exists_in_output(output, "work"));
    assert!(!session_exists_in_output(output, "missing"));
}

#[test]
fn build_launch_command_includes_name_cwd_and_payload() {
    let request = LaunchRequest {
        backend: Backend::Zellij,
        session: "work".to_string(),
        target: Target::NewPane,
        cwd: Some(PathBuf::from("/repo")),
        name: Some("nightly-report".to_string()),
        command: "make".to_string(),
        args: vec!["report".to_string()],
    };

    let command = build_launch_command(&request);

    assert_eq!(command.program, "zellij");
    assert_eq!(
        command.args,
        vec![
            "-s",
            "work",
            "run",
            "--name",
            "nightly-report",
            "--cwd",
            "/repo",
            "--",
            "make",
            "report",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>()
    );
    assert_eq!(command.cwd, Some(PathBuf::from("/repo")));
}
