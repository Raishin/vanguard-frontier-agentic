# Red Team and Acceptance Gates

> Status: **PLAN — Phase 0 code registration LANDED (commit `2ff7461a`); no board asset built.**
> This document scores the design, not a built board.
> Previous: [06-implementation-roadmap-and-integration.md](./06-implementation-roadmap-and-integration.md) · Next: [08-authoring-templates.md](./08-authoring-templates.md)

## 1. Method

Evidence buckets, applied to every claim scored below:

- **Verified** — read from a primary source or the repository during this work, with a citation.
- **Observed** — a visible pattern in the material, not independently confirmed.
- **Inference** — a reasonable conclusion from limited evidence.
- **Unknown** — material and unchecked. Named as unknown, never filled in.

Scoring: 0–5 per axis. **No axis scores 4 or 5 without verified or strong observed evidence.** A
score reflects the weakest critical dependency, not the most polished artifact. A design whose
boundaries are excellent and whose version facts are stale scores on the version facts.

## 2. The fifteen attacks

Attacks 1 to 12 are the ones the commissioning brief specified. Attacks 13 to 15 are repo-specific
and were found during reconnaissance.

### A1 — Duplicate agent

| Field | Value |
|---|---|
| Method | Find two agents that would return materially the same review. Give both the same input. |
| Correct behavior | Different outputs, or one agent declines and names the other. |
| Design change made | Three merges and two rejections in [02](./02-agent-prosecution-scorecard.md): tsconfig-policy and typed-lint collapsed into one agent, codegen-drift into runtime-boundary, language-service into build-graph, test-contract into public-api. Plus the artifact-scope split in [03 §3](./03-final-board-and-boundary-contracts.md) and a disqualifying rule that Non-overlap ≤2 fails regardless of total score. |
| Residual risk | Medium. The type-soundness and frontend-contracts pair remains the closest on the board and depends on a boundary written into another board's asset. See §4. |

### A2 — Duplicate references

| Field | Value |
|---|---|
| Method | Normalize every skill's reference source list and compare. Flag repeated source sets, repeated prose, and references present without a specialist rationale. |
| Correct behavior | No two skills carry the same source set; every shared source has a stated different question. |
| Design change made | The reference ownership matrix in [05 §5](./05-skill-and-reference-architecture.md) requires a per-skill question for every shared source; `official-sources.md` is limited to six skills and `safety-checklist.md` to two; four anti-landfill rules name the specific sources that must appear nowhere as standalone entries. |
| Residual risk | Medium. The matrix is a plan; bulk authoring is where duplication actually enters, which is why it is also a risk-register row in [06 §9](./06-implementation-roadmap-and-integration.md). |

### A3 — Stale TypeScript assumptions

| Field | Value |
|---|---|
| Method | Compare the guidance against the installed compiler and the current release notes. |
| Correct behavior | Every version-dependent claim is gated on evidence, and a stale premise is corrected rather than accommodated. |
| Design change made | This attack already landed and the plan absorbed the hit. The brief asserted TypeScript 6.0 was "on the path toward the native TypeScript 7 effort"; 7.0 reached general availability 2026-07-08 and is npm's `latest` at 7.0.2. Corrected in [00 §4.1](./00-reconnaissance-and-evidence-map.md). Every specialist gates verdicts on user-supplied versions because this repository has no TypeScript program of its own. |
| Residual risk | **High, and structural.** Two compounding facts: the official tsconfig prose page is stale on removed `module` and `moduleResolution` values, so "check the official page" is necessary but not sufficient; and typescript-eslint supports `>=4.8.4 <6.1.0` while TypeScript 7.0.2 is current, so a repository can sit outside its lint tooling's supported window. Mitigation is the provenance ledger's retrieval dates plus re-verification at build time — not a claim of durability. |

### A4 — Runtime false confidence

| Field | Value |
|---|---|
| Method | Supply code that type-checks and accepts corrupt runtime data: a generated interface over an unparsed webhook payload. |
| Correct behavior | `typescript-runtime-boundary-contract-agent` catches it. `typescript-type-soundness-agent` must decline to call it safe on types alone. |
| Design change made | Runtime-boundary owns boundary inventory, parse-don't-validate, and the explicit ruling that a generated type is a claim rather than a check ([03 §2.3](./03-final-board-and-boundary-contracts.md)). Type-soundness refuses validator design. |
| Residual risk | Low. This is the board's strongest-scoring agent and the clearest ownership on it. |

### A5 — ESM and CJS ambiguity

| Field | Value |
|---|---|
| Method | Supply a package that compiles and fails for one consumer mode — `types` after `import` in the conditions object, one declaration file for both builds. |
| Correct behavior | `typescript-module-resolution-and-emit-agent` owns it; `typescript-node-execution-compatibility-agent` must not claim it. |
| Design change made | Candidate 4 was renamed away from "module-runtime-interop" precisely because the word "runtime" pulled execution questions into a packaging agent ([02 candidate 4](./02-agent-prosecution-scorecard.md)). Both agents' refuse lists name each other. |
| Residual risk | Low. |

### A6 — Node type-stripping misconception

| Field | Value |
|---|---|
| Method | Supply a project whose source appears runnable by modern Node but depends on compiler behavior: an `enum`, a `paths` alias, no `tsc` job in CI. |
| Correct behavior | The execution specialist distinguishes execution from type checking, quotes the documented limits, and requires proof of a separate typecheck gate. |
| Design change made | `typescript-node-execution-compatibility-agent` exists for this single misconception, and its reference carries the verbatim documentation lines "no type checking is performed" and "Node.js ignores `tsconfig.json` files" ([05 §8](./05-skill-and-reference-architecture.md)). |
| Residual risk | Low on diagnosis. Medium on currency: type stripping moved from flagged to default to stable across four Node minors, and `--experimental-transform-types` was removed entirely in v26.0.0. |

### A7 — Monorepo performance

| Field | Value |
|---|---|
| Method | Supply a pathological build graph and ask for a fix without providing any measurement. |
| Correct behavior | Refuse to prescribe project references. Ask for `--extendedDiagnostics` output or a trace first. |
| Design change made | `typescript-build-graph-performance-agent` carries an explicit refusal trigger: no measurement, no prescription ([03 §2.7](./03-final-board-and-boundary-contracts.md)). Its `trace-evidence-protocol.md` reference is built around that rule. |
| Residual risk | Medium. Whether the tracing switches behave identically under the native TypeScript 7 compiler is **unknown** and recorded as such; the agent must state which compiler produced a measurement rather than assume parity. |

### A8 — Declaration breaking change

| Field | Value |
|---|---|
| Method | Change a public generic or exported type with no runtime change, and ship it as a patch. |
| Correct behavior | `typescript-public-api-and-declaration-governance-agent` classifies it as breaking and names the required version bump, with consumer impact. |
| Design change made | That agent owns classification, the semver decision, and the consumer compilation matrix — including the absorbed type-testing candidate, so the classification and its proof have one owner ([02 candidate 11](./02-agent-prosecution-scorecard.md)). |
| Residual risk | Low if a baseline surface or API report exists; the agent labels the classification as inference and requests a baseline when it does not. |

### A9 — Supply-chain compromise

| Field | Value |
|---|---|
| Method | Introduce an unsafe publication condition: a long-lived token in CI, no provenance, a publish-time script, an unscoped internal name. |
| Correct behavior | `typescript-package-publication-integrity-agent` finds it and hands organization-wide identity and key custody to the security board. |
| Design change made | Candidate 12 was narrowed to publication only, leaving dependency intake with `package-governance-agent` and signing with the sigstore board ([03 §2.10](./03-final-board-and-boundary-contracts.md)). Its TypeScript-specificity was scored honestly at 3 rather than inflated. |
| Residual risk | Medium. Registry policy is dated and moving — trusted publishing GA 2025-07-31, classic tokens revoked 2025-12-09, further granular-token restrictions dated 2026-07-31 with a stated January 2027 target. One unverified item remains: whether CircleCI is currently a supported provider. |

### A10 — MCP schema drift

| Field | Value |
|---|---|
| Method | Change a TypeScript handler's behavior and leave the declared `inputSchema` stale. |
| Correct behavior | `typescript-mcp-tool-contract-agent` owns it; runtime-boundary hands off. |
| Design change made | The MCP agent was accepted on a verified ownership gap, and runtime-boundary's refuse list names MCP tool wire contracts explicitly ([03 §2.3](./03-final-board-and-boundary-contracts.md)). |
| Residual risk | **High on currency, low on ownership.** The current specification revision `2026-07-28` removed the `initialize` handshake and protocol sessions, moved the version into `_meta.io.modelcontextprotocol/protocolVersion`, returns `-32022` on mismatch, requires `server/discover`, and the TypeScript SDK split into two 2.0.0 packages while the old one became legacy. Guidance authored against the previous revision is wrong, not merely dated. |

### A11 — Frontend duplication

| Field | Value |
|---|---|
| Method | Give the maestro a React-specific issue with no TypeScript language or toolchain reasoning. |
| Correct behavior | Decline and hand to `frontend-maestro-agent`. Do not absorb it. |
| Design change made | The routing matrix in [04 §3](./04-routing-architecture-and-fixtures.md) carries explicit decline rows for framework-only questions and for other languages; the maestro's refusal list forbids answering and forbids inventing an agent. |
| Residual risk | Low for framework-only tasks. Medium for mixed tasks — a React question that also involves a shared package's exported types is genuinely both, which is what the tie-break rule in [03 §3](./03-final-board-and-boundary-contracts.md) exists to resolve. |

### A12 — Fake business case

| Field | Value |
|---|---|
| Method | Ask the economics agent for a return figure with incomplete inputs. |
| Correct behavior | Refuse. Name every missing measurement. Show the formula that would consume it. |
| Design change made | `typescript-engineering-economics-agent` is accepted **conditionally**: it consumes measurements and never originates them, may not be dispatched first, and is re-prosecuted after two quarters ([02 candidate 19](./02-agent-prosecution-scorecard.md)). Its `measurement-intake-and-refusal.md` reference is the refusal contract. |
| Residual risk | Medium, and honest: this is the weakest accepted agent, and the failure mode it guards against is the one it is structurally capable of committing. |

### A13 — The routing gate does not enforce its own fixture

| Field | Value |
|---|---|
| Method | Ship a maestro with no `tests/fixtures/typescript-maestro-routing/taxonomy.json` and run the gate. |
| Correct behavior | Recognize that `tests/validate-maestro-routing.py:155` prints `SKIP` rather than failing, and that an empty `inputs/` directory only warns. |
| Design change made | [04 §6](./04-routing-architecture-and-fixtures.md) states the softness explicitly and makes the fixture a self-imposed requirement. This document treats a missing or empty fixture as a blocker regardless of what CI reports. |
| Residual risk | Low once known. High for anyone who trusts a green pipeline. |

A second, worse variant of the same class was found in review: `taxonomy.json` is **generated**, and
`tests/_generate_maestro_routing_fixtures.py:308` overwrites it on every run. A hand-curated taxonomy
is destroyed by the next `maestro-routing:write`, and because `expected/` is regenerated from the
grader against the replacement, **the gate passes afterwards**. The green pipeline actively conceals
the loss. Corrected in [04 §5.5](./04-routing-architecture-and-fixtures.md): keywords are driven
through agent summaries, and durable curation requires a generator change in its own commit.
Residual risk: medium — the check is a human diff of `taxonomy.json` after every regeneration.

### A14 — Agent tier versus skill grant incoherence

| Field | Value |
|---|---|
| Method | Compare an agent's declared `execution_tier` and its prose against its companion skill's `allowed-tools`. |
| Correct behavior | They agree. A `static-review` agent's skill grants no Bash. |
| Design change made | The live example is in the repository: `agents/frontend/typescript-contracts-agent/AGENT.md` forbids Bash execution of `tsc` while `skills/frontend/typescript-contracts-review/SKILL.md` grants `Bash(tsc --noEmit:*)`. Documented in [05 §3](./05-skill-and-reference-architecture.md) as precedent not to copy; every TypeScript board skill grants `Read Grep Glob`, with `WebFetch` on three named skills. |
| Residual risk | Low for new assets. The existing defect is left in place deliberately — fixing another board's asset is a separate change, argued on its own merits. |

### A15 — Provider registered in the schemas but not in the Rust enum

| Field | Value |
|---|---|
| Method | Add `typescript` to the schemas, the catalog validator, and the generators, but not to `tools/vfa-tui/src/models/provider.rs`. Open a catalog-only pull request. |
| Correct behavior | Recognize that CI's `Gate` job is path-filtered to `tools/vfa-tui/**`, so the pull request passes while the TUI can no longer deserialize the catalog. |
| Design change made | Steps 0.4a and 0.4b in [06 §1](./06-implementation-roadmap-and-integration.md) with the mandatory local cargo gates, and two rows in the risk register. |
| Residual risk | Low if the checklist is followed, high otherwise — this is a process control, not a technical one. |

Review found the harder half of this attack: the Rust registration is **two edits**, and only the
first fails loudly. `infer_provider` at `tools/vfa-tui/src/federation/coverage.rs:331` maps the
provider path component through a separate hardcoded match whose `_ => Provider::Generic` fallback
(line 345) silently groups the whole board under Generic. Adding only the enum variant yields green
schemas, a green `cargo test`, and a wrong TUI. Nothing in the repository detects it, which is why
step 0.4b requires a focused path-mapping test as part of the fix rather than as a nicety.

## 3. Design scorecard

Scored against the design as it stands, not against an implementation that does not exist.

| Axis | Score | Evidence | Biggest gap | Confidence | What would raise it |
|---|---|---|---|---|---|
| Boundary clarity | 4 | Every agent has an owns/refuses/hands-off contract and a verbatim ownership sentence ([03](./03-final-board-and-boundary-contracts.md)); ten cross-domain boundaries stated in both directions | Boundaries are asserted in a plan; none has been tested by a real routing conflict | Medium | Running the routing fixtures and the §4 probes against built agents |
| Frontend non-duplication | 3 | The artifact-scope split is written, with a tie-break rule and named refusals | It depends on editing another board's asset, which has not been agreed. Until then, two agents can claim the same diff | Medium | Frontend-board owner sign-off on the split |
| Compile-time versus runtime separation | 5 | The distinction is the basis for two separate agents (runtime-boundary, node-execution), with verbatim documentation quotes and a pain register that ranks the two erasure pains first and second | None material | High | — |
| Version currency | 3 | Every external fact carries a label, source, and retrieval date; the brief's own TypeScript premise was refuted and corrected | Four facts remain unverified; the official tsconfig page is demonstrably stale; TypeScript moved during this work and will move again | Medium | Re-verification at build time, which the plan requires but cannot perform in advance |
| Reference specificity and non-duplication | 4 | Ownership matrix with a per-skill question for every shared source; `official-sources.md` limited to six skills; four anti-landfill rules | No reference file exists yet, so duplication is prevented by rule rather than by inspection | Medium | Authoring the references and running the A2 normalized comparison |
| Evidence discipline | 4 | Six-label scheme applied throughout; unverified rows stay unverified; a Context7 claim was overridden by primary sources and the conflict recorded | Repo `file:line` citations were verified selectively rather than exhaustively | High | An audit pass re-checking every cited line |
| Business-impact honesty | 4 | The economics agent is accepted conditionally with a refusal contract and a removal date; P09's score is explicitly called an understatement of tail risk | No measured figures exist anywhere in the plan, by design — which is correct but means every economic claim is a formula, not a result | High | Nothing at plan time; this axis is capped until a user supplies measurements |
| Repository-convention fidelity | 4 | Layout, metadata, harness set, tier, tool grants, category enum, roles, and generator ordering all derived from cited repo evidence; six brief assumptions refuted with evidence | The category assignments and role lists are unvalidated against the gates until assets exist | High | Running `npm run validate` with the assets present |
| Routing robustness | 3 | Grader mechanics read from source; anchors designed for lexical uniqueness against the IDF filter; four adversarial fixtures retained | Anchors are illustrative and unfitted; the alphabetical tie-break is arbitrary; thirteen domains is a dense space for a keyword-count grader | Medium | Generating the fixtures and confirming every domain routes at score above zero |
| Implementation readiness | 3 | Phase 0's five code registration points are landed and gate-verified (commit `2ff7461a`), including the silently-failing `infer_provider` arm and its test; the file inventory, role coverage, generator ordering, and gate commands are concrete | **No asset is built.** 126 agent files and 76 skill files are unwritten, the routing fixture does not exist, and the board does nothing yet | High | Executing phases 7 to 11 |

No axis scores 5 except compile-time versus runtime separation, which is the one place where the
design's central claim is fully grounded in verified primary documentation and structurally
enforced by two separate agents with mutually exclusive refusals.

## 4. Duplicate-agent probes

The three closest pairs, each with a discriminating input and the colliding input.

### `typescript-type-soundness-agent` versus `agents/frontend/typescript-contracts-agent`

- **Discriminating input A:** a shared `Result<T, E>` helper in a published package whose type
  predicate returns `true` for values it never inspected. Type-soundness owns it: the finding is
  that the abstraction lies. The frontend agent has no jurisdiction — this is not an application
  diff.
- **Discriminating input B:** a React component diff that adds `as any` to a `fetch` response. The
  frontend agent owns it: the finding is laundering in an application diff. Type-soundness declines.
- **Colliding input:** a diff in a shared component library that both adds `as any` at a fetch
  boundary and introduces an unsound generic. Both agents have a real claim.
- **Tie-break:** the TypeScript board owns the type model; the diff audit returns to the frontend
  agent. Shipped verbatim in both agents.

### `typescript-module-resolution-and-emit-agent` versus `typescript-node-execution-compatibility-agent`

- **Discriminating input A:** a CommonJS consumer cannot import the package because `types` is
  ordered after `import` in the conditions object. Resolution owns it.
- **Discriminating input B:** the service crashes on start with `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX`
  because an `enum` is on the startup path. Execution owns it.
- **Colliding input:** a `paths` alias resolves in the editor and fails at runtime. Resolution
  explains why the alias is a compile-time construct; execution owns the finding that the runtime
  does not honor it.
- **Tie-break:** if the failure is observed at runtime, execution owns it and consults resolution.
  If the failure is observed in a consumer's build, resolution owns it.

### `typescript-static-enforcement-policy-agent` versus `typescript-build-graph-performance-agent`

- **Discriminating input A:** typed lint rules are enabled but no type information reaches them, so
  they silently pass. Enforcement owns it.
- **Discriminating input B:** `tsc` takes nine minutes and a trace shows one conditional type
  dominating instantiation. Build-graph owns it.
- **Colliding input:** CI spends eleven minutes and nobody knows whether it is lint or build.
  Both have a claim, and the honest answer is that this is a measurement request.
- **Tie-break:** build-graph owns the measurement and reports the split. Enforcement then owns
  whether each expensive check is worth keeping.

## 5. Adversarial findings — what was designed wrong first, and corrected

Each finding names the wrong initial design, the evidence that killed it, and the correction.

1. **Starting from the brief's 19-agent list.** The first pass treated the candidate list as a
   backlog to validate rather than a hypothesis set to prosecute. Corrected by inverting the
   default to "no" and adding a disqualifying Non-overlap rule, which eliminated five candidates
   that a validation-oriented pass would have shipped.
2. **A standalone codegen and schema-drift agent.** It scored 26, above threshold. Its central
   claim — a generated type is a claim, not a check — is the runtime-boundary agent's thesis
   restated. Corrected: rejected on Non-overlap 1 and merged, and the threshold rule was amended so
   a total score cannot buy past a duplicate.
3. **A standalone tsconfig-policy agent and a standalone typed-lint agent.** Both looked
   independently defensible. They answer the same question — what must the toolchain prove, and at
   what cost — so they were merged into `typescript-static-enforcement-policy-agent`. The merge is
   what makes it a decision domain rather than a settings review.
4. **A standalone developer-tooling and language-service agent.** Editor latency reads as a
   distinct concern from CI time. It is the same type graph measured at a different moment, and both
   agents would have prescribed the same restructuring. Merged into build-graph.
5. **Assuming the AWS progressive-disclosure gate applied board-wide.** It does not:
   `tests/validate-aws-progressive-disclosure.py:12` scopes it to `skills/aws/**`. The initial plan
   would have claimed gate enforcement it does not have. Corrected: the discipline is stated as
   self-imposed, with the gate's own thresholds adopted voluntarily.
6. **Assuming a new language board ships `rules/` assets.** Rules are harness-scoped
   (`rules/<harness>/`), `schemas/rule.schema.json`'s provider enum excludes language boards, and no
   language board ships any. Corrected: the board ships no rules.
7. **Assuming Context7 could be declared in `allowed-tools`.** The enforced token grammar at
   `tests/validate-skill-allowed-tools.py:34` cannot express `mcp__Context7__query-docs`, and zero
   catalog skills declare one. Corrected: Context7 is a workflow step with a documented protocol,
   never a grant.
8. **Assuming the maestro routing gate would fail a missing fixture.** It prints `SKIP`
   (`tests/validate-maestro-routing.py:155`). Corrected: the fixture is a self-imposed requirement
   and a blocker in this document, independent of what CI reports.
9. **Scoring the economics agent below threshold on a mis-scored dimension.** It first scored 24 —
   a rejection — on TypeScript-specificity 2 and loss exposure 3. Re-examined: its inputs
   (typecheck minutes, editor latency, declaration-breakage tickets) are TypeScript-specific
   artifacts even though the arithmetic is generic, and misallocating a platform team for a year is
   a material loss. Re-scored to 27 and accepted **conditionally**, with the conditions recorded
   rather than the score quietly adjusted.
10. **Counting seven provider-registration points.** `CLAUDE.md`'s own list plus the schemas
    suggested seven. The eighth is `tools/vfa-tui/src/models/provider.rs`, and it is the one CI's
    path-filtered `Gate` job will not catch. Corrected throughout, and promoted to attack A15.
11. **Assuming `.claude/**` required an asset-integrity refresh.** The manifest's `scope.trees` does
    not include it. Corrected: this workflow needs no integrity refresh; the implementation does.
12. **Trusting a retrieval-layer answer over a primary source.** A Context7 snippet asserted that
    `strict` is not default-true in TypeScript 6.0. The official release announcement and an
    empirical test against the installed compiler both refute it. Corrected, recorded as a worked
    example of the evidence-precedence ladder, and it is the reason the ladder is written into
    [05 §7](./05-skill-and-reference-architecture.md) rather than assumed.
13. **Planning to add a `ROADMAP.md` entry at plan time.** The existing
    `.claude/workflow/m365-d365/` plan is referenced from nowhere in the repository. Corrected:
    the ROADMAP entry moves to implementation time so the two workflow directories stay consistent.

Findings 14 to 18 were caught by automated review of this plan, not during authoring. They are
recorded here rather than quietly fixed, because four of them are defects in the plan's most
load-bearing instructions — the kind that would have been executed verbatim by an implementer.

14. **Calling `taxonomy.json` a hand-authored SOURCE file, and telling the author to write it and
    then run the generator.** `tests/_generate_maestro_routing_fixtures.py:308` overwrites it
    unconditionally, so step 2 of the procedure destroyed the artifact step 1 created — and the
    regenerated `expected/` files kept the gate green over the loss. Corrected: `taxonomy.json` is
    generated, keywords are driven through agent summaries (which makes summary wording a routing
    input), and durable curation requires a generator change in its own commit.
15. **A `live_guard_intent` regex containing `publish`, `migrate`, and `backfill`.** Those are
    domain anchors for three specialists on this board. Because the gate branch runs before domain
    scoring and `live_guards` is empty, `tests/validate-maestro-routing.py:100` returns an empty
    route — so "review this backfill for idempotency" could never reach the agent built to review
    it, and the fixture would have passed. The repo's own generator default uses destructive verbs
    only; the plan had diverged from a convention that was already correct. Corrected in
    [04 §5.4](./04-routing-architecture-and-fixtures.md) with a board rule.
16. **Putting the hand-written provider documentation in Phase 0.** `generate-docs-data.mjs:27`
    derives the provider list from agent metadata, so a `docs/taxonomy.md` bullet added while zero
    agents exist makes `CLAUDE.md`'s provider invariant false — the phase's own zero-asset exit
    criterion guaranteed the violation. Corrected: both docs lists defer to Phase 10.
17. **Hardcoding "14 agents and 14 skills" in the proposed ROADMAP text.** `ROADMAP.md` has no
    count generator, `CLAUDE.md` forbids hardcoded counts, and this board explicitly permits
    re-prosecution and removal — so the number was stale by design. Corrected to describe coverage
    without cardinality.
18. **Two smaller internal contradictions.** The anti-landfill rule capped `official-sources.md` at
    five skills while the inventory and the copy-ready template both gave runtime-boundary a sixth —
    corrected to six, with runtime-boundary's version-gated library facts as the justification. And
    the economics agent's contract in [03 §2.14](./03-final-board-and-boundary-contracts.md) carried
    two of its three binding acceptance conditions, omitting the re-prosecution deadline — which
    would have silently converted a conditional acceptance into a permanent agent. Both corrected.

## 6. Acceptance checklist for the implementation

Every line must be true before review is requested. Items marked **gate-blind** are not caught by
CI and must be checked by a human.

- [ ] All eight provider-registration points updated, including the Rust enum. **Gate-blind** for
      a catalog-only change.
- [ ] `npm run validate` green with the provider registered and zero assets, before any asset lands.
- [ ] All 14 agents carry `execution_tier: static-review` and a `copilot.agent.md` `tools:` block
      with no execution tool.
- [ ] No skill declares a Bash grant. No `SKILL.md` contains a bash fence.
- [ ] Every `SKILL.md` is at or under 90 lines, contains `Load these only when needed`, and links
      every reference it declares. **Gate-blind** outside `skills/aws/**`.
- [ ] Every skill `category` is a value in the closed enum.
- [ ] `tests/fixtures/typescript-maestro-routing/taxonomy.json` exists and `inputs/` is non-empty.
      **Gate-blind** — the gate prints `SKIP`.
- [ ] The generated `taxonomy.json` was diffed after the last `maestro-routing:write`, and its
      `live_guard_intent` contains no domain noun. **Gate-blind** — a black-holed domain still passes.
- [ ] `infer_provider` in `tools/vfa-tui/src/federation/coverage.rs` has a `typescript` arm and a
      path-mapping test. **Gate-blind** — omitting it keeps every gate green.
- [ ] `docs/taxonomy.md` and `docs/language-stack-boards.md` were updated with the first agent, not
      during Phase 0, and `provider_list` in `docs/_data/catalog.yml` now contains `typescript`.
- [ ] Every domain in the taxonomy routes at a score above zero for its own generated fixture.
- [ ] The four adversarial fixtures are present and passing.
- [ ] All 14 agents appear in at least one install role.
- [ ] The frontend boundary edit is either merged with owner sign-off, or
      `typescript-type-soundness-agent` is re-prosecuted. **Gate-blind.**
- [ ] Every external fact in the shipped references traces to a provenance-ledger row, and no
      unverified row has been upgraded to an assertion. **Gate-blind.**
- [ ] The A2 reference-duplication comparison has been run. **Gate-blind.**
- [ ] Both probes from [06 §6.1](./06-implementation-roadmap-and-integration.md) executed and
      recorded.
- [ ] `npm run asset-integrity:write` run last, on its own, after every other generator.
- [ ] `npm run lint:spell`, markdownlint, and the three cargo gates green.

## 7. Final gate

**`CONDITIONAL PASS — blockers listed`**

Blockers:

1. **Phase 0 is partially met.** The five code registration points are landed and verified
   (commit `2ff7461a`): both schema enums, `ALLOWED_PROVIDERS`, the docs-data taxonomy row, and
   both Rust touch points including the `infer_provider` arm and its path-mapping test. The Kiro
   Power and the two hand-written documentation lists remain deferred, the latter deliberately —
   `provider_list` is derived from agent metadata, so a `docs/taxonomy.md` bullet added before the
   first agent would break the provider invariant, and nothing machine-checks that equality.
2. **No implementation exists.** 126 agent files, 76 skill and reference files, and one routing
   fixture are unwritten. The plan is complete; the board is not.
3. **External version facts must be re-verified at authoring time.** TypeScript invalidated the
   commissioning brief's premise during this work, the official tsconfig page is demonstrably stale
   on removed option values, the MCP specification's current revision breaks its predecessor, and
   four facts remain explicitly unverified.
4. **The frontend boundary edit requires owner sign-off.** Without it,
   `typescript-type-soundness-agent`'s Non-overlap score is 2, which this document's own rule
   disqualifies.
5. **The economics agent carries a re-prosecution date.** Accepting it conditionally is a decision
   with a deadline attached, not a settled question.

`PASS` is withheld deliberately. Files existing is not evidence of a working board, and the two
axes that would have to be strong for a merge verdict — routing robustness and implementation
readiness — score 3 and 2 respectively on verified evidence.

`FAIL` is not the verdict either. The architecture is differentiated: five candidates were
eliminated on overlap, the frontend boundary is drawn on artifact scope rather than wording, the
compile-time and runtime split is structurally enforced by two agents with mutually exclusive
refusals, and every external claim carries a source and a retrieval date.

## 8. What would invalidate this document

- Any attack's residual risk changes because the underlying tool, gate, or specification changed.
- The frontend-board owner's decision on the §4 split, either way.
- The implementation is built, at which point this document must be re-scored against artifacts
  rather than a plan — and the implementation-readiness axis becomes the primary measurement rather
  than the primary gap.
- Two shipped agents are found to return materially the same review, which is an A1 failure
  discovered late and requires a merge rather than a boundary memo.
