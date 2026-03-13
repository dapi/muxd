# Stage 1 Delivery Plan

Date: 2026-03-13

## Goal

Deliver the first production stage of `muxd` to GitHub as merged code with:

- implemented `muxd launch`
- automated tests
- minimum `80%` line coverage for first-stage production code
- green GitHub CI

This plan ends only when the code is merged into the default branch.

## Scope

Stage 1 includes:

- `muxd launch`
- backend: `zellij` only
- existing session only
- target: `new_pane` only
- flags:
  - `--session`
  - `--target`
  - `--cwd`
  - `--name`
- payload command after `--`
- preflight validation:
  - `zellij` exists in `PATH`
  - requested session exists
  - payload command is present
- backend command construction for `new_pane`
- actual launch execution
- stable exit codes
- unit and integration tests
- GitHub Actions CI
- coverage gate at `80%`

Stage 1 explicitly excludes:

- `--wait`
- `floating_pane`
- `new_tab`
- session creation
- tab creation
- default agent selection
- prompt templates
- config/defaults
- tmux
- daemon
- IPC
- queueing

## Inputs

- `docs/product/prd.md`
- `docs/product/roadmap.md`
- `docs/product/use-cases/0001-hourly-github-issue-analysis.md`
- `docs/specs/2026-03-13-zellij-launch-wrapper.md`
- `docs/specs/2026-03-13-rust-cli-skeleton.md`
- `docs/plans/2026-03-13-implementation-plan.md`

## Definition of Done

Stage 1 is done only if all of the following are true:

- the approved stage scope is fully implemented
- acceptance criteria from the relevant specs are satisfied
- `cargo fmt --check` passes
- `cargo clippy --all-targets -- -D warnings` passes
- `cargo test` passes
- coverage for first-stage production code is at least `80%`
- GitHub Actions CI is green on the branch being merged
- a pull request is opened and reviewed
- the code is merged into the default GitHub branch

## Acceptance Criteria

- `cargo build` succeeds on a clean checkout
- `muxd launch --help` is available
- `muxd launch --session work --target new-pane -- -- echo hello` parses successfully
- missing payload command is rejected
- invalid target is rejected
- missing `zellij` is reported as backend/environment unavailable
- missing session is reported as unavailable
- `new_pane` launch command is constructed correctly for Zellij
- `--cwd` is propagated when provided
- `--name` is propagated when provided
- GitHub CI runs format, lint, test, and coverage checks

## Test Strategy

### Unit Tests

- CLI parsing: minimal invocation
- CLI parsing: optional fields
- request conversion
- invalid target rejection
- missing payload rejection
- exit-code mapping
- Zellij preflight parsing and failure branches
- Zellij command construction for `new_pane`

### Integration Tests

- CLI returns the expected code on invalid args
- CLI returns the expected code when `zellij` is missing
- CLI returns the expected code when session is missing
- CLI returns success when backend execution path is mocked as successful

### Mocking Rule

CI should not require a real running Zellij session.

External command execution should be isolated behind testable seams so:

- command construction is unit-tested
- preflight and execution paths can be mocked in tests

## Coverage Gate

Recommended tooling:

- `cargo llvm-cov`

Required gate:

- line coverage `>= 80%`

Coverage should apply to first-stage production modules, not to generated files or disposable spikes.

## Phases

### Phase 1: Contract lock

- freeze stage-1 scope
- lock initial target to `new_pane`
- finalize documented exit code set

### Phase 2: Code structure

- shape production crate modules
- keep `main.rs` thin
- move parser, model, backend, preflight, and exit-code logic into testable units

### Phase 3: Zellij implementation

- implement preflight checks
- implement `new_pane` command construction
- implement launch execution path

### Phase 4: Test suite

- complete unit tests
- complete integration tests
- reach coverage threshold

### Phase 5: GitHub CI

- run format check
- run clippy
- run tests
- run coverage gate

### Phase 6: Merge

- open PR
- review against scope and acceptance criteria
- wait for green CI
- merge to default branch

## Risks

- coverage may stay below `80%` if too much logic remains inside subprocess calls
- scope creep may reintroduce `--wait`, extra targets, or workspace creation too early
- using real Zellij in CI would make the suite brittle and slow

## Suggested Immediate Next Step

Implement the code structure and test seams first, then add coverage tooling before the real Zellij launch path grows beyond the current skeleton.
