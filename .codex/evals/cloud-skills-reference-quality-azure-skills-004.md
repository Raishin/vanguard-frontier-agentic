## EVAL DEFINITION: cloud-skills-reference-quality-azure-skills-004

### Scope
Exactly five Azure skills in stable sorted order:
1. skills/azure/azure-live-app-service-slot-swap-guard
2. skills/azure/azure-live-arm-deployment-stack-guard
3. skills/azure/azure-live-cost-budget-action-guard
4. skills/azure/azure-live-entra-role-assignment-guard
5. skills/azure/azure-live-keyvault-rotation-purge-guard

### Assumptions
- Microsoft Learn documentation proves documented Azure behavior only.
- Live mutations are out of scope; this batch updates repo guidance only.
- No tenant, subscription, quota, vault, cost, or app posture is proven without sampled read-only evidence.

### Capability Evals
- [ ] Each target has one component-specific operations guide with AgentCore-standard headings.
- [ ] Each target has refreshed official-sources, safety-checklist, workflow-and-output, and mcp-and-evidence references separating documentation claims from sampled current-state evidence.
- [ ] Each changed target bumps patch version to 0.1.3 in SKILL.md, metadata.json, and catalog/skills.json.
- [ ] References avoid internal environment identifiers and use generic configured documentation evidence phrasing.

### Regression Evals
- [ ] npm run validate:skill-schema passes.
- [ ] npm run manifest:check passes after manifest regeneration.
- [ ] npm run validate:asset-integrity passes after integrity regeneration.
- [ ] npm run validate passes.
- [ ] git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws returns no paths.

### Graders
- Structural Python grader for required guide headings, versions, and reference files.
- Prohibited wording grep for internal wording and sample identifiers.
- Repo validation commands listed in the report.

## EVAL REPORT
Pending implementation and validation.

## EVAL REPORT: cloud-skills-reference-quality-azure-skills-004

### Capability Evals
- component operations guides: PASS — all five targets include required AgentCore-standard headings.
- refreshed evidence/safety/workflow/source references: PASS — all five targets updated.
- version bump: PASS — SKILL.md, metadata.json, and catalog/skills.json are at 0.1.3 for all five targets.
- internal wording discipline: PASS — prohibited wording grep returned no matches.

### Regression Evals
- npm run validate:skill-schema: PASS.
- npm run manifest:check: PASS.
- npm run validate:asset-integrity: PASS.
- npm run validate: PASS.
- AWS scoped diff check: PASS, no AWS paths.

### Notes
- Microsoft Learn evidence was used for documented service behavior.
- No live Azure environment posture was sampled or claimed.
