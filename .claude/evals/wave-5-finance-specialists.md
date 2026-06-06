# EVAL DEFINITION: wave-5-finance-specialists
## Finance Platform — Wave 5: Missing Finance Pain Points

Agents: FP&A forecasting/budgeting, debt & capital structure, working capital management

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
- [ ] provider: "finance"

### CE-3: Security posture
- [ ] PERMISSIONS.md denies write to GL/ERP/treasury/trading systems
- [ ] Never accepts MNPI, live market data for execution, bank credentials, actual forecasts with confidential figures
- [ ] Advisory/educational only — not investment advice or a fairness opinion

### CE-4: Domain depth
- [ ] ≥ 3 jurisdictions or framework dimensions with specific deltas
- [ ] Correct standard/framework references (ASC §, IFRS §, Basel, covenant conventions)
- [ ] ≥ 6 substantive parts in SKILL.md

### CE-5: SKILL.md frontmatter
- [ ] allowed-tools: Skill Read WebFetch Glob (inline)
- [ ] category: finance
- [ ] name/description/metadata block present

## Regression Evals
- [ ] npm run validate exit 0 (all 20 gates)
- [ ] Catalog totals correct post-integration (448 agents, 426 skills)
- [ ] finance-maestro routing updated with all 3

## Run Log

| Run | Date | Capability | Regression | npm validate | Status |
|---|---|---|---|---|---|
| 1 | 2026-06-03 | 39/39 CE pass@1 | 5/5 RE pass^1 | EXIT=0 (20 gates) | PASS |
