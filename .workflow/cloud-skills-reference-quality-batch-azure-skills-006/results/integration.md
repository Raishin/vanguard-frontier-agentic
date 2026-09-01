# Integration Report: Azure Skills Batch 006

## Targets

1. `skills/azure/azure-platform-automation-devops`
2. `skills/azure/azure-private-endpoint-adoption-planner`
3. `skills/azure/azure-rbac-review`
4. `skills/azure/azure-resilience-bcdr-review`
5. `skills/azure/azure-resource-health-incident-triage`

## Integrated changes

- Added AgentCore-standard component operations guides for all five targets.
- Refreshed common references: `official-sources.md`, `safety-checklist.md`, `workflow-and-output.md`, and `mcp-and-evidence.md`.
- Replaced stale generic live/Microsoft evidence phrasing with Microsoft Learn documentation through the user's configured documentation MCP plus sampled read-only evidence discipline.
- Removed generic Azure tool documentation URLs from the five target metadata/catalog entries.
- Bumped all five targets to `0.1.1` in `SKILL.md`, `metadata.json`, and `catalog/skills.json`.
- Regenerated `catalog/skill-manifest.json` and `catalog/asset-integrity.json`.

## Evidence basis

Documentation evidence came from Microsoft Learn documentation through the user's configured documentation MCP. No live Azure tenant, subscription, resource, RBAC, private endpoint, deployment pipeline, DR, or health-event state was sampled in this batch.

## Validation

- Structural batch 006 grader: passed.
- Prohibited wording grep on the five target directories: passed with no matches.
- `npm run validate:skill-schema`: passed.
- `npm run manifest:check`: passed.
- `npm run validate:asset-integrity`: passed.
- `npm run validate`: passed.
- AWS scope guard `git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws`: passed with no paths.
