# muxd Design

Date: 2026-03-13

## Overview

`muxd` is a standalone task dispatcher for terminal multiplexers.

It runs as a local daemon, accepts task requests over a Unix socket, schedules them, launches them inside an existing multiplexer session, and reports lifecycle state back to CLI clients.

The first backend is Zellij. The architecture must stay suitable for a later tmux backend without rewriting the core queue, IPC, or task model.

## Product Goal

Make execution inside a terminal multiplexer scriptable, observable, and queueable through one stable interface.

Representative use cases:

- queue three review tasks into one session
- block in a shell script until a oneshot task finishes
- inspect pending and running work from another terminal
- later: switch backend from Zellij to tmux without changing the user-facing task model

## Non-Goals for MVP

- no network API
- no scheduler
- no webhooks
- no persistence beyond what is required for correct local daemon behavior
- no attempt to hide every backend-specific limitation

## Design Principles

- backend-neutral core
- backend-specific adapters
- explicit lifecycle semantics
- honest cancellation behavior
- script-friendly CLI and IPC

## User Stories

### US-1: Queue work

As a developer, I want to enqueue several tasks against the same multiplexer session so they run without manual tab or pane management.

### US-2: Wait in scripts

As a developer, I want `enqueue --wait` to block until a task finishes so I can use `muxd` in scripts and automation.

### US-3: Inspect load

As a developer, I want to list pending and running tasks so I can see what the dispatcher is doing.

### US-4: Add backends later

As a maintainer, I want backend-specific logic isolated so tmux support can be added later without disturbing queueing, IPC, and lifecycle code.

## Core Concepts

### Task

A task is one requested execution inside a multiplexer backend.

Canonical fields:

- `id`
- `prompt`
- `agent`
- `backend`
- `session`
- `target`
- `mode`
- `name`
- `cwd`
- `status`
- `created_at`
- `started_at`
- `completed_at`
- `exit_code`
- `error`
- `backend_handle`

### Backend

Backend is a first-class part of the task model even though MVP supports only Zellij.

Initial backend enum:

- `zellij`
- `tmux` reserved for later

### Target

Target describes where the backend should place the task.

Portable values for now:

- `new_tab`
- `new_pane`
- `floating_pane`

Not every backend will support every target in the same way. Unsupported combinations should fail explicitly.

### Mode

- `oneshot`
- `interactive`

`oneshot` means the dispatcher is expected to observe natural completion and record exit state.

`interactive` means the dispatcher launches a user-facing interactive process and monitors backend state until it exits or disappears.

### Status

- `pending`
- `running`
- `completed`
- `failed`
- `cancelled`

These statuses are backend-neutral. Backend details should be attached separately when needed, not leaked into the status enum.

## Lifecycle Semantics

### Completion

- `completed`: task finished successfully by backend-observed exit status
- `failed`: task terminated with backend-observed failure or launch error
- `cancelled`: task was cancelled by user intent, regardless of whether backend process termination was guaranteed

### Cancellation

`cancelled` means the dispatcher stopped treating the task as active work.

It does not automatically mean:

- subprocess definitely died
- pane/window definitely disappeared
- backend guaranteed termination

If a backend cannot guarantee hard termination, that limitation should be reflected in `error` or backend details, not hidden.

## Architecture

### Components

- daemon
- CLI client
- task store and scheduler
- IPC protocol
- backend adapter interface
- Zellij backend
- config loader

### Logical Flow

1. CLI sends a request over Unix socket
2. daemon validates request and creates a task
3. scheduler picks the next runnable task
4. backend adapter launches or monitors it
5. daemon updates lifecycle state
6. CLI queries or waits for terminal state

## Backend Architecture

## Separation Rule

The core must not know:

- exact Zellij command syntax
- exact tmux command syntax
- pane/window ids of a specific backend format
- backend-specific polling or cancellation tricks

That logic belongs inside backend adapters.

### Backend Adapter Responsibilities

- preflight validation
- launch execution
- lifecycle polling
- best-effort cancellation
- normalization of backend-specific handles into `backend_handle`

### Core Responsibilities

- queueing
- state transitions
- IPC
- config defaults
- CLI semantics
- status rendering

## Backend Capability Model

To avoid false portability, backend support should be modeled explicitly.

Each backend adapter should answer:

- supported targets
- supported modes
- whether oneshot can block directly
- whether interactive completion requires polling
- whether cancellation is best-effort or strong

This can stay internal for MVP, but the design should assume these capabilities differ.

## Zellij MVP

Confirmed working assumptions:

- session is selected with `zellij -s <session> ...`
- oneshot launch can block with `--block-until-exit`
- interactive launch requires polling
- pane and tab naming are available
- `--cwd` is supported natively
- arbitrary pane cancellation is limited

Implication:

- Zellij is a good first backend
- cancellation semantics must stay honest

## tmux Future Direction

tmux should be added as a second backend implementation, not as branching logic spread through daemon code.

Differences expected to stay backend-local:

- command construction
- session/window/pane identifiers
- launch placement model
- lifecycle inspection
- cancellation capabilities
- output capture

Before adding tmux, the project should review whether any Zellij assumptions leaked into core APIs.

## CLI and IPC

### CLI Shape

Expected command family:

- `muxd run`
- `muxd enqueue`
- `muxd list`
- `muxd status`
- `muxd cancel`

### IPC Choice

Local Unix socket with NDJSON is sufficient for MVP.

Reasons:

- easy to inspect manually
- easy to script
- stack-neutral
- low ceremony for daemon/CLI split

## Config

Config should express semantics, not implementation library preferences.

Required defaults:

- default backend
- default session
- default agent
- default target
- default mode
- max concurrent
- max pending

Backend-specific config should be namespaced so tmux and Zellij can evolve independently.

## Stack Decision

The implementation stack is intentionally still open.

The decision between Go and Rust is tracked in:

- `docs/adr/0001-stack-selection.md`

Architecture should stay stack-neutral until that ADR is accepted.

## Risks

- backend portability may tempt premature abstraction
- cancellation semantics may remain backend-specific
- agent CLIs may evolve independently from `muxd`
- one backend may need concepts that do not map cleanly to another

## Decision Guardrails

- keep task model backend-neutral
- keep backend capabilities explicit
- do not pretend portability where semantics differ
- add tmux only after Zellij MVP validates the core boundary
