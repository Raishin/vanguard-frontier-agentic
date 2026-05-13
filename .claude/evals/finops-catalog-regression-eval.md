[REGRESSION EVAL: catalog-and-validation-gates]
Date: 2026-05-13

Grader note: Sub-agent lookup used incorrect key (agents.agents instead of direct array access).
Corrected results confirmed via direct node -e verification.

Validation gates (npm run validate exits 0):
  validate:catalog: PASS — 673 catalog entries, no secrets
  validate:aws: PASS — 47 AWS skills validated
  manifest:check: PASS — 335 skill entries
  validate:allowed-tools: PASS — 335 skills
  validate:skill-schema: PASS — 335 skills
  validate:agent-schema: PASS — 334 agents
  validate:links: PASS — 1143 URLs (offline)
  validate:asset-integrity: PASS
  validate:mcp-trust-matrix: PASS
  validate:no-lifecycle-scripts: PASS
  validate:promotion-gatekeeper: PASS — 11 fixtures
  validate:install-coverage: PASS
  validate:maestro-routing: PASS — 366 scenarios across 15 maestros; FinOps 9/9
  validate:plugin-manifest: PASS — 334 claude-code agents, v1.8.0
  validate:kiro-powers: PASS
  validate:multi-harness-marketplace: PASS
  validate:codex-marketplace: PASS
  Overall: 17/17 PASS

Catalog coverage (verified via direct array lookup):
  finops-maestro-agent in agents.json: PRESENT (v0.1.0)
  finops-ai-economist-agent in agents.json: PRESENT (v0.1.0)
  finops-kubernetes-rightsizer-agent in agents.json: PRESENT (v0.1.0)
  finops-cloud-price-advisor-agent in agents.json: PRESENT (v0.1.0)
  finops-maestro in skills.json: PRESENT (v0.1.0)
  fetch-foundation-model-pricing in skills.json: PRESENT (v0.1.0)
  kubernetes-allocation-report in skills.json: PRESENT (v0.1.0)
  rightsize-recommendation in skills.json: PRESENT (v0.1.0)
  carbon-cost-pair in skills.json: PRESENT (v0.1.0)
  focus-spec-normalizer in skills.json: PRESENT (v0.1.0)

Install roles (catalog/install-roles.json):
  finops-maestro-agent: IN ROLES
  finops-ai-economist-agent: IN ROLES
  finops-kubernetes-rightsizer-agent: IN ROLES
  finops-cloud-price-advisor-agent: IN ROLES

Skill manifest (catalog/skill-manifest.json):
  All 6 skills: IN MANIFEST

Summary: Gates 17/17 PASS, Catalog 10/10 PRESENT, Install roles 4/4, Skill manifest 6/6
Status: PASS
Blockers: none
