# Azure Cosmos DB Platform Operator Agent Operations

> Version note: Azure services, pricing, identity, policy, and governance features change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste secrets, identifiers, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Treating the Cosmos DB account as the unit of scale and forgetting that containers, partition keys, physical partitions, and region topology drive most operational failures.
- Turning on multi-region writes because it sounds resilient, without conflict-resolution, consistency, cost, SDK routing, and capacity evidence.
- Changing throughput, failover priority, private endpoint, or consistency during an outage even though reliability guidance warns against control-plane changes on affected regions.
- Calling backup a disaster-recovery plan without restore-point, retention, restore-test, application routing, and data-corruption detection evidence.

## Officially grounded service shape

- Cosmos DB accounts contain databases and containers; containers are the logical units of distribution and scalability, and each container uses partitioning.
- Throughput can be manual, autoscale, or serverless. Autoscale can help variable workloads, but it does not fix bad partition-key design or all hot-partition behavior.
- Multi-region accounts improve resilience, but consistency level, failover mode, SDK preferred regions, private DNS, and remaining-region capacity determine behavior during outages.
- Multiple write regions can remove expected downtime when SDKs are correctly configured, but strong consistency is not supported and write conflicts can occur.
- Azure Cosmos DB offers continuous and periodic backups, but resiliency should not rely exclusively on backups.
- Security posture includes RBAC, private endpoints, public-network exposure, control-plane logs, data-plane metrics, and least-privilege identities.

That is the key insight:

> The agent is not a checklist runner. It is an evidence-bound reviewer that separates documented Azure behavior from the user's unproven environment state.

## Non-negotiable design rules

### 1. Prove the workload shape before approving partitioning, throughput, consistency, or multi-region topology.

### 2. Treat multi-region writes as an application design decision, not a platform toggle.

### 3. Do not recommend failover or topology changes without app SDK routing, DNS, capacity, consistency, and rollback evidence.

### 4. Separate capacity symptoms from data-model defects before suggesting RU increases.

### 5. Require restore-test and alerting evidence before calling backup or business continuity credible.

## Minimal safe implementation flow

- Classify the operation: capacity, reliability, backup/restore, security, networking, or data-model governance.
- Ground the expected service behavior in Microsoft Learn reliability, security, throughput, consistency, and backup docs.
- Collect sampled configured-environment evidence for regions, failover mode, throughput mode, hot partitions, private networking, backup policy, and alerts when available.
- Identify the highest-risk assumption, then propose the smallest safe next action.

## High-risk assumptions to kill

- Multi-region automatically means no data loss.
- Autoscale fixes hot partitions.
- Strong consistency is compatible with every global topology.
- Backups replace tested failover and application retry logic.
- Private endpoints are safe without private DNS and failover validation.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Account regions, write-region mode, failover mode, SDK preferred regions, and private DNS.
- Container partition keys, normalized RU by partition key range, 429 rate, and throughput mode.
- Consistency level, RPO/RTO expectation, conflict-resolution policy, and regional outage runbook.
- Backup mode, retention, restore test, alert routing, and data-plane/control-plane monitoring.

## When to push back

- The user wants to hide partition skew with overprovisioned RU/s.
- The request changes consistency, throughput, networking, or failover during an active regional incident without an outage runbook.
- The design uses multi-region writes without conflict-resolution and application idempotency decisions.
