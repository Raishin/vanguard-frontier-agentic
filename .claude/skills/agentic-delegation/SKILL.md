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

## Workflow templates

Three reusable orchestration shapes cover most multi-step tasks in this repo. Reach for one of
these before inventing a bespoke delegation plan.

### a) Recon sweep

Parallel Haiku `Explore` agents, one question each, citations required.

- Split the open-ended question into narrow, independent sub-questions — one per agent, one
  area of the tree each.
- Launch all Explore agents in the same message so they run in parallel, not sequentially.
- Require file:line citations in every finding, same as section (a) above.
- **When to use** — you don't yet know where something lives, or need a map of an unfamiliar
  area before deciding what to change.
- **Hard constraints** — read-only; Explore agents may not `Edit`/`Write`. No commits. If a
  sweep comes back thin or off-target, re-run it with a tighter prompt rather than accepting a
  vague report.

### b) Spec-driven implementation

Orchestrator writes an exact file-scoped spec, Sonnet implements, orchestrator reviews the diff
and runs decisive verification before accepting.

- Orchestrator writes the spec first: exact file paths, the content/code shape expected, which
  repo conventions to mirror, and acceptance criteria stated concretely.
- Delegate the spec verbatim to a Sonnet subagent — do not compress it to a one-line ask; a
  vague handoff produces a vague implementation.
- Orchestrator reads the resulting diff in full before running any gate — do not skip straight
  to "did the gate pass."
- Run the gate(s) relevant to the touched files (schema validation, `npm run validate`,
  `cargo test`, linters) and treat a pass as necessary, not sufficient, for acceptance.
- **When to use** — the shape of the change is fully known up front (new file, defined edit to
  an existing one) and doesn't require architectural judgment mid-implementation.
- **Hard constraints** — files it may touch: exactly the list in the spec, nothing else. No
  commits — the orchestrator commits after review.

### c) Gate run

Haiku runs the full repo gate suite and reports pass/fail with raw failure output.

- Delegate to Haiku: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` (for
  `tools/vfa-tui`), `npm run validate`, `codespell`, `npx markdownlint-cli2`, then
  `npm run asset-integrity:write` **last**, only after every other gate is green.
  Regenerating integrity before other generators finish stales the manifest — see
  the ordering caveat in `CLAUDE.md`/`AGENTS.md`.
- Require raw failure output verbatim in the report — not a paraphrase like "some tests
  failed." The orchestrator needs the actual error to decide the next move.
- **When to use** — verifying a change is ready before the orchestrator reviews/commits, or a
  periodic health check with no code changes attached.
- **Hard constraints** — this is a read/verify pass: the only file it may write is
  `catalog/asset-integrity.json` via `asset-integrity:write`, and only after all other gates
  pass. No other edits. No commits — report results back to the orchestrator, who decides
  whether to fix, re-run, or commit.
