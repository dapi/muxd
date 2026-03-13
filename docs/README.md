# muxd Documentation

## Purpose

This directory is the source of truth for product scope, architecture, specs, decisions, research, and delivery plans.

The repo started documentation-first. `docs/adr/0001-stack-selection.md` is now accepted, so production Rust code may be added as long as it follows the documented product and architecture boundaries.

## Document Map

- `docs/product/`
  - product intent and scope
- `docs/specs/`
  - implementation-ready feature specs
- `docs/architecture/`
  - system overview and technical reference
- `docs/adr/`
  - architectural decisions
- `docs/research/`
  - spikes, evaluations, and comparison notes
- `docs/plans/`
  - execution plans and phased delivery work
- `docs/process/`
  - documentation and delivery workflow
- `docs/templates/`
  - starting points for new docs

## Working Order

1. Start with the product problem in `docs/product/prd.md`.
2. Add research notes when important unknowns still exist.
3. Write a spec in `docs/specs/` before implementation work starts.
4. Record irreversible decisions in `docs/adr/`.
5. Turn the approved spec into an execution plan in `docs/plans/`.
6. Implement only after the spec, plan, and required ADRs exist.

## Naming Rules

- ADRs: `docs/adr/0001-topic-name.md`
- plans: `docs/plans/YYYY-MM-DD-topic-name.md`
- specs: `docs/specs/YYYY-MM-DD-topic-name.md`
- research: `docs/research/YYYY-MM-DD-topic-name.md`

## Current Entry Points

- architecture overview: `docs/design.md`
- PRD: `docs/product/prd.md`
- roadmap: `docs/product/roadmap.md`
- use cases: `docs/product/use-cases/`
- launch contract: `docs/architecture/launch-cli.md`
- workflow: `docs/process/spec-driven-development.md`
