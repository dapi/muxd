# muxd CLI and IPC

Date: 2026-03-13

## Purpose

This document captures the expected MVP contract for the local `muxd` CLI and daemon IPC.

It is more concrete than [design.md](../design.md) but still intentionally limited to local daemon behavior.

## CLI Commands

Expected command family:

- `muxd run`
- `muxd enqueue`
- `muxd get`
- `muxd list`
- `muxd status`
- `muxd cancel`

## CLI Semantics

### `muxd run`

Starts the local daemon and binds the Unix socket.

Expected behavior:

- fails if another daemon instance is already serving the same socket
- removes a stale socket before binding
- exits cleanly on SIGINT and SIGTERM

### `muxd enqueue`

Creates a task and returns its id.

Expected task fields supplied by CLI or daemon defaults:

- `prompt`
- `agent`
- `backend`
- `session`
- `target`
- `mode`
- `name`
- `cwd`

Expected useful flags:

- `--agent <name>`
- `--backend <name>`
- `--session <name>`
- `--target new_tab|new_pane|floating_pane`
- `--mode oneshot|interactive`
- `--name <task-name>`
- `--cwd <path>`
- `--wait`
- `--json`

`--wait` should be implemented client-side by polling `get` until the task reaches a terminal state.

### `muxd get`

Returns one task by id.

This exists mainly so scripts and `enqueue --wait` do not need to fetch the full task list repeatedly.

### `muxd list`

Lists tasks, filtered by state when requested.

Useful flags:

- `--state pending|running|completed|failed|cancelled`
- `--json`

Default human output should prefer active work over historical detail.

### `muxd status`

Returns daemon-level state such as:

- uptime
- pending count
- running count
- terminal task count if retained
- active backend defaults
- config path when applicable

### `muxd cancel`

Cancels a task by id.

Expected behavior:

- pending task: removed from runnable set and marked `cancelled`
- running task: best-effort backend cancellation only

## Exit Codes

Expected CLI exit codes:

| Code | Meaning |
| - | - |
| `0` | success |
| `1` | user or validation error |
| `2` | daemon unavailable |

For `enqueue --wait`, the client should return the task exit code when the task reaches `completed` or `failed` with a known backend exit status.

## Human Output

Example `enqueue` output:

```text
task-id: 01JARX5B2KQNR4T1GV0EDGP3HT
```

Example `list` output:

```text
ID         NAME          STATUS    BACKEND  SESSION  AGE
01JARX5B…  review-pr-42  running   zellij   zelda    2m
01JARX7K…  (unnamed)     pending   zellij   zelda    5s
```

Example `status` output:

```text
uptime: 2h 15m
queue: 1 pending, 2 running, 45 terminal
backend-default: zellij
config: ~/.config/muxd/config.kdl
```

## JSON Output

Example `enqueue --json` output:

```json
{"task_id":"01JARX5B2KQNR4T1GV0EDGP3HT","status":"pending"}
```

Example `get --json` output:

```json
{
  "id":"01JARX5B2KQNR4T1GV0EDGP3HT",
  "status":"running",
  "backend":"zellij",
  "session":"zelda",
  "target":"new_tab",
  "mode":"oneshot"
}
```

## IPC Transport

MVP transport is a local Unix socket using newline-delimited JSON.

Expected socket path policy:

- prefer `$XDG_RUNTIME_DIR/muxd/muxd.sock`
- fallback to `/tmp/muxd-$UID/muxd.sock`

The exact path helper should stay internal to the implementation, but the user-facing error messages should reference the resolved path.

## IPC Request Shape

Each request carries a protocol version.

```json
{"v":1,"method":"enqueue","params":{"prompt":"Review PR #42","agent":"claude","backend":"zellij","session":"zelda","target":"new_tab","mode":"oneshot"}}
{"v":1,"method":"get","params":{"task_id":"01JARX5B2KQNR4T1GV0EDGP3HT"}}
{"v":1,"method":"list","params":{"state":"running"}}
{"v":1,"method":"status"}
{"v":1,"method":"cancel","params":{"task_id":"01JARX5B2KQNR4T1GV0EDGP3HT"}}
```

## IPC Response Shape

Success:

```json
{"v":1,"ok":true,"result":{"task_id":"01JARX5B2KQNR4T1GV0EDGP3HT","status":"pending"}}
```

Failure:

```json
{"v":1,"ok":false,"error":{"code":"TASK_NOT_FOUND","message":"Task 01JARX5B2KQNR4T1GV0EDGP3HT not found"}}
```

## Error Codes

Recommended daemon error codes:

| Code | Meaning |
| - | - |
| `TASK_NOT_FOUND` | requested task id does not exist |
| `INVALID_PARAMS` | request parameters are invalid |
| `BACKEND_NOT_SUPPORTED` | backend is unknown or disabled |
| `SESSION_NOT_FOUND` | requested multiplexer session does not exist |
| `LAUNCH_FAILED` | backend launch command failed |
| `QUEUE_FULL` | pending queue limit was reached |
| `VERSION_MISMATCH` | client and daemon protocol versions differ |
| `INTERNAL_ERROR` | unexpected daemon failure |

## Relationship to Backend Docs

This document stays backend-neutral.

Backend-specific launch semantics, limitations, and monitoring behavior belong in backend docs such as:

- [backends/zellij.md](./backends/zellij.md)
