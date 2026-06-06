# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, migration project, network, telemetry, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/architecture/networking/architecture/hub-spoke
- https://learn.microsoft.com/azure/architecture/networking/architecture/hub-spoke-virtual-wan-architecture
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/network-topology-and-connectivity
- https://learn.microsoft.com/azure/architecture/networking/guide/private-link-hub-spoke-network
- https://learn.microsoft.com/azure/dns/private-resolver-architecture
- https://learn.microsoft.com/azure/virtual-network-manager/overview

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says hub-spoke topology uses a hub for shared network services and cross-premises connectivity, while spokes isolate workloads and may live across subscriptions and environments. Peering is nontransitive, DNS is a common hub-spoke dependency, forced tunneling and UDRs can centralize inspection, and regional hubs reduce blast radius. Virtual WAN is a managed alternative with different routing, scale, and operational tradeoffs.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
