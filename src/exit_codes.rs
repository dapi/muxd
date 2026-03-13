#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ProcessExitCode {
    Success = 0,
    InvalidInput = 1,
    BackendUnavailable = 2,
    ResourceUnavailable = 3,
    LaunchFailed = 4,
}

impl ProcessExitCode {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
