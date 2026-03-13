# Spec-driven development workflow

Date: 2026-03-13

## Purpose

This repository should move from problem framing to implementation through explicit documents, not through ad hoc coding.

The goal of the workflow is simple:

- product intent is written down before solution details drift
- technical decisions are made against a named spec
- plans execute approved specs instead of replacing them
- implementation starts only when the current slice is clear enough

## Artifact Roles

### PRD

Location:

- `docs/product/prd.md`

Purpose:

- define the product problem
- define target users, MVP scope, and non-goals
- stay stable across multiple implementation slices

Exit criteria:

- the problem is clear
- scope and non-goals are explicit
- open questions are named

### Research note

Location:

- `docs/research/YYYY-MM-DD-topic-name.md`

Purpose:

- compare options
- capture spike findings
- reduce uncertainty before a decision or spec

Exit criteria:

- decision inputs are concrete
- unknowns are either closed or clearly isolated

### Spec

Location:

- `docs/specs/YYYY-MM-DD-topic-name.md`

Purpose:

- define one implementation-ready slice
- describe requirements, constraints, acceptance criteria, and edge cases
- connect the slice back to the PRD

Exit criteria:

- the slice has clear in-scope and out-of-scope behavior
- acceptance criteria are testable
- dependencies and decisions are linked

### ADR

Location:

- `docs/adr/0001-topic-name.md`

Purpose:

- capture irreversible or expensive-to-reverse technical decisions

Exit criteria:

- context, options, decision, and consequences are explicit

### Plan

Location:

- `docs/plans/YYYY-MM-DD-topic-name.md`

Purpose:

- sequence approved work into implementation phases
- identify blockers, order, and validation work

Exit criteria:

- work can be executed without redefining the feature
- validation approach is included

## Working Flow

1. Update the PRD when the product problem, user scope, or MVP boundary changes.
2. Write a research note when major uncertainty exists.
3. Write a spec for the next implementation slice.
4. Add or update ADRs when the spec forces a durable technical decision.
5. Write the implementation plan from the approved spec.
6. Start implementation only after steps 1 through 5 are complete enough for the slice.
7. After implementation, update the spec, ADR, and plan status if reality changed.

## Approval Rule

For repository-changing implementation work, the expected minimum artifact set is:

- PRD
- one relevant spec
- any required ADRs
- one execution plan

If uncertainty is still high, add research before implementation instead of coding around the ambiguity.

## Current Repository Rule

`docs/adr/0001-stack-selection.md` is accepted and the implementation stack is Rust.

Current expectation:

- new production code should follow an approved spec, ADR set, and execution plan
- implementation should preserve the documented backend-neutral core and lifecycle semantics

## Authoring Guidance

- keep headings short and sentence-case
- prefer explicit scope boundaries over aspirational language
- separate product requirements from implementation steps
- separate decisions from research inputs
- separate plans from specs
