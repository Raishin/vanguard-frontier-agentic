[CAPABILITY EVAL: schema-contract-conformance]
Date: 2026-05-13

Results per asset:
- finops-maestro-agent/metadata.json: PASS — All 12 schema-required fields present; companion_skills: ["finops-maestro"]; execution_tier: read-only-runtime; lifecycle: beta
- finops-ai-economist-agent/metadata.json: PASS — All required fields; companion_skills: ["fetch-foundation-model-pricing","carbon-cost-pair"]; execution_tier: read-only-runtime; lifecycle: beta
- finops-kubernetes-rightsizer-agent/metadata.json: PASS — All required fields; companion_skills: ["rightsize-recommendation","kubernetes-allocation-report","carbon-cost-pair"]; execution_tier: read-only-runtime; lifecycle: beta
- finops-cloud-price-advisor-agent/metadata.json: FAIL — Missing fields: companion_skills (should be ["finops-cloud-price-advisor"]), execution_tier (should be read-only-runtime), lifecycle (should be beta)
- finops-maestro/SKILL.md: PASS — Valid frontmatter; allowed-tools: Agent Skill Read Grep Glob; no WebFetch (routing-only, correct)
- fetch-foundation-model-pricing/SKILL.md: PASS — Valid frontmatter; allowed-tools: Read Grep Glob WebFetch; WebFetch appropriate for live price fetching
- kubernetes-allocation-report/SKILL.md: PASS — Valid frontmatter; allowed-tools: Read Grep Glob WebFetch; WebFetch for live node pricing
- rightsize-recommendation/SKILL.md: PASS — Valid frontmatter; allowed-tools: Read Grep Glob; WebFetch correctly ABSENT (pure math)
- carbon-cost-pair/SKILL.md: PASS — Valid frontmatter; allowed-tools: Read Grep Glob WebFetch; WebFetch for live carbon factor data
- focus-spec-normalizer/SKILL.md: PASS — Valid frontmatter; allowed-tools: Read Grep Glob; WebFetch correctly ABSENT (offline normalization)

Catalog coverage:
- catalog/agents.json: 4/4 new agents present
- catalog/skills.json: 6/6 new skills present

Summary: 9/10 PASS
Status: NEEDS WORK
Action required: Add companion_skills, execution_tier, lifecycle to finops-cloud-price-advisor-agent/metadata.json
