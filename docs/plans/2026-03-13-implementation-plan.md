# muxd Implementation Plan

Date: 2026-03-13

## Goal

Build the first standalone version of `muxd` as a local daemon plus CLI for dispatching tasks into terminal multiplexer sessions.

Primary target:

- Zellij backend

Design constraint:

- core model must remain suitable for a later tmux backend

## Before Coding

This plan intentionally starts with a stack decision. Earlier dispatcher planning already proved the product direction is viable, but this new repository should not inherit a stack choice by inertia.

Reference documents:

- `docs/product/prd.md`
- `docs/design.md`
- `docs/architecture/cli-and-ipc.md`
- `docs/architecture/backends/zellij.md`
- `docs/adr/0001-stack-selection.md`
- `docs/process/spec-driven-development.md`

## Phase 0: Stack Decision

### Task 0.1: Define evaluation criteria

Write down explicit criteria before comparing Go and Rust:

- local daemon ergonomics
- Unix socket support
- subprocess management
- concurrent task orchestration
- ease of backend abstraction
- binary distribution story
- test ergonomics
- maintenance cost

### Task 0.2: Build two tiny spikes

Implement the same minimal program in Go and Rust:

- open Unix socket
- accept one JSON request
- spawn a subprocess
- return exit status

Keep both spikes disposable.

### Task 0.3: Record stack decision

Create a short decision note with:

- winner
- tradeoffs accepted
- why the loser was rejected for this project now

Exit criterion:

- repository commits to one stack for MVP

## Phase 1: Product Skeleton

### Task 1.1: Repository skeleton

Create:

- top-level source tree for chosen stack
- `docs/`
- formatter/linter config
- basic test command

### Task 1.2: CLI shape

Define CLI commands:

- `muxd run`
- `muxd enqueue`
- `muxd list`
- `muxd status`
- `muxd cancel`

Decide early:

- output format defaults
- JSON mode behavior
- exit code semantics

### Task 1.3: Core types

Define core types independent from Zellij:

- task
- backend
- target
- mode
- status

Do not leak backend-specific identifiers into public CLI output except through an explicit backend details field.

## Phase 2: Local Daemon and IPC

### Task 2.1: Unix socket protocol

Implement NDJSON request/response protocol with versioning.

Methods:

- `enqueue`
- `get`
- `list`
- `status`
- `cancel`

### Task 2.2: In-memory queue

Implement:

- FIFO pending queue
- running task tracking
- task lookup by id
- status filtering

### Task 2.3: Daemon lifecycle

Implement:

- socket path creation
- stale socket detection
- graceful shutdown
- clean socket removal

## Phase 3: Zellij Backend MVP

### Task 3.1: Backend interface

Define a backend adapter interface for the core:

- `launch`
- `poll`
- `cancel`
- `preflight`

The interface should support both blocking and polling-backed execution paths.

Guardrail:

- backend capability differences should be explicit in the adapter, not hidden in generic core code

### Task 3.2: Zellij preflight

Validate:

- `zellij` exists in `PATH`
- requested session exists or error is clear
- required CLI flags are supported by installed version

### Task 3.3: Oneshot launch path

Implement Zellij oneshot launches for:

- `new-tab`
- `new-pane`
- `floating-pane`

Use blocking commands where supported.

### Task 3.4: Interactive launch path

Implement interactive launch plus polling-based completion tracking.

Explicitly document any semantic differences from oneshot mode.

### Task 3.5: Cancellation semantics

Implement honest cancellation behavior:

- pending task: remove from runnable set and mark cancelled
- running task: best-effort cancellation only

Do not claim process termination if backend cannot guarantee it.

## Phase 4: Config and Defaults

### Task 4.1: Config format decision

Decide config format after stack choice, but keep semantics aligned with design:

- defaults block or equivalent
- backend default
- session default
- target default
- mode default
- max concurrent
- max pending

### Task 4.2: Server-side defaults

Ensure CLI does not overwrite daemon defaults implicitly.

Only explicit CLI flags should override config values.

### Task 4.3: Validation

Invalid config should fail fast with clear messages.

## Phase 5: Usability and Tests

### Task 5.1: `enqueue --wait`

Implement client-side wait via task polling by id.

Keep it backend-agnostic.

### Task 5.2: Smoke tests

Cover:

- daemon starts
- enqueue works
- list/status work
- task reaches terminal state
- socket cleaned on shutdown

### Task 5.3: Backend tests

At minimum:

- unit tests for queue and protocol
- integration tests for daemon IPC
- manual checklist for Zellij backend

## Phase 6: tmux Readiness Review

Before adding tmux, stop and review the architecture:

- did backend-neutral model hold up?
- are task status semantics still portable?
- did any Zellij-specific assumptions leak into core?

Output:

- short design review note
- list of refactors required before tmux support

## Current Open Questions

- Go vs Rust
- config format after stack selection
- whether persistence belongs in MVP or immediately after it
- whether agent execution should be first-class or just treated as arbitrary commands

## Suggested Immediate Next Step

Do Phase 0 first and avoid writing production code before the stack note exists.
