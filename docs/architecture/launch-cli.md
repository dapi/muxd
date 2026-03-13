# muxd Launch CLI

Date: 2026-03-13

## Purpose

This document captures the expected contract for the first `muxd` release: a thin launch wrapper over Zellij.

It is more concrete than [design.md](../design.md) and intentionally excludes daemon or IPC concerns.

## CLI Commands

Expected command family for the first release:

- `muxd launch`

Possible later additions:

- `muxd doctor`
- `muxd version`

## `muxd launch`

Launches an arbitrary command into an existing Zellij session.

Expected useful flags:

- `--session <name>`
- `--target <name>`
- `--cwd <path>`
- `--name <launch-name>`

Payload command:

- passed after `--`

Example:

```text
muxd launch --session work --target new_pane --cwd /repo --name nightly-report -- make report
```

## Expected Semantics

`muxd launch` should:

- validate required arguments
- validate that `zellij` exists in `PATH`
- validate that the requested session exists
- validate that the requested target is supported
- map the request to the correct backend command shape
- execute that backend command
- return a stable exit code to the caller

The first release should not:

- create a daemon
- assign task ids
- maintain state after launch
- expose queue semantics

## Exit Codes

Expected initial exit codes:

| Code | Meaning |
| - | - |
| `0` | launch request accepted and backend command executed successfully |
| `1` | invalid user input |
| `2` | backend or environment unavailable |
| `3` | requested session or target is unavailable |
| `4` | backend launch command failed |

The exact numeric mapping can still be refined during implementation, but it must be documented and stable.

## Human Output

Example success output:

```text
launched: nightly-report
backend: zellij
session: work
target: new_pane
```

Example failure output:

```text
error: zellij session "work" not found
```

## JSON Output

JSON output is optional for the first slice.

If added, it should be a direct mirror of launch result data, not a hidden task model.

## Integration with `systemd --user`

Example service `ExecStart`:

```text
/usr/local/bin/muxd launch --session work --target new_pane --cwd /repo --name nightly-report -- make report
```

The main integration requirement is:

- timer and service units should not need to embed raw Zellij syntax

## Relationship to Backend Docs

This document stays backend-neutral at the CLI surface.

Backend-specific launch mapping and limitations belong in:

- [backends/zellij.md](./backends/zellij.md)
