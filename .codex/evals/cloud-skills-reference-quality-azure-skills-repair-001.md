# EVAL REPORT: Azure skills AgentCore reference repair batch 001

## Scope

Processed exactly five Azure skills in stable sorted order:
1. `skills/azure/azure-ai-foundry-ops-governor`
2. `skills/azure/azure-aks-platform-operator`
3. `skills/azure/azure-app-service-production-readiness`
4. `skills/azure/azure-cosmosdb-application-developer`
5. `skills/azure/azure-cosmosdb-performance-investigator`

## Evidence

Documentation-based evidence came from Microsoft Learn documentation through the user configured documentation MCP, including Foundry architecture, AKS baseline architecture, App Service deployment/health/backup guidance, and Cosmos DB query/normalized-RU guidance.

## Capability evals

- AgentCore headings present: PASS — all five processed skill reference packs include `## High-risk assumptions to kill` and `## Safe command/code verification targets`.
- Version alignment: PASS — SKILL.md metadata, metadata.json, and catalog/skills.json versions are `0.1.3` for all five changed skills.
- Evidence language discipline: PASS — no prohibited internal environment wording found in the processed paths.
- AWS non-interference: PASS — `git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws` returned zero paths.

## Regression evals

- `npm run validate:skill-schema`: PASS.
- `npm run manifest:check`: PASS.
- `npm run validate:asset-integrity`: PASS before workflow/eval artifact creation; asset integrity regenerated again afterward.
- `npm run validate`: PASS, `VALIDATE_EXIT:0`; log: `/tmp/vfa-validate-azure-skills-repair-001.log`.

## Remaining work

This batch proves the first five Azure skills are repaired. The active objective is not complete because additional Azure skills and OCI assets remain.
