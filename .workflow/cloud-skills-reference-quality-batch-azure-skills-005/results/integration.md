# Integration Report: Azure Skills Batch 005

## Targets

1. `skills/azure/azure-live-pim-jit-activation-guard`
2. `skills/azure/azure-maestro`
3. `skills/azure/azure-migrate-landing-zone-cutover`
4. `skills/azure/azure-network-topology-review`
5. `skills/azure/azure-observability-investigator`

## Integrated changes

- Added or refreshed component operations guides using the six required AgentCore-style headings.
- Refreshed common references for source grounding, evidence labels, safety gates, workflow, and final response contracts.
- Bumped each target to `0.1.1` in `SKILL.md`, `metadata.json`, and `catalog/skills.json`.
- Updated Azure Maestro routing to avoid stale fixed catalog counts and include `azure-live-entra-role-assignment-guard-agent` in the seven live-guard set.
- Regenerated `catalog/skill-manifest.json` and `catalog/asset-integrity.json`.

## Evidence basis

Documentation evidence came from Microsoft Learn documentation through the user configured documentation MCP. No live Azure tenant, subscription, resource, quota, or deployment state was sampled in this batch.

## Validation

- Structural batch 005 grader: passed.
- Prohibited wording grep on the five target directories: passed with no matches.
- `npm run validate:skill-schema`: passed.
- `npm run manifest:check`: passed.
- `npm run validate:asset-integrity`: passed.
- `npm run validate`: passed.
- AWS scope guard `git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws`: passed with no paths.
