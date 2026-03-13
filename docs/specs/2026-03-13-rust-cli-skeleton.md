# Rust CLI skeleton spec

Date: 2026-03-13

Status: Draft

## Summary

Create the first production Rust skeleton for `muxd` so the repository has:

- a compilable crate
- a minimal CLI shape
- a typed launch request model
- automated tests
- GitHub CI

This slice is intentionally smaller than full Zellij launch execution.

## Problem Link

This slice supports the first implementation phase for the thin-wrapper product direction.

Related documents:

- `docs/product/prd.md`
- `docs/product/roadmap.md`
- `docs/specs/2026-03-13-zellij-launch-wrapper.md`
- `docs/plans/2026-03-13-implementation-plan.md`
- `docs/plans/2026-03-13-stage-1-delivery-plan.md`

## Scope

- initialize a Rust crate for `muxd`
- define `muxd launch`
- define typed launch request structures
- parse payload command after `--`
- add automated tests for parsing and request conversion
- add GitHub CI for formatting, linting, and tests

## Out of Scope

- actual `zellij` process execution
- session existence checks
- target-specific command construction
- scheduler-specific integration behavior beyond documentation

## Requirements

### R1

The repository must build with `cargo build`.

### R2

The crate must expose a minimal CLI with `muxd launch`.

### R3

The launch command must parse:

- `--session`
- `--target`
- `--cwd`
- `--name`
- payload command after `--`

### R4

The codebase must define typed request structures that are launcher-focused rather than task-focused.

### R5

The repository must include automated tests for CLI parsing and request conversion.

### R6

The repository must include GitHub Actions CI that runs:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`

## Acceptance Criteria

- `cargo build` succeeds on a clean checkout
- `cargo test` succeeds on a clean checkout
- `muxd launch --help` is available
- parsing succeeds for a minimal invocation with payload after `--`
- parsing succeeds for an invocation including `--cwd` and `--name`
- invalid target values are rejected by the CLI
- missing payload command is rejected by the CLI
- GitHub CI is configured to run format, lint, and test checks on pushes and pull requests

## Test Set

- unit test: minimal launch command parsing
- unit test: launch parsing with optional fields
- unit test: request conversion preserves session, target, cwd, name, and payload
- unit test: invalid target is rejected
- unit test: missing payload command is rejected

## Dependencies

- Rust toolchain
- accepted Rust ADR

## Follow-up Work

- add Zellij preflight validation
- add actual launch command construction
- add exit-code mapping for backend failures
