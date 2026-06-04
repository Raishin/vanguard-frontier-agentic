# Safety checklist for Azure Cosmos DB Application Developer

## Non-negotiable gates

- Never ask for account keys, connection strings, customer documents, full database dumps, tenant identifiers, subscription identifiers, or private data.
- Do not recommend a partition key without read/write access-pattern, cardinality, growth, and hot-key analysis.
- Do not recommend broad cross-partition scans as the normal path for high-volume user flows.
- Do not present RU cost as fixed without measured request charge and workload context.
- Require explicit approval before changing indexing policy, partition strategy, TTL, consistency, throughput, SDK retry policy, or data shape in existing workloads.

## High-risk assumptions to kill

- "We can model it like a relational database." Cosmos DB design is query and partition driven.
- "More RUs will fix the design." It can hide partition or query defects and increase cost.
- "Transactional batch works across arbitrary documents." It is scoped to operations with the same logical partition key.
- "Strong consistency is always safer." It can reduce throughput and increase latency; correctness requirements must justify it.
- "Index everything." Indexing helps queries but increases write RU consumption.

## Evidence labels

- `docs_only`: Microsoft Learn guidance only.
- `design_review`: user-supplied schema/API/access-pattern design reviewed, not measured.
- `measured_request`: request charge, query metrics, or diagnostics were provided or sampled.
- `mutation_ready`: change scope, backfill/migration, rollback, and approval are documented.

## Minimum safe evidence

- API surface, container purpose, entity types, expected document sizes, and growth.
- Top read and write access patterns with frequency, latency, and consistency requirements.
- Candidate partition keys with cardinality, distribution, hot-key risk, and transactional grouping needs.
- Query shapes, point-read opportunities, indexing needs, and expected RU budget.
- Migration/backfill strategy for existing containers.
