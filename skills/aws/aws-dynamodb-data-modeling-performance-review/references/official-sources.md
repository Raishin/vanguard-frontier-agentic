# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/data-modeling.html
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/bp-table-design.html
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.html
- https://docs.aws.amazon.com/prescriptive-guidance/latest/dynamodb-data-modeling/best-practices.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- DynamoDB data modeling guidance centers schema design on access patterns, partition keys, sort keys, secondary indexes, and single-table versus multi-table tradeoffs.
- Global secondary indexes support alternate partition/sort key schemas and projected attributes, but have throughput, storage, and synchronization considerations.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon DynamoDB as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `DynamoDB+DescribeTable` and `DynamoDB+Query` were reported `isAvailableIn` in those regions.

Review implications:
- Do not approve table design without enumerated access patterns, key cardinality/skew analysis, query-vs-scan proof, index write amplification, capacity mode, and cost/latency tradeoffs.
- Live table status, throttling, consumed capacity, and hot partitions must come from live metrics or repo/IaC evidence, not docs.
