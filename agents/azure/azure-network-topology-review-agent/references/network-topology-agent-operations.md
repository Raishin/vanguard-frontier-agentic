# Azure Network Topology Review operations

Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state.

## What people get wrong

Do not bless a topology because it is labeled hub-spoke. The evidence must prove routing, DNS, private access, egress, monitoring, and ownership boundaries.

## Officially grounded service shape

Hub-spoke centralizes shared services and connectivity while spokes isolate workloads; peering is nontransitive and DNS is a first-class design concern, especially with private endpoints. That is the key insight: topology safety depends on routing and name resolution behavior, not diagram labels.

## Non-negotiable design rules

1. Reject flat, overlapping, or undocumented address plans.
2. Require explicit routing, DNS, egress, ingress, and private endpoint behavior for every critical path.
3. Do not assume peering provides transitive spoke-to-spoke routing.
4. Separate platform-owned shared services from workload-owned controls.
5. Require diagnostics and Network Watcher-style validation targets for production recommendations.

## Minimal safe implementation flow

1. Classify topology pattern, regions, hubs, spokes, private endpoints, hybrid links, and shared services.
2. Ground hub-spoke, Private Link, and topology guidance in Microsoft Learn.
3. Review address plan, routing, DNS, firewall, DDoS, NSG, monitoring, and ownership evidence.
4. Identify blast-radius and operational risks before recommending changes.
5. Return verdict with safe next actions and validation probes.

## High-risk assumptions to kill

- Hub-spoke is automatically safe.
- VNet peering makes all spoke routing transitive.
- Private endpoint deployment solves DNS automatically.
- Centralized firewall rules replace workload ownership.
- Documentation proves deployed packet flow.

## Safe command/code verification targets

- Address spaces and overlap risk.
- Route tables, UDRs, firewall or NVA path, peering flags, gateway transit, and spoke-to-spoke intent.
- Private DNS zones, links, conditional forwarding, and private endpoint records.
- Diagnostics, flow logs, connection checks, and alerting for critical paths.

## When to push back

- The diagram omits DNS, routes, or private endpoint resolution.
- CIDR ranges overlap or future expansion is impossible.
- Shared-service ownership is unclear.
- A proposed change centralizes blast radius without compensating controls.
