# muxd

`muxd` is a task dispatcher for terminal multiplexers.

The long-term goal is a standalone daemon and CLI that can launch and monitor agent tasks in multiplexed terminal sessions through a single queueing interface. The first target is Zellij. The design should remain compatible with adding tmux later without rewriting the core model.

## Status

This repository is in planning stage.

Current documents:

- `docs/design.md` - product and architecture design
- `docs/adr/0001-stack-selection.md` - stack decision record for Go vs Rust
- `docs/plans/2026-03-13-implementation-plan.md` - phased implementation plan

## Scope

Initial scope:

- daemon process with local IPC
- enqueue, list, status, cancel commands
- launch tasks into existing Zellij sessions
- task lifecycle tracking
- defaults-driven config

Planned later:

- tmux backend
- persistence and history
- scheduler and webhooks
- richer logs and output capture

## Design Principles

- multiplexer-agnostic core
- backend-specific adapters
- explicit task lifecycle model
- local-first operations through Unix socket IPC
- predictable CLI semantics suitable for scripts

## Open Questions

- implementation stack: Go or Rust
- persistence format and storage strategy
- how much backend capability should be normalized vs exposed directly
- what "cancel" can reliably mean across different multiplexers
