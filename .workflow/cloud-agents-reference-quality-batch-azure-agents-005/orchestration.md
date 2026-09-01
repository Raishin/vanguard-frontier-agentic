# Azure agents reference-quality batch 005 orchestration

Date: 2026-06-05
Provider: Azure
Asset type: agents
Batch size: exactly 5

## Targets

1. agents/azure/azure-live-pim-jit-activation-guard-agent
2. agents/azure/azure-maestro-agent
3. agents/azure/azure-migrate-landing-zone-cutover-agent
4. agents/azure/azure-network-topology-review-agent
5. agents/azure/azure-observability-investigator-agent

## Evidence discipline

- Used Microsoft Learn documentation through the user configured documentation MCP for documented Azure service behavior.
- Treated documentation as documentation-based evidence only; it does not prove deployed tenant, subscription, RBAC, quotas, resources, incidents, or production readiness.
- No AWS assets were intentionally changed.
- No committed docs should mention private tool labels, workstation aliases, connection handles, or environment-specific identifiers.

## Integration notes

- Reworked each target to the AgentCore-style reference pack: operations, official sources, safety checklist, workflow/output, and documentation evidence discipline.
- Bumped each changed agent patch version and updated catalog agent entries.
- Regenerated Claude plugin, Cursor plugin, Kiro powers, and asset integrity outputs.
