use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Backend {
    Zellij,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Target {
    NewPane,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LaunchRequest {
    pub backend: Backend,
    pub session: String,
    pub tab: Option<String>,
    pub ensure_session: bool,
    pub ensure_tab: bool,
    pub target: Target,
    pub cwd: Option<PathBuf>,
    pub name: Option<String>,
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommandSpec {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
}

impl CommandSpec {
    pub fn new<I, S>(program: &str, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            program: program.to_string(),
            args: args.into_iter().map(Into::into).collect(),
            cwd: None,
        }
    }
}
