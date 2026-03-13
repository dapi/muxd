# muxd

[![CI](https://github.com/dapi/muxd/actions/workflows/ci.yml/badge.svg)](https://github.com/dapi/muxd/actions/workflows/ci.yml)

`muxd` is a thin launch wrapper for terminal multiplexers.

The current goal is a stable CLI that launches arbitrary commands into a predictable Zellij workspace for local automation and repeated manual runs. The design should remain compatible with adding tmux later without exposing raw backend syntax directly.

## Status

This repository is in active early implementation.

Current shipped slices:

- `muxd launch`
- Zellij backend
- stable exit codes
- config defaults for backend, session, tab, target, and cwd
- explicit `--tab`, `--ensure-session`, and `--ensure-tab` workflow helpers

Current documents:

- `docs/README.md` - documentation map and artifact roles
- `docs/product/prd.md` - product requirements document
- `docs/product/roadmap.md` - staged product roadmap
- `docs/product/use-cases/` - concrete user scenarios
- `docs/design.md` - product and architecture design
- `docs/process/spec-driven-development.md` - documentation and delivery workflow
- `docs/architecture/launch-cli.md` - launch CLI contract
- `docs/architecture/backends/zellij.md` - Zellij-specific launch semantics and limitations
- `docs/adr/0001-stack-selection.md` - accepted Rust stack decision
- `docs/research/2026-03-13-stack-evaluation.md` - comparison document used before accepting the stack ADR
- `docs/plans/2026-03-13-implementation-plan.md` - phased implementation plan

## Scope

Current scope:

- `muxd launch`
- Zellij backend
- session selection
- optional tab selection
- explicit session creation with `--ensure-session`
- explicit tab creation with `--ensure-tab`
- arbitrary payload command after `--`
- config defaults for launch inputs
- stable exit codes and validation errors

Planned later:

- `--wait` where backend behavior is honest
- more targets
- tmux backend
- richer launcher ergonomics

## Development

Core local commands:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo llvm-cov --summary-only --ignore-filename-regex '(tests|src/main.rs)' --fail-under-lines 80`

## Design Principles

- thin wrapper first
- backend-specific adapters
- predictable CLI semantics for automation
- explicit validation and failure modes
- no hidden orchestration layer

## Open Questions

- whether `--wait` belongs in the next slice
- which next target should ship after `new_pane`
- how far repeated agent-oriented ergonomics should go before the wrapper stops feeling thin
