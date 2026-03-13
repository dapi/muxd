# muxd PRD

Date: 2026-03-13

Status: Draft

## Product Summary

`muxd` is a local daemon plus CLI that dispatches tasks into terminal multiplexer sessions through one scriptable interface.

The first release targets Zellij. The product model must remain suitable for a later tmux backend.

## Problem

Developers already use terminal multiplexers to keep long-running or parallel work organized, but the workflow is still mostly manual:

- starting a task in the right session, tab, or pane takes hand-driven terminal work
- shell scripts cannot treat multiplexer execution as a clean queued interface
- observing pending versus running work across terminals is awkward
- backend choice leaks into the user workflow too early

There is no small local tool that makes multiplexer execution queueable, observable, and stable enough for scripting.

## Target Users

- developers who run repeated agent or shell tasks inside Zellij or tmux
- maintainers who want a backend-neutral task model instead of backend-specific scripts
- advanced shell users who need blocking and inspectable task execution in automation

## Product Goal

Make execution inside a terminal multiplexer scriptable, observable, and queueable through one stable interface.

## Jobs To Be Done

- enqueue several tasks against one existing multiplexer session
- block in a shell script until a oneshot task reaches a terminal state
- inspect pending and running work from another terminal
- preserve the same user-facing task model when a second backend is added later

## MVP Scope

The MVP must provide:

- a local daemon process
- a CLI client
- Unix socket IPC
- in-memory task queueing
- task lifecycle tracking
- Zellij backend support
- backend-neutral task statuses
- `enqueue`, `get`, `list`, `status`, and `cancel` flows
- client-side `enqueue --wait`

## Non-Goals

- network API
- distributed scheduling
- webhooks
- durable persistence beyond what correct local daemon behavior requires
- abstracting away every backend-specific limitation
- supporting both Zellij and tmux in the first release

## User Experience Principles

- local-first and script-friendly
- honest status and cancellation semantics
- backend-neutral core model
- explicit failure over fake portability
- human output for terminals, JSON output for automation

## Functional Requirements

### FR-1: Enqueue work

The user can create tasks for an existing multiplexer session and choose backend, target, mode, name, agent, and working directory.

### FR-2: Observe lifecycle

The user can inspect a task by id, list active or historical tasks, and see backend-neutral states.

### FR-3: Wait in scripts

The user can block until a task reaches a terminal state and receive a meaningful exit code.

### FR-4: Cancel honestly

The user can cancel pending tasks strongly and running tasks on a best-effort basis, without the product overstating backend guarantees.

### FR-5: Preserve backend boundary

The core queue, IPC, and lifecycle model must not depend on Zellij-specific command syntax or handle formats.

## Success Signals

The MVP is successful when:

- one daemon can manage multiple queued tasks in one Zellij session
- automation can use `enqueue --wait` without backend-specific glue
- users can inspect pending and running work from another shell
- adding tmux later looks like a second adapter, not a core rewrite

## Constraints

- the repository remains documentation-first until the stack ADR is accepted
- stack choice is still open between Go and Rust
- cancellation guarantees are limited by backend capabilities
- configuration semantics should stay stack-neutral until implementation starts

## Risks

- backend-neutral design may drift into lowest-common-denominator abstractions
- lifecycle semantics may become inconsistent across modes or targets
- Zellij assumptions may leak into the public task model
- stack selection may bias the architecture before requirements are fully locked

## Open Questions

- Go vs Rust for MVP implementation
- config format after stack selection
- whether persistence belongs in MVP or immediately after it
- whether `agent` should be a first-class concept or just command construction input

## Related Documents

- `docs/design.md`
- `docs/architecture/cli-and-ipc.md`
- `docs/architecture/backends/zellij.md`
- `docs/adr/0001-stack-selection.md`
- `docs/plans/2026-03-13-implementation-plan.md`
