# Azure Network Topology Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Drawing hubs and spokes without route tables, DNS, and ownership boundaries.
- Assuming peering is transitive.
- Centralizing every flow through a firewall without proving latency, SNAT, throughput, and failure-domain impact.
- Ignoring private endpoint DNS when adopting Private Link.
- Letting workload teams and platform teams both own the same control without decision authority.

## Officially grounded service shape

Microsoft Learn evidence says hub-spoke topology uses a hub for shared network services and cross-premises connectivity, while spokes isolate workloads and may live across subscriptions and environments. Peering is nontransitive, DNS is a common hub-spoke dependency, forced tunneling and UDRs can centralize inspection, and regional hubs reduce blast radius. Virtual WAN is a managed alternative with different routing, scale, and operational tradeoffs.

- Hub virtual networks host shared services such as gateways, firewall, Bastion, DNS, and egress/ingress controls.
- Spoke virtual networks isolate workloads and usually peer to one regional hub.
- Spoke-to-spoke traffic needs explicit peering, direct connectivity, Virtual Network Manager, or routing through an NVA/firewall.
- DNS resolution must align with Private Link, cross-premises routing, and firewall FQDN rules.
- Virtual WAN can replace customer-managed hubs when managed transit and scale tradeoffs fit.

## Non-negotiable design rules

- Start with traffic flows, ownership, regions, and failure domains before choosing topology.
- Prove routing symmetry and DNS behavior for every critical path.
- Keep hubs regional unless cross-region routing is explicitly designed and monitored.
- Use direct spoke connectivity only for justified same-workload or same-environment flows.
- Separate platform-owned guardrails from workload-owned network controls.

## Minimal safe implementation flow

- Scope regions, subscriptions, hubs, spokes, shared services, cross-premises links, and private connectivity.
- Map flows: north-south, east-west, management, DNS, private endpoint, ingress, egress, and break-glass.
- Review peering, UDRs, gateway transit, firewall/NVA, DNS resolver, and monitoring design.
- Rank risks by blast radius: DNS, routing loops, asymmetric paths, firewall bottleneck, and shared-service outage.
- Return a target topology or remediation sequence with assumptions and verification evidence.

## High-risk assumptions to kill

- A hub-spoke diagram without route tables, DNS paths, security rules, and ownership is not an architecture review.
- Virtual network peering is nontransitive; spoke-to-spoke paths need explicit design through peering, Virtual Network Manager, or routed inspection.
- Central firewall inspection can create SNAT, throughput, latency, DNS, and blast-radius risks that must be measured or bounded.
- Private endpoint adoption without private DNS design will fail in ways that look like application or firewall issues.
- Cross-region hubs and shared services reduce duplication but can expand failure domains unless regional isolation is deliberate.

## Safe command/code verification targets

- Verify hub, spoke, region, subscription, route table, peering, gateway transit, DNS, firewall/NVA, and Private Link evidence for every critical flow.
- Check whether direct spoke connectivity is justified by same-workload or same-environment needs.
- Validate forced tunneling, forwarded traffic settings, SNAT capacity, firewall logs, and Network Watcher coverage.
- Confirm private DNS zone links/resolvers for private endpoints and cross-premises consumers.
- Require connection tests or sampled path evidence before approving production topology claims.

## Safe verification targets

- Every spoke has intended hub, route table, DNS path, and ownership.
- Private endpoint DNS zones and resolvers are designed for all consuming networks.
- Firewall/NVA throughput, SNAT, logging, and failover fit expected traffic.
- Cross-premises and inter-spoke paths are explicit and tested.
- Network Watcher, diagnostics, flow logs, and connection monitors cover critical paths.

## When to push back

- The user wants topology approval without route and DNS evidence.
- A single shared hub creates unacceptable blast radius.
- Private endpoint adoption ignores DNS resolution.
- Forced tunneling will break PaaS, update, or control-plane dependencies.
