# muxd Workspace Ensure Delivery Plan

Date: 2026-03-13

Status: Draft

## Goal

Deliver the next product slice after defaults/config:

- explicit session ensure semantics
- explicit tab ensure semantics
- launch into a new pane in the selected tab

## Scope

Included:

- `--tab <name>`
- `--ensure-session`
- `--ensure-tab`
- Zellij-only implementation
- `new_pane` target only
- unit and integration tests
- green GitHub CI

Excluded:

- `--wait`
- additional targets
- agent presets
- prompt templates
- tmux

## Definition of Done

This slice is complete only when:

- the CLI can create a missing session when explicitly requested
- the CLI can create a missing tab when explicitly requested
- the CLI launches the payload into a new pane in the selected tab
- all new branches are covered by automated tests
- coverage remains at or above the repository gate
- GitHub CI is green
- the change is merged into `master`

## Work Items

### 1. Contract

Finalize:

- flag names
- precedence between config and CLI
- error messages and exit-code mapping for workspace failures

### 2. Request model

Add launch request fields for:

- tab name
- ensure-session boolean
- ensure-tab boolean

### 3. Zellij adapter

Implement testable adapter steps for:

- session existence check
- session creation
- tab existence check
- tab creation
- final `new_pane` launch in the chosen tab

### 4. App flow

Sequence launch flow as:

1. resolve request from CLI and config
2. validate backend availability
3. ensure session if requested
4. ensure tab if requested
5. perform final launch

### 5. Test suite

Add or extend:

- parser tests
- config resolution tests
- backend adapter tests
- CLI integration tests
- exit-code tests for workspace creation failures

### 6. Documentation

Update:

- PRD if the final flag shape changes
- launch CLI contract
- Zellij backend notes
- use case status if the workflow becomes directly supported

## Test Strategy

Required automated scenarios:

- existing-session existing-tab launch
- missing session without ensure flag
- missing session with ensure flag
- missing tab without ensure flag
- missing tab with ensure flag
- failed session creation
- failed tab creation
- CLI-over-config precedence for tab selection

The implementation should keep external Zellij calls behind test seams so CI does not depend on a real Zellij instance.

## Exit Criteria

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo llvm-cov --summary-only --ignore-filename-regex '(tests|src/main.rs)' --fail-under-lines 80`
- green GitHub Actions run on the branch and PR
- merged PR

## Risks

- tab inspection may be harder to model reliably than session inspection
- creation commands may have different semantics than launch commands
- it is easy to let the adapter hide too much backend-specific behavior

## Next Step After This Slice

Re-evaluate whether the highest-value next step is:

- `--wait`
- additional targets
- better ergonomics around repeated agent invocations
