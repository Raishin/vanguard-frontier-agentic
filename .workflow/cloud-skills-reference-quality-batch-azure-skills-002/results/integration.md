# Integration results: Azure skills batch 002 quality reset

Date: 2026-06-04

## Changed items

- `azure-cosmosdb-platform-operator` -> `0.1.2`
- `azure-cost-estimation-review` -> `0.1.2`
- `azure-cost-optimization-governor` -> `0.1.2`
- `azure-entra-id-specialist` -> `0.1.2`
- `azure-governance-policy-guardrails` -> `0.1.2`

## Generated files

- `catalog/skill-manifest.json`
- `catalog/asset-integrity.json`

## Validation

- Structural AgentCore-standard grader: PASS
- Prohibited wording grep: PASS
- `rtk npm run validate:skill-schema`: PASS
- `rtk npm run manifest:check`: PASS
- `rtk npm run validate:asset-integrity`: PASS
- `rtk npm run validate`: PASS
- AWS path diff check: PASS; no AWS assets changed
