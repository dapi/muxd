# muxd

`muxd` is a thin launch wrapper for terminal multiplexers.

The current goal is a stable CLI that launches arbitrary commands into an existing Zellij session for local automation, especially `systemd --user` timers. The design should remain compatible with adding tmux later without exposing raw backend syntax directly.

## Status

This repository is in planning stage.

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

Initial scope:

- `muxd launch`
- existing Zellij session
- arbitrary payload command after `--`
- stable exit codes and validation errors
- integration with `systemd --user` timers

Planned later:

- `--wait` where backend behavior is honest
- defaults and config
- more targets and workspace helpers
- tmux backend
- richer launcher ergonomics

## Design Principles

- thin wrapper first
- backend-specific adapters
- predictable CLI semantics for automation
- explicit validation and failure modes
- no hidden orchestration layer

## Open Questions

- which first target should ship: `new_pane` or `floating_pane`
- whether `--wait` belongs in the next slice
- how far defaults/config should go before the wrapper stops feeling thin
