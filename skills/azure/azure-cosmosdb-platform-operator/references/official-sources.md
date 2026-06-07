# Official sources for Azure Cosmos DB Platform Operator

Use Microsoft Learn documentation through the user's configured documentation MCP before making Cosmos DB platform claims. Documentation proves documented service behavior; it does not prove the user's account configuration, regions, throughput, private DNS, backups, or workload readiness.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [Reliability in Azure Cosmos DB](https://learn.microsoft.com/en-us/azure/reliability/reliability-cosmos-db) | Ground zone redundancy, multi-region, failover, backup, SDK resiliency, RPO, and control-plane outage caveats. |
| [Architecture best practices for Azure Cosmos DB](https://learn.microsoft.com/en-us/azure/well-architected/service-guides/cosmos-db) | Use for Well-Architected reliability, security, cost, operational excellence, and performance tradeoffs. |
| [Partitioning and horizontal scaling](https://learn.microsoft.com/en-us/azure/cosmos-db/partitioning) | Use for partition-key, logical partition, physical partition, and scale-risk review. |
| [Hierarchical partition keys](https://learn.microsoft.com/en-us/azure/cosmos-db/hierarchical-partition-keys) | Use when tenant/user/item or similar hierarchy is proposed; verify workload fit and limits. |
| [Consistency levels](https://learn.microsoft.com/en-us/azure/cosmos-db/consistency-levels) | Use for availability, latency, throughput, and correctness tradeoffs. |
| [Request Units](https://learn.microsoft.com/en-us/azure/cosmos-db/request-units) | Use for provisioned throughput, multi-region throughput, and consistency effects. |
| [Failover considerations for private endpoints](https://learn.microsoft.com/en-us/azure/cosmos-db/failover-considerations-for-private-endpoints) | Use for regional private endpoint, private DNS, and failover routing checks. |
| [Online backup and restore](https://learn.microsoft.com/en-us/azure/cosmos-db/online-backup-and-restore) | Use to distinguish data corruption recovery from availability features. |

## Source-grounding rules

- Do not approve multi-region readiness without SDK region preference, failover mode, capacity, private DNS, and DR drill evidence.
- Do not approve consistency changes without application correctness and throughput impact review.
- Do not approve partition or throughput changes from docs alone; require workload evidence.
- Treat preview features and failover modes as explicit caveats, not default recommendations.
