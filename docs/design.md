# muxd Design

Date: 2026-03-13

## Purpose

This document is the top-level architecture overview for `muxd`.

Use it to understand the current product shape before going into deeper documents.

Supporting documents:

- product scope: `docs/product/prd.md`
- roadmap: `docs/product/roadmap.md`
- workflow: `docs/process/spec-driven-development.md`
- launch CLI contract: `docs/architecture/launch-cli.md`
- backend notes: `docs/architecture/backends/zellij.md`
- execution plan: `docs/plans/2026-03-13-implementation-plan.md`
- decisions: `docs/adr/`

## Overview

`muxd` is a thin CLI wrapper for launching arbitrary commands into an existing terminal multiplexer session.

The first release targets Zellij and is intended to be called from `systemd --user` timers and other local automation.

The current product is intentionally not a daemon or dispatcher.

## Product Goal

Make recurring launches into an existing multiplexer session simple, stable, and script-friendly.

Representative use cases:

- a `systemd --user` timer runs `muxd launch` instead of embedding raw Zellij syntax
- a developer standardizes launch naming, target selection, and working directory through one CLI
- later: add tmux support without forcing callers to rewrite their launch contract

## Non-Goals for Current Release

- no daemon
- no Unix socket IPC
- no queue
- no task ids
- no `list`, `status`, or `cancel`
- no persistence
- no tmux in the first release

## Design Principles

- thin wrapper first
- backend-specific launch logic stays in adapters
- stable automation-facing CLI
- explicit validation and failure modes
- no fake abstraction over unsupported backend semantics

## User Stories

### US-1: Launch from timer

As a developer, I want a `systemd --user` timer to call one stable command that launches work into an existing Zellij session.

### US-2: Avoid backend syntax

As a developer, I want to avoid copying raw Zellij command forms into every script and timer unit.

### US-3: Add backends later

As a maintainer, I want backend command construction isolated so tmux can be evaluated later without changing the user-facing contract too early.

## Core Concepts

### Launch request

A launch request is one CLI invocation that asks `muxd` to place a command into a backend session.

Canonical fields for the first release:

- `backend`
- `session`
- `target`
- `name`
- `cwd`
- `command`
- `args`

### Backend

Backend remains a first-class concept even though the first release supports only Zellij.

Initial backend enum:

- `zellij`
- `tmux` reserved for later

### Target

Target describes where the backend should place the launched command.

The first release should support only the smallest safe target set.

Portable values to keep in view:

- `new_pane`
- `floating_pane`
- `new_tab`

Unsupported targets should fail explicitly.

## Architecture

### Components

- CLI parser
- launch request validator
- backend adapter interface
- Zellij backend adapter
- optional config loader in a later slice

### Logical Flow

1. caller invokes `muxd launch`
2. CLI validates arguments and environment
3. backend adapter maps the request to backend command syntax
4. `muxd` executes the backend command
5. CLI returns a stable exit code and user-facing message

## Backend Architecture

### Separation Rule

The core must not know:

- exact Zellij command syntax
- exact tmux command syntax
- backend-specific flag quirks
- backend-specific launch workarounds

That logic belongs inside backend adapters.

### Backend Adapter Responsibilities

- preflight validation
- launch command construction
- environment-specific feature checks
- best-effort support for optional blocking paths in later slices

### Core Responsibilities

- CLI semantics
- argument validation
- exit code semantics
- config defaults in later slices

## Zellij First Release

Confirmed working assumptions:

- session selection works through `zellij -s <session> ...`
- `--cwd` is supported for `run`
- pane and tab naming are available
- launch placement differs by target

Implication:

- Zellij is a good first backend for a thin wrapper
- target support should expand carefully rather than all at once

Detailed backend notes live in:

- `docs/architecture/backends/zellij.md`

## Scheduler Boundary

Scheduling belongs outside `muxd`.

The current intended scheduler is:

- `systemd --user` timers

This keeps the first release focused on launch semantics rather than scheduling or orchestration.

## Future Direction

If the thin wrapper proves useful, later slices may add:

- honest blocking behavior where the backend supports it
- defaults/config
- more launch targets
- tmux support

Dispatcher-style features should be treated as a later product expansion, not as hidden MVP work.

## Stack Decision

The implementation stack is Rust.

The decision is recorded in:

- `docs/adr/0001-stack-selection.md`
