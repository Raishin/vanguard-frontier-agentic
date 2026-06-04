# Azure Private Endpoint Adoption Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating a private endpoint as complete before DNS resolution is designed.
- Creating duplicate private DNS zones with the same name across VNets and hoping Azure merges records.
- Linking private DNS zones to the wrong VNets in hub-spoke or Virtual WAN topologies.
- Assuming central hub placement is always safer than workload-local endpoints.
- Ignoring private endpoint record lifecycle when endpoints are deleted or moved.

## Officially grounded service shape

- Microsoft Learn evidence says the service FQDN must resolve to the private endpoint IP, so DNS configuration is central to Private Link success.
- For peered hub-spoke workloads, Microsoft Learn calls for a single private DNS zone linked to all hub and spoke networks that need resolution.
- DNS zone groups associate private endpoints with private DNS zones and can manage records as endpoints change or are deleted, within documented limits.
- At scale, central private DNS zones, policy-driven DNS zone groups, DNS Private Resolver, conditional forwarding, and regional endpoint strategy become governance decisions.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, incident state, or production readiness.

## Non-negotiable design rules

- Design DNS before endpoint placement.
- Use the recommended private DNS zone name for each Azure service and avoid mixing unrelated services in one zone.
- Name every consumer VNet, peering path, hub, resolver, and on-premises forwarding dependency.
- Decide central versus workload-local endpoint placement from traffic flow, ownership, and regional resilience, not aesthetics.
- Require rollback checks for DNS records, VNet links, firewall/DNS proxy settings, and application connection strings.

## Minimal safe implementation flow

- Scope PaaS services, consumers, regions, subscriptions, VNets, and current DNS authority.
- Map resolution path from each consumer to the private endpoint FQDN and private IP.
- Choose endpoint placement and DNS zone ownership pattern.
- Define IaC or policy mechanism for DNS zone group creation and record lifecycle.
- Validate name resolution, connectivity, routing, monitoring, and rollback before broad rollout.

## Safe verification targets

- Private DNS zones match Microsoft service-specific zone names.
- Required VNets are linked to the authoritative private DNS zones.
- On-premises or custom DNS forwards to the correct resolver path.
- Endpoint FQDN resolves to expected private IP from each consumer network.
- Deleting or moving endpoints has a defined DNS cleanup path.

## When to push back

- The design says “private endpoint enabled” but cannot explain DNS.
- Multiple same-name zones are proposed without record-management ownership.
- Virtual WAN DNS behavior is assumed to match traditional hub-spoke behavior.
- The rollout has no staged validation or rollback for application connectivity.
