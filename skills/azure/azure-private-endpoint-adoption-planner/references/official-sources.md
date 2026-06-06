# Official sources

Use this reference when grounding current Azure behavior for `azure-private-endpoint-adoption-planner`.

## Microsoft Learn sources

- https://learn.microsoft.com/azure/private-link/private-endpoint-dns-integration
- https://learn.microsoft.com/azure/private-link/private-endpoint-dns
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/azure-best-practices/private-link-and-dns-integration-at-scale
- https://learn.microsoft.com/azure/architecture/networking/guide/private-link-virtual-wan-dns-guide
- https://learn.microsoft.com/azure/dns/private-resolver-endpoints-rulesets
- https://learn.microsoft.com/azure/networking/foundations/network-foundations-overview

## Current documentation refresh (2026-06-04)

- Microsoft Learn documentation through the user's configured documentation MCP is the primary source for documented Azure behavior.
- Documentation evidence is not live customer-state evidence. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, incident posture, private connectivity, automation state, or production readiness.
- Use sampled read-only Azure evidence only when the user has configured it and the task requires current-state confirmation. Label it as sampled evidence, not broad proof.

## Grounding rule

Docs explain service behavior. Current-state claims require sampled read-only evidence or sanitized user-provided evidence. If current state was not queried or shown, say so.
