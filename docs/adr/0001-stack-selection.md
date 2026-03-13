# ADR 0001: Implementation Stack Selection

Date: 2026-03-13

Status: Proposed

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

Decision deferred until minimal spikes are completed.

Comparison input for that decision lives in:

- `docs/stack-evaluation.md`

The repository will build two tiny equivalents in Go and Rust before production code starts:

- open Unix socket
- accept one JSON request
- spawn a subprocess
- return exit status

After that, this ADR should be updated from `Proposed` to `Accepted` with:

- chosen stack
- rejected stack
- short rationale
- concrete tradeoffs accepted

## Consequences

Until this ADR is accepted:

- no production code tree should be created
- docs may describe architecture and interfaces, but not stack-specific package layout
- config/parser decisions should remain semantic, not library-driven
