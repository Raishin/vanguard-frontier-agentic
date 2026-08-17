---
layout: default
title: "Agentic Delegation"
permalink: /docs/agentic-delegation/
---

# 🧭 Agentic Delegation

Most work in this repository decomposes into a large amount of cheap, parallelizable effort
plus a small amount that genuinely needs judgment. This page documents the delegation model
that split applies to, and the executable workflow that encodes it.

Two artifacts implement it:

| Artifact | Path | What it is |
|---|---|---|
| Doctrine | `.claude/skills/agentic-delegation/SKILL.md` | The rules a contributor or agent follows by hand |
| Executable form | `.claude/workflows/agentic-delegation.js` | The same rules as a runnable multi-agent workflow |

Both live under `.claude/`, which is excluded from the published site in `_config.yml` — this
page is the public description of what they do.

---

## The model

### Route by cost, not by habit

| Work | Delegate to | Why |
|---|---|---|
| Read-only reconnaissance — locating files, mapping call sites, summarizing structure | Haiku | Cheap, parallel, and mechanical. Findings are checkable because citations are mandatory |
| Bulk writing — docs, boilerplate, repetitive multi-file edits against an exact spec | Sonnet | The shape of the change is already decided; what remains is faithful execution |
| Verifying an external claim against primary documentation | Sonnet, with Context7 | Requires judging whether a retrieved snippet actually supports a claim |
| Running the gate suite | Haiku | Deterministic commands; the value is raw output, not interpretation |

Haiku **never orchestrates**. It explores and it runs gates; it does not plan, decompose, or
accept work. When Sonnet orchestrates, it runs at high reasoning effort at minimum — a weak
plan wastes every delegate downstream, so the planning step is the wrong place to economise.

### What never gets delegated

- Architecture and design decisions — schema shapes, scope boundaries, precedence rules.
- Security-sensitive code — auth, secrets handling, trust-boundary logic.
- Surgical edits to load-bearing logic — validation gates, schemas, catalog generators.
- Final verification and the commit itself.

### A delegate's self-report is not verification

This is the rule that makes the rest safe. A subagent reporting success is evidence that it
believes it succeeded. Before accepting any delegated output the orchestrator reads the diff
and runs the repo's own gates — `npm run validate`, `npm run lint:spell`, `markdownlint-cli2`,
and `cargo test` when `tools/vfa-tui/` changed. A passing gate is *necessary*, not sufficient.

---

## The executable workflow

`.claude/workflows/agentic-delegation.js` runs the model as four phases. Every field of its
input is optional, so a task supplies only the phases it needs.

```js
{
  objective: 'one line stating what the change is for',
  recon:  [{ question, where }],                       // Haiku Explore, parallel
  claims: [{ id, claim, libraryId, encodedAt }],       // Context7 verify, then refute
  specs:  [{ label, files, spec, conventions, acceptance }],  // Sonnet write, then review
  runGates: true,                                      // Haiku gate run
}
```

### Phase 1 — Recon

Parallel Haiku `Explore` sweeps, one narrow question each. Findings are returned under a
schema that **requires** a repo-relative path and a line number, so an uncitable finding
cannot be reported. Each sweep also returns a `gaps` list — what it could not establish —
because an empty gaps array is a claim that the agent looked, and silence is not.

This phase uses a barrier: the implementation spec is written against the union of what recon
found, so nothing downstream should start on a partial map.

### Phase 2 — Verify

For each external claim, one Context7 retrieval against a primary source, then an
**adversarial refuter** on anything that came back clean. The refuter exists because a
verifier asked "is this right?" agrees far too easily; it is instructed to hunt for adjacent
documentation the first pass missed — a timeline, an exception, a superseding recommendation,
an edition or version gate.

The verdict vocabulary is deliberately four-valued:

| Verdict | Means |
|---|---|
| `AGREE` | Documentation supports the claim, and the supporting snippet is quoted |
| `PARTIAL` | Literally true, but omits documented material that would change a reader's decision |
| `DISAGREE` | Documentation contradicts the claim as encoded |
| `UNVERIFIABLE` | Retrieval could not settle it — including an `AGREE` with no quoted evidence, which is downgraded |

`PARTIAL` earns its place: the most expensive documentation defect is usually not a false
statement but a true one that leaves out the part a reader needed.

This phase is pipelined — each claim refutes as soon as its own retrieval lands, so a slow
claim never blocks a fast one.

### Phase 3 — Implement

Sonnet writes against a file-scoped spec, then a reviewer reads the result. The reviewer is
told explicitly that the writer's self-report is not evidence, and checks three things: does
the content satisfy the spec, did it touch **only** the allowed files (confirmed with
`git status --short`), and does it mirror the conventions of neighbouring files.

Every writing delegate is hard-constrained on the files it may touch, and no delegate in any
phase may run a git write command — the orchestrator owns the commit.

### Phase 4 — Gates

One Haiku pass over the repo suite in dependency order, stopping at the first failure and
returning **raw failing output verbatim** rather than a paraphrase. A stale-manifest or
model-policy failure is a regeneration decision that belongs to the orchestrator, so the gate
stage reports it rather than fixing it.

This is the only stage permitted to write a file — `catalog/asset-integrity.json`, via
`npm run asset-integrity:write`, and only after every other gate is green. Regenerating
integrity before the other generators settle stales the manifest; see the ordering caveat in
[`CLAUDE.md`](https://github.com/VincentChuWaiChow/vanguard-frontier-agentic/blob/master/CLAUDE.md).

---

## Design properties

**Deterministic.** The script never reads the wall clock and never uses randomness. Behaviour
changes only when the committed script or the passed arguments change, so a run is
reproducible and CI cannot change colour without a commit.

**Capped, and never silently.** An agent budget holds the worst case at 14 subagents, in line
with this repository's workflow-size guideline. Every truncation is logged — a work-list that
was cut says so, because silent truncation reads as "covered everything" when it did not.

**Separated output.** The workflow returns a decision surface rather than a narrative:
everything requiring the orchestrator's attention is separated from everything that merely
passed, and the return value states in plain terms what the orchestrator still owns. A green
workflow is not a finished change.

---

## Worked example

The Snowflake board's verification pass used the model directly: three parallel Haiku sweeps
(volatile-claim inventory, cross-reference resolution, reference-quality audit) alongside a
Context7 verification of seven encoded vendor claims.

Six claims verified `AGREE` with quoted evidence. One returned a real defect — the encoded
statement that no Snowpipe Streaming retirement date is published was accurate but omitted a
documented *"Expected timeline and migration window"* section stating that a formal
announcement carrying the final end-of-life date was planned, with an 18-month sunset period
following it. Accurate, and materially under-informative: a planner reading the original text
would conclude the migration could not be scoped, when in fact an 18-month clock starts on
announcement.

That is the class of defect the adversarial phase exists to find, and it is exactly the class
a confirmation-seeking review misses.

---

## Cross-references

- [Architecture](architecture/) — the three-layer system the delegated work produces
- [Testing](testing/) — the validation gates the gate phase runs
- [Contributing](contributing/) — conventions a writing delegate is told to mirror
- [Execution Tiers](execution-tiers/) — the trust posture every generated asset must declare
