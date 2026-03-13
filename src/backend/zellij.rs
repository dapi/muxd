use std::path::Path;

use crate::app::LaunchSuccess;
use crate::error::MuxdError;
use crate::model::{Backend, CommandSpec, LaunchRequest, Target};
use crate::runtime::Runtime;

pub struct ZellijBackend;

impl ZellijBackend {
    pub fn launch<R: Runtime>(
        runtime: &R,
        request: &LaunchRequest,
    ) -> Result<LaunchSuccess, MuxdError> {
        if request.backend != Backend::Zellij {
            return Err(MuxdError::InvalidInput(
                "only the zellij backend is supported",
            ));
        }

        if request.target != Target::NewPane {
            return Err(MuxdError::ResourceUnavailable(format!(
                "target {:?} is not supported in stage 1",
                request.target
            )));
        }

        ensure_zellij_available(runtime)?;
        ensure_session_exists(runtime, &request.session)?;

        let command = build_launch_command(request);
        let result = runtime.run(&command)?;
        if result.status == 0 {
            return Ok(LaunchSuccess {
                backend: "zellij",
                session: request.session.clone(),
                target: "new_pane",
                name: request.name.clone(),
            });
        }

        Err(MuxdError::LaunchFailed(if result.stderr.is_empty() {
            format!("zellij launch failed with exit status {}", result.status)
        } else {
            result.stderr
        }))
    }
}

pub fn ensure_zellij_available<R: Runtime>(runtime: &R) -> Result<(), MuxdError> {
    if runtime.command_exists("zellij") {
        Ok(())
    } else {
        Err(MuxdError::BackendUnavailable(
            "zellij is not available in PATH".to_string(),
        ))
    }
}

pub fn ensure_session_exists<R: Runtime>(runtime: &R, session: &str) -> Result<(), MuxdError> {
    let output = runtime.run(&CommandSpec::new("zellij", ["list-sessions"]))?;
    if output.status != 0 {
        return Err(MuxdError::BackendUnavailable(
            "unable to query zellij sessions".to_string(),
        ));
    }

    if session_exists_in_output(&output.stdout, session) {
        Ok(())
    } else {
        Err(MuxdError::ResourceUnavailable(format!(
            "zellij session {:?} not found",
            session
        )))
    }
}

pub fn session_exists_in_output(output: &str, session: &str) -> bool {
    output
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .any(|candidate| candidate == session)
}

pub fn build_launch_command(request: &LaunchRequest) -> CommandSpec {
    let mut spec = CommandSpec::new("zellij", ["-s", request.session.as_str(), "run"]);

    if let Some(name) = &request.name {
        spec.args.push("--name".to_string());
        spec.args.push(name.clone());
    }

    if let Some(cwd) = &request.cwd {
        spec.args.push("--cwd".to_string());
        spec.args.push(path_as_string(cwd));
        spec.cwd = Some(cwd.clone());
    }

    spec.args.push("--".to_string());
    spec.args.push(request.command.clone());
    spec.args.extend(request.args.clone());
    spec
}

fn path_as_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
