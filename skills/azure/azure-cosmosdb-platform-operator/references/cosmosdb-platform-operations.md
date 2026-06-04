# Cosmos DB platform operations

## What people get wrong

- They treat account-level multi-region settings as proof the application fails over safely.
- They ignore SDK preferred-region configuration, private DNS, and remaining-region throughput when reviewing regional resiliency.
- They change consistency, failover priority, or throughput during stress without knowing the blast radius.
- They view backup as an availability feature instead of a data-recovery feature.
- They approve partitioning from a diagram instead of measured access patterns and distribution.

## Officially grounded service shape

Microsoft Learn describes Cosmos DB as an account containing databases and containers, with containers as logical units of distribution and scalability. Reliability features include zone redundancy, multi-region replication, several consistency levels, service-managed or customer-managed failover, SDK resiliency behavior, and continuous or periodic backup. The reliability guide also warns against control-plane changes on affected regions during outage scenarios and calls out private endpoint DNS considerations after failover.

## Non-negotiable design rules

1. Identify API type, write model, regions, consistency, backup, and throughput mode before giving a platform verdict.
2. Treat partitioning and throughput as workload-coupled, not platform-only decisions.
3. Verify SDK region preference and retry behavior for multi-region availability.
4. For private endpoints, design regional endpoints and DNS resolution deliberately.
5. Size remaining regions for failover load before claiming regional resilience.
6. Use backup/restore for data corruption/deletion scenarios; use multi-region and failover for availability scenarios.
7. Require diagnostics, Resource Health, Service Health, alerts, and DR runbooks.

## Minimal safe implementation flow

1. Inventory account topology: API, regions, write model, consistency, backup, throughput, network access, and private endpoints.
2. Review workload shape: partition key, hot partition risk, RU headroom, critical operations, and consistency needs.
3. Review reliability: zone redundancy, failover mode, preferred regions, capacity under region loss, and DR drills.
4. Review network: private endpoints, DNS zones, hub/spoke forwarding, firewall, and client routing.
5. Review operations: metrics, logs, alerts, backup restore testing, and owner/runbook evidence.
6. Rank blockers and reversible remediations.
7. Require explicit approval before any control-plane mutation.

## Safe verification targets

- Account regions, write region or multi-write state, failover priorities, and service-managed/forced failover process.
- Consistency level and documented RPO/availability implications.
- Throughput mode and normalized RU headroom by partition key range.
- Private endpoint count, DNS zones, VNet links, forwarding, and client resolution path.
- Backup mode, retention, restore test, and data-corruption response plan.
- SDK preferred regions, retry, partition-level circuit breaker or equivalent availability settings.

## When to push back

Push back on production signoff without failover evidence, private endpoint DNS proof, capacity-under-failure math, SDK routing proof, backup restore tests, or partition distribution evidence.
