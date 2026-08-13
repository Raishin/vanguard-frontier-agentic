# Skill and Reference Architecture

> Status: **PLAN — not implementation.** No skill or reference file was created.
> Previous: [04-routing-architecture-and-fixtures.md](./04-routing-architecture-and-fixtures.md) · Next: [06-implementation-roadmap-and-integration.md](./06-implementation-roadmap-and-integration.md)

Fourteen skills, one per agent. The reference architecture exists to prevent the failure the
commissioning brief names correctly: fourteen folders each containing the same four links.

## 1. Board skill rules

These are self-imposed. `tests/validate-aws-progressive-disclosure.py:12` scopes that gate to
`skills/aws/**`, so nothing in CI enforces the AgentCore discipline on
`skills/typescript/**`. The board adopts it anyway, because the alternative is a 2,000-line
`SKILL.md` that is loaded in full on every trigger.

| Rule | Reason |
|---|---|
| `SKILL.md` at or under 90 lines | matches the AWS gate's own threshold (`tests/validate-aws-progressive-disclosure.py:52`); a trigger-time document must be cheap to load |
| Contains the literal string `Load these only when needed` | the AWS gate's lazy-load marker; adopting the exact phrasing keeps the board consistent if the gate is ever generalized |
| Every declared reference is linked from `SKILL.md` | an unlinked reference is dead weight nobody loads |
| **No bash fence in `SKILL.md`** | `tests/validate-skill-coherence.py:311` requires every command in a `bash`/`sh`/`shell`/`console` fence to be covered by a Bash grant in `allowed-tools`. A `static-review` skill that shows a command therefore has to request execution capability it must not hold. |
| Deep material lives in `references/`, not `SKILL.md` | `tests/validate-skill-coherence.py` does not scan `references/*.md`, and progressive disclosure is the point |

## 2. Tool grants

| Skill | `allowed-tools` | Justification |
|---|---|---|
| `typescript-maestro` | `Agent Skill Read Grep Glob` | matches `skills/java/java-maestro/SKILL.md`; a router dispatches, so it needs `Agent` and `Skill` |
| `typescript-node-execution-compatibility` | `Read Grep Glob WebFetch` | its verdicts turn on the Node release schedule and the type-stripping documentation, both of which change per release |
| `typescript-package-publication-integrity` | `Read Grep Glob WebFetch` | registry policy is dated and moving — trusted publishing reached GA 2025-07-31 and classic tokens were revoked 2025-12-09; guidance from memory is wrong within a quarter |
| `typescript-mcp-tool-contract` | `Read Grep Glob WebFetch` | the specification revision is the contract; the current revision is `2026-07-28` and the previous one is not compatible with it |
| The other ten specialists | `Read Grep Glob` | least privilege; their evidence is the user's source and configuration |

No skill on this board grants Bash. Nothing on this board runs a command.

`typescript-estate-modernization-governor` does **not** get `WebFetch`, which is a deliberate and
arguable call: it consumes release-note facts but its deliverable is a sequencing plan built from
user-supplied inventory, and it carries an `official-sources.md` reference instead. If practice
shows it needs live fetching, widening the grant is a reviewable one-line change — which is the
right direction for a grant to move.

### 2.1 Context7 cannot be a tool grant

`tests/validate-skill-allowed-tools.py:34` defines the token grammar as
`^[A-Z][A-Za-z0-9]+(\([^)]+\))?$`. A name such as `mcp__Context7__query-docs` cannot match it:
lowercase start, underscores, and a hyphen all fail. Confirmed against the catalog — zero
`SKILL.md` files declare an `mcp__*` token, while 291 mention Context7 in their prose.

Context7 is therefore a **workflow step** in these skills, never a declared capability. The
protocol is §6.

## 3. The incoherence this board does not copy

`agents/frontend/typescript-contracts-agent/AGENT.md` states under Operating Rules: "no Bash
execution of `tsc`/build tooling". Its companion
`skills/frontend/typescript-contracts-review/SKILL.md` declares
`allowed-tools: Read Grep Glob Bash(git diff:*) Bash(tsc --noEmit:*) WebFetch`.

The prose forbids what the grant permits. A harness enforces the grant; a reviewer reads the
prose. **Board rule: an agent's declared tier and its companion skill's `allowed-tools` must
agree.** Every specialist on this board is `static-review` and grants no Bash.

## 4. `SKILL.md` section contract

Fixed order, every specialist:

```markdown
---
name: <skill-id>
description: <50-1500 chars, states the trigger and the refusal in one paragraph>
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-08-12"
  category: <closed enum value>
  lifecycle: experimental
---

# <skill-id>

## Purpose
One paragraph: the decision this skill makes and the failure it prevents.

## Trigger conditions
Concrete signals in the user's request or artifacts.

## When not to use
Explicit refusals, each naming the agent that owns it instead.

## Preconditions and evidence hierarchy
What must be supplied before a verdict is possible, and what counts as strong
versus weak evidence.

## Lean operating rules
CRITICAL / HIGH / MEDIUM / LOW tagged rules, each independently checkable.

## References
Load these only when needed:
- [<Title>](references/<file>.md) — when to load it.

## Response minimum
The output contract.
```

`metadata.category` must come from the closed enum in
`schemas/skill.frontmatter.schema.json`: `security`, `platform`, `data`, `finops`, `ai`,
`delivery`, `observability`, `compliance`, `resilience`, `networking`, `storage`, `database`,
`compute`, `architecture`, `messaging`, `serverless`, `cost-management`, `operational`,
`generation`, `devsecops`, `finance`.

**There is no `typescript` value and no `governance` value.** A brief that assumes either is
wrong about this repository. Assignments:

| Skill | `category` |
|---|---|
| `typescript-maestro` | `ai` |
| `typescript-type-soundness` | `architecture` |
| `typescript-runtime-boundary-contract` | `security` |
| `typescript-module-resolution-and-emit` | `platform` |
| `typescript-node-execution-compatibility` | `compute` |
| `typescript-public-api-and-declaration-governance` | `architecture` |
| `typescript-build-graph-performance` | `operational` |
| `typescript-static-enforcement-policy` | `devsecops` |
| `typescript-async-contract-reliability` | `resilience` |
| `typescript-package-publication-integrity` | `devsecops` |
| `typescript-estate-modernization-governor` | `architecture` |
| `typescript-mcp-tool-contract` | `ai` |
| `typescript-business-critical-automation-governance` | `compliance` |
| `typescript-engineering-economics` | `cost-management` |

## 5. Reference ownership matrix

Every row survives one test: the reference exists because a specific decision or diagnostic needs
it. A source shared between two skills requires each skill's local guidance to state the different
question it asks.

| Reference topic | Owning skill | Why this skill needs it | Canonical primary source | Shared? | Reason sharing is unavoidable |
|---|---|---|---|---|---|
| Soundness failure catalogue | type-soundness | to name the exact construct that lies rather than reporting "type unsafe" | TypeScript handbook narrowing, generics, and type-predicate pages | with static-enforcement | same handbook, different question: "does this construct prove X" versus "which flag must be on across the fleet" |
| Assertion and `any` escape audit for shared code | type-soundness | to separate a justified escape hatch from laundering in published code | [typescriptlang.org/tsconfig](https://www.typescriptlang.org/tsconfig) plus the handbook | with `skills/frontend/typescript-contracts-review` | an application-diff variant already exists; the local guidance must state the artifact-scope split from [03 §3](./03-final-board-and-boundary-contracts.md) |
| Boundary inventory and parse discipline | runtime-boundary | to enumerate every edge and require a parse rather than a cast | TypeScript erasure semantics plus the installed validator's documentation | no | — |
| Schema library selection and drift | runtime-boundary | to verify a validator against what the repository installed, and to detect codegen drift | [zod.dev](https://zod.dev), [ajv.js.org](https://ajv.js.org/), [json-schema.org](https://json-schema.org/specification) | with mcp-tool-contract on JSON Schema | different question: which dialect validates application input versus which dialect the protocol declares for tool input |
| Resolution-mode matrix | module-resolution | to map `module` and `moduleResolution` onto emit and declaration behavior | [typescriptlang.org/tsconfig](https://www.typescriptlang.org/tsconfig) plus the modules handbook | with node-execution on Node package docs | different question: how the compiler resolves and emits versus how the runtime loads |
| Dual-package consumer matrix | module-resolution | to prove the package works for every consumer mode it claims | [nodejs.org/api/packages.html](https://nodejs.org/api/packages.html), [publint.dev/rules](https://publint.dev/rules), [arethetypeswrong.github.io](https://arethetypeswrong.github.io) | with publication-integrity on publint | different question: does it resolve versus does it ship the right bytes |
| Type-stripping limits | node-execution | to refute "Node runs TypeScript so the compiler is optional" with quoted documentation | [nodejs.org/api/typescript.html](https://nodejs.org/api/typescript.html) | no | — |
| Node version and API gating | node-execution | to gate every verdict on a named Node version rather than a general impression | [github.com/nodejs/Release](https://github.com/nodejs/Release) | no | — |
| API surface and semver decision | public-api | to classify a type change and pick the version bump | TypeScript declaration documentation, [api-extractor.com](https://api-extractor.com/) | no | — |
| Declaration emit and rollup | public-api | to choose an emit strategy and decide on `isolatedDeclarations` | TypeScript declaration and modules appendices | no | — |
| Type-contract test matrix | public-api | to define consumer compilations and type-level assertions | [vitest.dev](https://vitest.dev/guide/testing-types), TypeScript `@ts-expect-error` documentation | no | — |
| Program-graph diagnosis | build-graph | to restructure the type graph from evidence rather than folklore | TypeScript project-references documentation, [github.com/microsoft/TypeScript/wiki/Performance](https://github.com/microsoft/TypeScript/wiki/Performance) | no | — |
| Trace evidence protocol | build-graph | to require a measurement before any prescription, and to record which compiler produced it | TypeScript performance-tracing documentation | no | — |
| Enforcement matrix | static-enforcement | to define what "passes" means per package, flags and rules together | [typescriptlang.org/tsconfig](https://www.typescriptlang.org/tsconfig), typescript-eslint rule documentation | with type-soundness | see the first row |
| Typed-lint cost model | static-enforcement | to price type-information-dependent rules and check the supported-version window | [typescript-eslint.io/packages/parser](https://typescript-eslint.io/packages/parser/) and its typed-linting performance pages | with build-graph on cost framing only | different artifact: lint program creation versus the build program graph |
| Promise and cancellation audit | async | to find ignored promises and missing cancellation paths | Node process and promise documentation, typed promise-rule documentation | with static-enforcement on rule names | different question: a defect instance versus rule-set policy |
| Backpressure and resource bounds | async | to bound concurrency and guarantee release | Node streams and async-iteration documentation | no | — |
| Publication identity and provenance | publication-integrity | to require OIDC-based publishing and provenance over long-lived tokens | [docs.npmjs.com](https://docs.npmjs.com/generating-provenance-statements) plus the dated registry policy posts | no | — |
| Tarball and types surface | publication-integrity | to check what actually ships, including declarations and source maps | npm `files` and pack documentation, [publint.dev/rules](https://publint.dev/rules) | with module-resolution on publint | see above |
| Upgrade risk inventory | modernization | to enumerate breaking changes across an estate | TypeScript release announcements and breaking-change notes | no | — |
| Staged strictness adoption | modernization | to sequence flag adoption without a code freeze | [typescriptlang.org/tsconfig](https://www.typescriptlang.org/tsconfig) read as an adoption path | with static-enforcement | different question: transition sequencing versus steady-state policy |
| Tool schema contract audit | mcp-tool-contract | to compare a declared schema against handler behavior | [modelcontextprotocol.io](https://modelcontextprotocol.io/specification/2026-07-28) tools section, TypeScript SDK documentation | no | — |
| Protocol version and error contract | mcp-tool-contract | to check negotiation, error classification, and cancellation | [modelcontextprotocol.io](https://modelcontextprotocol.io/specification/2026-07-28) lifecycle and error sections | no | — |
| Blast radius and dry-run controls | automation-governance | to gate privileged automation on controls that already exist in this repo | `docs/execution-tiers.md` (E2) | no | — |
| Evidence and rollback requirements | automation-governance | to require reconciliation, audit, and a named inverse operation | `docs/execution-tiers.md:143` onward (E2) | no | — |
| Cost model formulas | economics | to show arithmetic instead of asserting a saving | none — the formulas are the content | no | — |
| Measurement intake and refusal | economics | to refuse fabricated return and name the missing inputs | none — the contract is the content | no | — |

### 5.1 Anti-landfill rules

1. The TypeScript handbook root, the Node homepage, an OWASP index page, and a bare "use Context7"
   note appear as standalone entries in **zero** reference files.
2. A source appears only where it answers that specialist's question. Two skills may share a
   canonical source; they may not share the same question.
3. No duplicated reference prose. Where two skills need overlapping background, the second states
   its own question and points at the decision, not at a copy of the paragraph.
4. `official-sources.md` exists in exactly six skills — runtime-boundary, node-execution,
   module-resolution, mcp-tool-contract, publication-integrity, and modernization — because those
   six have verdicts that flip on a dated vendor fact. Runtime-boundary is in the set because its
   rulings depend on the installed validator's documented behavior and on the current JSON Schema
   dialect, both of which are versioned facts it must not assert from memory. The other eight do
   not get one: a file whose only purpose is to hold links the skill never consults is the
   landfill.
5. `safety-checklist.md` exists in exactly two skills — runtime-boundary and
   automation-governance — the two whose findings gate an action with real blast radius.

## 6. Per-skill reference inventory

Every skill has `references/workflow-and-output.md`: the diagnostic sequence, decision tree,
severity and confidence model, and output contract for that specialist. It is listed once here
rather than repeated in every row.

| Skill | Additional references | What each must contain |
|---|---|---|
| `typescript-maestro` | none | `skills/java/java-maestro/SKILL.md` carries its routing table inline with no `references/` directory; this board follows that precedent |
| `typescript-type-soundness` | `soundness-failure-catalog.md`, `assertion-escape-audit.md` | catalogue: each construct that can lie (bivariant generic, dishonest predicate, unreachable conditional branch, silently widening `satisfies`, unvalidated brand), with the check that detects it. Audit: how to classify an escape hatch in published code as justified or laundering, and the artifact-scope split. |
| `typescript-runtime-boundary-contract` | `boundary-inventory.md`, `schema-selection-and-drift.md`, `official-sources.md`, `safety-checklist.md` | inventory: the enumerable edge classes and how to find each in source. Selection and drift: how to verify which validator is installed, dialect implications, and the regenerate-and-diff drift check. Safety checklist: what must hold before a boundary finding is closed. |
| `typescript-module-resolution-and-emit` | `resolution-mode-matrix.md`, `dual-package-consumer-matrix.md`, `official-sources.md` | matrix: `module` × `moduleResolution` × emit × declaration behavior, with removed values flagged and the note that the compiler binary outranks the prose page. Consumer matrix: the minimum set of consumer configurations that must compile, and how to check condition ordering. |
| `typescript-node-execution-compatibility` | `type-stripping-limits.md`, `node-version-gating.md`, `official-sources.md` | limits: the quoted documentation on no type checking and ignored `tsconfig.json`, plus the syntax that throws, `node_modules` refusal, and mandatory import extensions. Gating: how to establish the Node version and what changes across the supported lines. |
| `typescript-public-api-and-declaration-governance` | `api-surface-and-semver.md`, `declaration-emit-and-rollup.md`, `type-contract-test-matrix.md` | semver: the classification table from change shape to required bump. Emit: `declaration`, `isolatedDeclarations`, rollup and API-report tradeoffs. Test matrix: compile-time assertion patterns and the consumer configuration set. |
| `typescript-build-graph-performance` | `program-graph-diagnosis.md`, `trace-evidence-protocol.md` | diagnosis: how a program graph is structured, what project references change, and what they cost. Protocol: which measurement to request, how to read it, and the rule that no prescription issues without one. |
| `typescript-static-enforcement-policy` | `enforcement-matrix.md`, `typed-lint-cost-model.md` | matrix: flags and rules against the defect classes they catch, with the per-package divergence view. Cost model: what type-aware linting costs, Project Service configuration, editor and CI parity, and the supported-version window check. |
| `typescript-async-contract-reliability` | `promise-and-cancellation-audit.md`, `backpressure-and-bounds.md` | audit: how to find ignored promises, `void`-position async functions, and unpropagated signals. Bounds: concurrency limits against real downstream capacity, stream backpressure, and guaranteed release. |
| `typescript-package-publication-integrity` | `publication-identity-and-provenance.md`, `tarball-and-types-surface.md`, `official-sources.md` | identity: publish authority models and what each removes, with verification. Surface: how to determine what the tarball actually contains and what the declarations expose. |
| `typescript-estate-modernization-governor` | `upgrade-risk-inventory.md`, `staged-strictness-adoption.md`, `official-sources.md` | inventory: the breaking-change classes to enumerate per package and how to detect each. Adoption: sequencing patterns with rollback points, and the criteria for not migrating. |
| `typescript-mcp-tool-contract` | `tool-schema-contract-audit.md`, `protocol-version-and-errors.md`, `official-sources.md` | audit: how to compare a declared schema against handler behavior, field by field, including structured output. Protocol: version negotiation, error classification, cancellation, and the current revision's departures from its predecessor. |
| `typescript-business-critical-automation-governance` | `blast-radius-and-dry-run.md`, `evidence-and-rollback.md`, `safety-checklist.md` | blast radius: how to bound scope and verify a dry-run covers the write path. Evidence: reconciliation, idempotency in both senses, audit requirements, and the named inverse operation. Checklist: the gate before any run recommendation. |
| `typescript-engineering-economics` | `cost-model-formulas.md`, `measurement-intake-and-refusal.md` | formulas: each calculation written out with its units and its sensitivity variables. Intake: the required input list, the labelling scheme (measured, supplied, assumed), and the refusal template naming what is missing. |

## 7. Context7 protocol

Context7 is a version-aware retrieval layer. It is not an oracle, and this plan has already caught
it being wrong ([00 §4.3](./00-reconnaissance-and-evidence-map.md)).

Sequence:

1. Identify the exact technology in scope.
2. Find the version **in the user's repository** — `package.json`, the lockfile, the installed
   binary. Never assume.
3. Call `resolve-library-id` when the canonical library ID is not already verified.
4. Call `query-docs` with a precise, version-sensitive question. Not "TypeScript best practices".
5. Record: resolved library ID, topic queried, the project's version, the version the returned
   material documents, the result used, and the retrieval date.
6. Cross-check against the official documentation for anything safety-, compatibility-, or
   migration-critical.
7. Flag any mismatch explicitly rather than silently preferring one source.

Evidence precedence, highest first:

1. The repository's actual configuration and runtime evidence.
2. Current official documentation or specification — with the caveat proven in
   [00 §4.2](./00-reconnaissance-and-evidence-map.md) that a prose page can lag its own compiler.
3. Version-matched Context7 material.
4. An official project issue or release note.
5. A secondary source, only where no primary evidence exists, and labelled as such.

Prohibited: inventing a library ID; querying a vague topic; treating a returned snippet as
evidence of the user's configuration; silently using a different major version's documentation;
letting Context7 override checked-in evidence.

Worked example, from this plan's own research: a Context7 snippet for the TypeScript library
asserted that `strict` is not default-true in 6.0. The official release announcement says it is,
and an empirical test confirmed the announcement. Precedence rule 2 beat rule 3, the conflict was
recorded, and the guidance the board will ship is the correct one.

## 8. Reference provenance ledger

Retrieval date for every row: 2026-08-12. Labels: `E3` primary source verified this cycle, `E4`
Context7-retrieved, `E5` unverified.

| Owning skill | Canonical source | Version/date documented | Claim supported | Label | Duplication rationale |
|---|---|---|---|---|---|
| module-resolution, static-enforcement, type-soundness, modernization | [typescriptlang.org/tsconfig](https://www.typescriptlang.org/tsconfig) | page as served 2026-08-12 | compiler option semantics; **and that its value tables are stale for removed `module`/`moduleResolution` values** | E3 | four skills, four questions: resolution behavior, fleet policy, construct soundness, adoption sequencing |
| modernization, static-enforcement | [announcing-typescript-6-0](https://devblogs.microsoft.com/typescript/announcing-typescript-6-0/) | 2026-03-23 | `strict` default true, `module` default `esnext`, removal of `amd`/`umd`/`system`, `--outFile`, `--downlevelIteration`, `target=es5` | E3 | modernization asks what breaks; static-enforcement asks what the new baseline is |
| modernization, build-graph | [announcing-typescript-7-0](https://devblogs.microsoft.com/typescript/announcing-typescript-7-0/) | 2026-07-08 | 7.0 GA, native Go compiler, no stable programmatic API until 7.1 | E3 | modernization asks about estate sequencing; build-graph asks about the cost model |
| module-resolution | empirical test against `typescript@7.0.2` | 7.0.2 | valid `moduleResolution` is `node16`/`nodenext`/`bundler`; removed values error TS5108 | E3 | — |
| public-api | TypeScript declaration documentation and [esm-cjs-interop](https://www.typescriptlang.org/docs/handbook/modules/appendices/esm-cjs-interop.html) | current handbook | dual ESM/CJS declaration hazards are documented in the modules appendix, **not** on the declaration-publishing page | E3 | — |
| node-execution | [nodejs.org/api/typescript.html](https://nodejs.org/api/typescript.html) | v26.7.0 docs | "no type checking is performed"; "Node.js ignores `tsconfig.json` files"; unsupported syntax throws `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX`; `.ts` under `node_modules` refused; extensions mandatory; stripping default since v23.6.0/v22.18.0 and stable since v25.2.0/v24.12.0; `--experimental-transform-types` removed in v26.0.0 | E3 | — |
| node-execution | [nodejs.org/learn/typescript/run-natively](https://nodejs.org/learn/typescript/run-natively) | current | the documented recommendation to run `tsc --noEmit` separately | E3 | — |
| node-execution | [github.com/nodejs/Release](https://github.com/nodejs/Release) | live schedule | v26 Current, v24 Active LTS, v22 Maintenance, v25 already EOL, with dates | E3 | — |
| module-resolution, node-execution | [nodejs.org/api/packages.html](https://nodejs.org/api/packages.html) | v26.x docs | condition ordering most-specific-first, `types` first and `default` last; the dual-package-hazard section is now a stub | E3 | resolution asks how the compiler and consumers see it; execution asks how the runtime loads it |
| async | Node process, CLI, and streams documentation | v26.x docs | `--unhandled-rejections` defaults to `throw`; "It is not safe to resume normal operation after `'uncaughtException'`"; `AbortController` and `AbortSignal` are stable globals | E3 | — |
| module-resolution | [nodejs.org/api/modules.html](https://nodejs.org/api/modules.html) | v26.x docs | `require(esm)` needs no flag; sync-only with `ERR_REQUIRE_ASYNC_MODULE` on top-level await | E3 | — |
| static-enforcement | [typescript-eslint.io/packages/parser](https://typescript-eslint.io/packages/parser/) and typed-linting pages | 8.67.0 | `projectService: true` is the current enablement; `allowDefaultProject` is capped and carries per-file overhead; type-aware lint time is comparable to build time; supported TypeScript range `>=4.8.4 <6.1.0` with a warning outside it | E3 | — |
| static-enforcement, async | typescript-eslint rule pages | 8.67.0 | `no-floating-promises`, `no-misused-promises`, `await-thenable`, `require-await` all require type information | E3 | enforcement asks whether the rule is worth its cost; async asks what the defect is |
| publication-integrity | [docs.npmjs.com](https://docs.npmjs.com/generating-provenance-statements) and dated registry policy posts | CLI v11; posts dated 2025-07-31, 2025-09-29, 2025-12-09, 2026-07-31 | trusted publishing GA and its supported providers; provenance prerequisites and `npm audit signatures`; classic-token revocation; granular-token expiry and 2FA restrictions | E3 | — |
| publication-integrity, module-resolution | [publint.dev/rules](https://publint.dev/rules), [arethetypeswrong.github.io](https://arethetypeswrong.github.io) | current | what each validator checks | E3 | publication asks what ships; resolution asks what resolves |
| public-api | [api-extractor.com](https://api-extractor.com/) | current | API report, `.d.ts` rollup, versioning; requires `tsc` with `declaration: true` first | E3 | — |
| mcp-tool-contract | [modelcontextprotocol.io](https://modelcontextprotocol.io/specification/2026-07-28) | revision 2026-07-28 | tool fields `name`, `title`, `description`, `icons`, `inputSchema`, `outputSchema`, `annotations`; schemas default to JSON Schema 2020-12 absent `$schema`; `structuredContent` validated against `outputSchema`; protocol errors versus `result.isError`; `initialize` and sessions removed; `_meta.io.modelcontextprotocol/protocolVersion` on every request; `-32022` on mismatch; `server/discover` required; transport-dependent cancellation | E3 | — |
| mcp-tool-contract | npm registry and the TypeScript SDK README on `main` | `@modelcontextprotocol/server` and `@modelcontextprotocol/client` at 2.0.0; `@modelcontextprotocol/sdk` at 1.30.0 (legacy) | the SDK split, `McpServer.registerTool` shape, Standard Schema input and output schemas | E3 | — |
| runtime-boundary, mcp-tool-contract | [json-schema.org](https://json-schema.org/specification) | 2020-12 | current release and dialect URI `https://json-schema.org/draft/2020-12/schema` | E3 | boundary asks which dialect validates application input; MCP asks which dialect the protocol declares |
| runtime-boundary | [zod.dev](https://zod.dev) and the npm registry | 4.4.3 | subpath exports; `parse` throws versus `safeParse` returning a result; `z.toJSONSchema()` throws by default on unrepresentable types | E3 | — |
| runtime-boundary | [ajv.js.org](https://ajv.js.org/) and the npm registry | 8.20.0 | the default export is draft-07 and 2020-12 requires `Ajv2020`; `allowUnionTypes` lifts a strict-mode restriction; "Do NOT use `allErrors` in production" | E3 | — |
| public-api | [vitest.dev](https://vitest.dev/guide/testing-types) | current | `expectTypeOf` and `assertType` are compile-time only; `--typecheck` required; the default type-test glob | E3 | — |
| public-api | TypeScript documentation on `@ts-expect-error` | current | the only TypeScript-team-documented compile-error assertion; self-flags when no error occurs | E3 | — |
| build-graph | [github.com/microsoft/TypeScript/wiki/Performance](https://github.com/microsoft/TypeScript/wiki/Performance) and project-reference documentation | current | project references, `composite`, `.tsbuildinfo`, `--build`, and the documented diagnostics switches | E3 | — |
| build-graph | — | — | whether `--generateTrace` and `--extendedDiagnostics` behave identically under the native TypeScript 7 compiler | **E5 — unverified.** The reference must state this as unknown and require the agent to record which compiler produced a trace. | — |
| publication-integrity | — | — | whether CircleCI is currently a supported trusted-publishing provider | **E5 — unverified.** npm docs list it; the dated GA post named only GitHub Actions and GitLab CI/CD. The reference must not assert it. | — |
| static-enforcement | — | — | the exact camelCase spellings of the typed config exports | **E5 — unverified.** Only the kebab-case documentation identifiers were confirmed; the reference must direct the reader to the installed package's exports. | — |
| node-execution | — | — | Node's own prose definition of `erasableSyntaxOnly` | **E5 — unverified.** The flag appears in Node's recommended tsconfig snippet with no prose definition on the API page; the compiler documentation is the source to use. | — |

Any row that could not be verified stays unverified. A reference file that upgrades an `E5` row to
an assertion is a defect, and [07](./07-red-team-and-acceptance-gates.md) treats it as one.

## 9. What would invalidate this document

- `tests/validate-aws-progressive-disclosure.py` is generalized beyond `skills/aws/**`, which
  converts §1 from a board rule into a gate and makes the 90-line limit hard.
- `tests/validate-skill-allowed-tools.py` changes its token grammar to admit MCP tool names, which
  changes §2.1 and would let Context7 become a declared grant.
- The `category` enum in `schemas/skill.frontmatter.schema.json` changes, which invalidates the §4
  assignments.
- Any `E3` row in §8 changes at its source. The TypeScript rows are the most likely, and have
  already moved once during this work.
- Two reference files are found to contain the same prose for the same question, which is a §5.1
  violation and requires a merge rather than an edit.
