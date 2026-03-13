# Use Case 0001: Hourly GitHub issue analysis flow

Date: 2026-03-13

Status: Draft

## Summary

A user wants to automate recurring GitHub issue analysis through a `systemd --user` timer that launches work into a dedicated Zellij workspace through `muxd`.

## Goal

Once per hour, the user wants `muxd` to start the default agent, `codex`, inside a dedicated Zellij session and tab, in a new pane, with a fixed prompt that kicks off issue analysis work.

The launched agent command should:

- find new GitHub issues that have not yet been processed
- analyze each issue
- collect the required supporting data
- produce proposed solution options

## Desired Flow

1. A `systemd --user` timer fires once per hour.
2. The timer invokes `muxd`.
3. `muxd` targets a dedicated Zellij session for GitHub issue analysis.
4. If that session does not exist, it is created.
5. `muxd` targets a dedicated tab inside that session.
6. If that tab does not exist, it is created.
7. `muxd` launches a new pane in that tab.
8. `muxd` starts the default agent, `codex`, with a predefined prompt.
9. The prompt instructs the agent to:
   - pick new unprocessed issues
   - gather context and supporting data
   - analyze the problem
   - produce candidate solution approaches

## Why This Use Case Matters

This scenario is a good reference case because it combines:

- recurring automation through `systemd --user`
- multiplexer workspace conventions
- one default agent workflow
- a repeatable prompt template
- incremental processing of new work

It is more representative of the intended product direction than a trivial `echo hello` launch.

## Product Implications

This use case suggests later support for:

- workspace conventions such as dedicated sessions and tabs
- an optional "ensure session/tab exists" flow
- default launch profiles for repeated automations
- reusable prompt templates
- agent-oriented launch presets, even if payload commands stay generic underneath

## Scope Note

This use case is intentionally broader than the current first release.

It contains several capabilities that are not part of the thin-wrapper MVP:

- create session if missing
- create or reuse dedicated tab if missing
- default agent selection
- default prompt templates

For the current MVP, only the thinnest useful slice should be assumed:

- `muxd launch` runs a caller-provided payload command into an existing Zellij session

The rest of this use case should inform the roadmap rather than silently expand the first implementation slice.
