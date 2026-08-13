# Ultracode Workflow — TypeScript Agentic Board

> Status: **BUILT — the board is in the catalog.** This directory is the execution plan for adding a
> TypeScript agent and skill board to `vanguard-frontier-agentic`, and the board it describes now
> exists: the provider value is registered across all eight points, and `agents/typescript/`,
> `skills/typescript/`, the routing fixture, the Kiro Power, the install roles, and the hand-written
> provider documentation are all in the tree. Read
> [00-reconnaissance-and-evidence-map.md](./00-reconnaissance-and-evidence-map.md) first.
>
> These documents are kept as the **design record, corrections included** — where implementation
> proved a plan decision wrong, the affected section says so in place rather than being quietly
> rewritten. Two such corrections are marked: the maestro skill's shape
> ([08 §7](./08-authoring-templates.md)) and the routing gate
> ([04 §5.4](./04-routing-architecture-and-fixtures.md), [08 §8](./08-authoring-templates.md)).
> A plan that reads as if it had been right all along is not a record.

## Executive verdict

- A TypeScript board should exist, but it is **not a second frontend board**. Its unit of
  analysis is the TypeScript *program* and the *published package* — compiler configuration,
  module resolution and emit, declaration surface, program-graph cost, Node execution, and the
  runtime boundaries that type erasure leaves undefended.
- **14 agents accepted**: one maestro plus 13 specialists. From the brief's 19 candidates,
  **5 were eliminated as standalone roles** (merged into a stronger owner), **1 was deferred**,
  and **7 were renamed** because their original names invited scope they must not have.
- The highest-loss risks the board addresses, in order: erased types trusted as validation at an
  I/O boundary; code reaching production that was never type-checked because the runtime strips
  types; a published package that resolves or emits wrongly for one consumer mode; a
  declaration-only breaking change shipped as a patch; and a floating promise that turns a
  partial write into a data-integrity incident.
- Differentiation from the existing boards is enforced by an **artifact-scope split**, not by
  wording. `agents/frontend/typescript-contracts-agent` keeps application-diff review;
  the TypeScript board owns shared and published program semantics. Application security keeps
  exploitation; this board keeps contract fidelity. Both directions are written into refusal
  lists, and one existing frontend asset must be edited to say so.
- **A verified ownership gap justifies the MCP specialist.** Nothing in the catalog owns generic
  MCP tool-schema contracts, tool-input validation, or tool-contract versioning — only
  NetSuite- and NVIDIA-specific assets touch MCP at all. In a repository that ships agentic
  assets, that gap is indefensible.
- **The brief's own TypeScript premise was stale, and the plan corrects it.** TypeScript 7.0 is
  already generally available and is npm's `latest`; it is not, as the brief states, a future
  effort that 6.0 leads toward. Every version-dependent claim in these documents carries a
  label, a source, and a retrieval date for exactly this reason.
- **Phase 0 was a hard gate; its code half is now done.** The provider is registered in both
  schema enums, the catalog validator, the docs-data generator, and both Rust touch points —
  including `infer_provider`, whose omission CI's path-filtered `Gate` job would not have caught
  (commit `2ff7461a`, gates green). Still outstanding by design: the Kiro Power, and the two
  hand-written documentation lists, which land in the same commit as the first agent because the
  generated provider list is derived from agent metadata.
- **The board ships static-review only.** No agent runs a command, publishes a package, or
  mutates anything. The automation-governance specialist reviews privileged scripts and never
  executes them; a live control plane is explicitly deferred with named entry criteria.
- **This repository contains no TypeScript program of its own**, so no agent may ground a
  verdict in "the installed version". Every version-gated conclusion depends on evidence the
  user supplies — a design constraint, not a caveat.
- Merge safety: **`CONDITIONAL PASS`** — unchanged, and now for a different reason. The plan-stage
  conditions (Phase 0 unmet, assets unwritten) are discharged; what remains conditional is that the
  external version facts in [00 §4](./00-reconnaissance-and-evidence-map.md) are point-in-time and
  the board's own verdicts depend on evidence a *user* supplies, since this repository still has no
  TypeScript program to ground them in. See
  [07-red-team-and-acceptance-gates.md](./07-red-team-and-acceptance-gates.md).

## Why this directory exists

A long adversarial brief asked for a Fortune-50-grade TypeScript board. This directory converts
that brief into **repo-accurate, sequenced, gated work** instead of a wishlist. It is
deliberately hostile to its own conclusions: every candidate agent is prosecuted before it is
accepted, every assumption the brief made about this repository is checked against the
repository, and the assumptions that do not hold are named and refuted rather than quietly
accommodated.

The precedent for this artifact shape is [`.claude/workflow/m365-d365/`](../m365-d365/README.md).

## The two findings that gate everything

**1. Registering `typescript` as a provider touches eight places.** The provider value is a
closed enum enforced independently in several files, plus two hand-written documentation lists.
The code points were registered first, in commit `2ff7461a`; the two documentation lists followed
with the first agent, for the reason given below. All eight are now done:

- `schemas/agent.schema.json` (provider enum)
- `schemas/skill.schema.json` (provider enum)
- `tests/validate-catalog.py` (`ALLOWED_PROVIDERS`, a separate hardcoded set)
- `tools/vfa-tui/` — **two edits.** `src/models/provider.rs` (the Rust `Provider` enum — the TUI
  deserializes the catalog with a strict enum, so a missing variant breaks `cargo test`) **and**
  `src/federation/coverage.rs` (`infer_provider`, whose `_ => Provider::Generic` fallback silently
  groups the whole board under Generic if the arm is missing — no gate detects it)
- `scripts/generate-docs-data.mjs` (taxonomy grouping)
- `scripts/generate-kiro-powers.mjs` (only if the board ships a Kiro Power — optional)
- `docs/taxonomy.md` (hand-written provider bullet list)
- `docs/language-stack-boards.md` (hand-written board enumeration and tables)

CI's `Gate` job is path-filtered to `tools/vfa-tui/**`, so a catalog-only pull request passes CI
while leaving the TUI unable to load the catalog. The cargo gates must be run locally.

The two hand-written documentation lists were the exception to "land it all in Phase 0": the
generated provider list is derived from agent metadata, so adding a `docs/taxonomy.md` bullet
before the first agent exists makes the repository's own provider invariant false. They landed with
the first agent, as designed.

**2. The repository has no TypeScript program to reason from.** There is no root
`tsconfig.json`, no `typescript` dependency in `package.json`, and the only `.ts` files are
security-detection test fixtures. Every specialist on this board therefore has an evidence
precondition: it must be given the user's compiler version, configuration, and runtime target,
and it must refuse to issue a version-gated verdict without them.

## Plan documents (read in order)

| File | Purpose |
|------|---------|
| [00-reconnaissance-and-evidence-map.md](./00-reconnaissance-and-evidence-map.md) | Repo conventions with file:line evidence, source-of-truth versus generated map, brief-versus-reality corrections, external version evidence |
| [01-enterprise-pain-register.md](./01-enterprise-pain-register.md) | The 16 ranked failure modes, each with stakeholder, trigger, consequence, current partial owner, and ownership verdict |
| [02-agent-prosecution-scorecard.md](./02-agent-prosecution-scorecard.md) | Every candidate scored across seven dimensions and prosecuted before acceptance, including the rejected and merged ones |
| [03-final-board-and-boundary-contracts.md](./03-final-board-and-boundary-contracts.md) | The accepted 14, each with owns/refuses/hands-off, adversarial hypotheses, and the cross-domain handoff matrix |
| [04-routing-architecture-and-fixtures.md](./04-routing-architecture-and-fixtures.md) | Maestro contract, domain taxonomy, routing matrix, and the routing-fixture design against the real grader |
| [05-skill-and-reference-architecture.md](./05-skill-and-reference-architecture.md) | Skill shape, the reference ownership matrix, the Context7 protocol, and the provenance ledger |
| [06-implementation-roadmap-and-integration.md](./06-implementation-roadmap-and-integration.md) | Phased, gated roadmap; exact file inventory; install roles; generator ordering; validation plan; risk register |
| [07-red-team-and-acceptance-gates.md](./07-red-team-and-acceptance-gates.md) | Fifteen attacks, the design scorecard, the corrections made during design, and the final gate |
| [08-authoring-templates.md](./08-authoring-templates.md) | Copy-ready agent, harness, skill, reference, fixture, and install-role templates |

## Evidence labels used throughout

`E1` user-supplied · `E2` repository pattern observed in this repo (file:line required) ·
`E3` primary vendor or specification document verified during this work ·
`E4` Context7-retrieved (resolved library ID and documented version recorded) ·
`E5` unverified — stated as unknown, never asserted · `E6` design judgment, not a fact

Rule: no external fact appears without a label and a source. Where verification failed, the
document says so. A claim that could not be verified is not upgraded to a claim that was.

## How this plan was executed

The sequence below is the one that was followed, and is the one to repeat for the next board.

1. Land Phase 0 from [06](./06-implementation-roadmap-and-integration.md) — provider
   registration across all eight points, then `npm run validate` and the three cargo gates in
   `tools/vfa-tui` with zero TypeScript assets present. **Hard gate.**
2. Re-verify the external version facts in [00 §4](./00-reconnaissance-and-evidence-map.md)
   before authoring any asset. TypeScript is the fastest-moving of them.
3. Build the 13 specialists before finalizing the maestro's routing table — the routing gate
   rejects a taxonomy that references an agent absent from `catalog/agents.json`.
4. Author skills and their references per [05](./05-skill-and-reference-architecture.md),
   respecting the reference ownership matrix. A shared source requires each skill to state the
   different question it asks.
5. Run the hostile acceptance tests in [07](./07-red-team-and-acceptance-gates.md) and remediate
   before requesting review.
6. Follow the repository's definition of done: generators first, `npm run asset-integrity:write`
   last and on its own, then `npm run validate`, `npm run lint:spell`, markdownlint, and the
   cargo gates.

## What would invalidate this plan

- A frontend-board owner rejects the artifact-scope split, in which case the type-soundness
  specialist loses its non-overlap justification and must be re-prosecuted.
- A Node.js board is created, in which case server-side async reliability migrates out of this
  board and the async specialist is re-scoped.
- An MCP-specific board is created, in which case the MCP tool-contract specialist moves.
- A future TypeScript release restores or removes compiler options this plan treats as settled,
  in which case the affected reference files and the modernization specialist's exposure list
  change.
- The economics specialist produces no decision within two quarters of shipping, in which case
  it is removed rather than defended.
