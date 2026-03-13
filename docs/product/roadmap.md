# muxd Roadmap

Date: 2026-03-13

## Purpose

This roadmap sequences product growth from a thin launch wrapper into richer multiplexer tooling only when each step is justified.

## R0: Zellij launch wrapper

Goal:

- make `muxd launch` useful for manual runs and simple shell scripts

Scope:

- existing Zellij session
- one launch command
- one safe target
- arbitrary payload command after `--`
- `--cwd`
- `--name`
- stable exit codes
- clear validation errors

Deliberately excluded:

- daemon
- queue
- task tracking
- tmux
- config

Reference scenario:

- the hourly GitHub issue analysis flow in `docs/product/use-cases/0001-hourly-github-issue-analysis.md`
- only the launch part of that scenario belongs in `R0`

Active execution plan:

- `docs/plans/2026-03-13-stage-1-delivery-plan.md`

## R1: Defaults and ergonomics

Goal:

- reduce repetition in shell invocations and scripts

Scope:

- config/defaults for session, target, and working directory
- clearer naming conventions
- better human help and examples
- one documented user-level config file

Active spec:

- `docs/specs/2026-03-13-launch-defaults-config.md`

Why next:

- this better supports repeated manual and shell-driven invocations
- it removes repeated boilerplate without pulling the product toward lifecycle orchestration

## R2: Workspace ensure semantics

Goal:

- make recurring workflows target a stable Zellij workspace without manual prep

Scope:

- optional session creation when missing
- tab selection by name
- optional tab creation when missing
- launch into a new pane inside the selected tab
- explicit flags so the behavior stays visible to the caller

Active spec:

- `docs/specs/2026-03-13-zellij-workspace-ensure.md`

Guardrails:

- no hidden scheduler or task model
- no agent-specific abstraction in the core CLI
- no prompt template system in this slice

## R3: Honest blocking path

Goal:

- add `--wait` only where the backend can support it cleanly

Scope:

- blocking launch for supported target paths
- documented semantic differences where Zellij cannot block directly

Guardrail:

- `--wait` must not smuggle daemon or task-tracking semantics back into the thin-wrapper product

## R4: Target expansion

Goal:

- support more placement options without hiding backend tradeoffs

Scope:

- additional Zellij targets
- explicit unsupported combinations

## R5: tmux evaluation

Goal:

- decide whether the thin-wrapper contract survives a second backend cleanly

Scope:

- review public CLI semantics
- identify which backend differences remain user-visible
- add tmux only if the contract still feels honest
