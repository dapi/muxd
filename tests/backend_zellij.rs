use muxd::backend::zellij::{
    build_create_session_command, build_launch_command, build_query_tab_names_command,
    build_select_tab_command, session_exists_in_output, tab_exists_in_output,
};
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
        tab: None,
        ensure_session: false,
        ensure_tab: false,
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

#[test]
fn tab_exists_parser_matches_trimmed_lines() {
    let output = "triage\nbacklog\n";
    assert!(tab_exists_in_output(output, "triage"));
    assert!(!tab_exists_in_output(output, "missing"));
}

#[test]
fn build_create_session_command_uses_background_attach() {
    let command = build_create_session_command("issue-bot");

    assert_eq!(command.program, "zellij");
    assert_eq!(
        command.args,
        vec!["attach", "--create-background", "issue-bot"]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>()
    );
}

#[test]
fn build_query_tab_names_command_targets_session() {
    let command = build_query_tab_names_command("work");

    assert_eq!(command.program, "zellij");
    assert_eq!(
        command.args,
        vec!["-s", "work", "action", "query-tab-names"]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>()
    );
}

#[test]
fn build_select_tab_command_can_request_creation() {
    let command = build_select_tab_command("work", "triage", true);

    assert_eq!(command.program, "zellij");
    assert_eq!(
        command.args,
        vec![
            "-s",
            "work",
            "action",
            "go-to-tab-name",
            "--create",
            "triage"
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>()
    );
}

#[test]
fn build_launch_command_uses_new_pane_when_tab_is_selected() {
    let request = LaunchRequest {
        backend: Backend::Zellij,
        session: "work".to_string(),
        tab: Some("triage".to_string()),
        ensure_session: false,
        ensure_tab: false,
        target: Target::NewPane,
        cwd: Some(PathBuf::from("/repo")),
        name: Some("nightly-report".to_string()),
        command: "make".to_string(),
        args: vec!["report".to_string()],
    };

    let command = build_launch_command(&request);

    assert_eq!(
        command.args,
        vec![
            "-s",
            "work",
            "action",
            "new-pane",
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
}
