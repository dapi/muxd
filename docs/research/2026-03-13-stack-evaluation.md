# Stack Evaluation: Go vs Rust

Date: 2026-03-13

## Purpose

This document compares Go and Rust for `muxd` before production code starts.

It is not the final decision record. The final decision should be captured in:

- `docs/adr/0001-stack-selection.md`

## Project Requirements

`muxd` needs:

- local daemon process
- Unix socket IPC
- subprocess launch and exit tracking
- concurrent task orchestration
- backend adapter boundary for Zellij now and tmux later
- stable CLI behavior suitable for scripts
- simple binary distribution

## Evaluation Criteria

We will compare stacks against these criteria:

1. daemon ergonomics
2. Unix socket support
3. subprocess management
4. concurrency model
5. backend abstraction quality
6. lifecycle correctness pressure
7. implementation speed for MVP
8. testing ergonomics
9. operational simplicity
10. long-term maintenance cost

## Summary Table

| Criterion | Go | Rust | Notes |
|---|---|---|---|
| Daemon ergonomics | Strong | Strong | Both are good fits for local daemons |
| Unix socket support | Strong | Strong | Both can handle this cleanly |
| Subprocess management | Strong | Strong | Both support child process control well |
| Concurrency model | Simpler | More explicit | Go is easier to iterate fast, Rust is stricter |
| Backend abstraction | Good | Excellent | Rust enums/traits help model capability differences |
| Lifecycle correctness pressure | Medium | High | Rust pushes more correctness into compile-time design |
| MVP speed | Excellent | Good | Go likely reaches first working daemon faster |
| Testing ergonomics | Good | Good | Different styles, both workable |
| Operational simplicity | Excellent | Excellent | Both ship as single binaries well |
| Long-term maintenance | Good | Excellent | Rust likely scales better if lifecycle logic grows |

## Option A: Go

### Why Go fits

- low-friction standard library for sockets, JSON, subprocesses
- fast path to a working daemon
- easy to write and discard backend spikes
- simple concurrency model for worker loops and IPC handlers

### Where Go is weaker

- task lifecycle invariants rely more on discipline than type system
- backend capability mismatches can drift into runtime checks and conventions
- accidental state ambiguity is easier to permit

### Best case for Go

Choose Go if the priority is:

- get a working multiplexer daemon fast
- keep codebase small and operationally boring
- optimize for implementation speed over stronger compile-time modeling

## Option B: Rust

### Why Rust fits

- strong enums and traits are a good fit for task state and backend capabilities
- compile-time pressure helps avoid invalid state transitions
- good long-term fit if backend count and lifecycle complexity grow
- easier to keep core/backend boundary rigorous

### Where Rust is weaker

- higher iteration cost during MVP
- async and process orchestration can be slower to prototype
- more engineering overhead before the product shape is fully settled

### Best case for Rust

Choose Rust if the priority is:

- enforce lifecycle correctness early
- preserve a strict backend abstraction boundary
- optimize for long-term maintainability over fastest MVP

## Criteria-by-Criteria Notes

### 1. Daemon ergonomics

Both stacks are viable. This is not a deciding criterion on its own.

### 2. Unix socket IPC

Both stacks support local IPC well enough for NDJSON over Unix socket.

### 3. Subprocess launch and monitoring

Both stacks can run child processes, capture exit status, and manage polling-based workflows.

The real difference is not capability, but how much structure we want around lifecycle transitions.

### 4. Concurrency model

Go has the lower cognitive load for:

- accept loop
- worker pool
- status polling
- shutdown orchestration

Rust has the stronger explicitness when concurrency interacts with ownership and state mutation.

### 5. Backend abstraction quality

This matters because `muxd` is intentionally not Zellij-only forever.

Rust has an edge here because:

- enums model backend capabilities naturally
- trait boundaries are clearer
- invalid combinations are easier to exclude

Go is still viable, but requires more design discipline to avoid a loose interface plus scattered conditionals.

### 6. Lifecycle correctness pressure

This is one of the most important criteria.

`muxd` is mostly about getting lifecycle semantics right:

- pending
- running
- completed
- failed
- cancelled

Rust is stronger if we expect these semantics to become subtle.

### 7. MVP speed

Go likely wins.

If the near-term goal is to validate product shape quickly with Zellij only, Go has a real advantage.

### 8. Testing ergonomics

Both are acceptable.

The stronger question is whether we expect most defects to come from:

- protocol and subprocess behavior
- or invalid state modeling

If mostly the first, Go is fine.
If both, Rust gains value.

### 9. Operational simplicity

This is effectively a tie.

### 10. Long-term maintenance

Rust has an advantage if:

- tmux support is definitely coming
- lifecycle and cancellation semantics become richer
- persistence is added
- backend capability matrix grows

Go has an advantage if:

- scope stays narrow
- codebase remains small
- product iteration speed matters more than type-level rigor

## Decision Frame

### Choose Go if

- Zellij-only MVP speed is the top priority
- we want the shortest path to a working daemon
- we accept more runtime discipline in exchange for speed

### Choose Rust if

- backend-neutral core quality is the top priority
- we expect tmux support to land relatively soon
- we want stricter lifecycle modeling from day one

## Recommended Next Step

Run the disposable implementation spikes in:

- `spikes/stack-decision/go`
- `spikes/stack-decision/rust`

Build and run commands are documented in:

- `spikes/stack-decision/README.md`

## Spike Validation Notes

Date:

- 2026-03-13

Observed environment:

- Rust toolchain was already installed
- Go toolchain was not installed and was downloaded temporarily to `/tmp/muxd-go`

Validated behavior:

- both spikes successfully bound a Unix socket, accepted one JSON request, ran `sh -c 'echo hello'`, and returned `exit_code: 0`
- both spikes successfully ran `sh -c 'exit 7'` and returned `exit_code: 7`
- both spikes exit after serving one request, keeping the validation path deterministic

Qualitative notes from the spike:

- Go reached a working shape with only the standard library and no external dependencies
- Rust required `serde` and `serde_json`, plus the initial Cargo dependency fetch
- both codebases stayed small and easy to reason about for this narrow flow
- Rust still feels stronger for modeling the later task lifecycle and backend capability boundary
- Go still feels faster for the shortest path to a working daemon MVP

Questions answered by the spikes:

1. Which version reached a clean shape faster?
   Go, once the toolchain existed locally.
2. Which version made backend boundaries easier to picture?
   Rust, because the request and response modeling already pushes toward stricter types.
3. Which version made task lifecycle state feel safer?
   Rust, for the same modeling reasons.
4. Which tradeoff matters more for `muxd` right now: MVP speed or stronger invariants?
   Still open, but now based on actual spike work rather than assumptions.

## Working Recommendation After Spikes

- slight lean toward Go for MVP speed and lower implementation friction
- slight lean toward Rust for architectural rigor and lifecycle modeling

The spikes confirmed the original tradeoff rather than collapsing it. The repository now has enough evidence to update the ADR deliberately instead of guessing.
