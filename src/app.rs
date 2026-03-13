use crate::backend::zellij::ZellijBackend;
use crate::cli::{Cli, Commands, resolve_launch_request};
use crate::config;
use crate::error::MuxdError;
use crate::runtime::SystemRuntime;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LaunchSuccess {
    pub backend: &'static str,
    pub session: String,
    pub target: &'static str,
    pub name: Option<String>,
}

pub fn run(cli: Cli) -> Result<LaunchSuccess, MuxdError> {
    match cli.command {
        Commands::Launch(args) => {
            let config = config::load()?;
            let request = resolve_launch_request(args, &config).map_err(MuxdError::InvalidInput)?;
            let runtime = SystemRuntime;
            ZellijBackend::launch(&runtime, &request)
        }
    }
}
