# EVAL REPORT: Azure skills AgentCore reference repair batch 004

## Scope

Processed exactly five Azure skills in stable sorted order after batch 003:
1. `skills/azure/azure-live-app-service-slot-swap-guard`
2. `skills/azure/azure-live-arm-deployment-stack-guard`
3. `skills/azure/azure-live-cost-budget-action-guard`
4. `skills/azure/azure-live-entra-role-assignment-guard`
5. `skills/azure/azure-live-keyvault-rotation-purge-guard`

## Evidence

Documentation-based evidence came from Microsoft Learn documentation through the user configured documentation MCP, including App Service deployment slot swap behavior, deployment stack deny/action-on-unmanage behavior, Cost Management budget alert behavior, Microsoft Entra PIM/role-assignment behavior, and Key Vault soft-delete/purge-protection behavior.

## Capability evals

- AgentCore headings present: PASS — all five processed skill reference packs include `## High-risk assumptions to kill` and `## Safe command/code verification targets`.
- Version alignment: PASS — SKILL.md metadata, metadata.json, and catalog/skills.json versions are `0.1.4` for all five changed skills.
- Evidence language discipline: PASS — no prohibited internal environment wording or raw-ID placeholder wording found in the processed paths.
- Credential and identifier boundary: PASS — touched guidance does not ask users to paste credentials, tenant IDs, subscription IDs, customer data, private keys, or secrets.
- AWS non-interference: PASS — `git diff --name-only` returned zero `skills/aws` paths.

## Regression evals

- `npm run validate:skill-schema`: PASS.
- `npm run manifest:check`: PASS.
- `npm run validate:asset-integrity`: PASS before workflow/eval artifact creation; asset integrity regenerated again afterward.
- `npm run validate`: PASS, `VALIDATE_EXIT:0`; log: `/tmp/vfa-validate-azure-skills-repair-004.log`.

## Remaining work

This batch proves Azure skills 16-20 are repaired. The active objective is not complete because additional Azure skills and OCI assets remain.
