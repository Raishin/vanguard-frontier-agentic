# EVAL REPORT: Azure skills AgentCore reference repair batch 003

## Scope

Processed exactly five Azure skills in stable sorted order after batch 002:
1. `skills/azure/azure-identity-governance-review`
2. `skills/azure/azure-key-vault-secret-lifecycle-auditor`
3. `skills/azure/azure-keyvault-certificate-issuer-review`
4. `skills/azure/azure-landing-zone-architect`
5. `skills/azure/azure-live-aks-rollout-guard`

## Evidence

Documentation-based evidence came from Microsoft Learn documentation through the user configured documentation MCP, including Microsoft Entra governance and least-privilege guidance, Key Vault secret and certificate protection/renewal guidance, Cloud Adoption Framework landing zone design guidance, and AKS rolling upgrade guidance.

## Capability evals

- AgentCore headings present: PASS — all five processed skill reference packs include `## High-risk assumptions to kill` and `## Safe command/code verification targets`.
- Version alignment: PASS — SKILL.md metadata, metadata.json, and catalog/skills.json versions are `0.1.3` for all five changed skills.
- Evidence language discipline: PASS — no prohibited internal environment wording or raw-ID placeholder wording found in the processed paths.
- Credential and identifier boundary: PASS — touched guidance does not ask users to paste credentials, tenant IDs, subscription IDs, customer data, private keys, or secrets.
- AWS non-interference: PASS — `git diff --name-only` returned zero `agents/aws` or `skills/aws` paths.

## Regression evals

- `npm run validate:skill-schema`: PASS.
- `npm run manifest:check`: PASS.
- `npm run validate:asset-integrity`: PASS before workflow/eval artifact creation; asset integrity regenerated again afterward.
- `npm run validate`: PASS, `VALIDATE_EXIT:0`; log: `/tmp/vfa-validate-azure-skills-repair-003.log`.

## Remaining work

This batch proves Azure skills 11-15 are repaired. The active objective is not complete because additional Azure skills and OCI assets remain.
