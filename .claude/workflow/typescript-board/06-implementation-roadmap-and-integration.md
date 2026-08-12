# Implementation Roadmap and Integration

> Status: **PLAN — not implementation.** Nothing in this document has been executed.
> Previous: [05-skill-and-reference-architecture.md](./05-skill-and-reference-architecture.md) · Next: [07-red-team-and-acceptance-gates.md](./07-red-team-and-acceptance-gates.md)

## 1. Phase 0 — provider registration (hard gate)

Nothing merges before all of this lands. Ordered checklist:

| Step | File | Change | Verify |
|---|---|---|---|
| 0.1 | `schemas/agent.schema.json` | add `"typescript"` to the `provider` enum | `npm run validate:agent-schema` |
| 0.2 | `schemas/skill.schema.json` | add `"typescript"` to the `provider` enum | `npm run validate:skill-schema` |
| 0.3 | `tests/validate-catalog.py` | add `"typescript"` to `ALLOWED_PROVIDERS` (line 21 onward) | `npm run validate:catalog` |
| 0.4 | `tools/vfa-tui/src/models/provider.rs` | add the `Typescript` variant; kebab-case serde already yields `typescript` | `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test` in `tools/vfa-tui` |
| 0.5 | `scripts/generate-docs-data.mjs` | add `typescript` to the `Developer Platforms` taxonomy row (line 59) | `npm run docs-data:write` then inspect `docs/_data/catalog.yml` |
| 0.6 | `scripts/generate-kiro-powers.mjs` | optional: add a `PROVIDERS` entry and `DERIVED_KEYWORDS` if the board ships a Kiro Power | `npm run kiro-powers:write && npm run validate:kiro-powers` |
| 0.7 | `docs/taxonomy.md` | add the `typescript` provider bullet | manual; the provider invariant in `CLAUDE.md` |
| 0.8 | `docs/language-stack-boards.md` | add the board section and update the enumerations and tables | manual |

**Step 0.4 is the one that gets skipped.** CI's `Gate` job is path-filtered to
`tools/vfa-tui/**`, so a pull request that only touches schemas, tests, and the catalog passes CI
while the TUI can no longer deserialize `catalog/agents.json`. The cargo gates must be run
locally, every time a provider is added.

Decision on step 0.6: **ship a Kiro Power.** The board is a first-class language board and Kiro
users should be able to install it in one step. This is a choice, not a requirement — netsuite and
finance ship no Power — and it must be made explicitly rather than by omission.

**Exit criterion:** `npm run validate` passes, and the three cargo gates pass, with the provider
registered and **zero** TypeScript assets present. Prove the registration in isolation before any
asset depends on it.

## 2. Phases 1 to 12

| Phase | Scope | Deliverable | Exit criterion | Commit |
|---|---|---|---|---|
| 1 | Reconnaissance | [00](./00-reconnaissance-and-evidence-map.md) | every repo fact carries a verified `file:line` | `docs(workflow):` |
| 2 | Pain model | [01](./01-enterprise-pain-register.md) | every commissioned investigation area maps to a pain or a named deferral | `docs(workflow):` |
| 3 | Candidate prosecution | [02](./02-agent-prosecution-scorecard.md) | all 19 candidates have an outcome; rejections name their absorbing agent | `docs(workflow):` |
| 4 | Boundary design | [03](./03-final-board-and-boundary-contracts.md) | every agent has an owns/refuses/hands-off contract and the frontend split is written | `docs(workflow):` |
| 5 | Reference architecture | [05](./05-skill-and-reference-architecture.md) | the ownership matrix has no unjustified shared source and the ledger labels every claim | `docs(workflow):` |
| 6 | Provider registration | §1 above | `npm run validate` and the cargo gates green with zero assets | `feat(schemas):` + `feat(vfa-tui):` |
| 7 | Specialist agents | 13 agent directories, 9 files each | `validate:agent-schema`, `validate:catalog`, `validate:agent-tool-tiers` green | `feat(typescript):` |
| 8 | Specialist skills | 13 skill directories plus references | `validate:skill-schema`, `validate:allowed-tools`, `validate:skill-coherence`, `manifest:check` green | `feat(typescript):` |
| 9 | Maestro agent, skill, and fixture | 1 agent directory, 1 skill, `taxonomy.json`, generated fixtures | `validate:maestro-routing` green with non-empty `inputs/` | `feat(typescript):` |
| 10 | Repository integration | `agents/typescript/README.md`, `catalog/install-roles.json`, docs, ROADMAP entry, generated outputs | `validate:install-coverage`, `validate:readme-counts`, `validate:plugin-manifest`, `validate:multi-harness-marketplace`, `validate:codex-marketplace` green | `feat(typescript):` + `docs:` |
| 11 | Validation | the full gate suite in §6 | zero failures across all four gate families | — |
| 12 | Hostile review | [07](./07-red-team-and-acceptance-gates.md) executed against the built assets | every attack has a recorded outcome; no unresolved duplicate | `docs(workflow):` |

**Phases 7 and 9 are ordered deliberately.** The maestro's `taxonomy.json` cannot reference an
agent that is absent from `catalog/agents.json` — `tests/validate-maestro-routing.py:123` fails on
exactly that. Specialists must exist and be cataloged before the routing table is finalized.

Phase 6 has two commits because the schema change and the Rust change are reviewed by different
people and the Rust one has its own gate job.

## 3. File inventory

```text
agents/typescript/
  README.md
  typescript-maestro-agent/
  typescript-type-soundness-agent/
  typescript-runtime-boundary-contract-agent/
  typescript-module-resolution-and-emit-agent/
  typescript-node-execution-compatibility-agent/
  typescript-public-api-and-declaration-governance-agent/
  typescript-build-graph-performance-agent/
  typescript-static-enforcement-policy-agent/
  typescript-async-contract-reliability-agent/
  typescript-package-publication-integrity-agent/
  typescript-estate-modernization-governor-agent/
  typescript-mcp-tool-contract-agent/
  typescript-business-critical-automation-governance-agent/
  typescript-engineering-economics-agent/
      # each agent directory contains:
      #   AGENT.md
      #   metadata.json
      #   harnesses/codex.toml
      #   harnesses/copilot.agent.md
      #   harnesses/claude-code.agent.md
      #   harnesses/cursor.agent.md
      #   harnesses/gemini.agent.md
      #   harnesses/kiro-ide.agent.md
      #   harnesses/kiro-cli.agent.json

skills/typescript/
  typescript-maestro/                                     # SKILL.md + metadata.json, no references/
  typescript-type-soundness/                              # + 3 references
  typescript-runtime-boundary-contract/                   # + 5 references
  typescript-module-resolution-and-emit/                  # + 4 references
  typescript-node-execution-compatibility/                # + 4 references
  typescript-public-api-and-declaration-governance/       # + 4 references
  typescript-build-graph-performance/                     # + 3 references
  typescript-static-enforcement-policy/                   # + 3 references
  typescript-async-contract-reliability/                  # + 3 references
  typescript-package-publication-integrity/               # + 4 references
  typescript-estate-modernization-governor/               # + 4 references
  typescript-mcp-tool-contract/                           # + 4 references
  typescript-business-critical-automation-governance/     # + 4 references
  typescript-engineering-economics/                       # + 3 references

tests/fixtures/typescript-maestro-routing/
  taxonomy.json          # authored
  inputs/                # generated
  expected/              # generated
```

Counts: 14 agent directories × 9 files = **126 agent files**, plus `agents/typescript/README.md`.
14 skills × 2 files = **28 skill files**, plus **48 reference files** (the per-skill counts above,
each including `workflow-and-output.md`). One authored `taxonomy.json`. Generated fixture pairs
follow from the domain count plus the four stress fixtures.

## 4. Install roles

Four roles in `catalog/install-roles.json`:

| Role | Agents |
|---|---|
| `typescript-application-review-engineer` | maestro, type-soundness, runtime-boundary, async, node-execution, static-enforcement |
| `typescript-library-maintainer` | maestro, public-api, module-resolution, publication-integrity, type-soundness |
| `typescript-platform-build-engineer` | maestro, build-graph, static-enforcement, modernization-governor, economics |
| `typescript-agentic-contract-engineer` | maestro, mcp-tool-contract, runtime-boundary, automation-governance |

Coverage proof against `tests/test-vfa-export-coverage.test.mjs`:

- Check A1 (`:99`) fails on any agent absent from every role. All 14 appear: maestro in all four;
  type-soundness in application-review and library-maintainer; runtime-boundary in
  application-review and agentic-contract; module-resolution in library-maintainer; node-execution
  in application-review; public-api in library-maintainer; build-graph in platform-build;
  static-enforcement in application-review and platform-build; async in application-review;
  publication-integrity in library-maintainer; modernization in platform-build; mcp-tool-contract
  in agentic-contract; automation-governance in agentic-contract; economics in platform-build.
- Check A2 (`:108`) fails on a provider with no role-covered agent. `typescript` is covered.

## 5. The one edit to an existing asset

`agents/frontend/typescript-contracts-agent/` (`AGENT.md` plus the seven harness bodies, since the
Markdown-family adapters carry the same body text) and
`skills/frontend/typescript-contracts-review/SKILL.md` receive the artifact-scope split and the
handoff to `typescript-type-soundness-agent` from
[03 §3](./03-final-board-and-boundary-contracts.md).

Constraints on this change:

- Separate commit, separate review. It touches another board.
- Requires frontend-board owner sign-off. If the owner declines, `typescript-type-soundness-agent`
  must be re-prosecuted, not shipped anyway — its Non-overlap score depends on this split.
- Re-hashes `catalog/asset-integrity.json` because `agents/**` is inside the integrity scope.
- Do **not** bundle it with the fix to that pair's tier and grant incoherence
  ([05 §3](./05-skill-and-reference-architecture.md)). That is a defect in an existing asset and
  belongs in its own change, argued on its own merits.

## 6. Generator ordering and validation

Generators, in this order:

```bash
npm run manifest:write:all
npm run docs-data:write
npm run maestro-routing:write
npm run asset-integrity:write
```

`manifest:write:all` runs its generators in parallel with `&` … `wait`, so
`asset-integrity:write` inside it can hash the tree before README counts, the plugin manifests,
and the Kiro Powers have finished writing. Running it again last, on its own, is what makes the
manifest describe the settled tree. This is the ordering caveat in `CLAUDE.md`, and it is the most
common way a change in this repo fails its own gate.

Then the four gate families, in order:

```bash
npm run validate
npm run lint:spell
npx --yes markdownlint-cli2 "**/*.md" "#node_modules"
cd tools/vfa-tui && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

`npm run validate` runs 20+ `validate:*` gates. The count is deliberately approximate here; an
exact number in documentation drifts the moment a gate is added.

### 6.1 Two decisive probes

Green gates prove the assets are well-formed. They do not prove the board works. Run both:

**Positive probe.** Give the maestro: "our nodenext package's exports resolve wrongly for a
CommonJS consumer". Expected: single dispatch to
`typescript-module-resolution-and-emit-agent`. Confirms the taxonomy anchors discriminate and the
agent is reachable.

**Negative probe.** Point one `domains` entry at an agent id that does not exist and run
`npm run validate:maestro-routing`. Expected: failure naming the unknown agent, per
`tests/validate-maestro-routing.py:123`. Confirms the gate is actually wired rather than skipping
the provider. Run this **before** the agents exist — it is the cheapest possible confirmation and
it also demonstrates §7's warning.

## 7. Static versus live capability

Static-review only, all 14 agents. The reasoning and the five controls a mutating tier would
require are in [03 §4](./03-final-board-and-boundary-contracts.md), grounded in
`docs/execution-tiers.md:143` onward.

`typescript-business-critical-automation-governance-agent` reviews privileged scripts and never
executes them. That is the entire point of the role: it is the agent that says no, and an agent
that can run the script has a conflict of interest with that judgment.

Entry criteria for a future `typescript-live-*` plane, all three required: a user brings a
recurring privileged TypeScript automation estate; a named human owner accepts the approval-token
workflow; and the live-guard fixture mode plus `validate:agent-tool-tiers` are wired to gate it.

## 8. ROADMAP entry

Milestone M3 ("Provider & domain breadth") in `ROADMAP.md` is already the home for new providers.
Add, under M3's candidate list:

```markdown
- TypeScript board (`provider: typescript`): 14 agents and 14 skills covering compiler policy,
  runtime boundaries, module resolution and emit, Node execution, declaration governance, build
  graph, async contracts, publication integrity, modernization, MCP tool contracts, privileged
  automation governance, and engineering economics. Plan and acceptance gates:
  `.claude/workflow/typescript-board/`. Entry criterion: Phase 0 provider registration across all
  eight points, including `tools/vfa-tui/src/models/provider.rs`.
```

`ROADMAP.md` is a hashed root file, so the integrity manifest must be refreshed after editing it.

Note for whoever executes this: the existing `.claude/workflow/m365-d365/` plan is referenced from
nowhere in the repository. This plan therefore does **not** add a ROADMAP entry at plan time, only
at implementation time, so the two workflow directories stay consistent with each other.

## 9. Implementation risk register

| Risk | Likelihood | Impact | Detection | Mitigation |
|---|---|---|---|---|
| Keyword collisions make routing fixtures unstable — the IDF filter drops anchors and a domain scores zero | High | Medium | `validate:maestro-routing` failures, or an `unclassified` result for a clear task | Design anchors for lexical uniqueness ([04 §5.3](./04-routing-architecture-and-fixtures.md)); regenerate rather than hand-edit; never let the alphabetical tie-break be load-bearing |
| The frontend-board owner rejects the artifact-scope split | Medium | High | review feedback on the Phase 6 commit | Re-prosecute `typescript-type-soundness-agent` under [02 §1](./02-agent-prosecution-scorecard.md) rather than shipping it with Non-overlap 2 |
| External version facts go stale between plan and build — TypeScript moved during this work | High | Medium | re-run the version probes in [00 §5](./00-reconnaissance-and-evidence-map.md) | The provenance ledger carries retrieval dates; re-verify before authoring, and treat TypeScript as the fastest-moving row |
| The compiler and lint tooling support different TypeScript majors, so the enforcement guidance is internally inconsistent | Medium | Medium | compare the installed compiler against the lint tooling's supported range | `typescript-static-enforcement-policy-agent` owns this explicitly; the reference must state the window rather than a single version |
| Reference duplication creeps in during bulk authoring | High | Medium | normalized comparison of reference source lists across skills | The ownership matrix and the five-skill limit on `official-sources.md`; attack A2 in [07](./07-red-team-and-acceptance-gates.md) |
| An author or delegate invents a compiler flag, CLI switch, or config key | Medium | High | cross-check every flag against the installed compiler and the provenance ledger | No claim ships without a ledger row; unverified rows stay unverified |
| The `category` value chosen for a skill is not in the closed enum | Medium | Low | `validate:skill-schema` | The assignment table in [05 §4](./05-skill-and-reference-architecture.md) |
| `tools/vfa-tui/src/models/provider.rs` is forgotten | High | High | `cargo test` locally; CI will not catch it on a catalog-only change | Phase 0 step 0.4 plus the mandatory local cargo gates |
| A skill's `SKILL.md` contains a bash fence and forces a Bash grant that contradicts its tier | Medium | Medium | `validate:skill-coherence`, and reading the frontmatter | Board rule: no bash fences in `SKILL.md` ([05 §1](./05-skill-and-reference-architecture.md)) |
| Scope creep back toward the rejected candidates during authoring — a codegen section grows inside runtime-boundary until it is a separate agent again | Medium | Medium | compare each agent's Owns list against [03](./03-final-board-and-boundary-contracts.md) | The Owns lists are a contract, not a starting point; additions require re-prosecution |
| `asset-integrity:write` runs before the other generators settle, so the manifest is stale on arrival | High | Low | `validate:asset-integrity` | Run it last, on its own, per §6 |
| The routing fixture is never authored and CI reports success anyway | Medium | Medium | check for a non-empty `inputs/` directory | Treated as a blocker in [07](./07-red-team-and-acceptance-gates.md) despite the gate's `SKIP` |

## 10. Definition of done for the implementation

In this order, because the integrity manifest must hash a settled tree:

1. Generated files regenerated (`manifest:write:all`, `docs-data:write`, `maestro-routing:write`,
   `model-policy:apply` if policy rules were added).
2. `npm run asset-integrity:write` last, on its own.
3. `npm run validate` — zero failures.
4. `npm run lint:spell` and `npx --yes markdownlint-cli2 "**/*.md" "#node_modules"` — zero
   failures.
5. `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` in
   `tools/vfa-tui` — required because Phase 0 touches it.
6. Both probes from §6.1 executed and recorded.
7. `git status` clean; conventional-commit messages; pushed to the working branch.

## 11. What would invalidate this document

- The provider registration surface changes, adding or removing a step in §1.
- `manifest:write:all` is changed to run its generators serially, which removes the §6 ordering
  caveat.
- The install-role coverage checks change, which changes the §4 proof.
- `ROADMAP.md`'s milestone structure changes, which changes where the §8 entry belongs.
- The frontend boundary edit is rejected, which removes §5 and reopens
  [02](./02-agent-prosecution-scorecard.md) candidate 2.
