# Integration Report: Azure Skills Batch 007

## Targets

1. `skills/azure/azure-role-selector`
2. `skills/azure/azure-security-posture-hardening`
3. `skills/azure/azure-subscription-resource-organization`
4. `skills/azure/azure-waf-cost-optimization-review`
5. `skills/azure/azure-waf-reliability-review`

## Integrated changes

- Added AgentCore-standard component operations guides for all five targets.
- Refreshed common references: `official-sources.md`, `safety-checklist.md`, `workflow-and-output.md`, and `mcp-and-evidence.md`.
- Replaced stale generic live/Microsoft evidence phrasing with Microsoft Learn documentation through the user's configured documentation MCP plus sampled read-only evidence discipline.
- Removed generic Azure tool documentation URLs from the five target metadata/catalog entries.
- Bumped all five targets to `0.1.1` in `SKILL.md`, `metadata.json`, and `catalog/skills.json`.
- Converted WAF cost/reliability primary docs to lean skill entrypoints that delegate detail to references.
- Regenerated `catalog/skill-manifest.json` and `catalog/asset-integrity.json`.

## Evidence basis

Documentation evidence came from Microsoft Learn documentation through the user's configured documentation MCP. No live Azure tenant, subscription, resource, RBAC, billing, security posture, or reliability state was sampled in this batch.

## Validation

- Structural batch 007 grader: passed.
- Prohibited wording grep on the five target directories: passed with no matches.
- `npm run validate:skill-schema`: passed.
- `npm run manifest:check`: passed.
- `npm run validate:asset-integrity`: passed.
- `npm run validate`: passed.
- AWS scope guard `git diff --name-only -- agents/aws skills/aws mcp/aws rules/aws`: passed with no paths.
