## EVAL DEFINITION: cloud-skills-reference-quality-azure-skills-003

### Scope
Exactly five Azure skills in stable sorted order:
1. skills/azure/azure-identity-governance-review
2. skills/azure/azure-key-vault-secret-lifecycle-auditor
3. skills/azure/azure-keyvault-certificate-issuer-review
4. skills/azure/azure-landing-zone-architect
5. skills/azure/azure-live-aks-rollout-guard

### Assumptions
- Microsoft Learn documentation is authoritative for documented Azure service behavior only.
- No tenant, subscription, RBAC, quota, deployment, or production-readiness state is proven unless sampled read-only evidence is explicitly available.
- This batch changes Azure skill assets only; no AWS assets are in scope.

### Capability Evals
- [ ] Each target has one component-specific operations guide with all AgentCore-standard headings: What people get wrong, Officially grounded service shape, Non-negotiable design rules, Minimal safe implementation flow, Safe verification targets, When to push back.
- [ ] Each target has refreshed official-sources, safety-checklist, workflow-and-output, and mcp-and-evidence references that separate documentation claims from sampled current-state evidence.
- [ ] Each changed target bumps patch version to 0.1.2 in SKILL.md, metadata.json, and catalog/skills.json.
- [ ] References avoid internal environment identifiers and phrase Azure evidence generically.

### Regression Evals
- [ ] npm run validate:skill-schema passes.
- [ ] npm run manifest:check passes after manifest regeneration.
- [ ] npm run validate:asset-integrity passes after integrity regeneration.
- [ ] npm run validate passes if generated marketplace or integrity outputs changed.
- [ ] git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws returns no paths.

### Graders
- Structural Python grader for required guide headings, versions, and reference files.
- Prohibited wording grep for internal wording and sample identifiers.
- Repo validation commands listed in the report.

## EVAL REPORT
Pending implementation and validation.

## EVAL REPORT: cloud-skills-reference-quality-azure-skills-003

### Capability Evals
- component operations guides: PASS — all five targets include required AgentCore-standard headings.
- refreshed evidence/safety/workflow/source references: PASS — all five targets updated.
- version bump: PASS — SKILL.md, metadata.json, and catalog/skills.json are at 0.1.2 for all five targets.
- internal wording discipline: PASS — prohibited wording grep returned no matches after cleanup.

### Regression Evals
- npm run validate:skill-schema: PASS.
- npm run manifest:check: PASS.
- npm run validate:asset-integrity: PASS.
- npm run validate: PASS.
- AWS scoped diff check: PASS, no AWS paths.

### Notes
- Microsoft Learn evidence was used for documented service behavior.
- No live Azure environment posture was sampled or claimed.
