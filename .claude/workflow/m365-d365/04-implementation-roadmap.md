# 04 — Implementation Roadmap: M365/D365 Agent Board

**Workflow:** `.claude/workflow/m365-d365/`
**Status:** PLAN (not started)
**Last updated:** 2026-06-16

---

## Preamble

This roadmap is execution-first. Every phase has a hard exit criterion: `npm run validate` green. Nothing
ships until the gate passes. No exceptions for "nearly green."

All Microsoft product facts, cert requirements, SDK names, API shapes, and Learn URLs cited in agent and
skill files MUST be re-grounded via **Microsoft Learn MCP** (`microsoft_docs_search` / `microsoft_docs_fetch`)
and **Context7** at the start of each build phase — not now, not at plan time. This document carries no
embedded product claims. Verification debt is tracked per phase.

**Evidence labels used below:**
- E0 — deduced from repo schema/code inspection
- E1 — observed in catalog directory listings
- E2 — confirmed by direct file read during exploration
- E3 — confirmed by npm/python script inspection
- E4 — Microsoft Learn official doc (fetch required per phase)
- E5 — Context7 library doc (fetch required per phase)
- E6 — adversarial red-team finding
- E7 — assumption / judgment call (flag for reverification)

---

## Wave-Sizing Rule

**Max 6 cataloged assets per PR** (agents + skills combined, excluding adapters and rules).

Justification: The repo runs 19+ validation gates (E2: `npm run validate`). Each new agent adds 7 harness
adapter files. Each new skill touches catalog JSON, skill-manifest.json, asset-integrity, and potentially
maestro routing fixtures. At 6 assets per PR the diff stays under ~90 files — reviewable in a single
sitting without missing cross-file inconsistencies. Larger batches cause the validate:multi-harness-marketplace
and validate:maestro-routing gates to fail in confusing ways that are hard to bisect.

Rule: **prototype assets in a feature branch; only merge when validate is green on that branch.**

---

## Definition of Done — Per Asset

Every agent and skill is NOT done until ALL of the following are true. Copy this checklist into each PR
description.

### Agent DoD
- [ ] `metadata.json` passes `validate:agent-schema` (provider = "microsoft", all required fields present)
- [ ] `companion_skills` declared for any 1:1 companion skill relationship
- [ ] 7 harness adapter files present: `codex.toml`, `copilot.agent.md`, `claude-code.agent.md`,
      `cursor.agent.md`, `gemini.agent.md`, `kiro-ide.agent.md`, `kiro-cli.agent.json`
- [ ] `allowed-tools` in each adapter reflects least-privilege baseline (no wildcard unless justified)
- [ ] All `official_docs` URLs verified live against Microsoft Learn (not from memory)
- [ ] `last_verified` date set to current date
- [ ] `references` has minimum 3 entries, all Microsoft Learn or official SDK sources
- [ ] Agent appears in `catalog/agents.json` (regenerated via `npm run manifest:write:all`)
- [ ] Asset integrity updated: `python3 tests/validate-asset-integrity.py --write`
- [ ] `npm run validate` green on branch

### Skill DoD
- [ ] `SKILL.md` frontmatter passes `validate:skill-schema`
- [ ] `allowed-tools` field present (E2: required by schema)
- [ ] Provider = "microsoft" declared
- [ ] `official_docs` URLs live-verified against Microsoft Learn
- [ ] `last_verified` date set to current date
- [ ] `references` minimum 3 entries
- [ ] Skill appears in `catalog/skill-manifest.json` (regenerated via `npm run manifest:write`)
- [ ] Asset integrity updated
- [ ] `npm run validate` green on branch

### Rule DoD
- [ ] Frontmatter passes `validate:rule-schema`
- [ ] Provider = "microsoft" if MS-specific
- [ ] Cross-referenced in at least one agent or skill that uses it
- [ ] `npm run validate` green on branch

---

## Commands Cheat-Sheet

Run these in order after any cataloged asset change. Do not skip steps.

```bash
# 1. Refresh skill manifest
npm run manifest:write

# 2. Refresh all manifests, docs-data, plugin manifests, kiro powers, readme counts
npm run manifest:write:all

# 3. Regenerate asset integrity hashes (MUST run after any file touched in catalog scope)
python3 tests/validate-asset-integrity.py --write
git add catalog/asset-integrity.json

# 4. Regenerate maestro routing fixtures (after adding/removing agents)
python3 tests/_generate_maestro_routing_fixtures.py

# 5. Full validation (all 19+ gates)
npm run validate

# 6. Kiro powers only (if only provider or powers changed)
npm run kiro-powers:write

# 7. Readme counts only
npm run readme-counts:write

# 8. Docs data only
npm run docs-data:write
```

**Never manually edit:** `package.json` version, `catalog/asset-integrity.json` hashes (use --write),
plugin manifest versions. Semantic-release owns version bumps: `feat:` → minor, `fix:` → patch (E2).

---

## Phase 0 — Provider Registration & Scaffolding

**THE HARD GATE. Nothing in Phase 1–6 starts until this is green.**

### Goal
Register "microsoft" as a valid provider across all repo schemas and generators. Prove it with one
throwaway or permanent test asset that passes `npm run validate` end-to-end.

### Work Items

**Step 1 — Schema edits (4 files, all provider enums)**

| File | Change |
|------|--------|
| `schemas/agent.schema.json` | Add `"microsoft"` to provider enum |
| `schemas/skill.schema.json` | Add `"microsoft"` to provider enum |
| `schemas/rule.schema.json` | Add `"microsoft"` to provider enum (needed for Phase 1 rules) |
| `schemas/mcp-reference.schema.json` | Add `"microsoft"` to provider enum (needed for Phase 5 MCP refs) |

Edit all four in one commit. Do not split — the validate:catalog gate checks them together.

**Step 2 — Catalog validator**

File: `tests/validate-catalog.py`
Change: Add `"microsoft"` to the `ALLOWED_PROVIDERS` set.

**Step 3 — Kiro powers generator**

File: `scripts/generate-kiro-powers.mjs`
Change: Add a `microsoft` entry to the `PROVIDERS` object:

```js
microsoft: {
  displayName: "Microsoft",
  description: "...",   // <=3 sentences — write from Microsoft Learn evidence at Phase 0 time
  keywords: [],         // populate from Learn docs at Phase 0 time
  invariants: []        // populate from Learn docs at Phase 0 time
}
```

Run: `npm run kiro-powers:write` → auto-generates `powers/vanguard-microsoft/POWER.md`

**Step 4 — Docs data generator**

File: `scripts/generate-docs-data.mjs`
Change: Add `"microsoft"` to the taxonomy category list.

**Step 5 — Plugin manifests (NO manual edit)**

`scripts/generate-plugin-manifest.mjs` and `scripts/generate-cursor-plugin.mjs` derive providers
dynamically from catalog. No manual edit. But regenerate:

```bash
npm run manifest:write:all
```

**Step 6 — Proof asset**

Create one minimal skill + one minimal agent under the "microsoft" provider. Suggested:
`skills/microsoft/microsoft-provider-smoke-test/SKILL.md` and a matching agent stub. These can be
promoted to `microsoft-maestro` or removed at the end of Phase 0. The point is to exercise the full
validate pipeline with a real microsoft-provider asset before any real build starts.

**Step 7 — Full validate**

```bash
npm run manifest:write:all
python3 tests/validate-asset-integrity.py --write
git add catalog/asset-integrity.json
npm run validate
```

All 19+ gates must be green.

### Exit Criteria
- `npm run validate` green with at least one microsoft-provider asset in catalog
- `powers/vanguard-microsoft/POWER.md` exists and passes validate:kiro-powers
- validate:catalog, validate:agent-schema, validate:skill-schema pass with "microsoft" provider
- No other provider's assets regressed

### Rollback / Abort
If any schema edit breaks an existing provider's assets: revert the four schema files atomically.
The enum addition is purely additive — if validation breaks, the generator or catalog path is wrong,
not the schema itself. Bisect by reverting one file at a time.

### Estimated Asset Count
2 assets (1 smoke-test skill + 1 smoke-test agent). ~16 files total with adapters.

### Risk
**HIGH** — this is the entire foundation. A botched enum or generator edit silently breaks other
providers. Run `npm run validate` on the unmodified repo first to establish a clean baseline diff.

---

## Phase 1 — Microsoft Foundation (Maestro Layer + Shared Rules)

### Goal
Build the maestro orchestration layer for the microsoft provider: 5 maestro agents, 5 maestro skills,
shared Microsoft safety/evidence/source-verification rules. Generate maestro routing fixtures.

### Verification Debt (do before building)
Re-ground all Microsoft Graph API, Copilot extensibility, Power Platform connector, and D365 SDK names
via `microsoft_docs_search` and Context7. Do not reuse any product name or API from memory.

### Work Items

**Maestro agents (5)** — in `agents/microsoft/`:
- `microsoft-maestro` — primary router for all microsoft-provider skills
- `m365-platform-maestro` — M365/Copilot scope
- `d365-platform-maestro` — D365/Finance/SCM/CS scope
- `power-platform-maestro` — Power Platform/Fabric scope
- `microsoft-security-compliance-maestro` — security, zero-trust, compliance scope

Each agent: `metadata.json` + 7 harness adapter files. `companion_skills` declared.

**Maestro skills (5)** — in `skills/microsoft/`:
- `microsoft-maestro-routing` — maestro routing coordination skill
- `m365-scope-triage` — triage incoming M365 requests to correct sub-agent
- `d365-scope-triage` — triage D365 requests
- `power-platform-scope-triage` — triage Power Platform requests
- `microsoft-evidence-grounding` — enforces E4/E5 evidence labels, blocks unverified claims

**Shared rules (3+)** — in `rules/microsoft/`:
- `microsoft-safety-guardrails.md` — no PII, no tenant data, no credentials in output
- `microsoft-source-verification.md` — all claims require Microsoft Learn or official SDK citation
- `microsoft-least-privilege.md` — all recommended permissions scoped to minimum required

**Maestro routing fixtures:**
```bash
python3 tests/_generate_maestro_routing_fixtures.py
```
Verify `validate:maestro-routing` passes. Add adversarial routing test cases (E6): ambiguous M365/D365
requests, cross-scope requests, requests that should refuse.

### Exit Criteria
- `npm run validate` green including `validate:maestro-routing`
- Maestro router adversarial cases: >=3 ambiguous inputs correctly routed
- All 5 agents have 7 harness adapters each
- Rules referenced by at least one agent metadata

### Rollback / Abort
If maestro routing fixtures break an existing provider's routing: isolate microsoft fixtures in a
separate fixture file. The generator should support per-provider fixture files — confirm before build.

### Estimated Asset Count
10 cataloged assets (5 agents + 5 skills) + 3 rules + routing fixtures. ~85 files.

### Risk
**MEDIUM** — Maestro routing logic is the hardest part of this layer. If the fixture generator does
not support a microsoft-scoped fixture file, that is a Phase 0 discovery that must be resolved first.

---

## Phase 2 — Highest-Risk Skills (Security, Identity, Governance)

### Goal
Deliver the skills and companion agents with the highest blast radius if done wrong: identity, zero-trust,
data governance, SoD, and migration cutover.

### Verification Debt (do before building)
Fetch and read from Microsoft Learn: Entra ID conditional access, Power Platform DLP policies, D365
Finance SoD configuration, Azure Migrate, Microsoft Purview data governance. Use `microsoft_docs_fetch`
on specific URLs. Do not rely on Phase 0 or Phase 1 research.

### Work Items (6 skills + 6 companion agents)

| Skill ID | Agent ID |
|----------|----------|
| `m365-copilot-readiness-governance` | `m365-copilot-readiness-agent` |
| `m365-identity-zero-trust` | `m365-identity-zero-trust-agent` |
| `power-platform-governance-dataverse-security` | `power-platform-governance-agent` |
| `d365-success-by-design-governance` | `d365-success-by-design-agent` |
| `d365-security-sod-governance` | `d365-sod-compliance-agent` |
| `d365-data-migration-cutover` | `d365-migration-cutover-agent` |

Build order: SoD and identity first (highest privilege blast radius). Migration cutover last in this
phase (requires SoD and identity patterns to be stable).

**Wave constraint:** Build 2 skill+agent pairs per PR (= 4 assets/PR, within the 6-asset limit).
Three PRs total for this phase.

### Exit Criteria
- `npm run validate` green
- Red-team scorecard (from `05-red-team-and-acceptance.md`) >=4/5 on all Phase 2 skills for:
  security, identity governance, data-governance dimensions
- All `official_docs` URLs return HTTP 200 at merge time
- SoD and identity skills explicitly declare minimum-privilege tool lists

### Rollback / Abort
If a skill's security guardrails cannot score >=4/5 after two remediation attempts: park it in
`skills/microsoft/_draft/` and exclude from catalog until remediated. Do not ship a weak security
skill.

### Estimated Asset Count
12 cataloged assets (6 skills + 6 agents). ~90 files with adapters.

### Risk
**VERY HIGH** — These skills touch identity, access control, and data migration. A hallucinated
permission scope or a wrong SoD rule is a production incident waiting to happen. The re-grounding
step is non-negotiable. The red-team gate is non-negotiable.

---

## Phase 3 — High-Value Business-Process Skills

### Goal
Deliver the core D365 and M365 business-process skills covering finance, supply chain, field service,
customer service, sales, and value realization.

### Verification Debt (do before building)
Fetch Microsoft Learn docs for: D365 Finance close processes, D365 Supply Chain Management plan-to-produce
flows, D365 Field Service work order lifecycle, D365 Customer Service omnichannel, D365 Sales pipeline
and revenue recognition, Success by Design value delivery framework.

### Work Items (6 skills + 6 companion agents)

| Skill ID | Agent ID |
|----------|----------|
| `d365-finance-close-to-report` | `d365-finance-close-agent` |
| `d365-supply-chain-plan-to-produce` | `d365-supply-chain-agent` |
| `d365-field-service-to-cash` | `d365-field-service-agent` |
| `d365-customer-service-contact-center` | `d365-customer-service-agent` |
| `d365-sales-revenue-operations` | `d365-sales-ops-agent` |
| `microsoft-business-impact-value-realization` | `microsoft-value-realization-agent` |

**Wave constraint:** 2 skill+agent pairs per PR. Three PRs.

### Exit Criteria
- `npm run validate` green
- Each business-process skill includes at least one adversarial test case (E6): user asks skill to
  perform an action outside its scope — skill must decline and route correctly
- `official_docs` URLs live-verified at merge time

### Rollback / Abort
If a skill's process coverage is materially incomplete (missing a key process step identified in
Microsoft Learn docs): mark as `status: draft` in metadata, exclude from catalog. Ship nothing half-baked.

### Estimated Asset Count
12 cataloged assets. ~90 files.

### Risk
**MEDIUM** — Lower blast radius than Phase 2, but process errors (e.g., wrong fiscal close sequence)
cause business impact. Verification debt is the main risk.

---

## Phase 4 — Power Platform & Copilot Scale

### Goal
Deliver the ALM, governance, automation-risk, and Fabric/Power BI skills that complete the Power
Platform and Copilot Studio coverage.

### Verification Debt (do before building)
Fetch Microsoft Learn: Power Platform ALM pipelines, Copilot Studio agent governance, Power Automate
DLP and governance, Microsoft Fabric lakehouse and Power BI deployment pipelines.

### Work Items (4 skills + 4 companion agents)

| Skill ID | Agent ID |
|----------|----------|
| `power-platform-alm-pipelines` | `power-platform-alm-agent` |
| `copilot-studio-agent-governance-alm` | `copilot-studio-governance-agent` |
| `power-automate-automation-risk-review` | `power-automate-risk-agent` |
| `fabric-power-bi-business-insights-governance` | `fabric-powerbi-governance-agent` |

**Wave constraint:** 2 skill+agent pairs per PR. Two PRs.

### Exit Criteria
- `npm run validate` green
- Copilot Studio governance skill explicitly scopes allowed-tools to read-only by default (no
  deployment actions without explicit human approval declared in skill frontmatter)
- Fabric/Power BI skill verified against current Fabric GA feature set (not preview) via Microsoft Learn

### Rollback / Abort
If Copilot Studio API surface has changed since plan time (high probability given release cadence):
fetch current docs at build time and adjust scope. Do not ship a skill referencing deprecated APIs.

### Estimated Asset Count
8 cataloged assets. ~62 files.

### Risk
**MEDIUM-HIGH** — Copilot Studio and Fabric are rapidly evolving. The verification-debt step is
critical here. Preview features must be explicitly labeled as such in skill frontmatter.

---

## Phase 5 — Cross-Functional Protocols

### Goal
Deliver the 15 cross-functional protocol skills in `skills/cross-functional/` that span multiple
providers or M365/D365 boundary-crossing workflows.

### Verification Debt (do before building)
Cross-functional protocols often depend on Microsoft Graph API, Power Platform connectors, and
Azure integration services simultaneously. Fetch current docs for each integration surface used.

### Work Items
Build the 15 protocol skills. Protocol skills do NOT necessarily have 1:1 companion agents — they
may reference agents from Phases 1–4. Declare `companion_skills` in the relevant maestro agents
where applicable.

**Wave constraint:** 4 skills per PR (protocols are typically lighter than full skills). Four PRs.

**After last protocol PR:**
```bash
python3 tests/_generate_maestro_routing_fixtures.py
npm run validate
```
Verify `validate:maestro-routing` still passes with cross-provider routing for protocols.

### Exit Criteria
- `npm run validate` green including `validate:maestro-routing`
- Protocol routing validated: each protocol routes correctly when invoked from each maestro agent
- No cross-functional protocol references a product name or API not verified via Microsoft Learn

### Rollback / Abort
If a protocol's cross-system integration path is not documented in official Microsoft Learn: park
the protocol as draft. Do not invent integration patterns.

### Estimated Asset Count
15 skills + 0–5 protocol-specific agents. ~45–80 files.

### Risk
**MEDIUM** — Risk is primarily verification debt and routing complexity, not individual skill blast
radius.

---

## Phase 6 — Red-Team, Hardening & Final Readiness

### Goal
Systematic adversarial evaluation of all microsoft-provider assets. Remediate anything scoring <4/5.
Resolve all verification-debt flags. Produce final readiness verdict.

### Work Items

**Step 1 — Run full red-team eval**
Execute the eval framework defined in `05-red-team-and-acceptance.md`. Score every microsoft-provider
skill and agent on all defined dimensions. Record results in a structured artifact (not committed to
repo — ephemeral eval output).

**Step 2 — Triage findings**
- Score >=4/5 on all dimensions: asset passes, no action
- Score 3/5 on one dimension: create remediation issue, fix in this phase
- Score <3/5 on any dimension: agent/skill is REMOVED from catalog until rebuilt. No exceptions.

**Step 3 — Deduplicate and consolidate**
After 5 phases of parallel builds, there will be reference duplication and overlapping coverage.
Audit for:
- Skills referencing the same Microsoft Learn URL >3 times across the catalog (consolidate into a
  shared reference rule)
- Agents with overlapping `companion_skills` declarations (clarify routing)
- Harness adapter files with copy-paste content that should be in shared templates

**Step 4 — Final asset integrity and catalog sync**
```bash
npm run manifest:write:all
python3 tests/validate-asset-integrity.py --write
git add catalog/asset-integrity.json
npm run validate
```

**Step 5 — Verify all `official_docs` URLs are live**
Run a URL health check on every `official_docs` entry in microsoft-provider agent metadata.json and
skill SKILL.md files. Any dead link blocks the readiness verdict.

**Step 6 — Final readiness verdict**
Document in a PR description (not a committed .md file):
- Total microsoft-provider assets shipped
- Red-team pass rate
- Any assets parked in `_draft/`
- Outstanding verification debt (should be zero)

### Exit Criteria
- `npm run validate` green (all 19+ gates)
- Red-team scorecard: no microsoft-provider asset scores <4/5 on any dimension
- All `official_docs` URLs return HTTP 200
- Zero open verification-debt flags
- `catalog/asset-integrity.json` reflects exact current state
- No manual version edits in `package.json`

### Rollback / Abort
If >20% of assets score <4/5 after one remediation round: halt Phase 6, do not merge, escalate
scope to a dedicated hardening sprint before returning here.

### Risk
**MEDIUM** — The red-team reveals what phases 1–5 missed. Budget for a 20% remediation rate on
first pass. The real risk is shipping without running this phase.

---

## Phase Summary Table

| Phase | Goal | Assets | Gate |
|-------|------|--------|------|
| 0 | Provider registration | 2 (smoke test) | validate green + kiro-powers |
| 1 | Maestro + rules | 10 + 3 rules | validate + maestro-routing |
| 2 | Security/identity/governance | 12 | validate + red-team >=4/5 |
| 3 | Business process | 12 | validate + adversarial routing |
| 4 | Power Platform + Copilot Scale | 8 | validate + preview labeling |
| 5 | Cross-functional protocols | 15–20 | validate + protocol routing |
| 6 | Red-team + hardening | 0 net new | validate + scorecard |
| **Total** | | **~59–64 cataloged assets** | |

---

## Sequencing Constraints

1. **Phase 0 blocks everything.** No PR from any other phase merges until Phase 0 is green.
2. **Phase 1 blocks Phase 2–5 maestro routing.** The maestro agents must exist before skills can
   declare routing to them.
3. **Phase 2 blocks Phase 3–4 where SoD/identity patterns are reused.** Build security patterns
   once, reference everywhere.
4. **Phases 3, 4, 5 can run in parallel branches** once Phase 2 is merged — but each branch must
   rebase on main before opening its PR to avoid asset-integrity conflicts.
5. **Phase 6 runs last, on main, after all prior phases are merged.**

---

## Verification-Debt Policy

Any claim about a Microsoft product, API, permission scope, process step, or certification requirement
that is written into a skill or agent file MUST be backed by a fetched Microsoft Learn URL at the time
of writing. If the URL cannot be fetched or the content does not match the claim:

- The claim is removed.
- A `TODO(verify):` comment is left in the draft.
- The asset is not merged until the TODO is resolved.

This policy applies to ALL phases. It is not optional. Unverified claims in security and governance
skills are a liability, not a feature.

---

## Next

See [`05-red-team-and-acceptance.md`](./../m365-d365/05-red-team-and-acceptance.md) for the adversarial
evaluation framework, scoring rubric, and acceptance criteria applied at Phase 6 (and progressively
from Phase 2 onward).
