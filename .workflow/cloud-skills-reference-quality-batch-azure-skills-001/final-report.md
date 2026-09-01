# Final report: Azure skills batch 001 AgentCore-standard reset

Date: 2026-06-04

## Scope

Exactly five Azure skills in stable sorted order were redone to the deeper AgentCore-style reference standard:

1. `skills/azure/azure-ai-foundry-ops-governor`
2. `skills/azure/azure-aks-platform-operator`
3. `skills/azure/azure-app-service-production-readiness`
4. `skills/azure/azure-cosmosdb-application-developer`
5. `skills/azure/azure-cosmosdb-performance-investigator`

## Evidence basis

Microsoft Learn documentation through the user's configured documentation MCP was used for current Azure service grounding. The resulting docs separate documentation-based behavior from configured-environment evidence and avoid environment-specific implementation details.

## Quality reset performed

Each skill now has:

- a component-specific operations guide linked from `SKILL.md`,
- service-specific `official-sources.md`,
- service-specific `safety-checklist.md`,
- service-specific `workflow-and-output.md`,
- generic evidence-boundary guidance in `mcp-and-evidence.md`,
- version bumped to `0.1.2` in `SKILL.md`, `metadata.json`, and `catalog/skills.json`.

## Validation

All required validation passed. See `results/integration.md` for command-level evidence.

## Commit status

Not committed. User has not asked to commit.
