# ADR 0001: Implementation Stack Selection

Date: 2026-03-13

Status: Accepted

## Context

`muxd` is a local daemon plus CLI with these requirements:

- Unix socket IPC
- subprocess launch and lifecycle tracking
- concurrent task orchestration
- backend adapter boundary for Zellij now and tmux later
- single static binary distribution

This repository intentionally starts without inheriting a stack from earlier experiments. The choice should be made for `muxd` itself, not for compatibility with another codebase.

## Options

### Option A: Go

Strengths:

- fast iteration for daemon/CLI software
- straightforward standard library support for sockets, processes, and JSON
- simple concurrency model
- lower implementation friction for disposable spikes

Costs:

- weaker type-level constraints around state transitions and backend interfaces
- more care needed around subtle lifecycle and cancellation bugs
- error handling discipline depends more on conventions than compiler pressure

### Option B: Rust

Strengths:

- stronger guarantees around task state, ownership, and backend boundaries
- good fit for long-lived infrastructure binaries
- strong enums and type system for protocol and lifecycle modeling
- static binaries and CLI tooling ecosystem are mature

Costs:

- higher implementation friction at MVP stage
- async and process orchestration can be more expensive to iterate on
- tmux/Zellij backend spikes take longer to prototype

## Decision Drivers

- clarity and safety of task lifecycle code
- speed of building a reliable daemon MVP
- ease of backend abstraction for future tmux support
- operational simplicity of shipping the binary
- long-term maintenance cost

## Decision

Choose Rust for the `muxd` MVP.

Chosen stack:

- Rust

Rejected stack for now:

- Go

Decision rationale:

- `muxd` is fundamentally a lifecycle and boundary-management system, not just a subprocess wrapper
- the core risk is semantic drift across task states, cancellation behavior, and backend adapter boundaries
- Rust gives stronger compile-time pressure for modeling valid states, explicit capability differences, and backend-neutral APIs
- that pressure aligns well with spec-driven and agent-assisted development, where the compiler can enforce more of the intended structure
- the spike confirmed that Go is faster for raw MVP iteration, but did not change the architectural preference toward stronger invariants

Evidence used for the decision:

- `spikes/stack-decision/`
- `docs/research/2026-03-13-stack-evaluation.md`

Tradeoffs accepted:

- slower MVP iteration than Go
- more up-front type and error-modeling work
- more implementation friction around concurrency and process orchestration

## Consequences

- a Rust production source tree may now be added
- package and module layout should follow Rust conventions, not generic placeholders
- protocol, task lifecycle, and backend capability modeling should lean into Rust enums and explicit typed boundaries
- config and parser choices may now be made in a Rust-specific implementation context
- Go remains a valid comparison reference, but it is no longer the active implementation path
