# muxd Zellij Backend

Date: 2026-03-13

## Purpose

This document records the Zellij-specific behavior that should stay out of the thin wrapper core.

It covers:

- launch command mapping
- supported target paths for the first release
- preflight checks
- backend limitations the CLI must expose honestly

## Validated Assumptions

Confirmed working assumptions for the current product direction:

- session selection works through `zellij -s <session> ...`
- `run` supports `--cwd`
- pane and tab naming are available
- target placement differs by command form
- target support should be introduced carefully, not all at once

## Useful Commands

### Validation

| Action | Command shape |
| - | - |
| list sessions | `zellij list-sessions` |

### Launch

| Target | Command shape | Notes |
| - | - | - |
| `new_pane` | `zellij -s <s> run --name <n> --cwd <dir> -- <cmd>` | strongest first candidate for MVP |
| `floating_pane` | `zellij -s <s> run --floating --name <n> --cwd <dir> -- <cmd>` | candidate for later slice |
| `new_tab` | `zellij -s <s> action new-tab --name <n> --close-on-exit -- <cmd>` | likely later because semantics differ |

## First Release Recommendation

Start with:

- `new_pane`

Reason:

- it maps cleanly to `zellij run`
- it supports `--cwd`
- it avoids widening the first release around target-specific differences

## Preflight Behavior

The adapter should validate:

- `zellij` exists in `PATH`
- the requested session is visible in `zellij list-sessions`
- the requested target is in the supported target set for the current release

## Naming

If the user supplies `--name`, pass it through to Zellij.

If no name is supplied, the CLI may either:

- omit the name entirely
- or generate a simple deterministic name in a later ergonomics slice

The first release should avoid inventing a hidden task identity model.

## Limitations

### Backend-specific syntax remains visible internally

The wrapper should hide raw command construction from users, but the implementation must still respect Zellij-specific flag and target differences.

### Target parity should not be faked

If one target behaves differently or is not yet well-supported, the CLI should reject it explicitly rather than pretending all targets are equivalent.

### `--wait` is not part of the first slice

Blocking semantics should be considered only after the basic launch path is in place.

Different targets may have different blocking support, so this should be added later and documented carefully.
