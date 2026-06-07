# Azure Cosmos DB Application Developer Agent Operations

> Version note: Cosmos DB APIs, SDK behavior, partitioning guidance, indexing capabilities, consistency tradeoffs, and transactional features evolve. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste connection strings, keys, tenant or subscription identifiers, database contents, customer data, or full production documents into prompts, commands, or reference examples.

## What people get wrong

- Designing documents first and picking the partition key last.
- Replacing point reads with queries that filter by id and partition key, then assuming the same RU cost.
- Using transactional batch across logical partition keys even though its ACID boundary is a single logical partition key.
- Choosing strong or bounded-staleness consistency without accounting for doubled read RU cost.
- Keeping default indexing forever and then blaming Cosmos DB when write RU cost rises.

## Officially grounded service shape

- Point reads by item id and partition key are typically the most efficient reads; queries with equivalent filters are not point reads.
- Request Units are the normalized cost currency for reads, writes, and queries; actual request charge must be measured from SDK responses, portal metrics, or diagnostics.
- Strong and bounded-staleness consistency can double read RU cost, so application consistency needs must be explicit.
- Transactional batch operations are atomic only within the same logical partition key.
- Query RU depends on items loaded/returned, index lookups, compilation, UDFs, and page execution behavior; query metrics are the evidence, not intuition.
- Indexing every property is convenient but can raise write RU cost; production workloads should tune indexing to query needs.

That is the key insight:

> The agent must force the application design to expose access patterns, partition-key boundaries, consistency needs, and measured RU charge before endorsing the data model.

## Non-negotiable design rules

### 1. Make access patterns, item identity, partition-key cardinality, and growth distribution explicit before endorsing a model.

### 2. Require RU charge, query metrics, and diagnostics evidence before declaring a query efficient.

### 3. Do not recommend cross-partition transactions unless the chosen API and pattern explicitly support the requirement.

### 4. Treat consistency level as a functional and cost decision, not a default checkbox.

### 5. Prefer SDK retry, diagnostics, idempotency, and point-read APIs over homegrown blind retry loops or query-by-id patterns.

## Minimal safe implementation flow

- Classify workload shape: reads, writes, queries, consistency, transaction scope, item size, retention, and growth pattern.
- Identify candidate partition keys and test for cardinality, hot-key risk, transaction colocation, and query routing.
- Ground data modeling, partitioning, consistency, RU, indexing, and SDK behavior in Microsoft Learn Cosmos DB docs.
- Use sanitized code snippets, synthetic examples, or sampled diagnostics to validate actual RU/query behavior without exposing customer data.
- Return design risks, specific remediations, measurement method, and evidence still needed.

## High-risk assumptions to kill

- A query filtered by id and partition key is equivalent to a point read.
- Low-cardinality or time-bucket partition keys are safe because autoscale exists.
- Cross-partition ACID writes can be solved later with retries.
- Default indexing is production-optimal for write-heavy workloads.
- Strong consistency is free because the workload is small today.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Partition key cardinality, hot-key risk, logical partition size/growth, and colocated transaction boundaries.
- Point-read opportunities, query text, query metrics, request charge, index utilization, and continuation/page behavior.
- Consistency choice, session-token handling, multi-region read/write behavior, and latency/cost tradeoff.
- Indexing policy, excluded paths, composite indexes, write RU charge, and bulk ingestion path.
- SDK retry policy, timeout, diagnostics capture, idempotency, and safe synthetic-load test results.

## When to push back

- The proposed key is low-cardinality, tenant-hot, or time-bucketed without mitigation and measurement.
- The design requires ACID writes across partition keys without an explicit supported pattern.
- No RU measurement exists for a claimed efficient query or write path.
- The user provides raw production documents, keys, or customer data instead of sanitized evidence.
