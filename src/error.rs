use std::fmt::{Display, Formatter};

use crate::exit_codes::ProcessExitCode;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MuxdError {
    InvalidInput(&'static str),
    InvalidConfig(String),
    BackendUnavailable(String),
    ResourceUnavailable(String),
    LaunchFailed(String),
}

impl MuxdError {
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::InvalidInput(_) => ProcessExitCode::InvalidInput.as_u8(),
            Self::InvalidConfig(_) => ProcessExitCode::InvalidInput.as_u8(),
            Self::BackendUnavailable(_) => ProcessExitCode::BackendUnavailable.as_u8(),
            Self::ResourceUnavailable(_) => ProcessExitCode::ResourceUnavailable.as_u8(),
            Self::LaunchFailed(_) => ProcessExitCode::LaunchFailed.as_u8(),
        }
    }
}

impl Display for MuxdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(f, "{message}"),
            Self::InvalidConfig(message) => write!(f, "{message}"),
            Self::BackendUnavailable(message)
            | Self::ResourceUnavailable(message)
            | Self::LaunchFailed(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for MuxdError {}
