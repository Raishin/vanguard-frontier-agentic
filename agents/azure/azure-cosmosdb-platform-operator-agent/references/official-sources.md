# Official sources

Use this reference when grounding current Azure Cosmos DB platform operations behavior.

## Microsoft Learn sources refreshed on 2026-06-04

- https://learn.microsoft.com/azure/reliability/reliability-cosmos-db
- https://learn.microsoft.com/azure/well-architected/service-guides/cosmos-db
- https://learn.microsoft.com/azure/cosmos-db/distribute-data-globally
- https://learn.microsoft.com/azure/cosmos-db/provision-throughput-autoscale
- https://learn.microsoft.com/azure/cosmos-db/consistency-levels
- https://learn.microsoft.com/azure/cosmos-db/security-considerations
- https://learn.microsoft.com/azure/cosmos-db/how-to-configure-private-endpoints
- https://learn.microsoft.com/azure/cosmos-db/online-backup-and-restore

## Current documentation refresh notes

- Microsoft Learn documentation through the user's configured documentation MCP proves documented Azure service behavior only.
- It does not prove the user's tenant, subscription, RBAC, quota, deployed resources, production readiness, cost posture, or incident status.
- If documentation and sampled configured-environment evidence conflict, report both and explain the narrower scope of the sample.

## Evidence handling

- `documentation-based`: cite Microsoft Learn URLs and state what the docs prove.
- `sampled evidence`: read-only configured-environment observation with scope and time window.
- `user-provided sanitized evidence`: user input after redaction; validate before relying on it.
- `inference`: a cautious conclusion that still needs proof.

## Grounding rule

Docs explain service behavior. They do not prove the user's licensing, live configuration, permissions, usage, data, resources, or business readiness.
