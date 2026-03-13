# Use Case 0001: Hourly GitHub issue analysis flow

Date: 2026-03-13

Status: Draft

## Summary

A user wants a repeatable GitHub issue analysis workflow that can be launched manually now and later reused from a shell script, always targeting the same dedicated Zellij workspace through `muxd`.

## Goal

Once per hour, or on demand, the user wants `muxd` to start the default agent, `codex`, inside a dedicated Zellij session and tab, in a new pane, with a fixed prompt that kicks off issue analysis work.

The launched agent command should:

- find new GitHub issues that have not yet been processed
- analyze each issue
- collect the required supporting data
- produce proposed solution options

## Desired Flow

1. The user runs `muxd` manually or from a shell script.
2. `muxd` targets a dedicated Zellij session for GitHub issue analysis.
3. If that session does not exist and the caller requested it, it is created.
4. `muxd` targets a dedicated tab inside that session.
5. If that tab does not exist and the caller requested it, it is created.
6. `muxd` launches a new pane in that tab.
7. `muxd` starts the default agent, `codex`, with a predefined prompt.
8. The prompt instructs the agent to:
   - pick new unprocessed issues
   - gather context and supporting data
   - analyze the problem
   - produce candidate solution approaches

## Why This Use Case Matters

This scenario is a good reference case because it combines:

- recurring or on-demand invocation
- multiplexer workspace conventions
- one default agent workflow
- a repeatable prompt template
- incremental processing of new work

It is more representative of the intended product direction than a trivial `echo hello` launch.

## Product Implications

This use case suggests later support for:

- workspace conventions such as dedicated sessions and tabs
- an explicit "ensure session/tab exists" flow
- default launch profiles for repeated automations
- reusable prompt templates
- agent-oriented launch presets, even if payload commands stay generic underneath

## Scope Note

This use case is intentionally broader than the current first release.

It contains several capabilities that are not part of the thin-wrapper MVP:

- create session if missing without explicit flags
- create or reuse dedicated tab if missing without explicit flags
- default agent selection
- default prompt templates

For the current implemented slice, only the thinnest useful path should be assumed:

- `muxd launch` runs a caller-provided payload command into an existing Zellij session

For the next slice, this use case directly motivates:

- explicit session ensure semantics
- explicit tab ensure semantics
- launch into a named tab through `new_pane`
