# Plan: Azure Skills Batch 004

Provider: azure
Asset type: skills
Batch size: 5
Date: 2026-06-04

Items:
1. skills/azure/azure-live-app-service-slot-swap-guard
2. skills/azure/azure-live-arm-deployment-stack-guard
3. skills/azure/azure-live-cost-budget-action-guard
4. skills/azure/azure-live-entra-role-assignment-guard
5. skills/azure/azure-live-keyvault-rotation-purge-guard

Verification targets:
- Exactly five Azure skills processed.
- Microsoft Learn documentation evidence is separated from sampled read-only configured-environment evidence.
- No AWS assets changed.
- Versions and dates aligned across SKILL.md, metadata.json, and catalog/skills.json.
- Skill manifest and asset integrity regenerated.
- Narrow gates pass before any commit.
