# EVAL DEFINITION: wave-4-operational-accounting
## Finance & Accounting Platform — Wave 4: Operational Pain Points

Agents: payroll, procure-to-pay, fixed-assets/impairment, equity-compensation, business-combinations

---

## Capability Evals (per agent)

### CE-1: Structure completeness
- [ ] AGENT.md, PERMISSIONS.md, metadata.json present
- [ ] All 7 harnesses (claude-code, codex, copilot, cursor, gemini, kiro-ide, kiro-cli)
- [ ] Companion skill: SKILL.md, metadata.json, README.md

### CE-2: Metadata correctness
- [ ] execution_tier: "read-only-runtime", lifecycle: "experimental"
- [ ] companion_skills populated, harness_variants 7 keys
- [ ] official_docs non-empty, public URLs only

### CE-3: Security posture
- [ ] PERMISSIONS.md denies GL/ERP writes
- [ ] Never accepts raw payroll data / PII / employee wages / SSNs (payroll agent)
- [ ] Advisory-only language throughout

### CE-4: Domain depth
- [ ] ≥ 3 jurisdictions covered with jurisdiction-specific deltas
- [ ] Correct standard codes (ASC §, IAS §, IFRS §)
- [ ] GAAP vs IFRS divergence explicitly called out

### CE-5: SKILL.md frontmatter
- [ ] allowed-tools: Skill Read WebFetch Glob (inline)
- [ ] category: finance
- [ ] name/description/metadata block present

## Regression Evals
- [ ] npm run validate exit 0 (all 20 gates)
- [ ] Catalog totals correct post-integration
- [ ] Maestro routing updated

## Run Log

| Run | Date | Capability | Regression | npm validate | Status |
|---|---|---|---|---|---|
| 1 | 2026-06-02 | 25/25 CE pass@1 | 5/5 RE pass^1 | EXIT=0 (20 gates) | PASS |
