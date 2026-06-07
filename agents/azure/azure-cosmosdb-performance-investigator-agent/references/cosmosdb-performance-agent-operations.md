# Azure Cosmos DB Performance Investigator Agent Operations

> Version note: Cosmos DB diagnostics, metrics, autoscale behavior, partition tooling, SDK retry behavior, and query guidance change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste connection strings, keys, tenant or subscription identifiers, database contents, customer data, or raw production diagnostics containing sensitive fields into prompts, commands, or reference examples.

## What people get wrong

- Increasing RU/s before proving whether the issue is overall demand, a hot partition, query shape, metadata throttling, transaction contention, or client behavior.
- Treating any 429 as a database failure; a small retried rate can indicate RU/s are being used effectively.
- Ignoring query metrics and SDK diagnostics while guessing at indexing or throughput fixes.
- Reading normalized RU at 100% as automatic proof that throughput must be increased.
- Enabling broad production diagnostic logging without cost, retention, and data-classification boundaries.

## Officially grounded service shape

- 429 means rate limiting, but the root cause can be overall RU exhaustion, hot partitions, metadata request limits, transient service errors, or transaction contention.
- Microsoft Learn notes 1-5% 429s can be healthy when latency is acceptable and partitions are evenly distributed.
- Normalized RU consumption is the maximum RU/s utilization across partition key ranges in the interval; it can reveal hot partitions but does not alone prove an application outage.
- Diagnostic logs can identify per-second RU consumption, operation type, status codes, partition-key range, and hot logical keys when enabled.
- Query RU is affected by items loaded/returned, index lookups, compilation, UDFs, and page execution behavior; query metrics are required for evidence-based fixes.
- Autoscale behavior smooths short spikes; consistently maxed normalized RU with high 429 rate can justify throughput or design changes.

That is the key insight:

> The agent must diagnose the bottleneck class before recommending cost, schema, indexing, throughput, or SDK changes; otherwise it is just turning performance incidents into spend.

## Non-negotiable design rules

### 1. Separate latency, RU charge, throttling percentage, normalized RU, hot partition evidence, and client retry behavior before proposing fixes.

### 2. Do not prescribe repartitioning, indexing changes, or throughput increases without evidence of the specific bottleneck.

### 3. Treat diagnostic logging as time-bound and data-classified because it can add cost and expose sensitive operational context.

### 4. Use sampled metrics honestly; do not generalize one container, region, operation, or time window to the entire account.

### 5. Prefer lowest-risk fixes first: query shape, point reads, index tuning, retry/timeout behavior, and hot-key mitigation before brute-force throughput.

## Minimal safe implementation flow

- Define symptom, time window, account/database/container, region, operation type, SDK/client path, and observed impact.
- Ground triage in Microsoft Learn 429, normalized RU, query metrics, request charge, monitoring, and best-practice docs.
- Collect read-only sampled metrics, sanitized SDK diagnostics, query metrics, and logs where available.
- Classify the bottleneck: overall RU exhaustion, hot partition, query inefficiency, write/index cost, metadata throttling, transaction contention, client retry, or service incident.
- Map each finding to the lowest-risk remediation, validation target, rollback path, and residual evidence gap.

## High-risk assumptions to kill

- Any 429 means Cosmos DB is failing.
- Increasing RU/s is always cheaper than fixing a hot partition or bad query.
- Autoscale removes the need to inspect partition-level normalized RU.
- Query metrics are optional if latency graphs are available.
- One sampled diagnostic window proves long-term workload behavior.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- 429 fraction, status-code breakdown, latency percentiles, retry count, request charge, and end-to-end client timing.
- Normalized RU by partition key range, top logical partition keys by RU, and consistency across regions/time windows.
- Query metrics, index utilization, document load count, UDF/system-function time, continuation/page behavior, and SDK diagnostics.
- Throughput mode, autoscale max/manual RU, maxed-RU duration, metadata request rate, transaction contention, and bulk ingestion behavior.
- Diagnostic log scope, retention, cost estimate, redaction/classification, alert thresholds, and rollback of observability changes.

## When to push back

- The requested fix is simply increase RU/s without bottleneck evidence.
- The workload has a hot partition but wants to hide it with overprovisioning.
- The user wants production logging changes without cost, retention, and sensitive-data boundaries.
- The evidence is one short sample window but the recommendation claims permanent workload truth.
