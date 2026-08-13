# Phase 0 — Reconnaissance and Evidence Map

> Status: **PLAN — Phase 0 code registration LANDED (commit `2ff7461a`); no board asset built.**
> No agent, skill, reference, or routing-fixture file exists.
> Previous: [README.md](./README.md) · Next: [01-enterprise-pain-register.md](./01-enterprise-pain-register.md)

## Evidence labels

`E1` user-supplied · `E2` repository pattern observed in this repo (file:line required) ·
`E3` primary vendor or specification document verified during this work ·
`E4` Context7-retrieved (resolved library ID and documented version recorded) ·
`E5` unverified — stated as unknown, never asserted · `E6` design judgment, not a fact

No external fact appears in these documents without a label and a source. Where verification
failed, the document says so. An unverified claim is never upgraded to a verified one.

## 1. The two findings that gate everything

### 1.1 Registering `typescript` as a provider touches eight places

The `provider` value is a closed enum enforced independently in several files, plus two
hand-written documentation lists (E2). Status as of commit `2ff7461a`: points 1 to 5 are
**LANDED**, point 6 (the Kiro Power) is deferred to Phase 10, and points 7 and 8 land in the same
commit as the first agent, for the invariant reason below:

| # | File | What must change | Failure if skipped |
|---|------|------------------|--------------------|
| 1 | `schemas/agent.schema.json` | add `typescript` to the `provider` enum | `validate:agent-schema` fails |
| 2 | `schemas/skill.schema.json` | add `typescript` to the `provider` enum | `validate:skill-schema` fails |
| 3 | `tests/validate-catalog.py:21` (`ALLOWED_PROVIDERS`) | add `"typescript"` | `validate:catalog` fails at `tests/validate-catalog.py:135` |
| 4 | `tools/vfa-tui/src/models/provider.rs` **and** `tools/vfa-tui/src/federation/coverage.rs:331` | add the `Typescript` variant (kebab-case serde) **and** the `"typescript" => Provider::Typescript` arm to `infer_provider` | the enum omission fails `cargo test` (the TUI cannot deserialize the catalog); the `infer_provider` omission fails **silently** — its `_ => Provider::Generic` fallback at line 345 groups every `agents/typescript/**` and `skills/typescript/**` row under Generic while all gates stay green |
| 5 | `scripts/generate-docs-data.mjs:59` | add `typescript` to the `Developer Platforms` taxonomy row | provider silently omitted from `provider_taxonomy` |
| 6 | `scripts/generate-kiro-powers.mjs` | optional — only if the board ships a Kiro Power | no Power generated (acceptable; netsuite and finance ship none) |
| 7 | `docs/taxonomy.md` (provider bullet list, lines 5–48) | add the `typescript` bullet | provider invariant in `CLAUDE.md` violated |
| 8 | `docs/language-stack-boards.md` | add the board section and update the enumerations | documentation drifts from the catalog |

Point 4 is the trap, twice over. CI's `Gate` job is path-filtered to `tools/vfa-tui/**`, so a
catalog-only pull request passes CI while leaving the TUI broken against the new provider — the
three cargo gates must be run locally (E2, `CLAUDE.md` "Adding a new provider"). And the Rust
point is **two edits, not one**: the strict enum in `provider.rs` is what breaks loudly, while
`infer_provider` in `federation/coverage.rs` maps the provider path component separately and falls
back to `Provider::Generic` for anything unlisted. Adding only the enum variant produces a build
that passes every gate and displays the entire TypeScript board as Generic in the federation
coverage view. Phase 0 therefore requires the `infer_provider` arm plus a focused path-mapping
test.

Points 7 and 8 have a **sequencing constraint** that is easy to get backwards, and it is not a
matter of taste. `scripts/generate-docs-data.mjs:27` computes `providers` as
`[...new Set(agents.map(a => a.provider))]` — the provider list is derived from **agent metadata
only**. `CLAUDE.md`'s provider invariant requires
`set(provider bullets in docs/taxonomy.md) == provider_list in docs/_data/catalog.yml ==
{distinct providers that have at least one agent}`. Adding the `typescript` bullet while zero
TypeScript agents exist breaks that invariant in the middle: the hand-written bullet says the
provider exists and the generated list cannot. **The documentation edits must land in the same commit as the
first agent** — not with the schema registration, and equally not later: the invariant is false with
a bullet and no agent, and just as false with an agent and no bullet. See
[06 §1.1](./06-implementation-roadmap-and-integration.md).

### 1.2 This repository has no TypeScript program to reason from

There is no root `tsconfig.json`. `package.json` declares no `typescript` dependency — its
`devDependencies` are semantic-release tooling and `fast-check` only. The only `.ts` files in the
tree are security-detection test fixtures under
`tests/fixtures/frontend-security-detection/**` (E2).

Consequence, and it is a design constraint rather than a caveat: **no agent on this board may
ground a verdict in "the installed version".** Every version-gated conclusion is preconditioned
on evidence the user supplies — compiler version, every relevant `tsconfig.json`, the runtime
target, the run command, and the lint configuration. An agent that issues a strictness or
resolution verdict without them is guessing, and the plan requires it to refuse instead.

## 2. Repository evidence map

| Concern | Repository evidence | Implication for the TypeScript board |
|---|---|---|
| Agent directory layout | `agents/<provider>/<agent-id>/` containing `AGENT.md`, `metadata.json`, `harnesses/` — e.g. `agents/java/java-maestro-agent/` | 14 agent directories × 9 files = 126 files |
| Harness adapters | exactly seven: `codex.toml`, `copilot.agent.md`, `claude-code.agent.md`, `cursor.agent.md`, `gemini.agent.md`, `kiro-ide.agent.md`, `kiro-cli.agent.json` | no new adapter formats may be invented |
| Agent metadata required fields | `schemas/agent.schema.json` `required[]`: `id`, `name`, `version`, `type`, `provider`, `harnesses`, `summary`, `source_type`, `official_docs`, `security_notes`, `last_verified`, `path`; `tests/validate-catalog.py:72` also requires `version` | every `metadata.json` carries all of them |
| Agent metadata optional fields | `companion_skills[]`, `execution_tier`, `lifecycle`, `harness_variants{}` | all four used; `companion_skills` is mandatory for 1:1 skills per `CLAUDE.md` |
| Execution tier enum | `static-review` / `read-only-runtime` / `mutating-runtime` (agent schema); the skill frontmatter `$defs.liveAgentFields` adds `sandbox-mutating` | board is `static-review` only in v1 |
| Skill directory layout | `skills/<provider>/<skill-id>/` with `SKILL.md` + `metadata.json`; `references/` optional and per-skill | 14 skill directories; references only where the specialist needs them |
| SKILL.md frontmatter contract | `schemas/skill.frontmatter.schema.json`: `name` (kebab), `description` 50–1500 chars, `allowed-tools`, `metadata.{author,version}`; optional `metadata.{updated,category,lifecycle}`; `category` is a closed enum | there is no `typescript` and no `governance` category value |
| `allowed-tools` token grammar | `tests/validate-skill-allowed-tools.py:34` — `^[A-Z][A-Za-z0-9]+(\([^)]+\))?$` | an MCP tool name such as `mcp__Context7__query-docs` is **unrepresentable**; zero SKILL.md files in the catalog declare one |
| Shell coherence gate | `tests/validate-skill-coherence.py:311` iterates `skills/*/*/SKILL.md`; every command in a `bash`/`sh`/`shell`/`console` fence must be covered by a Bash grant; `MAX_WILDCARDS = 12` at line 98; `references/*.md` are not scanned | a static-review skill must contain **no bash fence**, or it forces a Bash grant that contradicts its tier |
| Progressive-disclosure gate scope | `tests/validate-aws-progressive-disclosure.py:12` — `AWS_DIR = ROOT / "skills" / "aws"`; required references at line 14; the ≤90-line SKILL.md rule at line 52 | **not enforced for `skills/typescript/**`** — the discipline is adopted voluntarily as a board rule |
| AgentCore precedent | `skills/aws/aws-agentcore/` = `SKILL.md` (short) + `metadata.json` + six topic files in `references/` + `agents/openai.yaml` | one reference per component, lazy-load index, no encyclopedia SKILL.md |
| Java board precedent | `agents/java/java-maestro-agent/AGENT.md` names a Required Skill and routes only; the routing table lives in `skills/java/java-maestro/SKILL.md`, not in the agent | reproduce this split exactly |
| Specialist section contract | `agents/java/java-concurrency-and-virtual-thread-agent/AGENT.md`: Mission · Business pain removed · Failure classes prevented · Decision rights · Anti-goals · Required inputs · Outputs · Operating Rules · Escalation triggers · Validation gates · Metrics · Adversarial review checklist · Tools · Response Shape | reproduce this section set |
| Tool-tier gate | `tests/validate-agent-tool-tiers.py` — a tiered agent must carry an explicit `tools:` block in `copilot.agent.md`; execution tools are `execute/*` plus `run_terminal_command`, `runCommands`, `terminal` (lines 66–67, test at line 98); synthesized default is `["read", "search", "search/codebase"]` (line 86); network egress is reported, not failed | all 14 copilot adapters carry exactly `read`, `search`, `search/codebase` |
| Model policy projection | `scripts/model-policy.mjs:91` `HARNESS_CAPABILITIES` — codex projects `model` + `model_reasoning_effort` (`reasoning_key` at line 95); claude-code projects `model` + `effort` (line 102); cursor projects `model` only; copilot, gemini, and kiro are unmanaged | never hand-edit those keys; policy changes go in `catalog/model-policy.json` |
| Maestro routing fixture | `tests/fixtures/<provider>-maestro-routing/` with `taxonomy.json` + `inputs/` + `expected/`; **all three** generated by `tests/_generate_maestro_routing_fixtures.py` (discovers providers by their `*-maestro` skill directory; `:308` overwrites `taxonomy.json` from agent ids and summaries on every run) | fixture required for the board's maestro; **agent summary wording is a routing input**, not just documentation |
| Generated `live_guard_intent` | `tests/_generate_maestro_routing_fixtures.py:168` emits `GATE_INTENT["default"]`; java and php both carry `(destroy\|delete\|terminate\|rollout to prod\|…)` — destructive **verbs**, never domain nouns | a gate regex containing a domain noun black-holes that domain (see [04 §5.4](./04-routing-architecture-and-fixtures.md)) |
| Generated `parallel_threshold` | `tests/_generate_maestro_routing_fixtures.py:169` emits `0.8`; the validator's fallback when the key is absent is `0.6` (`tests/validate-maestro-routing.py:60`) | expect `0.8` in a generated taxonomy, not the validator default |
| Routing grader mechanics | `tests/validate-maestro-routing.py:65` lowercases the task; `:77` `evaluate()`; word-boundary match for word-only keywords and substring match for keywords containing non-word characters; score is the keyword hit count, sorted by score descending then domain name ascending; `:60` `DEFAULT_PARALLEL_THRESHOLD = 0.6`; `:106` score 0 returns `unclassified`; `:81` `live_guard_intent` regex triggers gate mode | domain keywords must be lexically discriminative |
| Routing gate hard failures | `:123` a domain's agent must exist in `catalog/agents.json`; `:128` the same for every `live_guards` entry; route or mode mismatch fails | the taxonomy cannot reference an agent before it exists |
| Routing gate softness | `:155` prints `SKIP` for a provider directory with no `taxonomy.json`; an empty `inputs/` directory only warns | the gate will **not** force the fixture — treat it as a self-imposed requirement |
| Install roles | `catalog/install-roles.json` is hand-maintained; `tests/test-vfa-export-coverage.test.mjs:99` fails on any agent absent from every role; `:108` fails on a provider with no role-covered agent | four roles must cover all 14 agents |
| README counts | `tests/validate-readme-counts.mjs` — block markers `<!-- readme-counts:start -->` / `<!-- readme-counts:end -->` plus inline `<!-- count:KEY -->N<!-- /count -->`; providers are derived from agent metadata | adding a provider moves four counts; regenerate, never hand-edit |
| Asset integrity scope | `catalog/asset-integrity.json` `scope.trees` = agents, rules, mcp, schemas, catalog, scripts, powers, plugins, `.claude-plugin`, `.cursor-plugin`, `.github/plugin`, `.agents/plugins`, tests; `scope.root_files` = README, SECURITY, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT, CLAUDE, AGENTS, GEMINI, `package.json`, `.releaserc.js` | **`.claude/**` is not hashed** — this workflow needs no integrity refresh; the implementation does |
| Rule assets | `rules/<harness>/<rule-id>.md` + `.metadata.json`; `schemas/rule.schema.json` provider enum excludes language boards; no language board ships rules | the board ships **no rules** |
| Existing TypeScript ownership | `agents/frontend/typescript-contracts-agent/` and `skills/frontend/typescript-contracts-review/` (references: `strict-flag-posture.md`, `trust-boundary-validation.md`, `public-api-surface-diff.md`) | the largest duplication risk on this board; see [03](./03-final-board-and-boundary-contracts.md) |
| MCP ownership | only `agents/netsuite/netsuite-ai-connector-mcp-agent/` (NetSuite AI Connector) and `agents/nvidia/nvidia-agentic-ai-platform-review-agent/` (NeMo signed tool definitions) touch MCP | generic MCP tool-schema contracts are unowned — a verified gap |

## 3. Precedent this board must not copy

`agents/frontend/typescript-contracts-agent/AGENT.md` states under Operating Rules: "no Bash
execution of `tsc`/build tooling". Its companion `skills/frontend/typescript-contracts-review/SKILL.md`
declares `allowed-tools: Read Grep Glob Bash(git diff:*) Bash(tsc --noEmit:*) WebFetch` (E2).

The agent's contract and its skill's tool grant contradict each other. The grant is what a
harness enforces; the prose is what a reviewer reads. **Board rule:** an agent's declared tier
and its companion skill's `allowed-tools` must agree, and every specialist on this board grants
`Read Grep Glob` — no Bash and no network tool, matching the `static-review` tier every agent on
the board declares.

## 4. Brief-versus-reality corrections

Each of these is an assumption the commissioning brief made that the evidence refutes.

### 4.1 "TypeScript 6.0 is on the path toward the native TypeScript 7 effort"

Refuted (E3). TypeScript **7.0 is already generally available** — released 2026-07-08 and
published as npm's `latest` at `7.0.2`
([devblogs.microsoft.com/typescript/announcing-typescript-7-0/](https://devblogs.microsoft.com/typescript/announcing-typescript-7-0/),
[registry.npmjs.org/typescript](https://registry.npmjs.org/typescript), retrieved 2026-08-12).
Current `dist-tags`: `latest=7.0.2`, `rc=7.0.1-rc`, `beta=6.0.0-beta`, `next=7.1.0-dev.20260811.1`.
`tsc` in 7.0.2 is the native Go binary — the package ships 19 platform `optionalDependencies`
and `lib/tsc.js` execs the native executable. Guidance written as if 7 is future work is wrong
on arrival.

### 4.2 "Stricter defaults and module-resolution deprecations" — confirmed, and stronger than stated

TypeScript 6.0 reached general availability 2026-03-23
([devblogs.microsoft.com/typescript/announcing-typescript-6-0/](https://devblogs.microsoft.com/typescript/announcing-typescript-6-0/), E3):
`strict` now defaults to `true`; the `module` default flipped to `esnext`; `target` floats to the
current-year ES version; the `amd`, `umd`, `system`, and `none` module values were removed along
with `--outFile`, `--downlevelIteration`, and `target=es5`; `node10` and `classic`
`moduleResolution` were deprecated and are now gone.

Verified empirically against an installed `typescript@7.0.2` (E3): `--module amd/umd/system` and
`--moduleResolution classic/node10` are **hard removed** (error TS5108), not merely deprecated.
Valid `module` values today: `commonjs`, `es6`/`es2015`, `es2020`, `es2022`, `esnext`, `node16`,
`node18`, `node20`, `nodenext`, `preserve`. Valid `moduleResolution` values: `node16`,
`nodenext`, `bundler`.

**The official tsconfig prose page is stale on this point** — its value tables still list the
removed values ([typescriptlang.org/tsconfig](https://www.typescriptlang.org/tsconfig), E3).
This is the sharpest lesson in the reconnaissance: "check the official page" is necessary but not
sufficient. The compiler binary is authoritative over the prose describing it.

### 4.3 A retrieval layer contradicted the primary source, and the ladder caught it

A Context7 snippet for the TypeScript library asserted that `strict` is *not* default-true in
6.0. That is wrong. It was overridden by the official release announcement and by an empirical
test (TS7006 fires on an untyped parameter with no `tsconfig.json` present) (E3 over E4). This is
the evidence-precedence ladder in [05](./05-skill-and-reference-architecture.md) doing its
intended job, and it is why Context7 is a retrieval layer in this design and never an oracle.

### 4.4 Other brief assumptions the repository refutes

| Brief assumption | Repository reality | Evidence |
|---|---|---|
| Every skill needs an identical file set | only `SKILL.md` + `metadata.json`; the three-reference requirement is AWS-only | `tests/validate-aws-progressive-disclosure.py:12` |
| READMEs are hand-edited | README counts are generated and gated | `tests/validate-readme-counts.mjs` |
| Role registries are auto-maintained | `catalog/install-roles.json` is hand-maintained and gated for orphans | `tests/test-vfa-export-coverage.test.mjs:99` |
| `provider` is free-form or equals the folder name | closed enum in eight places | §1.1 |
| A skill category such as `typescript` or `governance` is available | neither value exists in the closed category enum | `schemas/skill.frontmatter.schema.json` |
| A mixed static/live capability tier is available if useful | available, but every mutating action costs five controls, and every language board except python's governed live plane is static-review | `docs/execution-tiers.md:143` onward |

## 5. External version evidence

Consolidated load-bearing facts. Retrieval date for all rows: 2026-08-12.

| Fact | Label | Source | Version/date documented |
|---|---|---|---|
| `typescript` dist-tags `latest=7.0.2`, `beta=6.0.0-beta` | E3 | [registry.npmjs.org/typescript](https://registry.npmjs.org/typescript) | live registry |
| TypeScript 7.0 GA, native Go compiler, no stable programmatic API until 7.1 | E3 | [announcing-typescript-7-0](https://devblogs.microsoft.com/typescript/announcing-typescript-7-0/) | 2026-07-08 |
| TypeScript 6.0 GA: `strict` default true, `module` default `esnext`, removals listed in §4.2 | E3 | [announcing-typescript-6-0](https://devblogs.microsoft.com/typescript/announcing-typescript-6-0/) | 2026-03-23 |
| `moduleResolution` valid values are `node16`, `nodenext`, `bundler`; `classic`/`node10` removed (TS5108) | E3 | empirical test against `typescript@7.0.2` | 7.0.2 |
| tsconfig prose page still lists removed values | E3 | [typescriptlang.org/tsconfig](https://www.typescriptlang.org/tsconfig) | page as served 2026-08-12 |
| Dual ESM/CJS declaration hazards documented in the modules appendix, not the declaration-publishing page | E3 | [esm-cjs-interop](https://www.typescriptlang.org/docs/handbook/modules/appendices/esm-cjs-interop.html) | current handbook |
| Node type stripping enabled by default since v23.6.0 / v22.18.0; stable since v25.2.0 / v24.12.0; `--experimental-transform-types` removed entirely in v26.0.0 | E3 | [nodejs.org/api/typescript.html](https://nodejs.org/api/typescript.html) | v26.7.0 docs |
| "no type checking is performed" and "Node.js ignores `tsconfig.json` files" | E3 | [nodejs.org/api/typescript.html](https://nodejs.org/api/typescript.html) | v26.7.0 docs |
| `enum`, runtime `namespace`, parameter properties, `import =`, and decorators throw `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX`; `.ts` under any `node_modules` path is refused; import extensions are mandatory | E3 | [nodejs.org/api/typescript.html](https://nodejs.org/api/typescript.html) | v26.7.0 docs |
| "Use the TypeScript compiler separately… `npx tsc --noEmit`" | E3 | [nodejs.org/learn/typescript/run-natively](https://nodejs.org/learn/typescript/run-natively) | current |
| Node lines: v26 Current (EOL 2029-04-30), v24 Active LTS (EOL 2028-04-30), v22 Maintenance (EOL 2027-04-30), v25 already EOL | E3 | [github.com/nodejs/Release](https://github.com/nodejs/Release) `schedule.json` | live schedule |
| Condition ordering is "most specific to least specific in object order"; `types` first, `default` last | E3 | [nodejs.org/api/packages.html](https://nodejs.org/api/packages.html) | v26.x docs |
| The dual-package-hazard section is now a stub pointing at the package-examples repository | E3 | [nodejs.org/api/packages.html](https://nodejs.org/api/packages.html) | v26.x docs |
| `require(esm)` needs no flag today; sync-only, `ERR_REQUIRE_ASYNC_MODULE` on top-level await | E3 | [nodejs.org/api/modules.html](https://nodejs.org/api/modules.html) | v26.x docs |
| `--unhandled-rejections` default is `throw` (changed in v15.0.0); "It is not safe to resume normal operation after `'uncaughtException'`" | E3 | Node CLI and process API docs | v26.x docs |
| typescript-eslint `latest` is 8.67.0; typed linting enabled via `languageOptions.parserOptions.projectService: true`; supported TypeScript range `>=4.8.4 <6.1.0`, outside which the parser warns | E3 | [typescript-eslint.io/packages/parser](https://typescript-eslint.io/packages/parser/), [registry.npmjs.org/typescript-eslint](https://registry.npmjs.org/typescript-eslint) | 8.67.0 |
| "lint times should be roughly the same as your build times" for type-aware rules | E3 | [typescript-eslint.io](https://typescript-eslint.io/troubleshooting/typed-linting/performance) | current docs |
| `no-floating-promises`, `no-misused-promises`, `await-thenable`, `require-await` all require type information | E3 | typescript-eslint rule pages | 8.67.0 |
| npm trusted publishing (OIDC) GA 2025-07-31 for GitHub Actions and GitLab CI/CD; publishes provenance by default | E3 | GitHub Changelog | 2025-07-31 |
| Classic npm tokens permanently revoked 2025-12-09; granular-token expiry cut to a 7-day default / 90-day maximum (announced 2025-09-29); bypass-2FA granular tokens restricted from sensitive management actions 2026-07-31, with a stated January 2027 target to remove their direct-publish ability | E3 | GitHub Changelog | dated posts |
| `npm publish --provenance` requires CLI ≥9.5.0 and a supported cloud-hosted CI; consumers verify with `npm audit signatures` | E3 | [docs.npmjs.com](https://docs.npmjs.com/generating-provenance-statements) | CLI v11 docs |
| API Extractor consumes compiled `.d.ts` and requires `tsc` with `declaration: true` to run first | E3 | [api-extractor.com](https://api-extractor.com/) | current docs |
| MCP specification current revision `2026-07-28`; the `initialize` handshake and protocol sessions are removed; every request carries `_meta.io.modelcontextprotocol/protocolVersion`; mismatch returns JSON-RPC `-32022`; servers must implement `server/discover` | E3 | [modelcontextprotocol.io](https://modelcontextprotocol.io/specification/2026-07-28) | 2026-07-28 |
| Tool fields are `name`, `title`, `description`, `icons`, `inputSchema`, `outputSchema`, `annotations`; both schemas default to JSON Schema 2020-12 absent `$schema`; `structuredContent` is validated against `outputSchema`; protocol errors use JSON-RPC `error` while tool-execution errors use `result.isError: true` | E3 | [modelcontextprotocol.io](https://modelcontextprotocol.io/specification/2026-07-28) | 2026-07-28 |
| The TypeScript SDK split into `@modelcontextprotocol/server` and `@modelcontextprotocol/client`, both `latest = 2.0.0`; `@modelcontextprotocol/sdk` is the legacy 1.x line at `1.30.0` | E3 | npm registry + SDK README on `main` | 2.0.0 |
| `zod` `latest = 4.4.3`; subpaths `zod`, `zod/v3`, `zod/v4`, `zod/mini`, `zod/v4/core`; `z.toJSONSchema()` throws by default on unrepresentable types | E3 | [zod.dev](https://zod.dev), npm registry | 4.4.3 |
| `ajv` `latest = 8.20.0`; the default export is draft-07 and 2020-12 requires the separate `Ajv2020` class; docs warn "Do NOT use `allErrors` in production" | E3 | [ajv.js.org](https://ajv.js.org/), npm registry | 8.20.0 |
| Current JSON Schema release is 2020-12, dialect URI `https://json-schema.org/draft/2020-12/schema` | E3 | [json-schema.org](https://json-schema.org/specification) | 2020-12 |
| Vitest `expectTypeOf`/`assertType` are compile-time only, enabled by `--typecheck`, default glob `**/*.{test,spec}-d.?(c|m)[jt]s?(x)` | E3 | [vitest.dev](https://vitest.dev/guide/testing-types) | current docs |
| `@ts-expect-error` is the only TypeScript-team-documented compile-error assertion; `tsd` and `expect-type` are community tools | E3 | TypeScript docs | current |

### 5.1 Facts that could not be verified — carry forward as unknown

| Question | Status |
|---|---|
| Whether `--generateTrace` and `--extendedDiagnostics` behave identically under the native TypeScript 7 compiler | E5 — not swept. The build-graph specialist must record which compiler produced a trace and must not assume parity. |
| Whether CircleCI is currently a supported npm trusted-publishing provider | E5 — the npm docs list it, but the dated GA post named only GitHub Actions and GitLab CI/CD and called CircleCI planned. No dated source pins when it shipped. |
| The exact camelCase spellings of the typescript-eslint typed config exports | E5 — only the kebab-case documentation identifiers were confirmed. |
| Node's own prose definition of `erasableSyntaxOnly` | E5 — the flag appears in Node's recommended tsconfig snippet with no prose definition on the API page. |

## 6. Source-of-truth versus generated

| File | Kind | How it changes |
|---|---|---|
| `schemas/*.json`, `tests/validate-catalog.py`, `tools/vfa-tui/src/models/provider.rs`, `scripts/generate-*.mjs` | SOURCE | hand-edited |
| `docs/taxonomy.md`, `docs/language-stack-boards.md` | SOURCE | hand-edited (provider invariant) |
| `catalog/install-roles.json` | SOURCE | hand-edited |
| `agents/typescript/**`, `skills/typescript/**` | SOURCE | authored |
| `tests/fixtures/typescript-maestro-routing/taxonomy.json` | **GENERATED** | `npm run maestro-routing:write` — `tests/_generate_maestro_routing_fixtures.py:308` calls `build_taxonomy()` and unconditionally overwrites this file on every run. Any hand curation survives only until the next regeneration. See [04 §5.5](./04-routing-architecture-and-fixtures.md). |
| `tests/fixtures/typescript-maestro-routing/inputs/`, `expected/` | GENERATED | `npm run maestro-routing:write` |
| `catalog/agents.json`, `catalog/skills.json`, `catalog/skill-manifest.json` | GENERATED | `npm run manifest:write` and the manifest chain |
| `.claude-plugin/plugin.json` | GENERATED | `npm run plugin-manifest:write` |
| `.cursor-plugin/plugin.json` | GENERATED | `npm run cursor-plugin:write` |
| `powers/vanguard-*/**` | GENERATED | `npm run kiro-powers:write` |
| `docs/_data/catalog.yml` | GENERATED | `npm run docs-data:write` |
| README count markers | GENERATED | `npm run readme-counts:write` |
| `catalog/asset-integrity.json` | GENERATED | `npm run asset-integrity:write`, last and on its own |
| `.claude/workflow/typescript-board/**` | SOURCE | this plan; outside the integrity scope |

## 7. What would invalidate this document

- A provider registration point is added or removed from the repository, changing the count of
  eight.
- The AWS progressive-disclosure gate is generalized beyond `skills/aws/**`, in which case the
  board's self-imposed reference rules become gate-enforced and the ≤90-line limit becomes hard.
- The maestro routing validator is changed to fail rather than skip a provider with no
  `taxonomy.json`, in which case the fixture stops being self-imposed.
- The repository acquires a TypeScript program of its own, in which case §1.2's evidence
  precondition weakens for repo-internal review only.
- Any external version fact in §5 changes. TypeScript is the fastest-moving of them and has
  already invalidated the commissioning brief once.
