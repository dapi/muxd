# muxd Implementation Plan

Date: 2026-03-13

## Goal

Build the first standalone version of `muxd` as a thin Rust CLI wrapper that launches arbitrary commands into an existing Zellij session.

Primary target:

- `systemd --user` timers and other local automation

Primary backend:

- Zellij

## Inputs

- `docs/product/prd.md`
- `docs/product/roadmap.md`
- `docs/design.md`
- `docs/architecture/launch-cli.md`
- `docs/architecture/backends/zellij.md`
- `docs/specs/2026-03-13-zellij-launch-wrapper.md`
- `docs/specs/2026-03-13-rust-cli-skeleton.md`
- `docs/adr/0001-stack-selection.md`
- `docs/plans/2026-03-13-stage-1-delivery-plan.md`

## Preconditions

- Rust stack decision is accepted
- the first release stays thin and does not grow a daemon or queue

## Phase 1: Rust CLI skeleton

### Task 1.1: Repository skeleton

Create:

- Cargo workspace or crate layout for the thin wrapper
- formatter and lint configuration
- basic test command

### Task 1.2: Command shape

Define:

- `muxd launch`
- base help text
- payload command parsing after `--`

### Task 1.3: Core request model

Define launch request types for:

- backend
- session
- target
- name
- cwd
- payload command and args

Keep the model launcher-focused rather than task-focused.

## Phase 2: Zellij launch path

### Task 2.1: Preflight validation

Validate:

- `zellij` exists in `PATH`
- requested session exists
- requested target is supported
- payload command is present

### Task 2.2: Initial target support

Implement one safe target path first:

- `new_pane`

### Task 2.3: Launch execution

Implement backend command construction and execution for the initial target.

Ensure:

- `--cwd` is passed where supported
- `--name` is passed through cleanly

## Phase 3: Automation fit

### Task 3.1: Exit codes

Document and implement stable exit codes for:

- invalid input
- backend unavailable
- session or target unavailable
- backend launch failure

### Task 3.2: Human-facing errors

Ensure service logs are understandable without reading source code or raw backend usage.

### Task 3.3: `systemd --user` example

Add a minimal documented integration example showing how a unit or timer should call `muxd launch`.

## Phase 4: Validation

### Task 4.1: Manual checks

Cover:

- happy-path launch into an existing session
- missing Zellij binary
- missing session
- unsupported target
- missing payload command

### Task 4.2: Automated tests

At minimum:

- argument parsing tests
- command construction tests
- exit-code mapping tests

## Current Open Questions

- whether `new_pane` should be the only initial target
- whether defaults/config should stop at backend, session, target, and cwd or include naming conventions too
- whether `--wait` still makes sense after the defaults/config slice is complete

## Suggested Immediate Next Step

Start the defaults/config slice after stage 1, keeping it focused on reducing `systemd --user` timer boilerplate rather than introducing blocking semantics.

Reference spec:

- `docs/specs/2026-03-13-launch-defaults-config.md`
- `docs/plans/2026-03-13-defaults-config-delivery-plan.md`
