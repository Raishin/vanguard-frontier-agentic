# Agent Prosecution Scorecard

> Status: **PLAN — not implementation.**
> Previous: [01-enterprise-pain-register.md](./01-enterprise-pain-register.md) · Next: [03-final-board-and-boundary-contracts.md](./03-final-board-and-boundary-contracts.md)

Every candidate was prosecuted before it was accepted. The default answer to "should this agent
exist" is no.

## 1. Method

Seven dimensions, each scored 0–5, maximum 35:

| Dimension | Question |
|---|---|
| Loss exposure | Can failure cause meaningful money, outage, security, compliance, or productivity loss? |
| Frequency | Is the problem common enough to justify persistent specialization? |
| TypeScript specificity | Does solving it require real TypeScript judgment rather than general engineering? |
| Non-overlap | Is its ownership distinct from every existing and proposed agent? |
| Evidence reachability | Can it reach conclusions from concrete repository or runtime evidence? |
| Actionability | Does its output change an engineering decision? |
| Enterprise leverage | Does solving it scale beyond one developer or one file? |

**Acceptance rule: total ≥26 AND Non-overlap ≥3.**

**Disqualifying rule: Non-overlap ≤2 fails regardless of total.** An agent that returns
materially the same review as an existing one is a liability, not a specialist — it splits
ownership, doubles maintenance, and gives a reviewer two verdicts to reconcile with no rule for
which wins.

The threshold is set at 26 of 35 because the repository's existing boards demonstrate what a
weak-but-plausible agent costs: it survives review, ships, and then never gets dispatched
because the maestro cannot articulate when it beats a neighbour. A candidate that cannot clear
74 percent across seven honest dimensions is that agent.

## 2. Scorecard

Dimension order: Loss · Frequency · TS-specificity · Non-overlap · Evidence · Actionability · Leverage.

| # | Candidate | L | F | TS | NO | E | A | Lev | Total | Verdict |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `typescript-maestro-agent` | 3 | 5 | 2 | 5 | 3 | 4 | 5 | 27 | KEEP |
| 2 | `typescript-language-type-safety-agent` → `typescript-type-soundness-agent` | 4 | 5 | 5 | 3 | 5 | 4 | 4 | 30 | KEEP + RENAME |
| 3 | `typescript-runtime-boundary-contract-agent` | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 35 | KEEP + ABSORB 14 |
| 4 | `typescript-module-runtime-interop-agent` → `typescript-module-resolution-and-emit-agent` | 5 | 4 | 5 | 5 | 5 | 5 | 4 | 33 | KEEP + RENAME |
| 5 | `typescript-node-runtime-compatibility-agent` → `typescript-node-execution-compatibility-agent` | 5 | 4 | 5 | 5 | 5 | 5 | 4 | 33 | KEEP + RENAME |
| 6 | `typescript-library-api-contract-agent` → `typescript-public-api-and-declaration-governance-agent` | 5 | 4 | 5 | 5 | 5 | 5 | 5 | 34 | KEEP + RENAME + ABSORB 11 |
| 7 | `typescript-build-graph-performance-agent` | 3 | 5 | 5 | 4 | 5 | 5 | 5 | 32 | KEEP + ABSORB 16 |
| 8 | `typescript-tsconfig-policy-agent` | 3 | 5 | 4 | 2 | 5 | 4 | 4 | 27 | MERGE into 8′ |
| 9 | `typescript-eslint-static-analysis-agent` | 3 | 4 | 4 | 3 | 5 | 5 | 5 | 29 | MERGE into 8′ |
| 8′ | `typescript-static-enforcement-policy-agent` (merge of 8 and 9) | 4 | 5 | 5 | 4 | 5 | 5 | 5 | 33 | KEEP (merged) |
| 10 | `typescript-async-reliability-agent` → `typescript-async-contract-reliability-agent` | 5 | 4 | 4 | 4 | 5 | 5 | 4 | 31 | KEEP + RENAME |
| 11 | `typescript-test-contract-agent` | 3 | 4 | 5 | 2 | 4 | 4 | 3 | 25 | MERGE into 6 |
| 12 | `typescript-package-supply-chain-agent` → `typescript-package-publication-integrity-agent` | 5 | 3 | 3 | 4 | 5 | 5 | 5 | 30 | KEEP + RENAME + NARROW |
| 13 | `typescript-modernization-upgrade-agent` → `typescript-estate-modernization-governor-agent` | 4 | 4 | 5 | 4 | 4 | 5 | 5 | 31 | KEEP + RENAME |
| 14 | `typescript-codegen-schema-drift-agent` | 4 | 4 | 4 | 1 | 5 | 4 | 4 | 26 | REJECT → merge into 3 |
| 15 | `typescript-mcp-tool-contract-agent` | 4 | 3 | 5 | 5 | 5 | 5 | 4 | 31 | KEEP |
| 16 | `typescript-developer-tooling-language-service-agent` | 3 | 4 | 3 | 1 | 4 | 4 | 4 | 23 | REJECT → merge into 7 |
| 17 | `typescript-runtime-portability-agent` | 3 | 2 | 4 | 2 | 3 | 4 | 3 | 21 | DEFER |
| 18 | `typescript-business-critical-automation-governance-agent` | 5 | 2 | 4 | 4 | 4 | 5 | 4 | 28 | KEEP |
| 19 | `typescript-engineering-economics-agent` | 4 | 3 | 3 | 3 | 4 | 5 | 5 | 27 | KEEP — CONDITIONAL |

Candidate 14 is the instructive row: it clears the numeric threshold at 26 and is still rejected,
because Non-overlap 1 is disqualifying. A total score cannot buy its way past a duplicate.

## 3. Prosecution, candidate by candidate

Each block states the strongest case against the candidate before any rebuttal.

### Candidate 1 — `typescript-maestro-agent` · KEEP

| Field | Assessment |
|---|---|
| Why it may be unnecessary | A router adds a hop. A user who names a specialist gets a better answer faster, and 13 well-named specialists are self-selecting. |
| Existing owner risk | `agents/frontend/frontend-maestro-agent` already routes web work and could grow a TypeScript branch. |
| Overlapping candidate | None — it is the only router. |
| If ownership is ambiguous | Two routers claim the same task and the user gets two dispatches, or the frontend maestro answers a library-packaging question it has no specialist for. |
| Unique judgment | Choosing the narrowest specialist and detecting when the task is out of board entirely — declining a React question, a Python question, or a request to run a command. |
| Measurable pain reduced | Wrong-specialist answers, the most common failure mode of a large board. |
| Verdict | KEEP. TypeScript-specificity of 2 is correct for a router and is not a defect; the repository's board pattern and the routing fixture gate both assume one. |

### Candidate 2 — `typescript-type-soundness-agent` (renamed) · KEEP + RENAME

| Field | Assessment |
|---|---|
| Why it may be unnecessary | `agents/frontend/typescript-contracts-agent` already reviews narrowing correctness, `any` laundering, and exported type contracts. On its face this is the same agent. |
| Existing owner risk | Direct and severe. Given a component with `as any` on a fetch response, both agents produce the same finding. |
| Overlapping candidate | Candidate 8′ (flag policy) and candidate 6 (exported surface). |
| If ownership is ambiguous | The worst case on the board: two agents in two different boards issue overlapping verdicts on the same diff with no tie-break, and a reviewer picks whichever is more convenient. |
| Unique judgment | Whether a type-level abstraction in shared or published code actually proves what its signature claims — variance, conditional and mapped type correctness, predicate honesty, `satisfies` versus annotation, branded types constructible without validation — and whether the abstraction is complexity theatre. |
| Measurable pain reduced | P08. Runtime type errors in exactly the code the most teams depend on. |
| Verdict | KEEP only because the scope is cut back to shared and published program semantics and the artifact-scope split in [03](./03-final-board-and-boundary-contracts.md) is shipped in both agents' refusal lists. Without that split this candidate should be rejected. Non-overlap is scored 3, not 5, and that is honest. |

### Candidate 3 — `typescript-runtime-boundary-contract-agent` · KEEP + ABSORB 14

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Application security already reviews untrusted input. A validation library is a two-line dependency, not a specialism. |
| Existing owner risk | `agents/frontend/typescript-contracts-agent` flags a missing validator; `agents/frontend/api-integration-bff-agent` owns the client-to-backend contract. |
| Overlapping candidate | 14 (codegen drift), absorbed here. |
| If ownership is ambiguous | The single most expensive gap on the board goes unowned because each neighbour assumes another caught it. |
| Unique judgment | Enumerating every trust boundary in a program; distinguishing an annotation from a check; deciding where `unknown`-first ingestion is mandatory; keeping schema and type to one source of truth; ruling that a generated type is a claim rather than a check; designing an error taxonomy that does not leak internals. |
| Measurable pain reduced | P01 and P11 — the highest-scoring pains in the register. |
| Verdict | KEEP, 35 of 35. It addresses the defect class TypeScript itself creates, and application security owns exploitation rather than contract design. |

### Candidate 4 — `typescript-module-resolution-and-emit-agent` · KEEP + RENAME

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Bundlers hide most of this, and `agents/frontend/build-tooling-bundling-agent` already reviews build configuration. |
| Existing owner risk | Build tooling owns bundler configuration; package governance owns `package.json` dependency policy. Neither owns resolution semantics. |
| Overlapping candidate | 5 (execution) — the reason for the rename. |
| If ownership is ambiguous | An execution specialist rules on a packaging question, or vice versa, and the consumer matrix is never checked. |
| Unique judgment | The resolution-mode matrix; that valid `moduleResolution` values are now only `node16`, `nodenext`, `bundler`; condition ordering with `types` first and `default` last; declaration resolution differing by mode; that one compilation cannot validate every consumer scenario. |
| Measurable pain reduced | P03. |
| Verdict | KEEP. Renamed from "module-runtime-interop" because the word "runtime" invited exactly the execution questions candidate 5 owns. |

### Candidate 5 — `typescript-node-execution-compatibility-agent` · KEEP + RENAME

| Field | Assessment |
|---|---|
| Why it may be unnecessary | "Run the compiler in CI" is one line of advice. A whole agent for one misconception looks thin. |
| Existing owner risk | None. No Node board exists. |
| Overlapping candidate | 4 (resolution) and 17 (portability, deferred). |
| If ownership is ambiguous | Nobody checks whether the shipped artifact was ever type-checked — the most consequential unasked question in a 2026 TypeScript service. |
| Unique judgment | That Node performs no type checking and ignores `tsconfig.json` for execution; which syntax throws `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX`; that type stripping is default-on since v23.6.0/v22.18.0 and `--experimental-transform-types` was removed in v26.0.0; that `paths` are not honored at runtime; that import extensions are mandatory; and gating every verdict on a named Node version. |
| Measurable pain reduced | P02, score 100. |
| Verdict | KEEP. The advice is one line only after the diagnosis, and the diagnosis is the work. |

### Candidate 6 — `typescript-public-api-and-declaration-governance-agent` · KEEP + RENAME + ABSORB 11

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Semantic versioning is a release-process concern, and release governance already exists in other boards. |
| Existing owner risk | `agents/kotlin/kotlin-library-api-abi-governance-agent` is the same shape for another language, which proves the role rather than duplicating it. |
| Overlapping candidate | 11 (type tests), absorbed; 12 (publication), adjacent. |
| If ownership is ambiguous | A declaration-only breaking change ships as a patch because it looked like a refactor. |
| Unique judgment | What is actually public in the emitted `.d.ts` versus what the source appears to export; classifying a type change as breaking; declaration emit strategy including `isolatedDeclarations` and rollups; and defining the consumer compilation matrix that proves the classification. |
| Measurable pain reduced | P04 and P15. |
| Verdict | KEEP. Absorbing candidate 11 is not empire building — "what must a consumer compilation prove" is a sub-decision of publishing an API, and separating them creates two owners for one gate. |

### Candidate 7 — `typescript-build-graph-performance-agent` · KEEP + ABSORB 16

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Build speed is a platform and CI concern, and `agents/frontend/monorepo-dx-agent` already reviews monorepo build time. |
| Existing owner risk | Real, and the boundary must be explicit: monorepo-dx owns the **task** graph and remote caching; this agent owns the TypeScript **program** graph. |
| Overlapping candidate | 16 (language service), absorbed; 8′ (typed-lint cost), adjacent. |
| If ownership is ambiguous | Both agents prescribe "use project references" without a measurement, which is the failure the brief's attack 7 targets. |
| Unique judgment | Reading `--extendedDiagnostics` and `--generateTrace` output; distinguishing task-graph from type-graph cost; knowing the native compiler changes the cost model and that trace-tool parity under TypeScript 7 is unverified. |
| Measurable pain reduced | P06 — highest frequency on the board. |
| Verdict | KEEP, with a hard rule: no prescription without a measurement. |

### Candidate 8 — `typescript-tsconfig-policy-agent` · MERGE

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Compiler configuration is not a standalone decision. Alone, this agent is compiler trivia with a governance label. |
| Existing owner risk | Candidate 2 reviews the same file for a different question; candidate 7 restructures it for cost. |
| Overlapping candidate | 9, decisively. |
| If ownership is ambiguous | Two agents rule on the same `tsconfig.json` with different mandates. |
| Unique judgment | None that survives separation from candidate 9 — "which flags must be on" and "which lint rules must be on" are one question with one cost. |
| Measurable pain reduced | P07, P14 — real, but reachable by the merged agent. |
| Verdict | MERGE into 8′. Non-overlap 2 is disqualifying on its own. |

### Candidate 9 — `typescript-eslint-static-analysis-agent` · MERGE

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Lint configuration reads as stylistic tooling, and a rule list is not judgment. |
| Existing owner risk | Candidate 7 owns the cost of running type information; candidate 10 owns the specific promise defects the typed rules detect. |
| Overlapping candidate | 8, decisively. |
| If ownership is ambiguous | Nobody owns editor-versus-CI parity, which is where false-green lint lives. |
| Unique judgment | Real but not separable: which type-information-dependent rules are worth their cost, how the Project Service is configured, and the live hazard that typescript-eslint supports `>=4.8.4 <6.1.0` while TypeScript 7.0.2 is current. |
| Measurable pain reduced | P07. |
| Verdict | MERGE into 8′. |

### Candidate 8′ — `typescript-static-enforcement-policy-agent` · KEEP (merged)

| Field | Assessment |
|---|---|
| Why it may be unnecessary | It could still be read as "the config agent", which is close to the generic names Rule B forbids. |
| Existing owner risk | `agents/python/python-developer-tooling-build-agent` is the cross-language precedent for exactly this merged shape, which supports the design. |
| Overlapping candidate | 2 (per-construct verdicts) and 7 (graph restructuring), both excluded from its scope. |
| If ownership is ambiguous | The enforcement contract has no owner and each package sets its own bar invisibly. |
| Unique judgment | Defining what "passes" means per package across a fleet, pricing it, detecting silent loosening now that `strict` is default-true, owning suppression policy, and reconciling compiler-versus-lint supported versions. |
| Measurable pain reduced | P07, P14. |
| Verdict | KEEP. The merge is what makes it a decision domain rather than a settings review. |

### Candidate 10 — `typescript-async-contract-reliability-agent` · KEEP + RENAME

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Async correctness is general JavaScript engineering, and `agents/frontend/javascript-runtime-agent` already reviews promise composition. |
| Existing owner risk | Real but separable: that agent owns browser scheduling, DOM event lifecycle, and listener leaks. |
| Overlapping candidate | 8′ (the lint rules that detect these defects), 13 (partial writes in privileged scripts). |
| If ownership is ambiguous | A server-side unhandled rejection is reviewed by a browser specialist, or not at all. |
| Unique judgment | Ignored promises are reliably detectable only with type information; async functions passed where `void` is expected; typed cancellation contracts; that Node's default `--unhandled-rejections` mode is `throw` and that the docs state it is not safe to resume after `uncaughtException`; typed error channels versus thrown `unknown`. |
| Measurable pain reduced | P05. |
| Verdict | KEEP, with a stated migration condition: if a Node board is created, this agent moves to it. Naming that condition now is cheaper than defending the boundary later. |

### Candidate 11 — `typescript-test-contract-agent` · MERGE

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Test strategy already has two owners, and type tests are a small suite. |
| Existing owner risk | `agents/frontend/testing-quality-engineering-agent` and the `qa` board own runtime test strategy. |
| Overlapping candidate | 6, decisively. |
| If ownership is ambiguous | Consumer compilation coverage is claimed by a testing agent that does not own the API surface it must protect. |
| Unique judgment | Real — compile-time-only assertions, `@ts-expect-error` semantics, the consumer tsconfig matrix — but inseparable from the API it verifies. |
| Measurable pain reduced | P15. |
| Verdict | MERGE into 6. Non-overlap 2 is disqualifying. |

### Candidate 12 — `typescript-package-publication-integrity-agent` · KEEP + RENAME + NARROW

| Field | Assessment |
|---|---|
| Why it may be unnecessary | The mechanics are npm-generic, not TypeScript-specific, and `agents/frontend/package-governance-agent` already covers npm supply chain. |
| Existing owner risk | Genuine collision on dependency intake, which is why the scope is narrowed to publication only. |
| Overlapping candidate | 6 (what the tarball claims versus what the API report says). |
| If ownership is ambiguous | Both agents review `package.json` and neither owns the publish path. |
| Unique judgment | Moderate, and scored 3 rather than inflated: the TypeScript-specific part is that a published package's types are part of its compatibility and disclosure surface. The rest is publish authority — trusted publishing versus long-lived tokens, provenance, tarball contents, publish-time scripts. |
| Measurable pain reduced | P09, whose magnitude is unbounded. |
| Verdict | KEEP. Justified by the ownership gap plus tail risk, not by TypeScript specificity. Narrowed so that dependency intake stays with its existing owner and signing infrastructure stays with the sigstore board. |

### Candidate 13 — `typescript-estate-modernization-governor-agent` · KEEP + RENAME

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Upgrades are ordinary maintenance, and per-file fixes belong to other specialists. |
| Existing owner risk | `agents/frontend/frontend-migration-modernization-agent` owns framework migrations; the kotlin and python governors are the precedent for this shape. |
| Overlapping candidate | 8′ (steady-state policy), 19 (the financial case). |
| If ownership is ambiguous | A staged strictness rollout is argued file by file and never sequenced, so the upgrade stalls. |
| Unique judgment | Sequencing and reversibility across a portfolio; which removals bite which packages; when not to migrate; and the concrete near-term exposure — the TypeScript 6.0-to-7.0 transition where 7.0 is GA but its programmatic API is not stable until 7.1, so editor and framework tooling remains on 6.0. |
| Measurable pain reduced | P10. |
| Verdict | KEEP. Renamed onto the repository's existing governor naming because the deliverable is a sequenced plan, not a diff review. |

### Candidate 14 — `typescript-codegen-schema-drift-agent` · REJECT

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Its central claim — a generated type is a claim, not a check — is candidate 3's thesis stated again. |
| Existing owner risk | Candidate 3 must already rule on generated types at every boundary. |
| Overlapping candidate | 3, verbatim. |
| If ownership is ambiguous | Two agents produce the same finding on the same generated client, with no rule for which verdict governs. |
| Unique judgment | Only the mechanical part — regenerate and diff, and whether the generator runs in CI — which candidate 3 absorbs at no cost. |
| Measurable pain reduced | P11, fully covered by candidate 3. |
| Verdict | REJECT as standalone; merge into 3. Scored 26 and still rejected: Non-overlap 1 is disqualifying. |

### Candidate 15 — `typescript-mcp-tool-contract-agent` · KEEP

| Field | Assessment |
|---|---|
| Why it may be unnecessary | It looks like a niche for one protocol, and candidate 3 already owns input validation. |
| Existing owner risk | Checked directly: only `agents/netsuite/netsuite-ai-connector-mcp-agent` and `agents/nvidia/nvidia-agentic-ai-platform-review-agent` touch MCP, both vendor-scoped. Generic tool-schema contracts are unowned. |
| Overlapping candidate | 3 (validation) and 6 (contract versioning). |
| If ownership is ambiguous | A tool ships whose declared schema does not describe its handler, and the failure reads as model error rather than a contract defect. |
| Unique judgment | Comparing a declared schema to handler behavior in typed code; JSON Schema dialect fidelity where both schemas default to 2020-12 absent `$schema`; `structuredContent` versus `content`; protocol errors versus `isError`; and version churn — the current revision `2026-07-28` removed the `initialize` handshake and protocol sessions, moved the version into `_meta.io.modelcontextprotocol/protocolVersion`, returns `-32022` on mismatch, and the TypeScript SDK split into `@modelcontextprotocol/server` and `@modelcontextprotocol/client` at 2.0.0. |
| Measurable pain reduced | P12. |
| Verdict | KEEP. A verified ownership gap in a repository whose entire product is agentic assets. |

### Candidate 16 — `typescript-developer-tooling-language-service-agent` · REJECT

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Editor latency and CI typecheck time are the same type-graph problem measured at two moments. |
| Existing owner risk | Candidate 7 and candidate 8′ between them cover every input it would read. |
| Overlapping candidate | 7, decisively. |
| If ownership is ambiguous | Two agents prescribe the same restructuring and disagree about who owns the measurement. |
| Unique judgment | None that survives separation. Editor responsiveness is an output of the program graph. |
| Measurable pain reduced | P06, fully covered by candidate 7. |
| Verdict | REJECT as standalone; merge into 7. |

### Candidate 17 — `typescript-runtime-portability-agent` · DEFER

| Field | Assessment |
|---|---|
| Why it may be unnecessary | Most enterprise TypeScript estates target one server runtime, and edge portability is a framework concern more than a language one. |
| Existing owner risk | Frontend specialists cover browser and edge rendering; candidate 5 covers Node. |
| Overlapping candidate | 5 and 4. |
| If ownership is ambiguous | A portability verdict is issued from a Node-only evidence base. |
| Unique judgment | Real for a genuinely multi-runtime estate, but its evidence needs — per-runtime API support matrices across Deno, Bun, workers, and edge platforms — cannot be met credibly today without a source of truth this plan does not have. |
| Measurable pain reduced | Not currently quantifiable from any evidence in front of this design. |
| Verdict | DEFER with a named entry criterion: a user brings a multi-runtime estate and can supply the target matrix. Building it now would produce confident guidance on unverifiable ground, which is worse than no agent. |

### Candidate 18 — `typescript-business-critical-automation-governance-agent` · KEEP

| Field | Assessment |
|---|---|
| Why it may be unnecessary | It reads as generic operational governance in a TypeScript costume, and `agents/python/python-business-critical-automation-governance-agent` already exists. |
| Existing owner risk | The python agent is the same governance shape for a different ecosystem. Application security owns injection and credentials. |
| Overlapping candidate | 10 (partial writes), 5 (unchecked execution). |
| If ownership is ambiguous | A privileged backfill is reviewed for code quality and never for blast radius, reconciliation, or reversibility. |
| Unique judgment | The TypeScript-specific trigger is the intersection nobody else looks at: type-stripped, never-type-checked execution holding production credentials, combined with floating-promise partial commits. That is a distinct hazard from either parent pain. |
| Measurable pain reduced | P13 — low frequency, severe and often irreversible impact. |
| Verdict | KEEP, review-only. It reviews privileged scripts and never runs them; nothing on this board executes. |

### Candidate 19 — `typescript-engineering-economics-agent` · KEEP, CONDITIONAL

| Field | Assessment |
|---|---|
| Why it may be unnecessary | The strongest case against any agent on this board. TypeScript specificity is low, the arithmetic is generic, and an economics agent with no measurements is a machine for producing plausible fabricated savings. |
| Existing owner risk | `agents/frontend/frontend-finops-cost-to-serve-agent` for frontend infrastructure cost; the finops board for cloud cost; `agents/java/java-application-server-exit-agent` and `agents/kotlin/kotlin-kmp-portfolio-decision-agent` as precedents for consuming supplied figures. |
| Overlapping candidate | 7 and 13 both produce measurements it would consume. |
| If ownership is ambiguous | Specialists start estimating dollar figures inside technical reviews, which is how a board loses credibility fastest. |
| Unique judgment | Turning another specialist's measurements into a decision — formulas, sensitivity analysis, break-even, cost of postponement — and refusing when the inputs are absent. |
| Measurable pain reduced | P16, the lowest-scoring pain in the register. |
| Verdict | KEEP under three conditions, all of which must appear in its own contract: it consumes measurements and never originates them; it may not be dispatched first on any task; and it is re-prosecuted two quarters after shipping and removed if it has produced no decision. It is the weakest accepted agent and the plan says so rather than hiding it in a table. |

## 4. Rule B — generic names are a design smell

None of the following is accepted, in any form:

- `typescript-best-practices-agent`
- `typescript-code-quality-agent`
- `typescript-expert-agent`
- `typescript-review-agent`
- `typescript-clean-code-agent`

Each fails the same test: it names a subject area rather than a bounded decision problem, so its
refusal list cannot be written, and an agent that cannot say what it refuses cannot be routed to.

The accepted names must survive the same test. Each owns one bounded decision, stated in twelve
words or fewer:

| Agent | The one decision it owns |
|---|---|
| `typescript-maestro-agent` | which specialist, or none, handles this task |
| `typescript-type-soundness-agent` | does this abstraction prove what its signature claims |
| `typescript-runtime-boundary-contract-agent` | where must data be parsed rather than asserted |
| `typescript-module-resolution-and-emit-agent` | will every consumer mode resolve and import this correctly |
| `typescript-node-execution-compatibility-agent` | does this run on the target Node, and is it checked anywhere |
| `typescript-public-api-and-declaration-governance-agent` | is this type change breaking, and what version does it require |
| `typescript-build-graph-performance-agent` | what in the type graph costs this time, measured |
| `typescript-static-enforcement-policy-agent` | what must the toolchain prove, and at what cost |
| `typescript-async-contract-reliability-agent` | is every promise awaited, cancellable, and bounded |
| `typescript-package-publication-integrity-agent` | who may publish this, and what ships in it |
| `typescript-estate-modernization-governor-agent` | in what order does this estate migrate, and reversibly |
| `typescript-mcp-tool-contract-agent` | does the declared tool contract match the handler |
| `typescript-business-critical-automation-governance-agent` | may this privileged script run, under which controls |
| `typescript-engineering-economics-agent` | what do the supplied measurements make worth funding |

## 5. Tally and candidate mapping

Accepted: **14** — one maestro and 13 specialists.
Eliminated as standalone: **5**. Deferred: **1**. Renamed: **7**.

| Brief candidate | Outcome |
|---|---|
| 1 maestro | accepted as-is |
| 2 language-type-safety | accepted, renamed `type-soundness`, scope narrowed |
| 3 runtime-boundary-contract | accepted, absorbed candidate 14 |
| 4 module-runtime-interop | accepted, renamed `module-resolution-and-emit` |
| 5 node-runtime-compatibility | accepted, renamed `node-execution-compatibility` |
| 6 library-api-contract | accepted, renamed `public-api-and-declaration-governance`, absorbed candidate 11 |
| 7 build-graph-performance | accepted, absorbed candidate 16 |
| 8 tsconfig-policy | merged into 8′ |
| 9 eslint-static-analysis | merged into 8′ |
| — | 8′ `static-enforcement-policy` accepted (new merged agent) |
| 10 async-reliability | accepted, renamed `async-contract-reliability` |
| 11 test-contract | merged into candidate 6 |
| 12 package-supply-chain | accepted, renamed `package-publication-integrity`, narrowed to publication |
| 13 modernization-upgrade | accepted, renamed `estate-modernization-governor` |
| 14 codegen-schema-drift | rejected, merged into candidate 3 |
| 15 mcp-tool-contract | accepted as-is |
| 16 developer-tooling-language-service | rejected, merged into candidate 7 |
| 17 runtime-portability | deferred with a named entry criterion |
| 18 business-critical-automation-governance | accepted as-is |
| 19 engineering-economics | accepted conditionally, with a re-prosecution date |

Nothing was silently dropped: every one of the 19 candidates appears above with an outcome, and
the two rejections and one deferral name the agent or condition that now covers their concern.

## 6. What would invalidate this document

- The frontend-board owner declines the artifact-scope split, which drops candidate 2's
  Non-overlap to 2 and disqualifies it under this document's own rule.
- A Node board or an MCP board is created, which moves candidates 10 and 15 out of scope and
  requires re-scoring their Non-overlap.
- A user supplies a multi-runtime target matrix, which reopens candidate 17.
- Candidate 19 produces no decision within two quarters, in which case it is removed — the
  condition is part of its acceptance, not a courtesy.
- Any accepted agent proves, in use, to return materially the same review as a neighbour. That is
  a Non-overlap failure discovered late, and the remedy is a merge, not a boundary memo.
