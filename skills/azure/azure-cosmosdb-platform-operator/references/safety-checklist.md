# Safety checklist for Azure Cosmos DB Platform Operator

## Non-negotiable gates

- Never ask for account keys, connection strings, full documents, customer data, tenant identifiers, subscription identifiers, or database dumps.
- Do not approve production posture without evidence for regions, consistency, throughput mode, partitioning, backups, private networking, alerts, and failover drills.
- Do not perform or recommend account control-plane changes during an affected-region outage unless Microsoft guidance explicitly supports the action.
- Require explicit approval before changing consistency, regions, failover priority, multi-write, throughput, indexing, backup policy, private endpoints, firewall, or network settings.
- Treat SDK configuration as part of platform reliability; account settings alone do not prove application failover.

## High-risk assumptions to kill

- "Multi-region means no downtime." SDK routing, capacity in remaining regions, consistency, and app failover still matter.
- "Service-managed failover is fast enough." Microsoft Learn notes it can take significant time; forced failover may be needed for faster write recovery.
- "Private endpoint works after failover automatically." Regional private endpoints and DNS forwarding need explicit design.
- "Strong consistency is always safest." It changes availability and throughput tradeoffs.
- "Backups are the DR plan." Backups protect against corruption/deletion; they are not a substitute for availability architecture.

## Evidence labels

- `docs_only`: Microsoft Learn guidance only.
- `sampled_read_only`: account, metric, or network evidence was sampled safely.
- `config_review`: IaC or sanitized account configuration was reviewed but not proven live.
- `dr_drill_proven`: failover or restore behavior was tested and evidence exists.
- `mutation_ready`: blast radius, approval, and rollback are documented.

## Minimum safe evidence

- API type, account tier, regions, write model, failover mode, consistency, and backup policy.
- Throughput mode, hot partition indicators, RU headroom, and capacity during region loss.
- Partition key strategy and hierarchical partition key fit if used.
- Private endpoint layout, private DNS zones, hub/spoke forwarding, and application regional routing.
- Resource Health, Service Health, metrics, diagnostic logs, alerts, and DR runbook ownership.
