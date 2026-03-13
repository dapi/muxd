# muxd Spec: Zellij Workspace Ensure

Date: 2026-03-13

Status: Draft

## Goal

Extend `muxd launch` so a caller can target a predictable Zellij workspace without manually preparing it first.

This slice adds explicit session and tab ensure semantics while keeping the product a thin wrapper.

## Scope

In scope:

- select a session by name
- create the session when the caller explicitly requests it
- select a tab by name
- create the tab when the caller explicitly requests it
- launch the payload into a new pane inside the selected tab
- keep support limited to the Zellij backend
- keep `new_pane` as the only target

Out of scope:

- hidden workspace creation without flags
- prompt templates
- default agent selection
- multiple backend support
- task tracking
- `--wait`
- additional targets

## User Scenario

Reference scenario:

- `docs/product/use-cases/0001-hourly-github-issue-analysis.md`

Representative command shape:

```text
muxd launch \
  --session issue-bot \
  --ensure-session \
  --tab triage \
  --ensure-tab \
  --target new_pane \
  --cwd /repo \
  --name issue-analysis \
  -- codex exec "analyze new GitHub issues and propose solutions"
```

## Requirements

### R-1: Explicit session ensure

The CLI must support naming a target session.

If the session is missing:

- fail clearly by default
- create it only when the caller explicitly requested creation

### R-2: Explicit tab ensure

The CLI must support naming a target tab.

If the tab is missing:

- fail clearly by default
- create it only when the caller explicitly requested creation

### R-3: Launch in the selected tab

After workspace preflight succeeds, `muxd launch` must open a new pane in the selected tab and run the payload command there.

### R-4: Keep behavior visible

The CLI must not silently create sessions or tabs.

User-visible flags should make creation semantics explicit.

### R-5: Stable failure categories

The CLI must distinguish at least:

- invalid input
- backend unavailable
- workspace element unavailable
- workspace creation failed
- final launch failed

### R-6: Preserve existing precedence rules

Existing config defaults remain valid.

CLI flags still override config values.

## Proposed CLI Additions

Required or likely additions:

- `--tab <name>`
- `--ensure-session`
- `--ensure-tab`

The exact flag names may still be adjusted during implementation, but the semantics must remain explicit.

## Acceptance Criteria

1. `muxd launch --session work --target new_pane -- ...` still works unchanged for the existing-session path.
2. A command with `--ensure-session` creates the named session when it is missing, then proceeds with launch.
3. A command with `--tab <name> --ensure-tab` creates the named tab when it is missing, then launches into a new pane in that tab.
4. A command with `--tab <name>` but without `--ensure-tab` fails clearly when the tab is missing.
5. A command without `--ensure-session` fails clearly when the session is missing.
6. Failures in workspace creation do not get reported as successful launch acceptance.
7. The new behavior is covered by automated tests and keeps the repository coverage gate green.

## Edge Cases

- `--ensure-tab` without `--tab`
- `--tab` used together with an unsupported target
- session creation succeeds but tab creation fails
- tab already exists and should be reused rather than duplicated
- config supplies session or tab while CLI supplies ensure flags

## Test Plan

Automated coverage should include:

- CLI parsing for `--tab`, `--ensure-session`, and `--ensure-tab`
- request resolution with config defaults and CLI overrides
- backend command construction for session creation, tab creation, and final launch
- integration tests for missing-session, ensured-session, missing-tab, ensured-tab, and creation failure paths
- exit-code mapping for workspace errors

## Implementation Notes

- keep backend inspection and creation logic in the Zellij adapter
- keep orchestration steps testable without requiring a real Zellij instance in CI
- do not introduce an internal task or workspace state model beyond what is needed for one launch command

## Related Documents

- `docs/product/prd.md`
- `docs/product/roadmap.md`
- `docs/architecture/launch-cli.md`
- `docs/architecture/backends/zellij.md`
- `docs/plans/2026-03-13-workspace-ensure-delivery-plan.md`
