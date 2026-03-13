# muxd PRD

Date: 2026-03-13

Status: Draft

## Product Summary

`muxd` is a thin CLI wrapper that launches arbitrary commands into an existing terminal multiplexer session through a stable, automation-friendly interface.

The first release targets Zellij and is designed to be called from `systemd --user` timers. Long-term, the product may grow into a richer launcher layer and later support tmux, but the first release is intentionally narrow.

## Problem

Developers already automate recurring work with `systemd --user` timers, but launching that work into an existing multiplexer session is awkward:

- raw `zellij` commands are verbose and backend-specific
- timer units should call a stable command with predictable exit codes
- validation and error messages from direct backend commands are not product-shaped
- naming, working directory, and placement conventions must otherwise be repeated in every timer and script

For the first release, the problem is not queueing or lifecycle orchestration. The problem is creating one reliable launch interface for recurring automation into an existing Zellij session.

## Target Users

- developers who run recurring shell or agent commands from `systemd --user` timers
- users who want a stable wrapper around `zellij` for scripts and automation
- maintainers who want to keep room for a later tmux backend without exposing backend CLI syntax directly

Representative scenario:

- `docs/product/use-cases/0001-hourly-github-issue-analysis.md`

## Product Goal

Make launching recurring commands into an existing multiplexer session simple, stable, and script-friendly.

## Primary User Scenario

A developer has a `systemd --user` timer that should periodically run a command inside an already running Zellij session.

The timer should call one stable CLI, such as:

```text
muxd launch --session work --target new_pane --cwd /repo --name nightly-report -- make report
```

`muxd` is responsible for:

- validating arguments
- checking backend availability
- checking that the session exists
- constructing the correct backend command
- returning clear exit codes and user-facing errors

`systemd --user` remains responsible for scheduling.

## Jobs To Be Done

- launch a recurring command into an existing Zellij session from `systemd --user`
- keep timer unit files short and stable
- avoid duplicating backend-specific command syntax across scripts
- fail clearly when Zellij, the session, or the requested target is unavailable
- keep room for a later tmux backend without redesigning the user-facing CLI

## MVP Scope

The MVP must provide:

- one CLI command: `muxd launch`
- Zellij as the only backend
- existing-session launch only
- support for arbitrary commands after `--`
- support for a small target set, starting with the safest path first
- support for `--cwd`
- support for `--name`
- stable exit codes for automation
- error messages suitable for `systemd --user` logs
- a documented integration pattern for `systemd --user` timers

## Explicit MVP Exclusions

The MVP does not include:

- daemon process
- Unix socket IPC
- task ids
- queueing
- in-memory store
- `enqueue`, `get`, `list`, `status`, or `cancel`
- backend-neutral lifecycle tracking across multiple tasks
- persistence or history
- network API
- tmux support in the first release

## User Experience Principles

- small stable CLI surface
- direct automation fit
- explicit validation and failure modes
- thin wrapper over backend launch, not a hidden orchestration layer
- avoid pretending to support semantics the backend cannot guarantee

## Functional Requirements

### FR-1: Launch command

The user can launch an arbitrary command into an existing Zellij session through `muxd launch`.

### FR-2: Validate before launch

The CLI fails clearly if:

- `zellij` is not installed or not available in `PATH`
- the requested session does not exist
- the requested target is unsupported
- required arguments are missing

### FR-3: Support automation-friendly options

The CLI supports at least:

- `--session`
- `--target`
- `--cwd`
- `--name`

The payload command is passed after `--`.

### FR-4: Return stable exit behavior

The CLI returns documented exit codes that let `systemd --user` and shell scripts distinguish launch success from validation or environment failures.

### FR-5: Keep backend syntax behind the wrapper

The user-facing CLI must not require callers to know raw Zellij command forms.

## Success Signals

The MVP is successful when:

- a `systemd --user` timer can call `muxd launch` without embedding raw Zellij syntax
- failures in user timers are understandable from logs alone
- users can standardize launch conventions such as names, targets, and working directory through one CLI
- later support for tmux still looks like a backend addition, not a product rewrite

## Constraints

- the implementation must follow the accepted Rust stack decision
- the first release must stay thin and avoid premature daemon or queue design
- scheduling belongs to `systemd --user`, not to `muxd`
- backend guarantees must not be overstated

## Risks

- the wrapper may be too thin to justify its own binary if it adds no real stability or ergonomics over direct `zellij`
- backend-specific assumptions may still leak into the public CLI
- target support may expand too quickly and recreate hidden complexity
- later dispatcher ambitions could distort the first release scope

## Open Questions

- which single target should be the first supported path: `new_pane` or `floating_pane`
- whether `--wait` belongs in the first release or in the next slice
- whether defaults/config should land before or after the first usable launch flow

## Related Documents

- `docs/product/roadmap.md`
- `docs/product/use-cases/`
- `docs/design.md`
- `docs/architecture/launch-cli.md`
- `docs/architecture/backends/zellij.md`
- `docs/specs/2026-03-13-zellij-launch-wrapper.md`
- `docs/adr/0001-stack-selection.md`
