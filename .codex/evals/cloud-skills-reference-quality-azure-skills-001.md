# Eval: Azure skills batch 001 AgentCore-standard quality reset

Date: 2026-06-04
Scope: exactly five Azure skills in stable sorted order:

1. `skills/azure/azure-ai-foundry-ops-governor`
2. `skills/azure/azure-aks-platform-operator`
3. `skills/azure/azure-app-service-production-readiness`
4. `skills/azure/azure-cosmosdb-application-developer`
5. `skills/azure/azure-cosmosdb-performance-investigator`

## Capability criteria

- Each skill has a component-specific operations guide linked from `SKILL.md`.
- Each operations guide includes: `What people get wrong`, `Officially grounded service shape`, `Non-negotiable design rules`, `Minimal safe implementation flow`, `Safe verification targets`, and `When to push back`.
- `official-sources.md` contains Microsoft Learn URLs plus review implications and evidence-boundary rules.
- `safety-checklist.md` contains concrete risk gates, mutation boundaries, evidence labels, and minimum safe evidence.
- `workflow-and-output.md` contains a service-specific workflow and final response contract.
- `mcp-and-evidence.md` uses generic documentation-MCP language and does not name environment-specific implementation details.

## Regression criteria

- Versions are bumped consistently to `0.1.2` in `SKILL.md`, `metadata.json`, and `catalog/skills.json` for the five changed skills.
- Skill manifest and asset integrity are regenerated after skill changes.
- Narrow validation gates pass: `validate:skill-schema`, `manifest:check`, `validate:asset-integrity`.
- Full `npm run validate` passes after generated files change.
- No AWS assets are modified.
- Committed documentation does not include prohibited internal/server/profile/connector wording.

## Evidence used

Microsoft Learn documentation through the user's configured documentation MCP was used for:

- Microsoft Foundry architecture and RBAC.
- Azure AI security and AI platform networking guidance.
- AKS baseline architecture, Well-Architected guidance, network policy, and upgrade practices.
- App Service Well-Architected guidance, reliability, slots, Key Vault references, networking, health checks, backups, and zone-redundant baseline architecture.
- Cosmos DB RU consumption, partitioning, transactional batch, query/index metrics, normalized RU, throttling, and throughput redistribution caveats.

## Results

Pending final validation after regeneration.

## Final validation result

PASS on 2026-06-04.

- Structural AgentCore-standard grader: PASS.
- Prohibited wording grep for batch 001 docs: PASS.
- `rtk npm run manifest:write`: PASS.
- `rtk python3 tests/validate-asset-integrity.py --write`: PASS.
- `rtk npm run validate:skill-schema`: PASS.
- `rtk npm run manifest:check`: PASS.
- `rtk npm run validate:asset-integrity`: PASS.
- `rtk npm run validate`: PASS.
- AWS asset diff check: PASS, no AWS paths changed.
