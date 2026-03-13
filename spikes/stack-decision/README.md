# Stack decision spikes

These are disposable comparison spikes for the implementation stack decision.

Both variants implement the same narrow flow:

1. bind a Unix socket
2. accept one JSON request
3. run a subprocess
4. return JSON with exit status
5. exit

## Request shape

```json
{"command":"sh","args":["-c","echo hello"],"cwd":""}
```

## Response shape

```json
{"ok":true,"exit_code":0,"stdout":"hello\n","stderr":""}
```

## Go

Build:

```bash
/tmp/muxd-go/go/bin/go build -o /tmp/muxd-go-spike ./spikes/stack-decision/go
```

Run:

```bash
SOCK=/tmp/muxd-go-spike.sock
rm -f "$SOCK"
/tmp/muxd-go-spike "$SOCK" &
python3 - <<'PY'
import json, socket, time
sock_path = "/tmp/muxd-go-spike.sock"
for _ in range(50):
    try:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.connect(sock_path)
        break
    except OSError:
        time.sleep(0.05)
else:
    raise SystemExit("socket not ready")
s.sendall((json.dumps({"command": "sh", "args": ["-c", "echo hello"]}) + "\n").encode())
print(s.recv(4096).decode(), end="")
s.close()
PY
wait
```

## Rust

Build:

```bash
cargo build --manifest-path spikes/stack-decision/rust/Cargo.toml
```

Run:

```bash
SOCK=/tmp/muxd-rust-spike.sock
rm -f "$SOCK"
./spikes/stack-decision/rust/target/debug/muxd-stack-spike-rust "$SOCK" &
python3 - <<'PY'
import json, socket, time
sock_path = "/tmp/muxd-rust-spike.sock"
for _ in range(50):
    try:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.connect(sock_path)
        break
    except OSError:
        time.sleep(0.05)
else:
    raise SystemExit("socket not ready")
s.sendall((json.dumps({"command": "sh", "args": ["-c", "echo hello"]}) + "\n").encode())
print(s.recv(4096).decode(), end="")
s.close()
PY
wait
```
