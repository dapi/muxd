# muxd Zellij Backend

Date: 2026-03-13

## Purpose

This document records the Zellij-specific behavior that should stay out of the thin wrapper core.

It covers:

- launch command mapping
- supported target paths for the first release
- preflight checks
- workspace ensure helpers
- backend limitations the CLI must expose honestly

## Validated Assumptions

Confirmed working assumptions for the current product direction:

- session selection works through `zellij -s <session> ...`
- `run` supports `--cwd`
- pane and tab naming are available
- target placement differs by command form
- target support should be introduced carefully, not all at once

Assumptions that need to be validated in the next slice:

- session creation can be expressed through a stable non-interactive command path
- tab lookup and tab creation can be modeled without pretending all tab semantics are backend-neutral

## Useful Commands

### Validation

| Action | Command shape |
| - | - |
| list sessions | `zellij list-sessions` |
| list tab names in session | `zellij -s <s> action query-tab-names` |

### Launch

| Target | Command shape | Notes |
| - | - | - |
| `new_pane` | `zellij -s <s> run --name <n> --cwd <dir> -- <cmd>` | strongest first candidate for MVP |
| `floating_pane` | `zellij -s <s> run --floating --name <n> --cwd <dir> -- <cmd>` | candidate for later slice |
| `new_tab` | `zellij -s <s> action new-tab --name <n> --close-on-exit -- <cmd>` | likely later because semantics differ |

### Workspace ensure

| Action | Command shape | Notes |
| - | - | - |
| create missing session | `zellij attach --create-background <session>` | detached non-interactive session creation |
| select existing tab by name | `zellij -s <s> action go-to-tab-name <tab>` | fails if tab does not exist |
| select or create tab by name | `zellij -s <s> action go-to-tab-name --create <tab>` | explicit caller opt-in |
| launch into selected tab | `zellij -s <s> action new-pane --name <n> --cwd <dir> -- <cmd>` | used after tab selection |

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

For the workspace ensure slice, the adapter should also define:

- how to create a missing session when the caller requested it
- how to detect whether a named tab already exists
- how to create a named tab when the caller requested it
- how to keep those steps explicit in logs and errors

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

### Workspace creation should not hide backend differences

If Zellij cannot reliably inspect or create a workspace element through a clean non-interactive path, the CLI should expose that limitation instead of faking parity.
