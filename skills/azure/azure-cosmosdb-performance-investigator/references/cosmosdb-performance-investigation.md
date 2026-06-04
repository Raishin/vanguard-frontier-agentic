# Cosmos DB performance investigation

## What people get wrong

- They add RUs before proving whether the problem is global load, hot partitions, query scans, indexing, document size, consistency, or client retries.
- They look only at average latency and miss p95/p99, 429s, partition-level normalized RU, and retry inflation.
- They inspect query text without query metrics or request charge.
- They treat physical partition pressure as proof of a specific logical key before checking diagnostic logs.
- They change indexing or partition strategy without rollback and backfill planning.

## Officially grounded service shape

Microsoft Learn states that RU consumption depends on document size, indexing, consistency, query shape, and operation type. Each request exposes request charge. Query metrics break down query work, and Azure Monitor/Insights expose Total Request Units, normalized RU by partition key range, throttling, storage, and latency signals. Hot partitions appear when one or a few partition key ranges consume disproportionate RU/s.

## Non-negotiable design rules

1. Establish a time window before collecting metrics.
2. Separate account/container-level symptoms from operation/query-level symptoms.
3. Capture request charge and query metrics for representative operations.
4. Check normalized RU by partition key range before assuming global under-provisioning.
5. Check status codes and retry behavior before blaming service latency.
6. Treat indexing changes, throughput changes, and data reshaping as mutations requiring approval.
7. Prefer reversible experiments before structural changes.

## Minimal safe implementation flow

1. Define symptom, workload path, API type, region, and time window.
2. Collect aggregate metrics: total RU, 429s, latency percentiles, operation type, status code, and region.
3. Collect partition metrics: normalized RU by partition key range and top logical partition evidence when diagnostics are available.
4. Profile representative operations: request charge, query metrics, index metrics, result count, page count, and continuation behavior.
5. Profile client: SDK version, connection mode, retries, timeouts, diagnostics, and deployment region path.
6. Rank root causes and propose one low-risk experiment per suspected cause.
7. Document approval and rollback before changing throughput, index policy, partitioning, consistency, or SDK retry behavior.

## Safe verification targets

- `RequestCharge` per representative read, write, and query.
- Query metrics: index lookup, document load, output document count, retrieved document count, and execution time components.
- Index metrics for filter and sort support.
- Normalized RU consumption split by partition key range.
- 429 count, retry count, p95/p99 latency, and region routing.
- Before/after measurement for each remediation experiment.

## When to push back

Push back when the proposed fix is "increase RUs" without partition/query proof, repartitioning without distribution evidence, index changes without query metrics, or SDK changes without diagnostics.
