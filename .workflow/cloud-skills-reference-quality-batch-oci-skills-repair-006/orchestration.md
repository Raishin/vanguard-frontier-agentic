# OCI skills repair batch 006 orchestration

Date: 2026-06-05
Provider: OCI
Asset type: skills
Batch size: 5

## Items processed

1. skills/oci/oci-migration-cutover-architect
2. skills/oci/oci-multi-cloud-architect
3. skills/oci/oci-mysql-heatwave-ai-specialist
4. skills/oci/oci-network-architect
5. skills/oci/oci-observability-incident-responder

## Evidence plan

- Official OCI documentation for documented service behavior.
- OCI API evidence through the user’s configured read-only OCI MCP for sampled command/API surface shape.
- Microsoft Learn documentation through the user’s configured documentation MCP for Azure-OCI interconnect behavior in the multi-cloud skill.
- No customer identifiers, credentials, private endpoints, customer data, or environment-specific labels are committed.

## Success criteria

- Each skill has lean SKILL.md guidance and detailed AgentCore-style references.
- Each skill has operations, safety, evidence, workflow, and official-source references.
- Patch versions are bumped in SKILL.md metadata, metadata.json, and catalog/skills.json.
- Generated skill manifest and asset integrity are refreshed.
- Narrow validation and full validation pass.
