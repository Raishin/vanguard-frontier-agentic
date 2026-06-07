# Official sources for Azure Cosmos DB Application Developer

Use Microsoft Learn documentation through the user's configured documentation MCP before recommending data models or access patterns. Documentation proves documented Cosmos DB behavior; it does not prove the user's workload distribution, partition-key quality, RU budget, indexes, consistency setting, or latency profile.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [Partitioning and horizontal scaling in Azure Cosmos DB](https://learn.microsoft.com/en-us/azure/cosmos-db/partitioning) | Partition key choice is a design-time scalability decision; require even distribution and access-pattern alignment. |
| [Model and partition data in Azure Cosmos DB](https://learn.microsoft.com/en-us/azure/cosmos-db/modeling-data) | Use for embedding vs referencing, entity boundaries, denormalization, and query-driven document design. |
| [Understand request units consumption](https://learn.microsoft.com/en-us/azure/cosmos-db/understand-request-unit-consumption) | Ground RU costs in document size, indexing, consistency, query shape, and diagnostics. |
| [Request Units in Azure Cosmos DB](https://learn.microsoft.com/en-us/azure/cosmos-db/request-units) | Use for provisioned throughput, multi-region RU behavior, and consistency tradeoffs. |
| [Consistency levels](https://learn.microsoft.com/en-us/azure/cosmos-db/consistency-levels) | Use for correctness and latency tradeoffs in read semantics. |
| [Transactional batch operations](https://learn.microsoft.com/en-us/azure/cosmos-db/transactional-batch) | Use for ACID operations scoped to the same logical partition key and batch limits. |
| [Optimize request cost for reads and writes](https://learn.microsoft.com/en-us/azure/cosmos-db/optimize-cost-reads-writes) | Use for point reads, query tuning, indexing, and request-charge checks. |
| [Architecture best practices for Azure Cosmos DB](https://learn.microsoft.com/en-us/azure/well-architected/service-guides/cosmos-db) | Use for Well-Architected design tradeoffs across reliability, cost, performance, security, and operations. |

## Source-grounding rules

- Do not approve a partition key without access-pattern evidence.
- Do not recommend joins or cross-partition queries as the default path.
- Treat RU estimates as workload-dependent; require measured `RequestCharge` or metrics for precision.
- Keep API scope explicit: this skill defaults to Azure Cosmos DB for NoSQL unless the user names another API.
