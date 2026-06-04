# Safety checklist for Azure Cosmos DB Performance Investigator

## Non-negotiable gates

- Never ask for account keys, connection strings, full documents, customer data, tenant identifiers, subscription identifiers, or database dumps.
- Do not recommend throughput changes, indexing changes, repartitioning, SDK retry changes, consistency changes, or data reshaping without evidence and rollback.
- Treat diagnostic logs as potentially sensitive. Ask only for sanitized aggregates, query text with literals removed, or metrics screenshots summarized in text.
- Do not claim root cause from a single metric. Correlate RU, latency, status codes, partition distribution, query metrics, and client retries.
- Require explicit approval before any write or configuration mutation.

## High-risk assumptions to kill

- "429 means under-provisioned globally." It may be a hot partition or retry pattern.
- "High latency means service issue." It may be query fan-out, index miss, large documents, consistency, region path, or client connection mode.
- "Normalized RU at 100% always equals failures." It can spike without broad 429s; correlate with status and workload.
- "Indexing more properties improves performance." It can increase write cost and may not fix query shape.
- "Throughput redistribution fixes a bad partition key." It may reduce symptoms but not remove skewed design risk.

## Evidence labels

- `docs_only`: Microsoft Learn guidance only.
- `metrics_sample`: sanitized Azure Monitor or Insights metrics reviewed.
- `query_profile`: request charge, query metrics, index metrics, diagnostics, or SDK logs reviewed.
- `remediation_ready`: proposed change has measured baseline, blast radius, rollback, and approval.

## Minimum safe evidence

- Time window, region, API type, account/container, workload path, and symptom.
- Total Request Units, 429 counts, normalized RU by partition key range, latency percentiles, and operation type.
- Representative query text with sensitive literals removed, request charge, query metrics, index metrics, and result counts.
- SDK version, connection mode, retry policy, timeout behavior, and diagnostics for client-perceived latency.
- Recent changes in traffic, data volume, indexing, regions, consistency, or deployments.
