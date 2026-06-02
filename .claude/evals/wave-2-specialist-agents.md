# EVAL DEFINITION: wave-2-specialist-agents
## Finance & Accounting Platform — Wave 2 Specialists

Strategy anchors: Section C (product horizon "Now"), Section D (accounting transformation fabric),
Section E (jurisdiction matrix), Section H (adversarial tests).

---

## Agents in Scope

| Agent ID | Domain |
|---|---|
| `accounting-consolidation-intercompany-advisor-agent` | ASC 810/IFRS 10 consolidation, IC elimination |
| `accounting-fx-translation-advisor-agent` | ASC 830/IAS 21 FX translation/remeasurement |
| `finance-transfer-pricing-pillar-two-advisor-agent` | TP arm's length, BEPS, CbCR, Pillar Two GloBE |

---

## Capability Evals (per agent)

### CE-1: Structure completeness
- [ ] AGENT.md, PERMISSIONS.md, metadata.json present
- [ ] All 7 harnesses exist (claude-code, codex, copilot, cursor, gemini, kiro-ide, kiro-cli)
- [ ] Companion skill: SKILL.md, metadata.json, README.md present

### CE-2: Metadata correctness
- [ ] `execution_tier: "read-only-runtime"`
- [ ] `lifecycle: "experimental"`
- [ ] `companion_skills` array populated
- [ ] `harness_variants` lists all 7 keys
- [ ] `official_docs` non-empty, public URLs only

### CE-3: Security posture
- [ ] PERMISSIONS.md denies write to GL/ERP/ledger
- [ ] Never accepts raw financial data (trial balances, tax returns, PII)
- [ ] No MNPI / investment advice / fairness opinions (finance agents)
- [ ] Advisory-only language in AGENT.md

### CE-4: Domain depth (model-graded, spot-check)
- [ ] Covers ≥ 3 jurisdictions with fact-specific delta per standard
- [ ] References correct standard codes (ASC §, IAS §, IFRS §, OECD §)
- [ ] SKILL.md has ≥ 6 parts with substantive framework content
- [ ] GAAP vs IFRS divergence explicitly called out where it exists

### CE-5: SKILL.md frontmatter schema
- [ ] `allowed-tools: Skill Read WebFetch Glob` inline (not YAML list)
- [ ] `category: finance` (schema enum constraint)
- [ ] `name:`, `description:`, `metadata:` block present

---

## Regression Evals (catalog integration)

### RE-1: Catalog append
- [ ] All 3 agents appear in `catalog/agents.json` (total ≥ 438)
- [ ] All 3 skills appear in `catalog/skills.json` (total ≥ 416)
- [ ] `accounting-finance-advisor` install role updated (≥ 12 agents, ≥ 12 skills)

### RE-2: Maestro routing
- [ ] All 3 agents appear in their maestro routing table rows
- [ ] No existing routing rows removed or altered

### RE-3: Manifest/integrity sync
- [ ] `npm run validate` exits 0 (all 20 gates pass)
- [ ] `plugin-manifest:check` in sync
- [ ] `manifest:check` in sync
- [ ] `readme-counts` reflects updated totals

---

## Graders

### Code grader (deterministic — primary gate)
```bash
# File completeness
for a in accounting-consolidation-intercompany-advisor-agent accounting-fx-translation-advisor-agent; do
  for f in AGENT.md PERMISSIONS.md metadata.json; do
    test -f agents/accounting/$a/$f || echo "FAIL: missing $a/$f"
  done
  for h in claude-code.agent.md codex.toml copilot.agent.md cursor.agent.md gemini.agent.md kiro-cli.agent.json kiro-ide.agent.md; do
    test -f agents/accounting/$a/harnesses/$h || echo "FAIL: missing $a/harnesses/$h"
  done
done
# Validation gate (the definitive check)
npm run validate
```

### Model grader (domain depth — spot-check)
Sample question: "Does the consolidation SKILL.md explain the difference between ASC 810 VIE consolidation
triggers and IFRS 10 de-facto control with specific numeric/criteria differences?"
Pass: SKILL.md covers both paths with specific threshold/criteria differences cited.
Fail: Generic "ASC 810 and IFRS 10 differ" with no substance.

### Success metrics
- Capability evals: pass@3 ≥ 90%
- Regression evals: pass^3 = 100% (npm run validate must never break)

---

## Run Log

| Run | Date | Capability | Regression | npm validate | Status |
|---|---|---|---|---|---|
| 1 | 2026-06-02 | pending | pending | pending | IN PROGRESS |
