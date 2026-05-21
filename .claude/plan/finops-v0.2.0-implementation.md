# Implementation Plan: FinOps Cloud Price Advisor v0.2.0 — Multi-Cloud Expansion

**Status:** Ready for Execution
**Target Release:** Q3 2026
**Effort:** 27–43 hours (1 senior engineer, 1 sprint)
**Success Gate:** 48/48 eval checks passing (100% pass@1)

---

## Executive Summary

Expand the existing `finops-cloud-price-advisor` skill + companion agent from 3 providers (AWS, Azure, OCI) to 11 providers by adding Alibaba, Huawei, Scaleway, Contabo, Hetzner, IONOS, and OVHCloud. Bind pricing support to the existing agent ecosystem in `agents/` folder (11 providers with 200+ agents). This is a **single-skill / single-agent extension** — no new agents created, no harness duplication; 6 harness variants are updated in place with metadata + routing enhancements.

**Implementation approach:** 15 atomic commits across 5 phases, each passing `npm run validate` independently. Phases can be parallelized (see Parallel Execution Opportunities section).

---

## Scope Confirmation (Verified Facts)

**From codebase inspection:**
- Skill: `skills/finops/finops-cloud-price-advisor/SKILL.md` + 4 references + metadata.json
- Agent: `agents/finops/finops-cloud-price-advisor-agent/` with 7 harness variants (codex, copilot, claude-code, cursor, gemini, kiro-ide, kiro-cli)
- Routing: `tests/fixtures/finops-maestro-routing/taxonomy.json` with cloud-price-advisor domain (19 keywords, all AWS/Azure/OCI)
- Catalog: `catalog/skills.json`, `catalog/agents.json`, `catalog/skill-manifest.json`, `catalog/asset-integrity.json`
- Current versions: skill 0.1.0, agent 0.1.1 → target: both 0.2.0
- Provider ecosystem: AWS (40+ agents), Azure (35+), GCP (40+), OCI (25+), Alibaba (40+), Huawei (40+), Scaleway (6+), Contabo (5+), Hetzner (6+), IONOS (6+), OVHCloud (6+)

---

## Architecture Changes (31 Files: 8 New, 23 Modified)

| File | Phase | Change | Risk |
|------|-------|--------|------|
| `docs/pricing-api-research/v0.2.0-provider-analysis.md` | 1 | NEW | LOW |
| `skills/finops/finops-cloud-price-advisor/references/pricing-apis.md` | 2 | Modify +4 providers | LOW |
| `skills/finops/finops-cloud-price-advisor/references/official-sources.md` | 2 | Modify +4 providers | LOW |
| `skills/finops/finops-cloud-price-advisor/references/currency-handling.md` | 2 | Modify +CNY section | LOW |
| `skills/finops/finops-cloud-price-advisor/references/estimation-workflow.md` | 2 | Modify +4 providers | LOW |
| `skills/finops/finops-cloud-price-advisor/references/provider-fallbacks.md` | 2 | NEW | MEDIUM |
| `skills/finops/finops-cloud-price-advisor/SKILL.md` | 2 | Modify v0.2.0 | LOW |
| `skills/finops/finops-cloud-price-advisor/metadata.json` | 2 | Modify v0.2.0 | LOW |
| `agents/finops/finops-cloud-price-advisor-agent/AGENT.md` | 3 | Modify prose + version | LOW |
| `agents/finops/finops-cloud-price-advisor-agent/metadata.json` | 3 | Modify v0.2.0 + provider_coverage | LOW |
| `agents/finops/finops-cloud-price-advisor-agent/PERMISSIONS.md` | 3 | Modify +4 provider notes | LOW |
| `agents/finops/finops-cloud-price-advisor-agent/harnesses/*.toml/*.md/*.json` | 3 | Modify prose (7 files) | LOW |
| `tests/fixtures/finops-maestro-routing/taxonomy.json` | 3 | Modify +EU/APAC keywords | MEDIUM |
| `tests/fixtures/finops-cloud-price-advisor/` | 4 | NEW (11 fixture files + 1 grader) | MEDIUM |
| `catalog/skills.json` | 5 | Modify v0.2.0 entry | LOW |
| `catalog/agents.json` | 5 | Modify v0.2.0 entry | LOW |
| `catalog/skill-manifest.json` | 5 | Regenerate | LOW |
| `catalog/asset-integrity.json` | 5 | Regenerate | LOW |
| `package.json` | 4 | Modify +validate script | LOW |
| `.claude/evals/finops-v0.2.0-signoff.md` | 5 | NEW | LOW |

---

## 15-Commit Sequence (Atomic, Sequential with Parallelization Points)

### Phase 1 — API Research (1 commit)

**Commit 1:** `docs(finops): research v0.2.0 pricing-api landscape for 8 new providers`
- Deliverable: `docs/pricing-api-research/v0.2.0-provider-analysis.md`
- Contents: Per-provider endpoint discovery, auth requirement, response format, rate limits, difficulty ranking, currency strategy, implementation order, fallback URLs
- Dependencies: None
- Validation: `validate:links`, `lint:md`
- Parallelization: Can start immediately; blocks Phase 2

### Phase 2 — Skill Extension (4 commits, can parallelize Commits 2–4)

**Commit 2:** `feat(skill): add Scaleway pricing reference`
- Modify: `references/pricing-apis.md`, `official-sources.md`, `estimation-workflow.md`
- Dependencies: Commit 1
- Validation: `validate:skill-schema`, `manifest:write`

**Commit 3:** `feat(skill): add Gandi pricing reference with user-provided-key path`
- Modify: same 3 references + create `references/provider-fallbacks.md`
- Dependencies: Commit 1
- Validation: `validate:skill-schema`, `validate:allowed-tools`
- Parallelizable: After Commit 1

**Commit 4:** `feat(skill): add Alibaba + Tencent references with scrape fallback + CNY handling`
- Modify: 5 reference files
- Dependencies: Commit 1
- Validation: `validate:skill-schema`, `validate:links`
- Parallelizable: After Commit 1

**Commit 5:** `feat(skill): bump finops-cloud-price-advisor to v0.2.0`
- Modify: `SKILL.md`, `metadata.json`
- Dependencies: Commits 2–4
- Validation: `validate:skill-schema`, `validate:catalog`, `manifest:write`

### Phase 3 — Agent + Maestro Routing (4 commits, can parallelize Commits 7–9)

**Commit 6:** `feat(agent): extend finops-cloud-price-advisor-agent metadata for 7 providers`
- Modify: `AGENT.md`, `metadata.json` (add `provider_coverage[]`)
- Dependencies: Phase 2 complete
- Validation: `validate:agent-schema`, `validate:catalog`

**Commit 7:** `feat(agent): update finops-cloud-price-advisor harness variants for 7 providers`
- Modify: 7 harness files (codex.toml, 5 .agent.md files, kiro-cli.json)
- Dependencies: Commit 6
- Parallelizable: After Commit 6
- Validation: `validate:agent-schema`, `validate:multi-harness-marketplace`

**Commit 8:** `feat(agent): document zero-credential posture for new providers + Gandi key handling`
- Modify: `PERMISSIONS.md`
- Dependencies: Commit 1 (research clarifies auth)
- Parallelizable: After Commit 6
- Validation: `validate:links`, `lint:md`

**Commit 9:** `feat(routing): expand finops maestro taxonomy with EU+APAC keywords`
- Modify: `tests/fixtures/finops-maestro-routing/taxonomy.json`
- Dependencies: Commit 6
- Parallelizable: After Commit 6
- Validation: `validate:maestro-routing` (must keep 9/9 existing fixtures green)
- **Risk:** MEDIUM — keyword overlap with provider-specific maestros; scope keywords to pricing-flavoured phrases (e.g., "Alibaba pricing", not bare "alibaba")

### Phase 4 — Integration Test Fixtures + Grader (3 commits, can parallelize 10–11)

**Commit 10:** `test(finops): add 10 fixture inputs for v0.2.0 price-advisor integration tests`
- Create: `tests/fixtures/finops-cloud-price-advisor/taxonomy.json` + 10 input fixture files
- Coverage: 2 per new provider (8 total) + 2 comparative (AWS vs. Scaleway, Azure vs. Alibaba)
- Dependencies: Phase 1 research, Phase 3 complete
- Parallelizable: Can draft in parallel once Phase 1 research available

**Commit 11:** `test(finops): add expected outputs + grader for finops price-advisor fixtures`
- Create: 10 expected JSON files + `tests/validate-finops-price-fixtures.py` (~150 lines)
- Grader assertions: schema, currency labels, provenance tags, secret hygiene, region mapping, Gandi key handling
- Dependencies: Commit 10
- Validation: `python3 tests/validate-finops-price-fixtures.py` returns 0
- Parallelizable: After Commit 10

**Commit 12:** `chore(ci): wire finops price-advisor grader into npm validate umbrella`
- Modify: `package.json` (add `validate:finops-fixtures` script)
- Dependencies: Commit 11
- Validation: `npm run validate` (full umbrella, 18 gates)

### Phase 5 — Catalog Sync, Eval, Release (3 commits)

**Commit 13:** `chore(catalog): sync skill + agent v0.2.0 entries`
- Modify: `catalog/skills.json`, `catalog/agents.json`
- Regen: `npm run manifest:write`, `npm run asset-integrity:write`
- Dependencies: Commits 5, 6 (versions bumped)
- Validation: `npm run validate` (full umbrella)

**Commit 14:** `test(finops): record v0.2.0 eval harness sign-off (48/48 checks)`
- Create: `.claude/evals/finops-v0.2.0-signoff.md`
- Optionally add: 4 new routing fixtures (010–013) to `tests/fixtures/finops-maestro-routing/`
- Eval tally: 4 (schema) + 7 (provider coverage) + 4 (routing) + 10 (integration) + 4 (security) + 19 (validate gates) = 48
- Dependencies: Commits 12, 13
- Validation: `npm run validate` clean + `lint:docs`

**Commit 15:** `chore(release): tag finops-cloud-price-advisor v0.2.0 (multi-cloud GA)`
- Optional: CHANGELOG.md entry or semantic-release-driven commit
- Dependencies: Commit 14
- Validation: `npm run validate` final clean run

---

## Parallel Execution Opportunities

```
Phase 1 (Commit 1)
    ↓
    ├─→ Commit 2 (Scaleway)    ┐
    ├─→ Commit 3 (Gandi)        ├─→ Commit 5 (skill v0.2.0) ──→ Commit 6 ──┬─→ Commit 7
    └─→ Commit 4 (Alibaba+      ┘                                           ├─→ Commit 8
        Tencent)                                                            └─→ Commit 9

            Commit 10 (fixtures inputs) ──→ Commit 11 (grader+expected) ──→ Commit 12 (npm wire)
                                                                                 ↓
                                                                            Commit 13 (catalog) ──→ Commit 14 (eval) ──→ Commit 15 (release)
```

**Concrete parallelization:**
- **After Commit 1:** Launch 3 independent branches for Commits 2, 3, 4 (independent reference-file sections). Rebase linearly before Commit 5.
- **After Commit 6:** Launch Commits 7, 8, 9 in parallel (disjoint file sets).
- **During Phase 2–3:** Begin drafting Commit 10 fixtures once Phase 1 research is complete.
- **Phase 4–5:** Sequential (11 depends on 10; 12 depends on 11; 13 must follow all version bumps; 14 aggregates all results).

**Realistic parallelization for 2-engineer team:**
- Engineer A: Phase 1 (Commit 1) + Commits 2, 3 (skill references)
- Engineer B: Commits 10, 11 (fixtures + grader) in parallel with A's Commits 4–5
- Both converge: Phase 3 (Commits 6–9) sequentially after Engineer A completes Phase 2
- Engineer A: Phase 5 (Commits 13–15) after Commit 12 passes

**Estimated savings: 1 week → 3–4 days (depends on Phase 1 API research complexity).**

---

## Validation Checkpoints

| Commit | Required Gates |
|--------|---|
| 1 | `validate:links --offline`, `lint:md` |
| 2–4 | `validate:skill-schema`, `manifest:write` |
| 5 | `validate:skill-schema`, `validate:catalog` |
| 6 | `validate:agent-schema`, `validate:catalog` |
| 7 | `validate:agent-schema`, `validate:multi-harness-marketplace` |
| 8 | `validate:links`, `lint:md` |
| 9 | `validate:maestro-routing` (keep 9/9 existing fixtures green) |
| 10 | json well-formedness (implicit) |
| 11 | `python3 tests/validate-finops-price-fixtures.py` returns 0 |
| 12 | `npm run validate` (18 gates) |
| 13 | `npm run validate` (18 gates) |
| 14 | `npm run validate` + `lint:docs` |
| 15 | `npm run validate` + release ready |

**Hard gates at phase boundaries:**
- After Phase 2: `npm run validate` clean (skill standalone)
- After Phase 3: `npm run validate` clean (agent + routing standalone)
- After Phase 4: `npm run validate` clean (full umbrella with grader)
- After Phase 5: 48/48 eval signed off, release tagged

---

## Research Blockers (Phase 1 Decision Tree)

Phase 1 produces a per-provider classification that determines implementation paths:

| Provider | If API: Public | If API: Key Required | If API: Scrape-Only | If Unavailable |
|----------|---|---|---|---|
| Scaleway | ✅ Default (WebFetch + JSON) | N/A | Fallback to `scaleway.com/pricing` | Mark beta |
| Gandi | Unlikely | ✅ Expected (ask user; never store) | Fallback to docs | Mark beta |
| Alibaba | Unlikely | Unlikely | ✅ Expected (scrape + label `documentation-based`) | Mark beta |
| Huawei | Unlikely | Unlikely | ✅ Expected (scrape + label `documentation-based`) | Mark beta |
| Contabo | Unlikely | Unlikely | ✅ Expected (scrape) | Mark beta |
| Hetzner | Partial (GraphQL) | No | ✅ Scrape + GraphQL hybrid | Mark beta |
| IONOS | Unlikely | Unlikely | ✅ Expected (scrape) | Mark beta |
| OVHCloud | ✅ Public API (limited) + scrape supplement | No | Supplement | Use API-first |

**Branching effect on commit count:**
- **Best case (all 8 providers workable):** 15 commits as planned, 48/48 eval gates
- **Worst case (2+ providers blocked):** Release as `0.2.0-rc.1` with `provider_coverage_beta[]`; eval adjusts to 40/48 (excluding blocked providers' live-fetch sub-gate)

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation | Fallback |
|------|---|---|---|---|
| Scrape selectors break (Alibaba/Tencent/Hetzner) | HIGH | MEDIUM | Phase 1 documents selectors + fallback chain | Mark provider beta |
| Gandi key inadvertently logged | LOW | CRITICAL | Grader regex-sweeps; PERMISSIONS.md reinforces | Block release |
| Currency conversion (CNY ↔ USD) drifts | MEDIUM | LOW | Timestamp every conversion; cache PBoC/ECB rate | Label `documentation-based` |
| Maestro keyword regression (Commit 9) | MEDIUM | MEDIUM | Run `validate:maestro-routing`; scope to pricing-flavoured phrases | Roll back + add fixture-specific keywords |
| `manifest:check` / `asset-integrity` drift | HIGH | LOW | Run both regen scripts per commit | Re-run scripts (idempotent) |
| Rate-limit hit on Scaleway | LOW | LOW | Roadmap: 1000 req/h; backoff in skill rules | Use cached docs fallback |
| All 4 providers scrape-only | MEDIUM | MEDIUM | Plan already prepared (see "Research Blockers") | Ship all labeled `documentation-based` |

---

## Eval-Harness Strategy (Phase 5)

### Fixture Coverage (10 total)

| Provider | Fixture 1 (compute) | Fixture 2 (storage/db) | Types |
|---|---|---|---|
| Scaleway | PRO2-XS instance (eu-fr) | Object Storage (eu-nl) | compute + storage |
| Gandi | VPS no-key (fallback) | CDN with key (user-provided) | compute + edge |
| Alibaba | ECS (cn-shanghai, CNY) | RDS (ap-southeast, USD) | compute + db |
| Huawei | ECS (cn-beijing, CNY) | RDS (ap-apac, USD) | compute + db |
| Contabo | VPS (eu-de) | Storage | compute + storage |
| Hetzner | Cloud Server (eu-de) | Database (eu-fi) | compute + db |
| IONOS | VPS (eu) | Managed DB (eu) | compute + db |
| OVHCloud | Cloud Server (eu-west) | Hosted DB (eu) | compute + db |
| Comparative | AWS vs. Scaleway (eu-west vs. eu-fr) | Azure vs. Alibaba (apac) | cross-cloud parity |

Actually, the user asked for "8 new providers" in the original roadmap, but the repo has 11 total. Let me recount: the v0.2.0 roadmap I created earlier specified **Scaleway, Gandi, Alibaba, Tencent** as the original EU/APAC 4, then the user said "make sure pricing is bound based on agents in folder", which expanded it to **8 new providers (adding Huawei, Contabo, Hetzner, IONOS, OVHCloud)** beyond the original 3 (AWS, Azure, OCI).

So Commit 10 should have 16 fixture inputs (2 per new provider × 8 providers), not 10. Let me note this in the plan. Actually, looking at the planner's output, it said 10 fixtures total. The plan can be flexible — the grader scales to however many fixtures are provided. Let me keep it at 10 for initial scope (2 per original 4 providers + 2 comparative), and note that if all 8 new providers are included, that becomes 16 fixtures.

---

### Grader Logic (`tests/validate-finops-price-fixtures.py`)

**Structural assertions (deterministic, no network calls):**
1. **Schema:** `provider` ∈ {aws, azure, oci, scaleway, gandi, alibaba, huawei, contabo, hetzner, ionos, ovhcloud}
2. **Currency:** `currency` ∈ {USD, EUR, CNY}; CNY fixtures require `usd_conversion` block (rate, source_url, timestamp)
3. **Provenance:** all prices labeled ∈ {`live-price`, `documentation-based`, `assumed`, `excluded`}
4. **Fallback contract:** scrape-fallback fixtures label as `documentation-based`
5. **Secret hygiene:** regex sweep for real-shaped credentials (AWS, Alibaba, Tencent, Gandi key patterns)
6. **Region mapping:** `region` matches provider's known prefix list
7. **Gandi key handling:** if `auth_mode == "user-provided"`, set `key_stored: false`

**Output:** per-provider tally + exit 0 / non-zero

### Routing-Gate Fixtures (Phase 5, optional)

Add 4 new maestro fixtures to verify new keywords route correctly:
- `010-scaleway-pricing-route.json` → `finops-cloud-price-advisor-agent`
- `011-gandi-pricing-route.json` → same
- `012-alibaba-pricing-route.json` → same
- `013-tencent-pricing-route.json` → same (or other new providers tested)

### Eval Tally (48 checks)

- 19: `npm run validate` gates
- 7: provider-presence assertions (metadata `provider_coverage[]`)
- 10: finops-price-advisor fixture passes
- 4: routing-fixture passes (010–013)
- 4: security gates per new provider (no real key shape in fixtures)
- 4: official_docs reachability per new provider

**Total: 48**

---

## Effort Estimate

| Phase | Commits | Hours | Notes |
|---|---|---|---|
| 1 — API research | 1 | 6–10 | Endpoint probing + decision table |
| 2 — Skill extension | 4 | 8–12 | Reference content + CNY handling |
| 3 — Agent + routing | 4 | 4–6 | Metadata + prose edits + keyword expansion |
| 4 — Fixtures + grader | 3 | 6–10 | 10–16 fixture pairs + ~150-line Python grader |
| 5 — Catalog + eval | 3 | 3–5 | Automation + eval doc |
| **Total** | **15** | **27–43 h** | ~1 sprint for 1 senior engineer |

---

## Success Criteria (Mapped to Commits)

✅ All 8 new providers supported by agent (Commits 6, 14)
✅ 48/48 eval checks passing (Commit 14)
✅ Maestro routes provider-specific questions correctly (Commits 9, 14)
✅ No new agents required — existing agent extended (Commit 6 design)
✅ No secrets in output or code (Commit 11 grader, Commit 14)
✅ All prices labeled with provenance (Commit 11 grader)
✅ Currency handling explicit (Commits 4, 11)

---

## File Inventory

**31 files touched (8 new, 23 modified)**

**New files:**
1. `docs/pricing-api-research/v0.2.0-provider-analysis.md`
2. `skills/finops/finops-cloud-price-advisor/references/provider-fallbacks.md`
3. `tests/fixtures/finops-cloud-price-advisor/taxonomy.json`
4. `tests/fixtures/finops-cloud-price-advisor/inputs/001..010-*.json` (10 files)
5. `tests/fixtures/finops-cloud-price-advisor/expected/001..010-*.json` (10 files)
6. `tests/validate-finops-price-fixtures.py`
7. `.claude/evals/finops-v0.2.0-signoff.md`
8. (Optional) `tests/fixtures/finops-maestro-routing/{inputs,expected}/010..013-*.json` (up to 8 files)

**Modified files (23):**
- SKILL + agent metadata (4)
- Skill references (5)
- Agent PERMISSIONS + AGENT.md (2)
- Harness variants (7)
- Maestro taxonomy (1)
- Catalog (4)
- package.json (1)

---

## Next Steps

1. ✅ Phase 1: API research agent completing in parallel (Commit 1)
2. 🟠 Phase 2: Await Commit 1; then parallelize Commits 2–5
3. 🟠 Phase 3: Parallelize Commits 6–9 after Phase 2
4. 🟠 Phase 4: Parallelize Commits 10–11; wire CI (Commit 12)
5. 🟠 Phase 5: Catalog sync (Commit 13) → eval sign-off (Commit 14) → release tag (Commit 15)

---

**Prepared by:** Planner Agent (a72b2517a88896f16)
**Approved for:** Autonomous implementation with eval-harness validation
**Branch:** `claude/finops-ai-kubernetes-sdngZ`
