use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Request {
    command: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    cwd: String,
}

#[derive(Debug, Serialize)]
struct Response {
    ok: bool,
    exit_code: i32,
    stdout: String,
    stderr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = std::env::args()
        .nth(1)
        .ok_or("usage: muxd-stack-spike-rust <socket-path>")?;
    let _ = fs::remove_file(&socket_path);

    let listener = UnixListener::bind(&socket_path)?;
    let (mut stream, _) = listener.accept()?;

    let request = read_request(&stream)?;
    let response = run(request);
    write_response(&mut stream, &response)?;

    fs::remove_file(&socket_path).ok();
    Ok(())
}

fn read_request(
    stream: &std::os::unix::net::UnixStream,
) -> Result<Request, Box<dyn std::error::Error>> {
    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut line)?;

    let request: Request = serde_json::from_str(&line)?;
    if request.command.is_empty() {
        return Err("command is required".into());
    }

    Ok(request)
}

fn run(request: Request) -> Response {
    let mut command = Command::new(&request.command);
    command.args(&request.args);
    if !request.cwd.is_empty() {
        command.current_dir(&request.cwd);
    }

    match command.output() {
        Ok(output) => {
            let code = output.status.code().unwrap_or(1);
            Response {
                ok: output.status.success(),
                exit_code: code,
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                error: if output.status.success() {
                    None
                } else {
                    Some(format!("subprocess exited with status {code}"))
                },
            }
        }
        Err(error) => Response {
            ok: false,
            exit_code: 1,
            stdout: String::new(),
            stderr: String::new(),
            error: Some(error.to_string()),
        },
    }
}

fn write_response(
    stream: &mut std::os::unix::net::UnixStream,
    response: &Response,
) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = serde_json::to_string(response)?;
    stream.write_all(encoded.as_bytes())?;
    stream.write_all(b"\n")?;
    Ok(())
}
