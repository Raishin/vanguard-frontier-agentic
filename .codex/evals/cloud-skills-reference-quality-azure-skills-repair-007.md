# EVAL REPORT: Azure skills AgentCore reference repair batch 007

## Scope

Processed exactly five Azure skills in stable sorted order after batch 006:
1. `skills/azure/azure-role-selector`
2. `skills/azure/azure-security-posture-hardening`
3. `skills/azure/azure-subscription-resource-organization`
4. `skills/azure/azure-waf-cost-optimization-review`
5. `skills/azure/azure-waf-reliability-review`

## Evidence

Documentation-based evidence came from Microsoft Learn documentation through the user configured documentation MCP, including Azure RBAC role-selection guidance, Defender for Cloud CSPM and secure score guidance, Cloud Adoption Framework resource-organization guidance, Well-Architected Cost Optimization guidance, Cost Management/Advisor guidance, and Well-Architected Reliability and disaster-recovery guidance.

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
- `npm run validate`: PASS, `VALIDATE_EXIT:0`; log: `/tmp/vfa-validate-azure-skills-repair-007.log`.

## Remaining work

This batch proves Azure skills 31-35 are repaired. The active objective is not complete because one Azure skill and OCI assets remain.
