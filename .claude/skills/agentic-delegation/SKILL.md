---
name: agentic-delegation
description: "Delegate exploration sweeps to Haiku subagents and bulk writing to Sonnet subagents while the orchestrator keeps architecture, security-sensitive edits, and commits; use at the start of any multi-step task in this repo to minimize token spend by delegating to cheaper models."
allowed-tools: ["Agent", "TaskCreate", "TaskUpdate"]
---

# Agentic Delegation

## Doctrine

Most tasks in this repo decompose into cheap, parallelizable work plus a small amount of
work that genuinely needs the orchestrator's judgment. Default to delegating the former.
Before doing multi-step work yourself, ask: can a cheaper model do this step just as well?

## a) Exploration and reconnaissance → Haiku

- Use the `Explore` agent type with `model: haiku` for read-only reconnaissance: locating
  files, grepping for symbols, mapping call sites, summarizing existing structure.
- Scope each Explore task tightly — one question, one area of the tree. Do not send an
  Explore agent an open-ended "understand the whole system" ask; split it into targeted
  sweeps instead.
- Require file:line citations in every finding. A report without exact paths and line
  numbers is not actionable — re-run it with a tighter prompt rather than accepting it.

## b) Bulk writing → Sonnet

- Route bulk writing — docs, guides, boilerplate, test scaffolding, repetitive multi-file
  edits — to Sonnet subagents.
- Give each writing task a precise spec: exact file paths to create or edit, the content
  shape expected, and which repo conventions to mirror (frontmatter shape, heading
  structure, existing tone).
- Hard-constrain every writing delegate:
  - **Files it may touch** — list them explicitly; nothing outside that list.
  - **Linters/gates it must pass** — e.g. `npx markdownlint-cli2`, `codespell`, or the
    schema/validation gate relevant to the files it is touching.
  - **No commits** — delegates write files; only the orchestrator commits.

## c) What the orchestrator keeps

Never delegate:

- Architecture and design decisions (schema shapes, scope boundaries, precedence rules).
- Security-sensitive code (auth, secrets handling, trust-boundary logic).
- Surgical edits to load-bearing logic (validation gates, schemas, catalog generators).
- Final verification and the commit itself.

## d) Every delegate gets

- Exact file paths — absolute, not "somewhere in docs/".
- Acceptance criteria — what "done" looks like, stated concretely and checkably.
- An explicit "do NOT" list — files not to touch, commands not to run (no
  `npm run validate`, no `cargo test`, no `git commit` inside a delegate unless
  explicitly asked to run them for verification).

## e) Verify before accepting

- Run the repo's own gates on delegate output before treating it as done: `npm run
  validate`, `cargo test` (for `tools/vfa-tui`), `npx markdownlint-cli2`, `codespell`.
- A delegate's self-report is not verification — read the diff, run the gate, then accept.
