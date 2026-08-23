# Diagnosis from Profile and History

How to get from 'it is slow' to a named mechanism, in the order that avoids the most wasted work. Load at the start of every performance investigation.

## Decompose before diagnosing

- Elapsed time splits into compilation, queueing, and execution. Establishing which one dominates costs one query and eliminates most of the possible investigations.
- **Queue-dominated** — the warehouse is saturated. Remedies: separate the workloads, adjust the scaling policy, add clusters. A larger single cluster does not reduce queueing.
- **Compilation-dominated** — usually very large or highly dynamic SQL, deeply nested views, or extreme numbers of expressions. Remedies live in the SQL and the model, not in the warehouse.
- **Execution-dominated** — now the Query Profile is the arbiter. Find the dominant operator, then read pruning and spilling.
- Report the decomposition in the finding. A reader who disagrees with the conclusion can then disagree with the evidence rather than with an assertion.

## Reading the profile

- **Pruning** — compare partitions scanned to partitions total. A ratio near 1.0 on a large table means the predicate is not pruning, and no warehouse size fixes that; it only makes the full scan faster and more expensive.
- **Spilling to local storage** — the operation exceeded available memory. Two remedies with very different cost profiles: more memory (a bigger warehouse, charged continuously) or less data (narrower projection, earlier aggregation, better join order). Present both.
- **Spilling to remote storage** — it exceeded local disk as well. This is severe and usually indicates an exploding join, an unbounded sort, or a window function over far more data than intended. Fix the query before touching the warehouse.
- **A dominant join operator with output far larger than its inputs** — a fan-out from a missing or non-unique join key. This is a correctness smell as much as a performance one; check whether the result is even right.
- **Large bytes sent over the network between operators** — data movement dominating. Look at the join strategy and the model rather than at capacity.
- **A dominant sort with no user-visible ordering requirement** — ordering imposed by a view or a window that the consumer does not need.

## Predicates that defeat pruning

- Wrapping the partitioned column in a function on the filtered side commonly prevents pruning. Rewrite the predicate to compare the raw column against a computed bound instead.
- Filtering on a column that has no correlation with insertion order gives nothing to prune on — this is the case where clustering may be justified, and the pruning ratio is the evidence for it.
- Predicates hidden behind a view, a UDF, or a semi-structured path may not be usable for pruning in the way the author expects. Check the profile rather than the intent.
- A join whose filter is only applied after the join has already scanned everything is a plan-shape problem: push the selective predicate to the scan.

## Evidence queries

Decompose elapsed time for a workload — the first query of any investigation.

```sql
SELECT query_id,
       LEFT(query_text, 120)                      AS query_snippet,
       warehouse_name,
       warehouse_size,
       total_elapsed_time/1000                    AS elapsed_s,
       compilation_time/1000                      AS compile_s,
       queued_overload_time/1000                  AS queued_s,
       execution_time/1000                        AS exec_s,
       partitions_scanned,
       partitions_total,
       ROUND(partitions_scanned / NULLIF(partitions_total, 0), 3) AS prune_ratio,
       bytes_spilled_to_local_storage             AS spill_local,
       bytes_spilled_to_remote_storage            AS spill_remote
  FROM SNOWFLAKE.ACCOUNT_USAGE.QUERY_HISTORY
 WHERE start_time >= DATEADD(day, -7, CURRENT_TIMESTAMP())
   AND query_type = 'SELECT'
 ORDER BY total_elapsed_time DESC
 LIMIT 50;
-- prune_ratio near 1.0 on a large table: no warehouse size fixes that.
-- spill_remote > 0: fix the query before touching the warehouse.
```

Distinguish a concurrency problem from a query problem before proposing multi-cluster scaling.

```sql
SELECT start_time,
       warehouse_name,
       avg_running,
       avg_queued_load,
       avg_queued_provisioning,
       avg_blocked
  FROM SNOWFLAKE.ACCOUNT_USAGE.WAREHOUSE_LOAD_HISTORY
 WHERE start_time >= DATEADD(day, -7, CURRENT_TIMESTAMP())
   AND warehouse_name = 'MY_WH'
 ORDER BY start_time;
-- Sustained avg_queued_load with modest avg_running is a scaling problem.
-- Near-zero queueing means more clusters buy nothing and cost continuously.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/ui-query-profile — Operator semantics and the meaning of the pruning, spilling, and data-movement statistics
- https://docs.snowflake.com/en/sql-reference/account-usage/query_history — The elapsed-time decomposition columns and the partition and spill statistics used above
