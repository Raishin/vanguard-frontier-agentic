# Workflow and Output Contract

## Workflow

1. Classify the topology before recommending anything.
   - hub-spoke, flat peering mesh, Virtual WAN-adjacent, or unclear,
   - single region or multi-region,
   - single subscription or multi-subscription,
   - Azure-only or hybrid with on-premises connectivity.
2. Identify ownership and trust boundaries.
   - which controls are platform-owned,
   - which controls are workload-owned,
   - which services are shared: DNS, firewall, Bastion, gateways, inspection, private endpoints, or monitoring,
   - whether the current design accidentally centralizes too much or delegates too much.
3. Map the current shared-services pattern.
   - hub resources and their role,
   - spoke purpose by environment or workload,
   - whether spokes are isolated by subscription, resource group, environment, or team,
   - whether cross-spoke communication is required, forbidden, or poorly defined.
4. Challenge the connectivity model.
   - hub-and-spoke is not automatically correct,
   - flat peering becomes operational debt at scale,
   - direct spoke-to-spoke connectivity changes blast radius,
   - a central hub can become an outage choke point if routing, DNS, or firewall decisions are brittle.
5. Review routing dependencies explicitly.
   - peering is non-transitive,
   - gateway-transit expectations must be deliberate,
   - UDR ownership must be clear,
   - forced tunneling through Azure Firewall or an NVA changes east-west and egress behavior,
   - hybrid paths need route and failure-domain thinking, not just connectivity diagrams.
6. Review DNS dependencies explicitly.
   - shared DNS is often a hidden platform dependency,
   - private endpoint resolution requires deliberate private DNS zone linkage,
   - custom DNS and on-premises resolution paths can become the real failure point,
   - if name resolution is undefined, the topology is incomplete even if peering exists.
7. Review security and inspection boundaries.
   - what the hub inspects,
   - what stays workload-local,
   - whether central egress or ingress policy is justified,
   - whether NSGs, firewalls, NVAs, and private endpoints are being mixed without a clear control model,
   - whether the design reduces or expands lateral-movement risk.
8. Review shared versus workload-local private networking decisions.
   - if many workloads need the same PaaS resource, central hub placement may simplify routing and control,
   - if only specific workloads need access, workload-local placement may better preserve least privilege,
   - call out `/32` route implications and DNS dependencies when private endpoints are involved.
   - if Virtual WAN is in play, remember private endpoints belong in connected spokes, not in the hub itself.
9. Surface bottlenecks and failure domains.
   - single hub per region assumptions,
   - shared firewall throughput or rule-management bottlenecks,
   - DNS forwarder fragility,
   - operational coupling between unrelated workloads,
   - unclear rollback path for route-table or peering changes.
10. Return concrete topology corrections.
   - what should stay centralized,
   - what should move to workload boundaries,
   - what routing or DNS assumptions must be fixed first,
   - what should be validated before any production change.

## Output contract

Return all of the following:

- **Topology summary**: current or proposed model, regions, subscriptions, and connectivity pattern.
- **Ownership model**: platform-owned controls versus workload-owned controls.
- **Key risks**: routing, DNS, blast radius, operational bottlenecks, and security boundary weaknesses.
- **Recommended topology adjustments**: specific changes to hub, spokes, peering, shared services, or private-network placement.
- **Validation checks**: bounded checks for routes, DNS resolution, peering intent, and access paths.
- **Open questions and assumptions**: what is still unknown and why it matters.
- **Evidence used**: Microsoft Learn pages and any Azure MCP namespaces actually used.

Use this response shape:

```text
Topology summary
- Model: ...
- Regions/subscriptions: ...
- Shared services: ...

Ownership model
- Platform-owned: ...
- Workload-owned: ...

Key risks
- Routing: ...
- DNS: ...
- Security / blast radius: ...
- Operations: ...

Recommended topology adjustments
- ...

Validation checks
- Confirm peering and gateway-transit intent
- Confirm effective routes or UDR ownership
- Confirm DNS resolution path
- Confirm security-control boundary

Open questions and assumptions
- ...
```

## Eval gate

The skill output fails if it does not do all of the following:

1. classify the actual topology and connectivity model,
2. identify shared services and trust boundaries,
3. separate platform-owned controls from workload-owned controls,
4. call out routing dependencies, including peering non-transitivity where relevant,
5. call out DNS dependencies instead of treating them as an implementation detail,
6. address security and blast-radius implications,
7. return concrete topology corrections rather than generic “use hub-spoke” advice,
8. state assumptions and missing facts explicitly.

Minimum scenarios this skill should handle:

1. hub-spoke review for a multi-subscription landing zone,
2. shared-services boundary critique across platform and workload teams,
3. hybrid-routing or private-networking concern review where DNS and route dependencies matter.

## Safety notes

- Do not say a flat network is acceptable by default just because it is simpler.
- Do not recommend centralizing every control in the hub; that often creates a high-blast-radius bottleneck.
- Do not recommend workload-local exceptions without explaining the operational and security tradeoff.
- Do not ignore DNS. A topology that “works on paper” but has no credible name-resolution path is not production-ready.
- Do not treat route tables, gateway transit, or forced tunneling as minor implementation details; they are architecture decisions.
- Do not blur platform-shared controls and workload-local controls. Ambiguous ownership becomes outage fuel.
- Be explicit when advice is inference from incomplete topology evidence rather than confirmed current-state facts.
