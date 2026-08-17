# Attribution and Idle

How to attribute Snowflake credits defensibly, and why the gap between query-attributed and total warehouse credits is the most actionable number in a cost review. Load for chargeback, showback, or any 'who is spending this' question.

## Two different numbers

- `QUERY_ATTRIBUTION_HISTORY` reports credits attributed to query execution and excludes warehouse idle time. It is the right basis for asking which workload is expensive.
- `WAREHOUSE_METERING_HISTORY` reports what the warehouse actually consumed, idle included. It is the right basis for the invoice.
- The difference between them is idle. Idle is a configuration finding — auto-suspend, scheduling, workload consolidation — not a query-tuning finding, and sending it to the query team wastes their time.
- Where the total cost of a query is needed, query acceleration credits are a separate component and must be added to attributed compute rather than assumed included.
- Attribution excludes idle by design, so an attribution breakdown never sums to the bill. Say so before presenting one, or the first question in the room will be why the numbers do not match.

## Allocating shared idle honestly

- A defensible model distributes shared idle proportionally to each tag's attributed usage on that warehouse. State it as an allocation method with an assumption, not as a measurement.
- The test of a chargeback model is whether the team being charged can reproduce the number from evidence they can see. A model they cannot reproduce will be disputed and then abandoned, and the programme restarts a year later.
- Report untagged workload as its own line — 'untagged' — rather than distributing it silently. Its size is the honest measure of how much the attribution can be trusted.
- Query tags are set by the client, so tagging coverage is a workload-engineering problem as much as a FinOps one. Route the coverage gap to the teams that own the workloads, with the specific untagged warehouses and users named.

## Evidence queries

Attribute compute credits by query tag, excluding idle — the 'which workload is expensive' question.

```sql
SELECT COALESCE(NULLIF(query_tag, ''), 'untagged') AS tag,
       SUM(credits_attributed_compute)           AS compute_credits,
       SUM(credits_used_query_acceleration)      AS qas_credits,
       COUNT(*)                                  AS queries
  FROM SNOWFLAKE.ACCOUNT_USAGE.QUERY_ATTRIBUTION_HISTORY
 WHERE start_time >= DATEADD(month, -1, CURRENT_DATE)
 GROUP BY tag
 ORDER BY compute_credits DESC;
-- The 'untagged' row is the finding. Report its share before anything else.
```

Isolate the idle line — the gap between what queries consumed and what the warehouse cost.

```sql
WITH attributed AS (
  SELECT warehouse_name,
         SUM(credits_attributed_compute) AS query_credits
    FROM SNOWFLAKE.ACCOUNT_USAGE.QUERY_ATTRIBUTION_HISTORY
   WHERE start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
   GROUP BY warehouse_name
),
metered AS (
  SELECT warehouse_name,
         SUM(credits_used_compute) AS total_credits
    FROM SNOWFLAKE.ACCOUNT_USAGE.WAREHOUSE_METERING_HISTORY
   WHERE start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
   GROUP BY warehouse_name
)
SELECT m.warehouse_name,
       m.total_credits,
       COALESCE(a.query_credits, 0)                        AS query_credits,
       m.total_credits - COALESCE(a.query_credits, 0)      AS idle_credits,
       ROUND(100 * (m.total_credits - COALESCE(a.query_credits, 0))
             / NULLIF(m.total_credits, 0), 1)              AS idle_pct
  FROM metered m
  LEFT JOIN attributed a ON a.warehouse_name = m.warehouse_name
 ORDER BY idle_credits DESC;
-- High idle_pct is an auto-suspend and scheduling finding, not a query finding.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/sql-reference/account-usage/query_attribution_history — That the view tracks credits attributed to query execution excluding warehouse idle time, and that query acceleration credits are a separate component to be summed
- https://docs.snowflake.com/en/user-guide/cost-attributing — The documented approaches to attributing cost by query tag, including the proportional distribution of idle time across tags
