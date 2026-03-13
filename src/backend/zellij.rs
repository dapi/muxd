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
        ensure_session_ready(runtime, request)?;
        ensure_tab_ready(runtime, request)?;

        let command = build_launch_command(request);
        let result = runtime.run(&command)?;
        if result.status == 0 {
            return Ok(LaunchSuccess {
                backend: "zellij",
                session: request.session.clone(),
                tab: request.tab.clone(),
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

pub fn ensure_session_ready<R: Runtime>(
    runtime: &R,
    request: &LaunchRequest,
) -> Result<(), MuxdError> {
    if session_exists(runtime, &request.session)? {
        return Ok(());
    }

    if !request.ensure_session {
        return Err(MuxdError::ResourceUnavailable(format!(
            "zellij session {:?} not found",
            request.session
        )));
    }

    let output = runtime.run(&build_create_session_command(&request.session))?;
    if output.status == 0 {
        return Ok(());
    }

    Err(MuxdError::WorkspaceSetupFailed(stderr_or_status(
        output.status,
        &output.stderr,
        format!("failed to create zellij session {:?}", request.session),
    )))
}

pub fn ensure_session_exists<R: Runtime>(runtime: &R, session: &str) -> Result<(), MuxdError> {
    if session_exists(runtime, session)? {
        Ok(())
    } else {
        Err(MuxdError::ResourceUnavailable(format!(
            "zellij session {:?} not found",
            session
        )))
    }
}

fn session_exists<R: Runtime>(runtime: &R, session: &str) -> Result<bool, MuxdError> {
    let output = runtime.run(&CommandSpec::new("zellij", ["list-sessions"]))?;
    if output.status != 0 {
        return Err(MuxdError::BackendUnavailable(
            "unable to query zellij sessions".to_string(),
        ));
    }

    Ok(session_exists_in_output(&output.stdout, session))
}

pub fn session_exists_in_output(output: &str, session: &str) -> bool {
    output
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .any(|candidate| candidate == session)
}

pub fn ensure_tab_ready<R: Runtime>(runtime: &R, request: &LaunchRequest) -> Result<(), MuxdError> {
    let Some(tab) = &request.tab else {
        return Ok(());
    };

    if request.ensure_tab {
        let output = runtime.run(&build_select_tab_command(&request.session, tab, true))?;
        if output.status == 0 {
            return Ok(());
        }

        return Err(MuxdError::WorkspaceSetupFailed(stderr_or_status(
            output.status,
            &output.stderr,
            format!(
                "failed to select or create zellij tab {:?} in session {:?}",
                tab, request.session
            ),
        )));
    }

    if !tab_exists(runtime, &request.session, tab)? {
        return Err(MuxdError::ResourceUnavailable(format!(
            "zellij tab {:?} not found in session {:?}",
            tab, request.session
        )));
    }

    let output = runtime.run(&build_select_tab_command(&request.session, tab, false))?;
    if output.status == 0 {
        return Ok(());
    }

    Err(MuxdError::WorkspaceSetupFailed(stderr_or_status(
        output.status,
        &output.stderr,
        format!(
            "failed to select zellij tab {:?} in session {:?}",
            tab, request.session
        ),
    )))
}

fn tab_exists<R: Runtime>(runtime: &R, session: &str, tab: &str) -> Result<bool, MuxdError> {
    let output = runtime.run(&build_query_tab_names_command(session))?;
    if output.status != 0 {
        return Err(MuxdError::BackendUnavailable(format!(
            "unable to query zellij tabs for session {:?}",
            session
        )));
    }

    Ok(tab_exists_in_output(&output.stdout, tab))
}

pub fn tab_exists_in_output(output: &str, tab: &str) -> bool {
    output
        .lines()
        .map(str::trim)
        .any(|candidate| candidate == tab)
}

pub fn build_create_session_command(session: &str) -> CommandSpec {
    CommandSpec::new("zellij", ["attach", "--create-background", session])
}

pub fn build_query_tab_names_command(session: &str) -> CommandSpec {
    CommandSpec::new("zellij", ["-s", session, "action", "query-tab-names"])
}

pub fn build_select_tab_command(session: &str, tab: &str, create: bool) -> CommandSpec {
    let mut spec = CommandSpec::new("zellij", ["-s", session, "action", "go-to-tab-name"]);
    if create {
        spec.args.push("--create".to_string());
    }
    spec.args.push(tab.to_string());
    spec
}

pub fn build_launch_command(request: &LaunchRequest) -> CommandSpec {
    let mut spec = if request.tab.is_some() {
        CommandSpec::new(
            "zellij",
            ["-s", request.session.as_str(), "action", "new-pane"],
        )
    } else {
        CommandSpec::new("zellij", ["-s", request.session.as_str(), "run"])
    };

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

fn stderr_or_status(status: i32, stderr: &str, fallback: String) -> String {
    if stderr.is_empty() {
        format!("{fallback} (exit status {status})")
    } else {
        stderr.to_string()
    }
}
