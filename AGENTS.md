# Repository Guidelines

## Project Structure & Module Organization

This repository is currently documentation-first. Keep product and architecture notes in [`docs/design.md`](./docs/design.md), decisions in `docs/adr/`, and execution plans in `docs/plans/`. Follow the existing naming patterns:

- ADRs: `docs/adr/0001-topic-name.md`
- Plans: `docs/plans/YYYY-MM-DD-topic-name.md`

There is no production source tree yet. Per `docs/adr/0001-stack-selection.md`, code should not be added until the Go vs. Rust spike decision is accepted.

## Build, Test, and Development Commands

No build, test, or lint toolchain is checked in yet. Current contribution work is centered on reviewing and updating the docs:

- `sed -n '1,200p' docs/design.md` reviews the current architecture draft.
- `sed -n '1,200p' docs/adr/0001-stack-selection.md` checks the active stack decision status.
- `rg --files docs` lists all tracked design documents quickly.

When the first implementation lands, add the canonical build/test commands to `README.md` and update this guide in the same change.

## Coding Style & Naming Conventions

Match the existing Markdown style: short sections, sentence-case headings, concise bullets, and direct language. Prefer ASCII unless a file already requires Unicode. Use descriptive kebab-case filenames for new docs. Keep repository-wide terminology consistent with the design docs, especially `backend`, `task`, `status`, and `target`.

For future code, keep layout and naming driven by the accepted stack decision rather than by temporary experiments.

## Testing Guidelines

There is no automated test suite yet. For documentation changes, verify cross-references, filenames, and dates by hand before submitting. If you add spike code for the stack evaluation, include a runnable command and a short validation note in the relevant ADR or plan document.

## Commit & Pull Request Guidelines

The repository has no commit history yet, so use a simple convention: imperative, present-tense subjects such as `Add stack evaluation notes` or `Refine muxd lifecycle model`. Keep commits focused on one logical change.

Pull requests should include a short summary, affected files, and any follow-up decisions needed. Link the relevant plan or ADR when applicable. For structural or terminology changes, explain how they align with `docs/design.md`.
