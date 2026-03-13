use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::config::FileConfig;
use crate::model::{Backend, LaunchRequest, Target};

#[derive(Debug, Parser)]
#[command(name = "muxd")]
#[command(about = "Thin launch wrapper for terminal multiplexers")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Launch(LaunchArgs),
}

#[derive(Debug, Clone, clap::Args)]
pub struct LaunchArgs {
    #[arg(long, value_enum)]
    pub backend: Option<BackendArg>,

    #[arg(long)]
    pub session: Option<String>,

    #[arg(long, value_enum)]
    pub target: Option<TargetArg>,

    #[arg(long)]
    pub cwd: Option<PathBuf>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(required = true, last = true, num_args = 1.., allow_hyphen_values = true)]
    pub payload: Vec<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
pub enum BackendArg {
    Zellij,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
pub enum TargetArg {
    NewPane,
}

impl Cli {
    pub fn parse_from<I, T>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        <Self as Parser>::parse_from(itr)
    }

    pub fn try_parse_from<I, T>(itr: I) -> Result<Self, clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        <Self as Parser>::try_parse_from(itr)
    }
}

impl TryFrom<LaunchArgs> for LaunchRequest {
    type Error = &'static str;

    fn try_from(args: LaunchArgs) -> Result<Self, Self::Error> {
        let (command, args_vec) = args
            .payload
            .split_first()
            .ok_or("payload command is required")?;

        Ok(LaunchRequest {
            backend: match args.backend {
                Some(BackendArg::Zellij) => Backend::Zellij,
                None => return Err("backend is required"),
            },
            session: args.session.ok_or("session is required")?,
            target: match args.target {
                Some(TargetArg::NewPane) => Target::NewPane,
                None => return Err("target is required"),
            },
            cwd: args.cwd,
            name: args.name,
            command: command.clone(),
            args: args_vec.to_vec(),
        })
    }
}

pub fn resolve_launch_request(
    args: LaunchArgs,
    config: &FileConfig,
) -> Result<LaunchRequest, &'static str> {
    let (command, args_vec) = args
        .payload
        .split_first()
        .ok_or("payload command is required")?;

    let backend = match args.backend {
        Some(BackendArg::Zellij) => Backend::Zellij,
        None => config.defaults.backend.unwrap_or(Backend::Zellij),
    };

    let session = args
        .session
        .or_else(|| config.defaults.session.clone())
        .ok_or("session is required")?;

    let target = match args.target {
        Some(TargetArg::NewPane) => Target::NewPane,
        None => config.defaults.target.ok_or("target is required")?,
    };

    Ok(LaunchRequest {
        backend,
        session,
        target,
        cwd: args.cwd.or_else(|| config.defaults.cwd.clone()),
        name: args.name,
        command: command.clone(),
        args: args_vec.to_vec(),
    })
}
