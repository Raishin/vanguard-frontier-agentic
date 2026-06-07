# Azure Agents Reference Quality Batch 008 Eval Report

Date: 2026-06-06
Provider: Azure
Asset type: agents
Batch size: 5

## Targets

1. `agents/azure/azure-ai-foundry-ops-governor-agent`
2. `agents/azure/azure-aks-platform-operator-agent`
3. `agents/azure/azure-app-service-production-readiness-agent`
4. `agents/azure/azure-cosmosdb-application-developer-agent`
5. `agents/azure/azure-cosmosdb-performance-investigator-agent`

## Evidence used

Documentation-based evidence from Microsoft Learn documentation through the user’s configured documentation MCP:

- Microsoft Foundry: platform/resource model, architecture, RBAC/networking alignment, private link and managed network guidance.
- AKS: production cluster reliability, node pool upgrades, rolling upgrade behavior, autoscaling and platform operations guidance.
- App Service: deployment slots, private endpoints, backups/restores, managed identities, and deployment best practices.
- Azure Cosmos DB application development: partitioning, point reads/request cost, transactional batch, consistency levels.
- Azure Cosmos DB performance investigation: RU monitoring, normalized RU/hot partition evidence, 429 troubleshooting, indexing metrics and RU consumption.

No tenant/subscription-specific evidence was committed. Documentation proves Azure service behavior only, not deployed-state posture.

## Changes made

- Replaced stale `live evidence` labels in Codex harness instructions with `sampled configured-environment evidence` wording.
- Updated component-specific official Microsoft Learn documentation URLs.
- Bumped changed asset versions to `0.2.2` in `AGENT.md`, adjacent `metadata.json`, and `catalog/agents.json`.
- Updated `last_verified` to `2026-06-06`.
- Regenerated plugin, Cursor plugin, Kiro Powers, and asset integrity generated files.

## Validation

- Targeted stale phrase audit: PASS
- Version/catalog consistency audit: PASS
- `npm run validate:agent-schema`: PASS
- `npm run validate:asset-integrity`: PASS
- `npm run validate`: PASS (`VALIDATE_EXIT:0`)

## Residual risk

- Microsoft Learn documentation does not prove the user’s tenant, subscriptions, RBAC, quotas, private networking, deployed resources, or production readiness.
- Any configured-environment observations must remain explicitly labeled as sampled evidence at runtime.
