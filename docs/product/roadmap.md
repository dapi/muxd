# muxd Roadmap

Date: 2026-03-13

## Purpose

This roadmap sequences product growth from a thin launch wrapper into richer multiplexer tooling only when each step is justified.

## R0: Zellij launch wrapper

Goal:

- make `muxd launch` useful from `systemd --user` timers

Scope:

- existing Zellij session
- one launch command
- one or two safe targets
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

## R1: Honest blocking path

Goal:

- add `--wait` only where the backend can support it cleanly

Scope:

- blocking launch for supported target paths
- documented semantic differences where Zellij cannot block directly

## R2: Defaults and ergonomics

Goal:

- reduce repetition in timer units and scripts

Scope:

- config/defaults for session, target, and working directory
- clearer naming conventions
- better human help and examples
- optional default launch profile for recurring workflows such as issue analysis

## R3: Target expansion

Goal:

- support more placement options without hiding backend tradeoffs

Scope:

- additional Zellij targets
- explicit unsupported combinations
- "ensure workspace exists" helpers such as session or tab creation, if they prove necessary

## R4: tmux evaluation

Goal:

- decide whether the thin-wrapper contract survives a second backend cleanly

Scope:

- review public CLI semantics
- identify which backend differences remain user-visible
- add tmux only if the contract still feels honest
