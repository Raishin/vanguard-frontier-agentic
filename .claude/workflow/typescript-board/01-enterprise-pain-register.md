# Enterprise Pain Register

> Status: **PLAN — not implementation.**
> Previous: [00-reconnaissance-and-evidence-map.md](./00-reconnaissance-and-evidence-map.md) · Next: [02-agent-prosecution-scorecard.md](./02-agent-prosecution-scorecard.md)

Pains are ranked by `business impact × likelihood × TypeScript specificity`, each scored 1–5.
The score is a triage aid, not a verdict — see the note on P09.

## 1. Ranked register

| # | Pain | Impact | Likelihood | TS-spec | Score | Owner |
|---|---|---|---|---|---|---|
| P01 | Erased types trusted as validation at an I/O boundary | 5 | 5 | 5 | 125 | runtime-boundary |
| P02 | Production code never type-checked because the runtime strips types | 5 | 4 | 5 | 100 | node-execution |
| P03 | Published package resolves or emits wrongly for one consumer mode | 4 | 4 | 5 | 80 | module-resolution |
| P04 | Declaration-only breaking change shipped as a patch | 4 | 4 | 5 | 80 | public-api |
| P05 | Floating or unhandled promise causes process exit or a partial write | 5 | 4 | 4 | 80 | async |
| P07 | False-green static enforcement | 4 | 4 | 5 | 80 | static-enforcement |
| P10 | Compiler or module-resolution upgrade stalls the estate | 4 | 4 | 5 | 80 | modernization-governor |
| P06 | Type-graph cost in CI minutes and editor latency | 3 | 5 | 5 | 75 | build-graph |
| P11 | Codegen drift between generated types and their producer | 4 | 4 | 4 | 64 | runtime-boundary |
| P08 | Unsound abstraction in shared code | 4 | 3 | 5 | 60 | type-soundness |
| P12 | MCP tool schema drifts from the TypeScript handler | 4 | 3 | 5 | 60 | mcp-tool-contract |
| P15 | No type-level or consumer-compilation tests | 3 | 4 | 5 | 60 | public-api |
| P14 | Per-package tsconfig divergence across a monorepo | 3 | 4 | 4 | 48 | static-enforcement |
| P13 | Privileged TypeScript automation without dry-run or idempotency | 5 | 2 | 4 | 40 | automation-governance |
| P09 | Publication compromise | 5 | 2 | 3 | 30 | publication-integrity |
| P16 | Investment misallocation with no cost model | 3 | 4 | 2 | 24 | economics |

**Rank P09 by tail risk, not by score.** Its product is low because likelihood is low, but the
magnitude is unbounded: a compromised publish reaches every consumer of the package, and no
subsequent review recovers the trust. The score column is not the priority column for it.

## 2. Pain detail

Each pain carries the same ten fields.

### P01 — Erased types trusted as validation at an I/O boundary

| Field | Value |
|---|---|
| Failure mode | An external payload is typed (annotation, `as`, generated interface) but never parsed. The compiler endorses every downstream assumption about a value that was never checked. |
| Affected stakeholder | On-call engineer, data-integrity owner, security reviewer, the customer whose record is corrupted |
| Technical trigger | `JSON.parse(...) as Order`, a typed `fetch` wrapper, a generated OpenAPI client, `process.env.X!`, a queue consumer typed from a schema file that nobody enforces at runtime |
| Consequence | Corrupt or hostile data reaches business logic behind a green build; the failure surfaces far from its entry point, often as a data-integrity incident rather than a crash |
| Existing partial owner | `agents/frontend/typescript-contracts-agent` flags the *absence* of a validator at a trust boundary but does not own validation-boundary design, and is scoped to frontend application diffs |
| TypeScript-specific judgment | Distinguishing an annotation from a check; knowing that types are fully erased; deciding where `unknown`-first ingestion is required versus where a parse already exists upstream; judging whether a generated type is a claim or a check |
| Evidence available | Boundary source, declared schemas, installed validator and version, generator configuration, error-handling paths |
| Ownership | `typescript-runtime-boundary-contract-agent` |
| Handoff boundary | Exploitation, authorization, secrets → application security. Organization-wide API compatibility → API governance. MCP tool wire contracts → `typescript-mcp-tool-contract-agent`. |
| Dedicated agent justified | Yes. This is the defect class TypeScript itself creates, and nothing in the catalog owns the design of the validation boundary. |

### P02 — Production code never type-checked because the runtime strips types

| Field | Value |
|---|---|
| Failure mode | The team runs `.ts` directly and concludes the compiler is optional. Nothing type-checks the shipped code. |
| Affected stakeholder | Service owner, on-call, release manager |
| Technical trigger | `node server.ts` in a container entrypoint, `tsx` in a start script, a CI pipeline with tests but no `tsc --noEmit` job |
| Consequence | Every guarantee the team believes it has is absent in the deployed artifact; type errors reach production as runtime failures |
| Existing partial owner | None |
| TypeScript-specific judgment | Knowing that Node performs no type checking and ignores `tsconfig.json` for execution (E3, [nodejs.org/api/typescript.html](https://nodejs.org/api/typescript.html)); which syntax throws `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` (`enum`, runtime `namespace`, parameter properties, `import =`, decorators); that `.ts` under any `node_modules` path is refused; that import extensions are mandatory; that `--experimental-transform-types` was removed in v26.0.0 |
| Evidence available | Node version, exact run command and flags, CI job definitions, every `tsconfig.json`, container entrypoint |
| Ownership | `typescript-node-execution-compatibility-agent` |
| Handoff boundary | Resolution and emit design → module-resolution. Browser and edge runtimes → deferred. |
| Dedicated agent justified | Yes. It exists to kill one specific and current misconception that no other agent is positioned to catch. |

### P03 — Published package resolves or emits wrongly for one consumer mode

| Field | Value |
|---|---|
| Failure mode | The package compiles and its own tests pass, but one consumer configuration cannot import it or resolves the wrong declarations |
| Affected stakeholder | Every downstream team; the package maintainer absorbing the support load |
| Technical trigger | `exports` conditions in the wrong order, a missing `types` condition, `.mts`/`.cts` mismatch, dual ESM and CJS builds sharing one declaration file, `moduleResolution: bundler` output consumed by Node |
| Consequence | Broken builds for consumers the maintainer never tested; a support queue that scales with adoption |
| Existing partial owner | `agents/frontend/build-tooling-bundling-agent` owns bundler configuration, not package resolution semantics |
| TypeScript-specific judgment | The resolution-mode matrix; that valid `moduleResolution` values are now only `node16`, `nodenext`, `bundler` (E3, empirical against 7.0.2); condition ordering is most-specific-first with `types` first and `default` last (E3, [nodejs.org/api/packages.html](https://nodejs.org/api/packages.html)); that a single compilation cannot validate every consumer scenario |
| Evidence available | `package.json`, every `tsconfig.json`, emitted output or `--showConfig`, the declared consumer list |
| Ownership | `typescript-module-resolution-and-emit-agent` |
| Handoff boundary | Runtime support and execution → node-execution. Publish identity → publication-integrity. |
| Dedicated agent justified | Yes. |

### P04 — Declaration-only breaking change shipped as a patch

| Field | Value |
|---|---|
| Failure mode | A type signature changes with no runtime change, so it reads as a refactor and ships without a major bump |
| Affected stakeholder | Every consumer team; the maintainer's release process |
| Technical trigger | Widening a parameter, narrowing a return, adding a required generic parameter, changing a union member, exporting a type that was previously internal |
| Consequence | Downstream builds break on a patch upgrade — the failure mode most likely to erase trust in a shared package |
| Existing partial owner | None for TypeScript; `agents/kotlin/kotlin-library-api-abi-governance-agent` is the cross-language precedent |
| TypeScript-specific judgment | What is actually public in the emitted `.d.ts` versus what the source appears to export; classifying a type change as breaking; whether `isolatedDeclarations` or a rollup changes the surface; whether a consumer compilation would fail |
| Evidence available | Previously published surface or API report, current `.d.ts`, the consumer `tsconfig` set |
| Ownership | `typescript-public-api-and-declaration-governance-agent` |
| Handoff boundary | Runtime behavior → the relevant specialist. Publish mechanics → publication-integrity. |
| Dedicated agent justified | Yes. Highest cost per unit of code changed on the board. |

### P05 — Floating or unhandled promise causes process exit or a partial write

| Field | Value |
|---|---|
| Failure mode | A promise is created and never awaited or handled; a write sequence is interrupted mid-way; work continues after its caller has given up |
| Affected stakeholder | On-call, data-integrity owner, the customer with a half-applied change |
| Technical trigger | A missing `await`, an async callback passed where a `void` return is expected, no `AbortSignal` plumbed through a request path, an unbounded `Promise.all` over user-sized input |
| Consequence | Node's default `--unhandled-rejections` mode is `throw` (E3), so an unhandled rejection terminates the process — and the docs state it is not safe to resume after `uncaughtException`. A partial write is worse than a crash. |
| Existing partial owner | `agents/frontend/javascript-runtime-agent` owns browser event-loop and DOM listener lifecycle, a different runtime and defect set |
| TypeScript-specific judgment | Ignored promises are reliably detectable only with type information (`no-floating-promises`, `no-misused-promises` — both require type info, E3); typed cancellation contracts; `Promise<T>` in an interface implemented synchronously; thrown `unknown` versus a typed error channel |
| Evidence available | Source, Node version, lint configuration, declared concurrency limits |
| Ownership | `typescript-async-contract-reliability-agent` |
| Handoff boundary | Browser scheduling and DOM → `javascript-runtime-agent`. Broker and queue architecture, distributed retry policy → the relevant platform board. |
| Dedicated agent justified | Yes, with a stated caveat: if a Node board is ever created, this agent migrates to it. |

### P06 — Type-graph cost in CI minutes and editor latency

| Field | Value |
|---|---|
| Failure mode | Typecheck and lint dominate CI; the editor lags on every keystroke; nobody can name which construct is responsible |
| Affected stakeholder | Every engineer, every day; the platform team owning CI spend |
| Technical trigger | One program spanning the whole monorepo, no project references, `.tsbuildinfo` discarded between runs, generated code inflating the graph, a conditional type instantiated at scale |
| Consequence | A recurring tax proportional to headcount; slow feedback loops that push engineers to skip local checks |
| Existing partial owner | `agents/frontend/monorepo-dx-agent` owns the task graph and remote caching, not the TypeScript program graph |
| TypeScript-specific judgment | Distinguishing task-graph from type-graph cost; reading `--extendedDiagnostics` and `--generateTrace` output; knowing that the native compiler changes the cost model and that trace-tool parity under TypeScript 7 is unverified (E5) |
| Evidence available | Measured timings, a trace, the tsconfig graph, package topology |
| Ownership | `typescript-build-graph-performance-agent` |
| Handoff boundary | Runner topology and cache infrastructure → CI/platform. Bundle output size → frontend build tooling. |
| Dedicated agent justified | Yes. Highest-frequency pain on the board. It must refuse to prescribe project references without a measurement. |

### P07 — False-green static enforcement

| Field | Value |
|---|---|
| Failure mode | "It passed" proves far less than the team believes: a weakened compiler configuration, untyped lint, or a misconfigured project service |
| Affected stakeholder | Every reviewer trusting the checkmark; the security reviewer relying on it |
| Technical trigger | A package that opts out of `strict` (which is default-true since 6.0, E3), lint rules that silently need type information they do not have, `allowDefaultProject` masking files outside any tsconfig, editor and CI disagreeing |
| Consequence | Reviews approve on evidence that does not exist; defect classes the team believes are impossible ship regularly |
| Existing partial owner | `agents/frontend/typescript-contracts-agent` reviews strictness posture for a frontend application diff only |
| TypeScript-specific judgment | Since 6.0 the question inverted — the work is detecting *silent loosening*, not turning `strict` on. Plus a live version hazard: typescript-eslint supports `>=4.8.4 <6.1.0` (E3), so a repository on TypeScript 7.0.2 is outside the supported range and the parser only warns. |
| Evidence available | Every `tsconfig.json`, the lint configuration, CI job definitions and durations |
| Ownership | `typescript-static-enforcement-policy-agent` |
| Handoff boundary | Per-construct soundness verdicts → type-soundness. Build restructuring → build-graph. |
| Dedicated agent justified | Yes, as the merged owner of compiler-flag and typed-lint policy. Neither half is a separable decision. |

### P08 — Unsound abstraction in shared code

| Field | Value |
|---|---|
| Failure mode | A type-level abstraction claims a guarantee it does not provide, so the compiler actively endorses a wrong assumption |
| Affected stakeholder | Every caller of the abstraction; the reviewer who trusted the signature |
| Technical trigger | A type predicate that does not check what it claims, a bivariant generic, a conditional type with an unreachable branch, `satisfies` used where an annotation was required, a branded type constructible without validation |
| Consequence | Runtime type errors despite a green compile, in exactly the code the most teams depend on |
| Existing partial owner | `agents/frontend/typescript-contracts-agent` (application diffs); `agents/python/python-language-contracts-typing-agent` is the cross-language precedent |
| TypeScript-specific judgment | Variance, conditional and mapped type correctness, predicate honesty, whether complexity buys a real polymorphism need |
| Evidence available | Source, every relevant tsconfig, which files are published |
| Ownership | `typescript-type-soundness-agent` |
| Handoff boundary | Frontend application diffs → `typescript-contracts-agent`. Validator design → runtime-boundary. Flag policy → static-enforcement. Exported surface → public-api. |
| Dedicated agent justified | Yes, only under the artifact-scope split in [03](./03-final-board-and-boundary-contracts.md). Without that split it duplicates an existing agent. |

### P09 — Publication compromise

| Field | Value |
|---|---|
| Failure mode | The publish path, not the code, is the weakness: a long-lived token, no provenance, an over-broad tarball, an install script |
| Affected stakeholder | Every consumer; the security organization; the company's customers |
| Technical trigger | A stored npm token in CI, publishing without provenance, `files` omissions shipping tests or sources, an unscoped package name, publish-time lifecycle scripts |
| Consequence | Unbounded. A compromised publish is distributed before it is detected. |
| Existing partial owner | `agents/frontend/package-governance-agent` owns dependency *intake*; the sigstore board owns signing infrastructure |
| TypeScript-specific judgment | Moderate and honestly scored. The TypeScript-specific part is that a published TypeScript package's *types* are part of its compatibility and disclosure surface — shipped `.d.ts`, source maps, and the `types` condition. The OIDC and provenance mechanics are npm-generic. |
| Evidence available | Publish workflow, `.npmrc` and `publishConfig`, packed file list, registry settings |
| Ownership | `typescript-package-publication-integrity-agent` |
| Handoff boundary | Dependency intake → `package-governance-agent`. Organization-wide secrets and identity → security board. Signing infrastructure → sigstore board. |
| Dedicated agent justified | Yes, narrowed to publication. Current context makes it timely: trusted publishing reached GA 2025-07-31 and classic tokens were permanently revoked 2025-12-09 (E3). |

### P10 — Compiler or module-resolution upgrade stalls the estate

| Field | Value |
|---|---|
| Failure mode | A major compiler upgrade cannot land because too many packages depend on removed behavior, and nobody owns the sequencing |
| Affected stakeholder | Platform team, every product team blocked behind the upgrade |
| Technical trigger | Removed `module` and `moduleResolution` values (E3, empirical: TS5108 on `amd`/`umd`/`system` and `classic`/`node10`), `--outFile` and `target=es5` removal, `strict` becoming default-true, accumulated `skipLibCheck` and `@ts-ignore` debt |
| Consequence | Multi-quarter stalls; security and tooling improvements gated behind the upgrade; a growing gap between the estate and its dependencies. TypeScript 7.0 is already GA and its programmatic API is not stable until 7.1, so editor and framework tooling remains on 6.0 (E3) — the estate must plan for a split. |
| Existing partial owner | `agents/frontend/frontend-migration-modernization-agent` owns framework migrations; `agents/kotlin/kotlin-estate-modernization-governor-agent` and `agents/python/python-estate-modernization-governor-agent` are the precedents |
| TypeScript-specific judgment | Which removals bite which packages; staged strictness sequencing; when not to migrate; the 6.0-to-7.0 tooling split |
| Evidence available | Current versions, package inventory, debt counts, ownership map |
| Ownership | `typescript-estate-modernization-governor-agent` |
| Handoff boundary | Steady-state enforcement policy → static-enforcement. Per-file fixes → type-soundness. The financial case → economics. |
| Dedicated agent justified | Yes. |

### P11 — Codegen drift between generated types and their producer

| Field | Value |
|---|---|
| Failure mode | Types were generated from a schema once; the schema moved; the checked-in types did not, and they are trusted anyway |
| Affected stakeholder | Consumers of the generated client; the on-call engineer chasing a field that is no longer sent |
| Technical trigger | A checked-in OpenAPI or GraphQL client, database row types generated in a developer's shell, a generator not wired into CI |
| Consequence | The compiler confidently describes a payload shape that the producer no longer sends |
| Existing partial owner | None |
| TypeScript-specific judgment | That a generated type is a claim, not a check; whether the generator is reproducible in CI; whether drift is detectable by regenerate-and-diff |
| Evidence available | Generator configuration, checked-in output, producer schema, CI wiring |
| Ownership | `typescript-runtime-boundary-contract-agent` (absorbed; a standalone agent would duplicate P01's thesis verbatim) |
| Handoff boundary | Schema design and migration safety → the database board. API compatibility policy → API governance. |
| Dedicated agent justified | No — merged into runtime-boundary. See [02](./02-agent-prosecution-scorecard.md) candidate 14. |

### P12 — MCP tool schema drifts from the TypeScript handler

| Field | Value |
|---|---|
| Failure mode | The declared `inputSchema` or `outputSchema` no longer describes what the handler accepts or returns; or the protocol contract itself has moved |
| Affected stakeholder | Every agent calling the tool; the team operating the server; the user whose action silently misfires |
| Technical trigger | A handler edited without its schema, `structuredContent` diverging from `outputSchema`, an error returned as a protocol error instead of `isError`, an unhandled protocol-version mismatch, tool descriptions carrying injectable text |
| Consequence | Silent tool misbehavior that looks like model error. The specification churn is real: the current revision is `2026-07-28`, which removed the `initialize` handshake and protocol sessions, moved the version into `_meta.io.modelcontextprotocol/protocolVersion`, returns `-32022` on mismatch, requires `server/discover`, and the TypeScript SDK split into `@modelcontextprotocol/server` and `@modelcontextprotocol/client` at 2.0.0 while `@modelcontextprotocol/sdk` became the legacy 1.x line (E3). |
| Existing partial owner | Only `agents/netsuite/netsuite-ai-connector-mcp-agent` and `agents/nvidia/nvidia-agentic-ai-platform-review-agent`, both vendor-specific |
| TypeScript-specific judgment | Comparing a declared schema to handler behavior in typed code; JSON Schema dialect fidelity (both schemas default to 2020-12 absent `$schema`, E3); which SDK generation the code targets |
| Evidence available | Tool definitions, handler source, SDK package and version, declared protocol version |
| Ownership | `typescript-mcp-tool-contract-agent` |
| Handoff boundary | Server hosting, transport, network posture, organization MCP trust policy → `mcp/` and the security board. Vendor connector governance → the NetSuite and NVIDIA agents. |
| Dedicated agent justified | Yes. A verified ownership gap in a repository that ships agentic assets. |

### P13 — Privileged TypeScript automation without dry-run or idempotency

| Field | Value |
|---|---|
| Failure mode | A backfill, migration, or reconciliation script holds production credentials and has no dry-run, no idempotency, and no rollback |
| Affected stakeholder | Data owner, finance or compliance owner, the customer whose records are altered |
| Technical trigger | A one-off script run with `tsx` against production, a loop that writes without checkpointing, an unawaited write in a batch, no reconciliation step |
| Consequence | Data damage that is expensive or impossible to reverse, with no audit trail proving what ran |
| Existing partial owner | `agents/python/python-business-critical-automation-governance-agent` is the pattern, for a different ecosystem |
| TypeScript-specific judgment | The distinctive trigger is *type-stripped, unchecked execution against production credentials* combined with floating-promise partial commits — the intersection of P02 and P05 in a privileged context |
| Evidence available | Script source, run command, credential scope (names only), scheduler or CI configuration, existing runbook |
| Ownership | `typescript-business-critical-automation-governance-agent` |
| Handoff boundary | Executing anything → nobody on this board. Generic application security → security board. Accounting or legal policy → those boards. Distributed retry mechanics → the relevant platform board. |
| Dedicated agent justified | Yes, as a review-only governance role. It reviews privileged scripts and never runs them. |

### P14 — Per-package tsconfig divergence across a monorepo

| Field | Value |
|---|---|
| Failure mode | Each package quietly sets its own bar; there is no fleet policy and no visibility into which packages are weaker |
| Affected stakeholder | Platform lead, reviewers moving between packages |
| Technical trigger | Inherited configs overridden per package, a package opting out of a strict-family flag, `skipLibCheck` set locally to silence one dependency |
| Consequence | Review quality varies invisibly by directory; the weakest package sets the real security posture |
| Existing partial owner | None |
| TypeScript-specific judgment | Resolving effective configuration across `extends` chains; which divergences are legitimate and which are debt |
| Evidence available | Every `tsconfig.json`, `--showConfig` output per package |
| Ownership | `typescript-static-enforcement-policy-agent` |
| Handoff boundary | Program graph restructuring → build-graph. Migration sequencing → modernization-governor. |
| Dedicated agent justified | No separate agent — it is the same policy decision as P07. |

### P15 — No type-level or consumer-compilation tests

| Field | Value |
|---|---|
| Failure mode | The runtime is tested; the type contract is not. Regressions surface as consumer tickets |
| Affected stakeholder | Consumer teams; the maintainer's support load |
| Technical trigger | No `expectTypeOf`/`assertType` suite, no `@ts-expect-error` assertions for intended compile failures, no compilation against a representative consumer tsconfig matrix |
| Consequence | Every type change is unverified until a consumer upgrades |
| Existing partial owner | `agents/frontend/testing-quality-engineering-agent` and the `qa` board own runtime test strategy, not type contracts |
| TypeScript-specific judgment | That Vitest type tests are compile-time only and need `--typecheck` (E3); that `@ts-expect-error` is the only TypeScript-team-documented compile-error assertion; which consumer configurations must be in the matrix |
| Evidence available | Test configuration, existing type tests, the consumer tsconfig set |
| Ownership | `typescript-public-api-and-declaration-governance-agent` (absorbed) |
| Handoff boundary | Runtime test strategy → frontend testing and the `qa` board. |
| Dedicated agent justified | No — inseparable from publishing an API. See [02](./02-agent-prosecution-scorecard.md) candidate 11. |

### P16 — Investment misallocation with no cost model

| Field | Value |
|---|---|
| Failure mode | Platform investment is argued by conviction. Nobody can say what the typecheck tax costs or what a migration would return |
| Affected stakeholder | Engineering leadership, the platform team competing for headcount |
| Technical trigger | A migration proposal with no cost model; a CI bill nobody attributes; a strictness debate with no measured defect data |
| Consequence | Either a year of platform effort with no measurable return, or a cheap high-return fix deferred indefinitely |
| Existing partial owner | `agents/frontend/frontend-finops-cost-to-serve-agent` (infrastructure cost of frontend decisions); `agents/java/java-application-server-exit-agent` and `agents/kotlin/kotlin-kmp-portfolio-decision-agent` are the precedents for consuming supplied figures |
| TypeScript-specific judgment | Low, and scored accordingly. The inputs are TypeScript-specific artifacts (typecheck minutes, editor latency, declaration-breakage tickets, migration effort); the arithmetic is not. |
| Evidence available | User-supplied figures only |
| Ownership | `typescript-engineering-economics-agent` |
| Handoff boundary | Cloud and infrastructure cost → finops board. Frontend cost-to-serve → `frontend-finops-cost-to-serve-agent`. |
| Dedicated agent justified | Conditionally. It is the weakest accepted agent and carries a re-prosecution date. It must refuse to originate a measurement. |

## 3. Coverage of the commissioned investigation areas

| Area the brief required | Pain IDs | Owner |
|---|---|---|
| Type-system false confidence | P08, P07 | type-soundness, static-enforcement |
| Runtime boundary corruption | P01, P11 | runtime-boundary |
| Module and runtime incompatibility | P03 | module-resolution |
| Node native TypeScript execution assumptions | P02 | node-execution |
| Monorepo and build-graph economics | P06 | build-graph |
| Typed lint architecture | P07, P14 | static-enforcement |
| Public API and declaration governance | P04, P15 | public-api |
| Async and concurrency reliability | P05 | async |
| Supply-chain and publishing risk | P09 | publication-integrity |
| Type-level and consumer testing | P15 | public-api (absorbed) |
| Modernization | P10 | modernization-governor |
| AI and MCP TypeScript contracts | P12 | mcp-tool-contract |
| Business-critical TypeScript automation | P13 | automation-governance |
| Business impact and decision economics | P16 | economics (conditional) |
| Multi-runtime portability (Deno, Bun, edge, workers) | none | **Deferred** — see [02](./02-agent-prosecution-scorecard.md) candidate 17. Real but low-frequency for this board's cases, and its evidence needs cannot be met credibly today. |

## 4. Pains this board deliberately does not own

| Pain | Owner |
|---|---|
| DOM XSS, CSP, Trusted Types, client-side script integrity | `agents/frontend/frontend-security-agent` |
| Framework specifics for React, Next.js, Angular, Vue, Svelte | the corresponding frontend specialists |
| Bundler configuration, code splitting, bundle budgets | `agents/frontend/build-tooling-bundling-agent` |
| Monorepo task graph, remote caching, false-green cache reuse | `agents/frontend/monorepo-dx-agent` |
| Dependency intake, lockfile policy, dependency confusion on install | `agents/frontend/package-governance-agent` |
| Accessibility, CSS architecture, design tokens | the corresponding frontend specialists |
| Artifact signing and SLSA attestation infrastructure | the sigstore board |
| Cluster, image, and deployment concerns | the kubernetes and provider boards |
| Database schema design and migration safety | the database and provider boards |
| Organization-wide secrets, identity, and MCP trust policy | the security board and `mcp/` |

## 5. What would invalidate this document

- A Node.js board is created, moving P05 out of this board.
- An MCP board is created, moving P12 out of this board.
- The frontend board expands `typescript-contracts-agent` to cover shared and published program
  semantics, in which case P08's ownership must be re-argued rather than assumed.
- A user brings a multi-runtime estate, which reopens the deferred portability area.
- P16 produces no decision within two quarters, in which case it is removed from the register as
  an owned pain rather than defended.
