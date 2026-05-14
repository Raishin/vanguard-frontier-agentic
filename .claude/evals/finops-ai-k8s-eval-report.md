EVAL REPORT: finops-ai-kubernetes-sdngZ
=======================================
Date: 2026-05-13
Branch: claude/finops-ai-kubernetes-sdngZ
PR: #25
Methodology: Eval-Driven Development (EDD) — 5 parallel sonnet eval teams

Capability Evals
----------------

[T1] schema-contract-conformance
pass@1: 9/10 assets conformant
Defect found: finops-cloud-price-advisor-agent/metadata.json missing companion_skills, execution_tier, lifecycle
Action taken: Fields added; asset-integrity.json regenerated; all 17 validation gates re-confirmed PASS
Final result: 10/10 — PASS (post-fix)

[T2] maestro-routing-correctness
Grader: tests/validate-maestro-routing.py — exit 0
Fixture results: 9/9 correct
  - 3 happy-path (single specialist) — PASS
  - 2 parallel (parallel_threshold=0.6) — PASS
  - 4 adversarial (2 unclassified, 1 instruction-injection routed, 1 credential-bait clean) — PASS
Adversarial integrity: XKIA prefix (not AKIA) confirmed clean — no real AWS key pattern
Taxonomy: 3 domains x (24/25/19 keywords), parallel_threshold=0.6, live_guards=[]
Final result: 9/9 — PASS

[T3] finops-skills-quality (model grader)
All 6 skills evaluated on clarity, FOCUS compliance, least-privilege, security posture, provenance labeling:
  finops-maestro:                 5/5 — PASS
  fetch-foundation-model-pricing: 5/5 — PASS
  kubernetes-allocation-report:   5/5 — PASS
  rightsize-recommendation:       5/5 — PASS (WebFetch absent, headroom formula, Karpenter criteria all verified)
  carbon-cost-pair:               5/5 — PASS (Scope 2 market-based, CSRD/SEC, kgCO2e all verified)
  focus-spec-normalizer:          5/5 — PASS (WebFetch absent, all 4 vendor adapters, FOCUS v1.2 columns verified)
Final result: 6/6 — SHIP

Regression Evals
----------------

[T4] catalog-and-validation-gates
npm run validate: 17/17 gates PASS
Catalog coverage: 4/4 agents + 6/6 skills in catalog files
Install roles: 4/4 agents in cloud-finops-analyst role
Skill manifest: 6/6 skills in catalog/skill-manifest.json
Final result: PASS

[T5] security-posture
Secrets scan: CLEAN
PERMISSIONS.md: 4/4 agents have PERMISSIONS.md with explicit no-Bash/no-Write/no-creds
live_guards: [] — no mutating specialists wired (correct for v1)
Copilot adapter k8s-rightsizer: execute/runInTerminal ABSENT — PASS
Credential refusal in AGENT.md: 4/4 present
Maestro no-auto-mutation: enforced with handoff packet + explicit written confirmation gate
Final result: 6/6 checks — PASS

Metrics
-------

pass@1 (first-attempt correctness per eval):
  Schema contract:    9/10 = 90%
  Maestro routing:    9/9  = 100%
  Skills quality:     6/6  = 100%
  Catalog regression: 17/17 = 100%
  Security posture:   6/6  = 100%
  Overall pass@1:     47/48 = 97.9%

Post-fix pass@1: 48/48 = 100%

Defects Found and Fixed
-----------------------

| # | Severity | Location | Finding | Resolution |
|---|---|---|---|---|
| 1 | MEDIUM | finops-cloud-price-advisor-agent/metadata.json | Missing companion_skills, execution_tier, lifecycle fields | Added all 3 fields; asset-integrity.json regenerated |

Cross-Cutting Observations
--------------------------

1. Security posture consistently strong — unconditional credential refusal with per-format enumeration in companion docs
2. Provenance labeling (live-price / documentation-based / assumed / excluded) first-class in every data-producing skill
3. WebFetch boundary correctly drawn: rightsize-recommendation and focus-spec-normalizer correctly exclude it
4. FOCUS v1.2 column coverage accurate; gaps documented with null handling and resolution notes
5. Maestro live_guards=[] correct for v1 — no mutating specialists wired

Status: SHIP IT

All 17 validation gates pass, all 5 eval domains pass, 1 defect found and fixed in the same session.
