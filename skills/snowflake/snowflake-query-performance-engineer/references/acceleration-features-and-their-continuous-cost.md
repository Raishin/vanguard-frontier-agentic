# Acceleration Features and Their Continuous Cost

What each Snowflake acceleration feature actually accelerates, what it charges forever, and when it is the wrong tool. Load before recommending any of them.

## Each one is a standing bill

- **Clustering** — reorganizes a table so predicates prune better. Automatic clustering runs continuously as the table changes and is charged continuously. Justify it with a pruning ratio and a stable access pattern, and check the clustering history afterwards to confirm the maintenance cost is what was predicted.
- **Materialized views** — precompute a result and refresh as the base table changes. On a high-churn base table the refresh cost can exceed the query cost being saved. Show both numbers. They also carry definition restrictions, so confirm the query shape is even eligible.
- **Search optimization** — accelerates specific lookup and point-query shapes by maintaining an additional structure, which costs continuously. Confirm the query shape matches before recommending it; it is not a general index.
- **Query acceleration** — offloads eligible portions of a scan to serverless compute, billed separately. Check the eligibility view before recommending it, and remember that its credits are a separate line from warehouse compute and must be added when totalling query cost.
- **Caching** — free, and the reason most benchmarks are wrong. Result cache, metadata cache, and warehouse data cache each make a repeat run faster for reasons unrelated to any change made.

## Benchmarks that can fail

- State the cache condition explicitly. A comparison of a cold first run against a warm second run measures the cache and nothing else.
- State the data volume. A tuning result on a subset frequently inverts at full scale, particularly where spilling begins.
- State the concurrency. A query that meets its SLA alone and misses it at production concurrency has not been benchmarked, it has been previewed.
- State the comparison and the falsification criterion before running it: 'if p95 does not improve by at least X at concurrency N, the hypothesis is wrong and the change is reverted'.
- Measure credits per successful run alongside latency. A change that halves latency and triples credits is a trade to be decided with FinOps, not a win to be announced.

## Evidence queries

Check the standing cost of accelerations already in place before adding another.

```sql
SELECT table_name,
       SUM(credits_used) AS clustering_credits_30d
  FROM SNOWFLAKE.ACCOUNT_USAGE.AUTOMATIC_CLUSTERING_HISTORY
 WHERE start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
 GROUP BY table_name
 ORDER BY clustering_credits_30d DESC;

SELECT table_name,
       SUM(credits_used) AS mv_refresh_credits_30d
  FROM SNOWFLAKE.ACCOUNT_USAGE.MATERIALIZED_VIEW_REFRESH_HISTORY
 WHERE start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
 GROUP BY table_name
 ORDER BY mv_refresh_credits_30d DESC;
```

Confirm query acceleration would apply at all, before recommending it.

```sql
SELECT query_id,
       warehouse_name,
       eligible_query_acceleration_time
  FROM SNOWFLAKE.ACCOUNT_USAGE.QUERY_ACCELERATION_ELIGIBLE
 WHERE start_time >= DATEADD(day, -7, CURRENT_TIMESTAMP())
 ORDER BY eligible_query_acceleration_time DESC
 LIMIT 25;
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/tables-clustering-keys — Clustering key semantics, clustering depth, and that automatic clustering is a continuous maintenance operation
- https://docs.snowflake.com/en/user-guide/search-optimization-service — The query shapes search optimization accelerates and that it maintains an additional structure
- https://docs.snowflake.com/en/user-guide/query-acceleration-service — How eligibility is determined and that acceleration credits are billed separately from warehouse compute
- https://docs.snowflake.com/en/user-guide/warehouses-considerations — Sizing, scaling policy, multi-cluster behaviour, and the caching layers that make naive benchmarks misleading
