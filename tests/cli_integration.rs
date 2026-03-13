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
    let fake = FakeZellij::new("other\n");

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    command.env("PATH", fake.bin_dir()).args([
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
fn successful_launch_uses_expected_zellij_command_shape() {
    let fake = FakeZellij::new("work\n");
    let working_dir = TempDir::new().expect("working dir");

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    command
        .env("PATH", fake.bin_dir())
        .env("FAKE_ZELLIJ_LOG", fake.log_path())
        .args([
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
    assert!(log.contains("SESSION=work"));
    assert!(log.contains("ARGS=--name|nightly-report|--cwd|"));
    assert!(log.contains("--|echo|hello|"));
    assert!(log.contains(working_dir.path().to_str().expect("utf8 path")));
}

#[test]
fn launch_failure_returns_launch_failed_exit_code() {
    let fake = FakeZellij::new("work\n");

    let mut command = Command::cargo_bin("muxd").expect("binary should build");
    command
        .env("PATH", fake.bin_dir())
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
        .code(4)
        .stderr(predicate::str::contains("backend launch failed"));
}

struct FakeZellij {
    _root: TempDir,
    bin_dir: PathBuf,
    log_path: PathBuf,
}

impl FakeZellij {
    fn new(sessions_output: &str) -> Self {
        let root = TempDir::new().expect("temp dir");
        let bin_dir = root.path().join("bin");
        fs::create_dir(&bin_dir).expect("bin dir");
        let log_path = root.path().join("zellij.log");
        let script_path = bin_dir.join("zellij");
        fs::write(&script_path, fake_zellij_script(sessions_output)).expect("script write");
        let mut permissions = fs::metadata(&script_path)
            .expect("script metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&script_path, permissions).expect("script chmod");

        Self {
            _root: root,
            bin_dir,
            log_path,
        }
    }

    fn bin_dir(&self) -> &Path {
        &self.bin_dir
    }

    fn log_path(&self) -> &Path {
        &self.log_path
    }
}

fn fake_zellij_script(sessions_output: &str) -> String {
    format!(
        r#"#!/bin/bash
set -euo pipefail

if [[ "${{1:-}}" == "list-sessions" ]]; then
  printf '%s' '{sessions_output}'
  exit 0
fi

if [[ "${{1:-}}" == "-s" && "${{3:-}}" == "run" ]]; then
  session="${{2}}"
  shift 3
  log_file="${{FAKE_ZELLIJ_LOG:-}}"
  if [[ -n "$log_file" ]]; then
    {{
      printf 'PWD=%s\n' "$PWD"
      printf 'SESSION=%s\n' "$session"
      printf 'ARGS='
      for arg in "$@"; do
        printf '%s|' "$arg"
      done
      printf '\n'
    }} >> "$log_file"
  fi
  if [[ -n "${{FAKE_ZELLIJ_RUN_STDERR:-}}" ]]; then
    printf '%s\n' "${{FAKE_ZELLIJ_RUN_STDERR}}" >&2
  fi
  exit "${{FAKE_ZELLIJ_RUN_EXIT:-0}}"
fi

echo "unexpected invocation: $*" >&2
exit 99
"#
    )
}
