# Cosmos DB application design

## What people get wrong

- They start with entity diagrams instead of access patterns. Cosmos DB data modeling starts from how the application reads and writes.
- They choose a human-friendly partition key that creates hot logical partitions.
- They query where they could point-read by ID and partition key.
- They over-index every property and then wonder why writes are expensive.
- They assume transactional batch crosses partition keys. It does not.

## Officially grounded service shape

Microsoft Learn explains that RUs are a normalized measure affected by document size, indexing, consistency, query shape, and operations. Point reads using item ID plus partition key are the most efficient reads. Transactional batch provides ACID semantics for operations sharing the same logical partition key. In multi-region accounts, throughput is provisioned in each region and consistency choices affect read throughput.

## Non-negotiable design rules

1. Design containers around dominant access patterns, not just entity type.
2. Choose partition keys with high cardinality, even request distribution, growth headroom, and locality for transactions or common queries.
3. Prefer point reads for hot paths. If a query is required, keep it partition-scoped where possible.
4. Treat document size and indexed properties as write-cost drivers.
5. State consistency requirements explicitly for each user flow.
6. Use transactional batch only when all operations share a logical partition key and fit documented limits.
7. Measure request charge and query metrics before asserting cost or performance.

## Minimal safe implementation flow

1. List top user flows and background jobs with expected read/write frequency.
2. Define documents, IDs, partition keys, and denormalization based on those flows.
3. Map each critical read to point read, partition-scoped query, or cross-partition query.
4. Define indexing policy from actual filters, sort orders, and projections.
5. Decide consistency per flow, not globally by habit.
6. Validate with sample data, realistic distribution, request charge logging, query metrics, and failure tests.
7. Document migration and rollback for existing containers.

## Safe verification targets

- Candidate partition-key distribution and top hot-key risks.
- Request charge for representative create, replace, patch, delete, point read, and query operations.
- Query metrics showing index utilization, document load, output count, and scan behavior.
- Transactional batch operation count, payload size, and single-partition proof.
- SDK retry, timeout, connection mode, and diagnostics logging behavior.

## When to push back

Push back when the user wants a schema without access patterns, a partition key without cardinality proof, a critical cross-partition query, a cross-partition transaction, or an RU estimate with no measurement plan.
