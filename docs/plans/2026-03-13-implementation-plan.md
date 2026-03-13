# muxd Implementation Plan

Date: 2026-03-13

## Goal

Build the first standalone version of `muxd` as a thin Rust CLI wrapper that launches arbitrary commands into a consistent Zellij workspace.

Primary target:

- manual shell use and other local automation

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

### Task 3.3: Shell usage example

Add a minimal documented example showing how a shell command should call `muxd launch`.

## Phase 4: Defaults and ergonomics

### Task 4.1: Config defaults

Add:

- one user-level config file
- defaults for backend, session, target, and cwd
- CLI-over-config precedence

### Task 4.2: Validation

Cover:

- config parsing
- precedence rules
- invalid config handling

## Phase 5: Workspace ensure semantics

### Task 5.1: Session ensure

Implement explicit session creation when requested.

### Task 5.2: Tab ensure

Implement:

- named tab selection
- explicit tab creation when requested
- new pane launch in the selected tab

### Task 5.3: Error handling

Ensure the CLI clearly distinguishes:

- missing workspace element
- failed workspace creation
- failed final launch

## Phase 6: Validation

### Task 6.1: Manual checks

Cover:

- happy-path launch into an existing session
- missing Zellij binary
- missing session
- unsupported target
- missing payload command
- session creation when requested
- tab creation when requested

### Task 6.2: Automated tests

At minimum:

- argument parsing tests
- command construction tests
- exit-code mapping tests
- workspace ensure tests

## Current Open Questions

- whether `new_pane` should be the only initial target
- which CLI flag shape keeps workspace creation explicit without overcomplicating `launch`
- how much of Zellij tab inspection can be tested without brittle integration coupling
- whether `--wait` still makes sense after the defaults/config slice is complete

## Suggested Immediate Next Step

Start the workspace ensure slice after defaults/config, keeping it focused on explicit session/tab creation semantics rather than lifecycle tracking.

Reference spec:

- `docs/specs/2026-03-13-zellij-workspace-ensure.md`
- `docs/plans/2026-03-13-workspace-ensure-delivery-plan.md`
