# EVAL REPORT: Azure skills AgentCore reference repair batch 006

## Scope

Processed exactly five Azure skills in stable sorted order after batch 005:
1. `skills/azure/azure-platform-automation-devops`
2. `skills/azure/azure-private-endpoint-adoption-planner`
3. `skills/azure/azure-rbac-review`
4. `skills/azure/azure-resilience-bcdr-review`
5. `skills/azure/azure-resource-health-incident-triage`

## Evidence

Documentation-based evidence came from Microsoft Learn documentation through the user configured documentation MCP, including Azure IaC pipeline and deployment-stack behavior, Private Endpoint DNS integration, Azure RBAC least-privilege best practices, Azure BCDR/reliability guidance, and Service Health/Resource Health incident triage guidance.

## Capability evals

- AgentCore headings present: PASS — all five processed skill reference packs include `## High-risk assumptions to kill` and `## Safe command/code verification targets`.
- Version alignment: PASS — SKILL.md metadata, metadata.json, and catalog/skills.json versions are `0.1.2` for all five changed skills.
- Evidence language discipline: PASS — no prohibited internal environment wording or raw-ID placeholder wording found in the processed paths.
- Credential and identifier boundary: PASS — touched guidance does not ask users to paste credentials, tenant IDs, subscription IDs, customer data, private keys, or secrets.
- AWS non-interference: PASS — `git diff --name-only` returned zero `skills/aws` paths.

## Regression evals

- `npm run validate:skill-schema`: PASS.
- `npm run manifest:check`: PASS.
- `npm run validate:asset-integrity`: PASS before workflow/eval artifact creation; asset integrity regenerated again afterward.
- `npm run validate`: PASS, `VALIDATE_EXIT:0`; log: `/tmp/vfa-validate-azure-skills-repair-006.log`.

## Remaining work

This batch proves Azure skills 26-30 are repaired. The active objective is not complete because additional Azure skills and OCI assets remain.
