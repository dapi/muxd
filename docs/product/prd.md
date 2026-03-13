# muxd PRD

Date: 2026-03-13

Status: Draft

## Product Summary

`muxd` is a thin CLI wrapper that launches arbitrary commands into an existing terminal multiplexer session through a stable, automation-friendly interface.

The first release targets Zellij and is designed to be useful both from an interactive shell and from simple Bash automation. Long-term, the product may grow into a richer launcher layer and later support tmux, but the first release is intentionally narrow.

## Problem

Developers already automate recurring work with shell scripts and sometimes want to trigger the same workflow manually, but launching that work into a consistent multiplexer workspace is awkward:

- raw `zellij` commands are verbose and backend-specific
- reusable shell commands should call a stable command with predictable exit codes
- validation and error messages from direct backend commands are not product-shaped
- session, tab, naming, working directory, and placement conventions must otherwise be repeated in every command and script

For the first release, the problem is not queueing or lifecycle orchestration. The problem is creating one reliable launch interface for recurring automation into an existing Zellij session.

## Target Users

- developers who run recurring shell or agent commands manually and from Bash scripts
- users who want a stable wrapper around `zellij` for scripts and automation
- maintainers who want to keep room for a later tmux backend without exposing backend CLI syntax directly

Representative scenario:

- `docs/product/use-cases/0001-hourly-github-issue-analysis.md`

## Product Goal

Make launching recurring commands into a consistent multiplexer workspace simple, stable, and script-friendly.

## Primary User Scenario

A developer wants to run a recurring command inside a dedicated Zellij workspace, sometimes manually and later from a shell script.

The command should call one stable CLI, such as:

```text
muxd launch --session work --tab issue-analysis --target new_pane --cwd /repo --name nightly-report -- make report
```

`muxd` is responsible for:

- validating arguments
- checking backend availability
- ensuring that the requested session and tab exist when explicitly asked
- constructing the correct backend command
- returning clear exit codes and user-facing errors

Scheduling remains outside the product.

## Jobs To Be Done

- launch a recurring command into a dedicated Zellij workspace
- keep shell aliases and scripts short and stable
- avoid duplicating backend-specific command syntax across scripts
- fail clearly when Zellij, the requested workspace, or the requested target is unavailable
- keep room for a later tmux backend without redesigning the user-facing CLI

## MVP Scope

The MVP must provide:

- one CLI command: `muxd launch`
- Zellij as the only backend
- existing-session launch first, then explicit session/tab ensure helpers
- support for arbitrary commands after `--`
- support for a small target set, starting with the safest path first
- support for selecting a tab by name
- support for optional "create if missing" behavior for session and tab
- support for `--cwd`
- support for `--name`
- stable exit codes for automation
- error messages suitable for shell use and script logs

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
- keep workspace creation explicit rather than magical

## Functional Requirements

### FR-1: Launch command

The user can launch an arbitrary command into an existing Zellij session through `muxd launch`.

### FR-2: Validate before launch

The CLI fails clearly if:

- `zellij` is not installed or not available in `PATH`
- the requested session does not exist and creation was not requested
- the requested tab does not exist and creation was not requested
- the requested target is unsupported
- required arguments are missing

### FR-3: Support automation-friendly options

The CLI supports at least:

- `--session`
- `--tab`
- `--target`
- `--cwd`
- `--name`

The payload command is passed after `--`.

It may also support explicit workspace helpers such as:

- `--ensure-session`
- `--ensure-tab`

### FR-4: Return stable exit behavior

The CLI returns documented exit codes that let shell scripts distinguish launch success from validation or environment failures.

### FR-5: Keep backend syntax behind the wrapper

The user-facing CLI must not require callers to know raw Zellij command forms.

## Success Signals

The MVP is successful when:

- a user can run `muxd launch` manually or from Bash without embedding raw Zellij syntax
- failures are understandable from CLI output alone
- users can standardize workspace conventions such as session, tab, name, target, and working directory through one CLI
- later support for tmux still looks like a backend addition, not a product rewrite

## Constraints

- the implementation must follow the accepted Rust stack decision
- the first release must stay thin and avoid premature daemon or queue design
- scheduling belongs outside `muxd`
- backend guarantees must not be overstated

## Risks

- the wrapper may be too thin to justify its own binary if it adds no real stability or ergonomics over direct `zellij`
- backend-specific assumptions may still leak into the public CLI
- workspace helpers may expand too quickly and recreate hidden complexity
- later dispatcher ambitions could distort the first release scope

## Open Questions

- which exact flag shape makes workspace creation explicit without turning `launch` into a hidden orchestrator
- whether tab selection should be mandatory when `--ensure-tab` is used
- whether launch profiles should stay generic or grow agent-oriented affordances later

## Next Slice Decision

The next product slice after the first usable launch flow is:

- session and tab ensure semantics

Reason:

- it makes the documented GitHub issue analysis workflow practical without requiring manual Zellij setup each time
- it still keeps `muxd` in thin-wrapper territory because the caller must request the behavior explicitly
- it is more immediately useful than `--wait` for the current product direction

`--wait` is deferred until after defaults/config.

Expected scope of that slice:

- select a tab by name
- create session if missing when explicitly requested
- create tab if missing when explicitly requested
- keep payload handling generic
- no named profiles or prompt templates yet

## Related Documents

- `docs/product/roadmap.md`
- `docs/product/use-cases/`
- `docs/design.md`
- `docs/architecture/launch-cli.md`
- `docs/architecture/backends/zellij.md`
- `docs/specs/2026-03-13-zellij-launch-wrapper.md`
- `docs/adr/0001-stack-selection.md`
