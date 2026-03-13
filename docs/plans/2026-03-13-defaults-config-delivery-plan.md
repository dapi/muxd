# Defaults and Config Delivery Plan

Date: 2026-03-13

Status: Completed

## Goal

Deliver the defaults/config slice for `muxd` so recurring automation can omit repeated launch flags while keeping the product a thin wrapper.

## Scope

- one user-level config file
- defaults for:
  - backend
  - session
  - target
  - cwd
- CLI-over-config precedence
- clear invalid-config errors
- automated tests
- existing CI and coverage gate remain green

## Deliberate Exclusions

- named profiles
- workspace creation
- agent defaults
- prompt templates
- `--wait`

## Definition of Done

This slice is done only if:

- config-backed launches work without changing stage-1 launch semantics
- explicit CLI values override config values
- invalid config fails clearly
- `cargo fmt --check` passes
- `cargo clippy --all-targets -- -D warnings` passes
- `cargo test` passes
- `cargo llvm-cov --summary-only --ignore-filename-regex '(tests|src/main.rs)' --fail-under-lines 80` passes
- code is merged into the default branch

## Completion Note

- implemented and merged after stage 1
