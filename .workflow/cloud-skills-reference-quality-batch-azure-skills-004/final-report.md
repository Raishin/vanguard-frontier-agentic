# Final Report: Azure Skills Batch 004

Status: validated, not committed.

Targets:
1. skills/azure/azure-live-app-service-slot-swap-guard
2. skills/azure/azure-live-arm-deployment-stack-guard
3. skills/azure/azure-live-cost-budget-action-guard
4. skills/azure/azure-live-entra-role-assignment-guard
5. skills/azure/azure-live-keyvault-rotation-purge-guard

Evidence:
- Microsoft Learn documentation was used for App Service slot swaps, ARM/Bicep Deployment Stacks, Cost Management budgets and alerts, Azure RBAC/PIM, and Key Vault soft delete, purge protection, and rotation behavior.
- No live tenant, subscription, app, cost, role, quota, or vault posture was claimed.

Changes:
- Refreshed component operations guides for each target.
- Refreshed official-sources.md, safety-checklist.md, workflow-and-output.md, and mcp-and-evidence.md for each target.
- Bumped versions to 0.1.3 in SKILL.md, metadata.json, and catalog/skills.json.
- Regenerated catalog/skill-manifest.json and catalog/asset-integrity.json.

Validation:
- PASS structural AgentCore-standard grader.
- PASS prohibited wording grep.
- PASS npm run validate:skill-schema.
- PASS npm run manifest:check.
- PASS npm run validate:asset-integrity.
- PASS npm run validate.
- PASS AWS diff check; no AWS paths changed.

Commit:
- Not committed; user did not ask to commit.
