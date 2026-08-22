# Baseline, Prediction, and Rollback Trigger

The three artifacts that make a compute change measurable, and the preconditions that stop the common changes that cost credits without helping. Load during preflight.

## The baseline

- Capture credits by day, the query-attributed share versus total (the idle line), p50 and p95 elapsed time, queue time, and spill volumes — over a stated window, 30 days by default.
- The idle share decides which change is even appropriate: a high idle share is an auto-suspend and scheduling problem, and no size change addresses it.
- Capture the affected-workload list at the same time. The change reaches every workload on the warehouse, and the ones that did not prompt it have owners who were not asked.
- Record the baseline in the attestation. A post-change measurement without a recorded baseline is an assertion, and the argument about whether the change helped is unwinnable.

## Preconditions that prevent useless changes

- **Scaling changes require observed queueing.** Warehouse load history showing near-zero queue time means additional clusters add continuous cost and no latency benefit.
- **Size reductions require a spill check.** A workload already spilling will spill more at a smaller size, run longer, and can consume more credits in total — the expected saving inverts.
- **Auto-suspend reductions require an arrival-pattern check.** A bursty workload with a very short auto-suspend resumes constantly and loses its warehouse cache each time.
- **Monitor thresholds must sit above the observed baseline.** A limit below current consumption fires immediately; that is a scheduled outage, not a budget.
- **Suspend-capable actions require a what-breaks analysis**: which warehouses, which workloads, the hour the threshold would plausibly be crossed, and the named person who can raise the limit then.

## The rollback trigger

- Define it before the change, in the approval: the specific observation that causes a revert. 'p95 for the reporting workload exceeds 40 seconds', 'daily credits exceed 180', 'any spill to remote storage'.
- A trigger defined afterwards becomes a negotiation, and the change survives because reverting looks like an admission of error. Defining it in advance makes reverting a pre-agreed outcome rather than a defeat.
- State who watches for the trigger and for how long. A trigger nobody is measuring is a sentence in a document.
- Exclude the mixed-state window from the measurement: running queries continue under the prior configuration, so the first period after a change reflects both settings.
- Normalize for workload volume before concluding anything. A credit reduction that coincides with a demand reduction proves nothing about the change.

## Evidence queries

Capture the baseline the prediction will be measured against.

```sql
-- Credits and the idle share.
WITH metered AS (
  SELECT DATE_TRUNC('day', start_time) AS d,
         SUM(credits_used_compute)     AS total_credits
    FROM SNOWFLAKE.ACCOUNT_USAGE.WAREHOUSE_METERING_HISTORY
   WHERE warehouse_name = '<WH>'
     AND start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
   GROUP BY d
),
attributed AS (
  SELECT DATE_TRUNC('day', start_time)   AS d,
         SUM(credits_attributed_compute) AS query_credits
    FROM SNOWFLAKE.ACCOUNT_USAGE.QUERY_ATTRIBUTION_HISTORY
   WHERE warehouse_name = '<WH>'
     AND start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
   GROUP BY d
)
SELECT m.d,
       m.total_credits,
       COALESCE(a.query_credits, 0)                   AS query_credits,
       m.total_credits - COALESCE(a.query_credits, 0) AS idle_credits
  FROM metered m LEFT JOIN attributed a ON a.d = m.d
 ORDER BY m.d;
```

Capture the performance baseline and enumerate the affected workloads in one pass.

```sql
SELECT user_name,
       role_name,
       query_type,
       COUNT(*)                                                  AS queries,
       MEDIAN(total_elapsed_time)/1000                           AS p50_s,
       APPROX_PERCENTILE(total_elapsed_time, 0.95)/1000          AS p95_s,
       AVG(queued_overload_time)/1000                            AS avg_queue_s,
       SUM(bytes_spilled_to_local_storage)                       AS spill_local,
       SUM(bytes_spilled_to_remote_storage)                      AS spill_remote
  FROM SNOWFLAKE.ACCOUNT_USAGE.QUERY_HISTORY
 WHERE warehouse_name = '<WH>'
   AND start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
 GROUP BY 1, 2, 3
 ORDER BY queries DESC;
-- Every row is a workload the change affects. spill_remote > 0 blocks a
-- size reduction; near-zero avg_queue_s blocks a scaling increase.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/resource-monitors — That resource monitors cover warehouse and cloud-services credits and that their actions include suspending warehouses — the basis for treating them as availability controls
- https://docs.snowflake.com/en/user-guide/warehouses-considerations — Sizing, scaling policy, auto-suspend, and caching behaviour — the mechanisms behind the preconditions above
