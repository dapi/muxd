# Zellij launch wrapper spec

Date: 2026-03-13

Status: Draft

## Summary

Build the first production slice of `muxd` as a thin Rust CLI wrapper that launches arbitrary commands into an existing Zellij session.

The first intended caller is `systemd --user`.

## Problem Link

This slice implements the primary scenario from the PRD:

- recurring automation should call one stable CLI instead of embedding raw Zellij commands in timer units

Related documents:

- `docs/product/prd.md`
- `docs/product/roadmap.md`
- `docs/design.md`

## Scope

- one command: `muxd launch`
- Zellij only
- existing session only
- arbitrary payload command after `--`
- support for one initial target
- support for `--cwd`
- support for `--name`
- documented exit codes
- examples for `systemd --user` unit integration

## Out of Scope

- daemon
- queueing
- task ids
- `enqueue`, `get`, `list`, `status`, `cancel`
- polling-based lifecycle tracking
- persistent state
- tmux

## Requirements

### R1

The CLI must require a session name and payload command.

### R2

The CLI must validate that `zellij` exists in `PATH`.

### R3

The CLI must validate that the requested session exists before launch.

### R4

The CLI must support at least one target path and fail clearly for unsupported targets.

### R5

The CLI must support `--cwd` and `--name`.

### R6

The CLI must execute the payload through the correct Zellij command shape for the selected target.

### R7

The CLI must document exit codes suitable for use in `systemd --user` units.

## Acceptance Criteria

- `muxd launch --session <name> -- <cmd>` launches a command into an existing Zellij session
- missing `zellij` produces a clear validation error
- missing session produces a clear validation error
- unsupported target produces a clear validation error
- `--cwd` changes launch working directory where Zellij supports it
- a minimal `systemd --user` service example can invoke the command successfully

## Edge Cases

- stale or misspelled session name
- Zellij installed but incompatible command flags
- payload command missing after `--`
- target path chosen for MVP later proves awkward for `--wait`

## Dependencies

- `docs/architecture/launch-cli.md`
- `docs/architecture/backends/zellij.md`
- `docs/adr/0001-stack-selection.md`

## Decisions

- payload commands are treated as arbitrary commands, not as first-class agent concepts
- `systemd --user` is the scheduler; `muxd` is only the launcher

## Validation

- manual launch against an existing Zellij session
- negative checks for missing session and invalid target
- documented example for `systemd --user`

## Follow-up Work

- decide whether `--wait` belongs in the next slice
- add defaults/config if timer units still feel too repetitive
