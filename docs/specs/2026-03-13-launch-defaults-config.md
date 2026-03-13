# Launch defaults and config spec

Date: 2026-03-13

Status: Draft

## Summary

Add a small configuration layer to `muxd` so recurring shell invocations and scripts do not have to repeat the same launch flags on every invocation.

This slice should keep `muxd` thin. It is about defaults for launch inputs, not about orchestration, profiles with hidden behavior, or workspace management.

## Problem Link

Stage 1 proved that `muxd launch` works as a stable thin wrapper, but repeated automation still has too much boilerplate:

- `--session`
- `--target`
- `--cwd`
- `--name` conventions

This slice reduces repetition while staying consistent with the thin-wrapper product direction.

Related documents:

- `docs/product/prd.md`
- `docs/product/roadmap.md`
- `docs/product/use-cases/0001-hourly-github-issue-analysis.md`
- `docs/architecture/launch-cli.md`

## Scope

- load config from one documented user-level config file location
- support defaults for:
  - backend
  - session
  - tab
  - target
  - cwd
- define precedence between CLI flags and config values
- document behavior clearly for shell and script callers
- keep `muxd launch` semantics explicit when values come from config

## Out of Scope

- `--wait`
- workspace creation
- tab creation
- prompt templates
- default agent selection
- hidden launch profiles with multiple side effects
- environment-variable expansion beyond what is explicitly documented
- multiple layered config files

## Requirements

### R1

`muxd` must look for one user-level config file in a documented location.

### R2

If the config file is missing, `muxd` must continue normally with no defaults applied.

### R3

Supported defaults in this slice must be limited to:

- backend
- session
- tab
- target
- cwd

### R4

CLI flags must override config values.

### R5

If neither CLI nor config provides a required value such as `session` or `target`, `muxd` must fail clearly.

### R6

Invalid config must fail clearly with a user-facing validation error.

## Acceptance Criteria

- `muxd launch` works without a config file
- `muxd launch` can omit `--session` when the config provides a default session
- `muxd launch` can omit `--tab` when the config provides a default tab
- `muxd launch` can omit `--target` when the config provides a default target
- `muxd launch` can omit `--cwd` when the config provides a default cwd
- explicit CLI flags override config values
- invalid config is rejected with a clear error
- config does not introduce hidden behavior beyond filling missing launch inputs

## Edge Cases

- config file missing
- config file unreadable
- invalid target in config
- config default points to a nonexistent path
- caller provides both CLI and config value for the same field

## Decisions

- this slice should prefer one simple config file over layered configuration sources
- this slice should not add named profiles yet
- defaults should remain transparent: callers must still be able to understand the effective launch request

## Validation

- unit tests for config parsing
- unit tests for CLI-over-config precedence
- integration tests showing config-backed invocations
- documented example for a short shell invocation using `muxd launch`

## Follow-up Work

- decide whether named launch profiles are needed after basic defaults land
- revisit `--wait` only after defaults/config has proven useful
