use std::env;
use std::path::Path;
use std::process::Command;

use crate::error::MuxdError;
use crate::model::CommandSpec;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommandOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

pub trait Runtime {
    fn command_exists(&self, program: &str) -> bool;
    fn run(&self, command: &CommandSpec) -> Result<CommandOutput, MuxdError>;
}

pub struct SystemRuntime;

impl Runtime for SystemRuntime {
    fn command_exists(&self, program: &str) -> bool {
        command_exists_in_path(program)
    }

    fn run(&self, command: &CommandSpec) -> Result<CommandOutput, MuxdError> {
        let mut process = Command::new(&command.program);
        process.args(&command.args);
        if let Some(cwd) = &command.cwd {
            process.current_dir(cwd);
        }

        let output = process.output().map_err(|error| {
            MuxdError::BackendUnavailable(format!("failed to run {}: {}", command.program, error))
        })?;

        Ok(CommandOutput {
            status: output.status.code().unwrap_or(1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

fn command_exists_in_path(program: &str) -> bool {
    let program_path = Path::new(program);
    if program_path.components().count() > 1 {
        return program_path.is_file();
    }

    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).any(|dir| executable_exists(&dir.join(program))))
        .unwrap_or(false)
}

fn executable_exists(path: &Path) -> bool {
    path.is_file()
}
