# DynamoDB Access Patterns and Capacity Guide

Use this reference for DynamoDB table design, partition/sort keys, GSIs/LSIs, hot partitions, capacity mode, adaptive capacity, TTL, streams, global tables, transactions, DAX, and cost/performance reviews.

## What people get wrong

The lazy story is:

> Pick a partition key and add GSIs when queries need them.

Wrong. DynamoDB design starts with access patterns, cardinality, item size, write distribution, consistency needs, and growth model. Indexes are not a rescue plan for unknown queries.

Common bad assumptions:

- High-cardinality key always avoids hot partitions.
- Adaptive capacity fixes bad key design.
- Scans are acceptable until scale arrives.
- On-demand capacity eliminates throttling and cost planning.
- GSI backfill is operationally harmless.
- Global tables solve DR without conflict and latency tradeoffs.

## DynamoDB-specific failure modes

- Partition key concentrates reads/writes on tenant, status, date, or celebrity keys.
- Sort key does not support required range, prefix, ordering, or uniqueness patterns.
- GSI projection/backfill doubles write cost or throttles production.
- LSI item collection limits or large items break growth assumptions.
- TTL deletion timing is treated as immediate business logic.
- Transactions, conditional writes, streams, and idempotency are not modeled for retries.

## Minimum safe workflow

1. List concrete access patterns: operation, key condition, filter, consistency, frequency, latency, and expected cardinality.
2. Model entities and item collections against partition/sort keys before proposing tables/indexes.
3. Check write/read distribution, hot-key risk, item size, projected growth, and capacity mode.
4. Review indexes: GSI/LSI keys, projection, sparse behavior, backfill risk, and query shapes.
5. Validate operational controls: autoscaling/on-demand, alarms, Contributor Insights, TTL, streams, backups, and global tables.
6. Estimate cost and failure mode for peak traffic, backfills, retries, and scans.
7. State what cannot be proven without production traffic or representative workload tests.

## Verification targets

- access pattern matrix with key condition expressions, not just entity names
- table partition/sort keys, item collection examples, GSI/LSI definitions, projections, and sparse-index behavior
- CloudWatch metrics: throttles, consumed capacity, account/table limits, latency, system errors, and hot key signals
- Contributor Insights, adaptive capacity symptoms, split-for-heat context, and key distribution evidence
- capacity mode, autoscaling targets, reserved capacity, on-demand cost, backfill plan, and alarms
- TTL, streams, transactions, conditional writes, DAX, global tables, PITR/backups, and restore testing

## When to push back

Push back if the user asks to:

- approve a table without access patterns
- use scans or filters as primary query strategy
- add GSIs blindly for every future query
- ignore hot partition risk because adaptive capacity exists
- use TTL for exact deletion workflows
- enable global tables without conflict, replication, and failover design
