use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

#[test]
fn help_is_available() {
    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    command.arg("launch").arg("--help");
    command
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: muxd launch"));
}

#[test]
fn missing_zellij_returns_backend_unavailable() {
    let empty_path = TempDir::new().expect("temp dir");

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    command.env("PATH", empty_path.path()).args([
        "launch",
        "--session",
        "work",
        "--target",
        "new-pane",
        "--",
        "echo",
        "hello",
    ]);

    command
        .assert()
        .code(2)
        .stderr(predicate::str::contains("zellij is not available in PATH"));
}

#[test]
fn missing_session_returns_resource_unavailable() {
    let fake = FakeZellij::new(&[]);

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command.args([
        "launch",
        "--session",
        "work",
        "--target",
        "new-pane",
        "--",
        "echo",
        "hello",
    ]);

    command
        .assert()
        .code(3)
        .stderr(predicate::str::contains("session \"work\" not found"));
}

#[test]
fn ensure_session_creates_missing_session_before_launch() {
    let fake = FakeZellij::new(&[]);

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command.args([
        "launch",
        "--session",
        "work",
        "--ensure-session",
        "--target",
        "new-pane",
        "--",
        "echo",
        "hello",
    ]);

    command.assert().success();

    let log = fs::read_to_string(fake.log_path()).expect("log should exist");
    assert!(log.contains("ACTION=create-session"));
    assert!(log.contains("SESSION=work"));
    assert!(log.contains("ACTION=run"));
}

#[test]
fn config_can_supply_session_target_cwd_and_tab() {
    let fake = FakeZellij::with_workspace(&["work"], &[("work", &["triage"])]);
    let config_home = TempDir::new().expect("config dir");
    let repo_dir = TempDir::new().expect("repo dir");
    write_config(
        &config_home,
        &format!(
            "[defaults]\nsession = \"work\"\ntab = \"triage\"\ntarget = \"new-pane\"\ncwd = \"{}\"\n",
            repo_dir.path().display()
        ),
    );

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command
        .env("XDG_CONFIG_HOME", config_home.path())
        .args(["launch", "--", "echo", "hello"]);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("tab: triage"));

    let log = fs::read_to_string(fake.log_path()).expect("log should exist");
    assert!(log.contains("ACTION=select-tab"));
    assert!(log.contains("TAB=triage"));
    assert!(log.contains("ACTION=new-pane"));
    assert!(log.contains("ARGS=--cwd|"));
    assert!(log.contains(repo_dir.path().to_str().expect("utf8 path")));
}

#[test]
fn cli_values_override_config_values() {
    let fake = FakeZellij::with_workspace(
        &["config-session", "cli-session"],
        &[
            ("config-session", &["config-tab"]),
            ("cli-session", &["cli-tab"]),
        ],
    );
    let config_home = TempDir::new().expect("config dir");
    let config_repo = TempDir::new().expect("config repo dir");
    let cli_repo = TempDir::new().expect("cli repo dir");
    write_config(
        &config_home,
        &format!(
            "[defaults]\nsession = \"config-session\"\ntab = \"config-tab\"\ntarget = \"new-pane\"\ncwd = \"{}\"\n",
            config_repo.path().display()
        ),
    );

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command.env("XDG_CONFIG_HOME", config_home.path()).args([
        "launch",
        "--session",
        "cli-session",
        "--tab",
        "cli-tab",
        "--target",
        "new-pane",
        "--cwd",
        cli_repo.path().to_str().expect("utf8 path"),
        "--",
        "echo",
        "hello",
    ]);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("tab: cli-tab"));

    let log = fs::read_to_string(fake.log_path()).expect("log should exist");
    assert!(log.contains("SESSION=cli-session"));
    assert!(log.contains("TAB=cli-tab"));
    assert!(log.contains(cli_repo.path().to_str().expect("utf8 path")));
    assert!(!log.contains("SESSION=config-session"));
    assert!(!log.contains("TAB=config-tab"));
}

#[test]
fn invalid_config_returns_invalid_input_exit_code() {
    let fake = FakeZellij::new(&["work"]);
    let config_home = TempDir::new().expect("config dir");
    write_config(
        &config_home,
        "[defaults]\ntarget = \"floating-pane\"\nsession = \"work\"\n",
    );

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command
        .env("XDG_CONFIG_HOME", config_home.path())
        .args(["launch", "--", "echo", "hello"]);

    command
        .assert()
        .code(1)
        .stderr(predicate::str::contains("invalid config"));
}

#[test]
fn successful_launch_uses_expected_zellij_command_shape() {
    let fake = FakeZellij::new(&["work"]);
    let working_dir = TempDir::new().expect("working dir");

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command.args([
        "launch",
        "--session",
        "work",
        "--target",
        "new-pane",
        "--cwd",
        working_dir.path().to_str().expect("utf8 path"),
        "--name",
        "nightly-report",
        "--",
        "echo",
        "hello",
    ]);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("launched: nightly-report"));

    let log = fs::read_to_string(fake.log_path()).expect("log should exist");
    assert!(log.contains("PWD="));
    assert!(log.contains("ACTION=run"));
    assert!(log.contains("SESSION=work"));
    assert!(log.contains("ARGS=--name|nightly-report|--cwd|"));
    assert!(log.contains("--|echo|hello|"));
    assert!(log.contains(working_dir.path().to_str().expect("utf8 path")));
}

#[test]
fn launch_in_named_tab_uses_new_pane_action() {
    let fake = FakeZellij::with_workspace(&["work"], &[("work", &["triage"])]);
    let working_dir = TempDir::new().expect("working dir");

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command.args([
        "launch",
        "--session",
        "work",
        "--tab",
        "triage",
        "--target",
        "new-pane",
        "--cwd",
        working_dir.path().to_str().expect("utf8 path"),
        "--name",
        "issue-analysis",
        "--",
        "echo",
        "hello",
    ]);

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("tab: triage"));

    let log = fs::read_to_string(fake.log_path()).expect("log should exist");
    assert!(log.contains("ACTION=select-tab"));
    assert!(log.contains("TAB=triage"));
    assert!(log.contains("CREATED=false"));
    assert!(log.contains("ACTION=new-pane"));
    assert!(log.contains("ARGS=--name|issue-analysis|--cwd|"));
}

#[test]
fn missing_tab_returns_resource_unavailable() {
    let fake = FakeZellij::new(&["work"]);

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command.args([
        "launch",
        "--session",
        "work",
        "--tab",
        "triage",
        "--target",
        "new-pane",
        "--",
        "echo",
        "hello",
    ]);

    command
        .assert()
        .code(3)
        .stderr(predicate::str::contains("tab \"triage\" not found"));
}

#[test]
fn ensure_tab_creates_missing_tab_before_launch() {
    let fake = FakeZellij::new(&["work"]);

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command.args([
        "launch",
        "--session",
        "work",
        "--tab",
        "triage",
        "--ensure-tab",
        "--target",
        "new-pane",
        "--",
        "echo",
        "hello",
    ]);

    command.assert().success();

    let log = fs::read_to_string(fake.log_path()).expect("log should exist");
    assert!(log.contains("ACTION=select-tab"));
    assert!(log.contains("TAB=triage"));
    assert!(log.contains("CREATED=true"));
    assert!(log.contains("ACTION=new-pane"));
}

#[test]
fn session_creation_failure_returns_workspace_setup_failed() {
    let fake = FakeZellij::new(&[]);

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command.env("FAKE_ZELLIJ_ATTACH_EXIT", "17").env(
        "FAKE_ZELLIJ_ATTACH_STDERR",
        "failed to create background session",
    );
    command.args([
        "launch",
        "--session",
        "work",
        "--ensure-session",
        "--target",
        "new-pane",
        "--",
        "echo",
        "hello",
    ]);

    command.assert().code(4).stderr(predicate::str::contains(
        "failed to create background session",
    ));
}

#[test]
fn tab_creation_failure_returns_workspace_setup_failed() {
    let fake = FakeZellij::new(&["work"]);

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command
        .env("FAKE_ZELLIJ_GO_TO_TAB_EXIT", "18")
        .env("FAKE_ZELLIJ_GO_TO_TAB_STDERR", "failed to create tab");
    command.args([
        "launch",
        "--session",
        "work",
        "--tab",
        "triage",
        "--ensure-tab",
        "--target",
        "new-pane",
        "--",
        "echo",
        "hello",
    ]);

    command
        .assert()
        .code(4)
        .stderr(predicate::str::contains("failed to create tab"));
}

#[test]
fn launch_failure_returns_launch_failed_exit_code() {
    let fake = FakeZellij::new(&["work"]);

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    fake.apply_env(&mut command);
    command
        .env("FAKE_ZELLIJ_RUN_EXIT", "17")
        .env("FAKE_ZELLIJ_RUN_STDERR", "backend launch failed")
        .args([
            "launch",
            "--session",
            "work",
            "--target",
            "new-pane",
            "--",
            "echo",
            "hello",
        ]);

    command
        .assert()
        .code(5)
        .stderr(predicate::str::contains("backend launch failed"));
}

struct FakeZellij {
    _root: TempDir,
    bin_dir: PathBuf,
    log_path: PathBuf,
    sessions_file: PathBuf,
    tabs_dir: PathBuf,
}

impl FakeZellij {
    fn new(sessions: &[&str]) -> Self {
        Self::with_workspace(sessions, &[])
    }

    fn with_workspace(sessions: &[&str], tabs: &[(&str, &[&str])]) -> Self {
        let root = TempDir::new().expect("temp dir");
        let bin_dir = root.path().join("bin");
        fs::create_dir(&bin_dir).expect("bin dir");

        let state_dir = root.path().join("state");
        let tabs_dir = state_dir.join("tabs");
        fs::create_dir_all(&tabs_dir).expect("tabs dir");
        let sessions_file = state_dir.join("sessions.txt");
        fs::write(&sessions_file, sessions.join("\n")).expect("sessions state write");

        for (session, names) in tabs {
            let tab_file = tabs_dir.join(format!("{session}.txt"));
            fs::write(&tab_file, names.join("\n")).expect("tabs state write");
        }

        let log_path = root.path().join("zellij.log");
        let script_path = bin_dir.join("zellij");
        fs::write(&script_path, fake_zellij_script()).expect("script write");
        let mut permissions = fs::metadata(&script_path)
            .expect("script metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&script_path, permissions).expect("script chmod");

        Self {
            _root: root,
            bin_dir,
            log_path,
            sessions_file,
            tabs_dir,
        }
    }

    fn apply_env(&self, command: &mut Command) {
        let system_path = std::env::var("PATH").unwrap_or_default();
        let combined_path = if system_path.is_empty() {
            self.bin_dir().display().to_string()
        } else {
            format!("{}:{}", self.bin_dir().display(), system_path)
        };
        command
            .env("PATH", combined_path)
            .env("FAKE_ZELLIJ_LOG", self.log_path())
            .env("FAKE_ZELLIJ_SESSIONS_FILE", &self.sessions_file)
            .env("FAKE_ZELLIJ_TABS_DIR", &self.tabs_dir);
    }

    fn bin_dir(&self) -> &Path {
        &self.bin_dir
    }

    fn log_path(&self) -> &Path {
        &self.log_path
    }
}

fn fake_zellij_script() -> &'static str {
    r#"#!/bin/bash
set -euo pipefail

sessions_file="${FAKE_ZELLIJ_SESSIONS_FILE:?}"
tabs_dir="${FAKE_ZELLIJ_TABS_DIR:?}"
log_file="${FAKE_ZELLIJ_LOG:-}"

log_action() {
  if [[ -n "$log_file" ]]; then
    {
      printf 'PWD=%s\n' "$PWD"
      printf 'ACTION=%s\n' "$1"
      printf 'SESSION=%s\n' "$2"
      shift 2
      while [[ $# -gt 0 ]]; do
        printf '%s\n' "$1"
        shift
      done
    } >> "$log_file"
  fi
}

session_exists() {
  local session="$1"
  [[ -f "$sessions_file" ]] && grep -Fxq "$session" "$sessions_file"
}

session_tab_file() {
  printf '%s/%s.txt' "$tabs_dir" "$1"
}

tab_exists() {
  local session="$1"
  local tab="$2"
  local tab_file
  tab_file="$(session_tab_file "$session")"
  [[ -f "$tab_file" ]] && grep -Fxq "$tab" "$tab_file"
}

add_session() {
  local session="$1"
  if ! session_exists "$session"; then
    if [[ -s "$sessions_file" ]]; then
      printf '\n%s' "$session" >> "$sessions_file"
    else
      printf '%s' "$session" >> "$sessions_file"
    fi
  fi
  touch "$(session_tab_file "$session")"
}

add_tab() {
  local session="$1"
  local tab="$2"
  local tab_file
  tab_file="$(session_tab_file "$session")"
  touch "$tab_file"
  if ! tab_exists "$session" "$tab"; then
    if [[ -s "$tab_file" ]]; then
      printf '\n%s' "$tab" >> "$tab_file"
    else
      printf '%s' "$tab" >> "$tab_file"
    fi
  fi
}

if [[ "${1:-}" == "list-sessions" ]]; then
  [[ -f "$sessions_file" ]] && cat "$sessions_file"
  exit 0
fi

if [[ "${1:-}" == "attach" ]]; then
  shift
  session=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --create-background|--create)
        shift
        ;;
      -*)
        shift
        ;;
      *)
        session="$1"
        shift
        ;;
    esac
  done
  if [[ -n "${FAKE_ZELLIJ_ATTACH_STDERR:-}" ]]; then
    printf '%s\n' "${FAKE_ZELLIJ_ATTACH_STDERR}" >&2
  fi
  if [[ -n "${FAKE_ZELLIJ_ATTACH_EXIT:-}" ]]; then
    exit "${FAKE_ZELLIJ_ATTACH_EXIT}"
  fi
  add_session "$session"
  log_action "create-session" "$session"
  exit 0
fi

if [[ "${1:-}" == "-s" && "${3:-}" == "action" && "${4:-}" == "query-tab-names" ]]; then
  session="${2}"
  if ! session_exists "$session"; then
    echo "missing session" >&2
    exit 41
  fi
  tab_file="$(session_tab_file "$session")"
  [[ -f "$tab_file" ]] && cat "$tab_file"
  exit 0
fi

if [[ "${1:-}" == "-s" && "${3:-}" == "action" && "${4:-}" == "go-to-tab-name" ]]; then
  session="${2}"
  shift 4
  create="false"
  if [[ "${1:-}" == "--create" ]]; then
    create="true"
    shift
  fi
  tab="${1:-}"
  if ! session_exists "$session"; then
    echo "missing session" >&2
    exit 41
  fi
  if [[ -n "${FAKE_ZELLIJ_GO_TO_TAB_STDERR:-}" ]]; then
    printf '%s\n' "${FAKE_ZELLIJ_GO_TO_TAB_STDERR}" >&2
  fi
  if [[ -n "${FAKE_ZELLIJ_GO_TO_TAB_EXIT:-}" ]]; then
    exit "${FAKE_ZELLIJ_GO_TO_TAB_EXIT}"
  fi
  if [[ "$create" == "true" ]]; then
    add_tab "$session" "$tab"
    log_action "select-tab" "$session" "TAB=$tab" "CREATED=true"
    exit 0
  fi
  if tab_exists "$session" "$tab"; then
    log_action "select-tab" "$session" "TAB=$tab" "CREATED=false"
    exit 0
  fi
  echo "missing tab" >&2
  exit 42
fi

if [[ "${1:-}" == "-s" && "${3:-}" == "action" && "${4:-}" == "new-pane" ]]; then
  session="${2}"
  shift 4
  if [[ -n "${FAKE_ZELLIJ_NEW_PANE_STDERR:-}" ]]; then
    printf '%s\n' "${FAKE_ZELLIJ_NEW_PANE_STDERR}" >&2
  fi
  if [[ -n "${FAKE_ZELLIJ_NEW_PANE_EXIT:-}" ]]; then
    exit "${FAKE_ZELLIJ_NEW_PANE_EXIT}"
  fi
  if ! session_exists "$session"; then
    echo "missing session" >&2
    exit 41
  fi
  args_line="ARGS="
  for arg in "$@"; do
    args_line+="${arg}|"
  done
  log_action "new-pane" "$session" "$args_line"
  exit 0
fi

if [[ "${1:-}" == "-s" && "${3:-}" == "run" ]]; then
  session="${2}"
  shift 3
  if [[ -n "${FAKE_ZELLIJ_RUN_STDERR:-}" ]]; then
    printf '%s\n' "${FAKE_ZELLIJ_RUN_STDERR}" >&2
  fi
  if [[ -n "${FAKE_ZELLIJ_RUN_EXIT:-}" ]]; then
    exit "${FAKE_ZELLIJ_RUN_EXIT}"
  fi
  if ! session_exists "$session"; then
    echo "missing session" >&2
    exit 41
  fi
  args_line="ARGS="
  for arg in "$@"; do
    args_line+="${arg}|"
  done
  log_action "run" "$session" "$args_line"
  exit 0
fi

echo "unexpected invocation: $*" >&2
exit 99
"#
}

fn write_config(config_home: &TempDir, contents: &str) {
    let config_dir = config_home.path().join("muxd");
    fs::create_dir_all(&config_dir).expect("config dir");
    fs::write(config_dir.join("config.toml"), contents).expect("config write");
}
