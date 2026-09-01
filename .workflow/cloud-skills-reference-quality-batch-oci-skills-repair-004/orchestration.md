# OCI skills repair batch 004 orchestration

Date: 2026-06-05
Provider: OCI
Asset type: skills
Batch size: 5

## Items processed

1. skills/oci/oci-limits-capacity-planner
2. skills/oci/oci-live-autonomous-db-lifecycle-guard
3. skills/oci/oci-live-cost-budget-runaway-guard
4. skills/oci/oci-live-iam-policy-compartment-guard
5. skills/oci/oci-live-network-security-rule-guard

## Evidence plan

- Official OCI documentation for documented service behavior.
- OCI API evidence through the user’s configured read-only OCI MCP for sampled command/API surface shape.
- No customer identifiers, credentials, private endpoints, customer data, or environment-specific labels are committed.

## Success criteria

- Each skill has lean SKILL.md guidance and detailed AgentCore-style references.
- Each skill has operations, safety, evidence, workflow, and official-source references.
- Patch versions are bumped in SKILL.md metadata, metadata.json, and catalog/skills.json.
- Generated skill manifest and asset integrity are refreshed.
- Narrow validation and full validation pass.
