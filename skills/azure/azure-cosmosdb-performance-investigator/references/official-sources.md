# Official sources for Azure Cosmos DB Performance Investigator

Use Microsoft Learn documentation through the user's configured documentation MCP before recommending performance fixes. Documentation proves service behavior; it does not prove this user's hot partitions, RU budget, indexes, SDK behavior, retry inflation, or latency root cause.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [Troubleshoot query performance](https://learn.microsoft.com/en-us/azure/cosmos-db/troubleshoot-query-performance) | Use for slow query triage, query metrics, index utilization, and query-shape remediation. |
| [Query metrics](https://learn.microsoft.com/en-us/azure/cosmos-db/query-metrics) | Require query metrics before claiming where query time or RU cost is going. |
| [Index metrics](https://learn.microsoft.com/en-us/azure/cosmos-db/index-metrics) | Use to validate whether current indexes support filters and sorts. |
| [Understand request units consumption](https://learn.microsoft.com/en-us/azure/cosmos-db/understand-request-unit-consumption) | Ground RU cost in document size, indexing, consistency, and query shape. |
| [Monitor and debug with insights](https://learn.microsoft.com/en-us/azure/cosmos-db/use-metrics) | Use for throughput, storage, normalized RU, request charge, and partition-key-range views. |
| [Monitor normalized request units](https://learn.microsoft.com/en-us/azure/cosmos-db/monitor-normalized-request-units) | Use to detect physical partition pressure and hot partition indicators. |
| [Diagnose and troubleshoot request rate too large](https://learn.microsoft.com/en-us/azure/cosmos-db/troubleshoot-request-rate-too-large) | Use for 429 and throttling diagnostics, including hot partition analysis. |
| [Redistribute throughput across partitions](https://learn.microsoft.com/en-us/azure/cosmos-db/how-to-redistribute-throughput-across-partitions) | Treat as preview/limited remediation for eligible accounts; verify limitations before suggesting. |
| [Performance tips for .NET SDK v3](https://learn.microsoft.com/en-us/azure/cosmos-db/performance-tips-dotnet-sdk-v3) | Use for SDK-side diagnostics, retry, connection, and request-charge instrumentation patterns when .NET is in scope. |

## Source-grounding rules

- Never recommend "add RUs" before separating hot partition, inefficient query, indexing, consistency, document size, and client retry causes.
- Query metrics and request charge are stronger evidence than aggregate charts for a single query path.
- Normalized RU by partition key range is a hot-partition signal, not proof of the exact logical key unless diagnostics identify it.
- Throughput redistribution has eligibility and preview caveats; do not present it as a default fix.
