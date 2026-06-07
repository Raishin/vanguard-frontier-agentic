# EVAL REPORT: Azure skills AgentCore reference repair batch 002

## Scope

Processed exactly five Azure skills in stable sorted order after batch 001:
1. `skills/azure/azure-cosmosdb-platform-operator`
2. `skills/azure/azure-cost-estimation-review`
3. `skills/azure/azure-cost-optimization-governor`
4. `skills/azure/azure-entra-id-specialist`
5. `skills/azure/azure-governance-policy-guardrails`

## Evidence

Documentation-based evidence came from Microsoft Learn documentation through the user configured documentation MCP, including Cosmos DB reliability/private endpoint failover, Azure pricing and Cost Management, Microsoft Entra Conditional Access/workload identity risk, and Azure Policy safe deployment/remediation guidance.

## Capability evals

- AgentCore headings present: PASS — all five processed skill reference packs include `## High-risk assumptions to kill` and `## Safe command/code verification targets`.
- Version alignment: PASS — SKILL.md metadata, metadata.json, and catalog/skills.json versions are `0.1.3` for all five changed skills.
- Evidence language discipline: PASS — no prohibited internal environment wording found in the processed paths.
- AWS non-interference: PASS — `git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws` returned zero paths.

## Regression evals

- `npm run validate:skill-schema`: PASS.
- `npm run manifest:check`: PASS.
- `npm run validate:asset-integrity`: PASS before workflow/eval artifact creation; asset integrity regenerated again afterward.
- `npm run validate`: PASS, `VALIDATE_EXIT:0`; log: `/tmp/vfa-validate-azure-skills-repair-002.log`.

## Remaining work

This batch proves Azure skills 6-10 are repaired. The active objective is not complete because additional Azure skills and OCI assets remain.
