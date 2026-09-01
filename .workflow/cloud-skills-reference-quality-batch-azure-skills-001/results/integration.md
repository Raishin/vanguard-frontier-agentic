# Integration results: Azure skills batch 001 quality reset

Date: 2026-06-04

## Changed items

- `azure-ai-foundry-ops-governor` -> `0.1.2`
- `azure-aks-platform-operator` -> `0.1.2`
- `azure-app-service-production-readiness` -> `0.1.2`
- `azure-cosmosdb-application-developer` -> `0.1.2`
- `azure-cosmosdb-performance-investigator` -> `0.1.2`

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
