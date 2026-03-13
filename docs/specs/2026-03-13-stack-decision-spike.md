# Stack decision spike spec

Date: 2026-03-13

Status: Draft

## Summary

Build two disposable equivalent prototypes in Go and Rust to reduce uncertainty before accepting the implementation stack ADR.

The spike is not production code. It exists only to compare implementation friction and code shape for the narrow `muxd` core loop:

- open a Unix socket
- accept one JSON request
- spawn a subprocess
- return exit status

## Problem Link

This slice exists because the PRD and architecture are clear enough to start implementation planning, but the repository still has one blocking decision:

- Go vs Rust for the daemon and CLI implementation

Related product and architecture context:

- `docs/product/prd.md`
- `docs/design.md`
- `docs/adr/0001-stack-selection.md`

## Scope

- one minimal Go prototype
- one minimal Rust prototype
- equivalent request and response shape
- runnable commands for local validation
- implementation notes captured in repository docs

## Out of Scope

- daemon lifecycle beyond one request
- queueing
- task persistence
- backend adapters
- CLI UX
- production repository layout
- benchmarking beyond developer ergonomics and basic behavior validation

## Requirements

### R1

Each spike must listen on a Unix socket path passed as an argument.

### R2

Each spike must read one JSON request with enough data to run a subprocess.

### R3

Each spike must execute the requested subprocess and wait for completion.

### R4

Each spike must write a JSON response containing success state and exit status.

### R5

Each spike must stop after serving one request so validation stays deterministic.

### R6

The repository must include commands that reproduce the spike run.

## Acceptance Criteria

- both spike programs build locally in the current environment or the blocking toolchain gap is documented explicitly
- both spike programs accept the same request shape
- both spike programs are run against a simple command such as `sh -c 'echo hello'`
- validation notes are added to the relevant research or planning docs
- the spike produces concrete input for the ADR decision

## Edge Cases

- missing toolchain for one candidate language
- subprocess exits non-zero
- stale Unix socket path already exists
- malformed JSON request

## Dependencies

- local Rust toolchain
- local Go toolchain, whether preinstalled or downloaded temporarily for the spike
- `docs/research/2026-03-13-stack-evaluation.md`
- `docs/adr/0001-stack-selection.md`

## Decisions

The spike should stay disposable and isolated under a dedicated `spikes/` directory so it does not imply the final production layout.

## Validation

Validation should record:

- exact build and run commands
- whether each implementation ran successfully
- qualitative notes on implementation friction and code clarity

## Follow-up Work

- update stack evaluation findings with actual spike observations
- accept or revise the stack ADR
- if the ADR is accepted, replace spike work with a real production skeleton
