# Data profiling playbook

Use this reference when the user needs a detailed, step-by-step Cosmos DB performance investigation path.

## Goal

Determine whether the workload problem is caused primarily by:

- bad query shape,
- index mismatch,
- partition-key skew,
- insufficient or badly distributed throughput,
- cross-region or client-side latency,
- retry amplification,
- poor workload access patterns,
- or multiple issues at once.

## Step 1: Define the symptom precisely

Capture exactly which of these is true:

1. RU charge is too high.
2. Query latency is too high.
3. 429 throttling is frequent.
4. One partition or one workload slice is much worse than others.
5. The issue is intermittent rather than constant.

If the user only says “Cosmos is slow,” push back and force narrower symptom definition.

## Step 2: Fix the observation window

Before analysis, anchor the time range.

Recommended windows:

- **1 hour** for a sharp incident or deploy regression
- **24 hours** for day-pattern investigation
- **7 days** for partition-skew or periodic workload behavior

Do not compare unrelated windows.

## Step 3: Establish workload scope

Identify:

- account
- database
n- container
- API type
- top queries or endpoints
- whether the issue is read-heavy, write-heavy, or mixed

If the user cannot identify the hot path, say the conclusion will remain weaker.

## Step 4: Measure request charge first

For code-facing workloads, collect request charge from the SDK or response headers.

Why:

- RU cost is the fastest way to distinguish inefficient operations from mere latency complaints.
- A stable high request charge suggests query/data/index issues.
- A low request charge with high latency suggests retries, distance, contention, or client behavior.

Collect examples for:

- representative point read
- representative query
- representative write or batch operation

## Step 5: Get query metrics

For each expensive or slow query:

1. collect query metrics
2. compare **Retrieved Document Count** vs **Output Document Count**
3. flag scan-heavy behavior when retrieved greatly exceeds output

Interpretation:

- **Retrieved much higher than output** → likely index miss or scan-heavy predicate/function behavior
- **Retrieved approximately equal to output** with high RU → maybe wide result sets, cross-partition cost, or costly ordering/filter combinations
- **RU acceptable but latency high** → move to proximity, retries, concurrency, and buffering checks

## Step 6: Check index metrics only when troubleshooting

Use index metrics when query behavior is suspicious and the indexing path is unclear.

Look for:

- utilized indexed paths
- recommended indexed paths
- evidence that the current indexing policy does not support the query efficiently

Do not blindly expand indexing on every field without considering write cost.

## Step 7: Test for hot partitions

Use portal insights or equivalent telemetry to inspect:

- **Normalized RU Consumption (%) By PartitionKeyRangeID**

Interpretation:

- one or a few ranges consistently near saturation while others are low strongly suggests skew
- even usage with high overall pressure suggests general underprovisioning or globally inefficient workload design

## Step 8: Move from physical skew to logical key offenders

If hot partitions are suspected:

1. enable and inspect diagnostic logs if available
2. use partition-key RU consumption data to identify the top logical partition keys
3. review at least several days when possible, not just one bursty hour

This is where “add more RUs” often fails. If a few keys dominate, throughput growth can mask the problem without fixing it.

## Step 9: Profile query shape versus access pattern

Ask or verify:

- should this be a point read instead of a query?
- is the partition key included where it should be?
- is the query filtering on properties that align with the partition strategy?
- is ORDER BY combined with filters in a way the index supports?
- is the application doing repeated small round trips that should be consolidated?

Common anti-patterns:

- using queries where a point read would do
- cross-partition fan-out for tenant-local data
- filtering on secondary properties without an index strategy
- embedding data that grows without bound and inflates read/write cost

## Step 10: Separate server-side from client-side latency

If RU looks acceptable but latency is still poor, check:

- region proximity between app and Cosmos DB account
- throttling retries inflating end-to-end time
- MaxConcurrency for parallel queries
- MaxBufferedItemCount / prefetch behavior
- client connection reuse / SDK posture

Do not blame the database alone without this separation.

## Step 11: Profile storage and index footprint

Inspect storage insights when available:

- data usage
- index usage
- document usage

Use this to spot cases where index footprint is high relative to value or where document shape is inflating costs.

## Step 12: Classify the root cause before prescribing remediation

Use one or more of these buckets:

1. **Query/index mismatch**
2. **Partition-key skew / hot partitions**
3. **General underprovisioning**
4. **Client-side latency / retries / distance**
5. **Data-model inefficiency**
6. **Mixed causes**

If evidence is incomplete, say so.

## Step 13: Recommend bounded remediations

Good remediation examples:

- capture request charge for the top 3 slow queries before changing throughput
- enable query metrics on one hot query and compare retrieved vs output counts
- inspect normalized RU consumption by partition key range for 7 days
- review whether a hot query should become a point read
- test one indexing-policy adjustment against one proven expensive query

Bad remediation examples:

- “just increase RUs”
- “just repartition everything”
- “turn on every index”
- “switch consistency” without naming the application tradeoff

## Step 14: Decide when to hand off to another role

Hand off or recommend adjacent roles when:

- the platform posture itself is the main issue → **Azure Cosmos DB Platform Operator**
- the data model / query code is the main issue → **Azure Cosmos DB Application Developer**
- alerting / visibility is the main issue → **Azure Observability Investigator**
- ongoing RU waste and budget posture dominate → **Azure Cost Optimization Governor**
