use clap::{Parser, error::ErrorKind};
use muxd::app;
use muxd::cli::Cli;
use muxd::error::MuxdError;
use muxd::exit_codes::ProcessExitCode;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let code = if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) {
                ProcessExitCode::Success.as_u8()
            } else {
                ProcessExitCode::InvalidInput.as_u8()
            };
            error.print().expect("failed to print clap error");
            return ExitCode::from(code);
        }
    };

    match app::run(cli) {
        Ok(success) => {
            let display_name = success.name.as_deref().unwrap_or(&success.session);
            println!("launched: {}", display_name);
            println!("backend: {}", success.backend);
            println!("session: {}", success.session);
            if let Some(tab) = &success.tab {
                println!("tab: {}", tab);
            }
            println!("target: {}", success.target);
            ExitCode::from(ProcessExitCode::Success.as_u8())
        }
        Err(error) => render_error(error),
    }
}

fn render_error(error: MuxdError) -> ExitCode {
    eprintln!("error: {error}");
    ExitCode::from(error.exit_code())
}
