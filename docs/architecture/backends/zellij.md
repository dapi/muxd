# muxd Zellij Backend

Date: 2026-03-13

## Purpose

This document records the Zellij-specific behavior that should stay out of the backend-neutral core.

It combines:

- confirmed CLI capabilities
- MVP launch mapping
- completion and cancellation semantics
- backend limitations the core must not hide

## Validated Assumptions

Confirmed working assumptions for MVP:

- session selection works through `zellij -s <session> ...`
- `zellij run` supports `--block-until-exit`
- pane and tab naming are available
- `--cwd` is supported
- `list-panes` and `list-tabs` can be used for polling-based inspection
- arbitrary pane cancellation remains limited

## Useful Commands

### Launch

| Action | Command shape | Notes |
| - | - | - |
| new tab with command | `zellij -s <s> action new-tab --name <n> --close-on-exit -- <cmd>` | returns tab id |
| new pane with command | `zellij -s <s> run --name <n> --cwd <dir> -- <cmd>` | returns pane id unless blocking |
| floating pane | `zellij -s <s> run --floating --name <n> -- <cmd>` | returns pane id unless blocking |
| block until exit | `zellij -s <s> run --block-until-exit -- ...` | useful for oneshot |

### Monitoring

| Action | Command shape |
| - | - |
| list panes | `zellij -s <s> action list-panes --json --command --state --tab` |
| list tabs | `zellij -s <s> action list-tabs --json --panes --state` |
| list sessions | `zellij list-sessions` |
| close tab by id | `zellij -s <s> action close-tab-by-id <id>` |

## Task Naming

Each launched task should be named `task-<task-id>`.

This gives the backend adapter a stable way to:

- find a pane or tab later through polling
- distinguish daemon-launched work from unrelated user panes
- correlate terminal objects with daemon task state

## Launch Mapping

### Mode to Zellij behavior

| Mode | Target | Blocking path | Close on exit |
| - | - | - | - |
| `oneshot` | `new_pane` | yes | yes |
| `oneshot` | `floating_pane` | yes | yes |
| `oneshot` | `new_tab` | no | yes |
| `interactive` | any supported target | no | no |

### Target to command shape

| Target | Command shape |
| - | - |
| `new_tab` | `zellij -s <session> action new-tab --name "task-<id>" --close-on-exit -- <agent-cmd>` |
| `new_pane` | `zellij -s <session> run --name "task-<id>" --cwd <cwd> -- <agent-cmd>` |
| `floating_pane` | `zellij -s <session> run --floating --name "task-<id>" --cwd <cwd> -- <agent-cmd>` |

## Completion Semantics

### Oneshot

Preferred path:

- use `zellij run --block-until-exit --close-on-exit`
- read exit status directly from the blocking command

Special case:

- `new_tab` does not support `--block-until-exit`
- for `new_tab` oneshot, the adapter must poll `list-panes` or `list-tabs`

### Interactive

Interactive launches should not use the blocking path.

The adapter should:

- launch the task in a user-facing pane or tab
- poll backend state until the pane disappears or exits
- treat user-driven closure as normal interactive completion unless a stronger failure signal is observed

## Cancellation Semantics

Zellij cancellation is best-effort.

Known limitation:

- there is no reliable `close-pane-by-id` equivalent for arbitrary panes without focus switching

Implications:

- cancelling a pending task is strong and deterministic
- cancelling a running `new_tab` task can use `close-tab-by-id` when the adapter has a tab id
- cancelling a running pane task may only mark the task cancelled and attempt backend termination opportunistically

The core must not overstate these guarantees.

## Backend Limitations

### `new_tab` cannot block directly

`action new-tab` does not provide a blocking completion path comparable to `run --block-until-exit`.

The adapter must fall back to polling.

### Blocking `run` does not return pane id

When `--block-until-exit` is used, the blocking command path should be treated as an exit-status channel, not as a reliable pane-id discovery mechanism.

### Output inspection is limited

`dump-screen` is focused-pane oriented, so arbitrary output capture should not be assumed for MVP.

### Pane closure is limited

The absence of strong pane-by-id termination is the main reason cancellation semantics must remain explicitly best-effort.

## Recommended Adapter Behavior

- preflight: confirm `zellij` exists and requested session is visible
- launch: prefer blocking execution for oneshot pane targets
- poll: use named tasks and JSON listing commands
- cancel: expose best-effort truthfully
- normalize: store any tab id or pane id in backend-local details only

## Manual Validation Notes

Earlier shell validation established these useful facts:

- `run --block-until-exit` blocks correctly for oneshot execution
- `--close-on-exit` removes finished panes
- `list-panes --json --command --state` exposes enough state for polling
- `new_tab` requires polling-based completion tracking

Those findings justify Zellij as the first backend, but they do not justify leaking Zellij-specific behavior into the core task model.
