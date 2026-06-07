# Network Routing and DNS Guide

Use this reference for AWS network architecture reviews involving VPCs, subnets, route tables, Transit Gateway, Direct Connect, Site-to-Site VPN, Cloud WAN, Route 53 Resolver, private endpoints, ingress, egress, and inspection paths.

## What people get wrong

The lazy story is:

> If packets route in the diagram, the network design is fine.

Wrong. AWS networking fails at the boundaries: overlapping CIDRs, asymmetric routing, DNS resolver loops, endpoint policy gaps, inspection bypass, quota limits, and failover paths nobody tested.

Common bad assumptions:

- Transit Gateway centralization automatically simplifies routing.
- Security groups and NACLs are interchangeable controls.
- Private subnets are private if they lack public IPs.
- Route 53 private DNS works the same across VPCs, accounts, and hybrid networks.
- Direct Connect or VPN redundancy exists because two links are drawn.
- VPC endpoints remove all egress and data-exfiltration risk.

## Network-specific failure modes

- Overlapping or exhausted CIDR ranges block peering, TGW attachments, or future account expansion.
- Asymmetric routing breaks stateful firewalls, NAT, inspection appliances, or hybrid return paths.
- Route propagation leaks reachability across environments or bypasses inspection VPCs.
- Resolver forwarding rules create split-horizon surprises, loops, or missing conditional forwarding.
- Interface endpoint policies, private DNS, security groups, or shared services accounts are mis-scoped.
- Failover path lacks BGP, health checks, route priority, DNS TTL, or operational runbook evidence.

## Minimum safe workflow

1. Identify scope: VPCs, accounts, Regions, CIDRs, routing domains, DNS zones, and hybrid links.
2. Draw actual traffic flows for ingress, east-west, egress, service endpoints, and management access.
3. Verify route-table, TGW route-table, propagation, association, DNS resolver, and endpoint-policy boundaries.
4. Classify exposure: public internet, partner network, on-premises, shared services, workload VPC, and control plane.
5. Check availability and failover: AZ independence, NAT/endpoint placement, DX/VPN redundancy, DNS TTL, and runbook.
6. Identify least risky next actions; do not mutate routes, DNS, or firewall rules without change approval.
7. State what evidence is live, documented, user-provided, or inferred.

## Verification targets

- VPC/subnet CIDRs, route tables, NAT gateways, internet gateways, egress-only gateways, and NACL/security group intent
- Transit Gateway attachments, route tables, propagation, associations, appliance mode, and blackhole/static routes
- Direct Connect gateway/VIF, Site-to-Site VPN tunnels, BGP routes, and failover test evidence
- Route 53 Resolver inbound/outbound endpoints, forwarding rules, private hosted zones, DHCP options, and split-horizon behavior
- VPC interface/gateway endpoints, private DNS, endpoint policies, and service control boundaries
- flow logs, reachability analyzer output, packet path evidence, and recent network-change timeline

## When to push back

Push back if the user asks to:

- add broad routes such as `0.0.0.0/0` or propagated access without blast-radius proof
- bypass inspection to fix latency or connectivity quickly
- accept overlapping CIDRs as a temporary problem
- change DNS forwarding without rollback and test queries
- claim private connectivity prevents exfiltration without endpoint/resource policy evidence
- treat a diagram as proof of live routing state
