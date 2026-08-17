# Attribution and Idle

How to attribute Snowflake credits defensibly, and why the gap between query-attributed and total warehouse credits is the most actionable number in a cost review. Load for chargeback, showback, or any 'who is spending this' question.

## Two different numbers

- `QUERY_ATTRIBUTION_HISTORY` reports credits attributed to query execution and excludes warehouse idle time. It is the right basis for asking which workload is expensive.
- `WAREHOUSE_METERING_HISTORY` reports what the warehouse actually consumed, idle included. It is the right basis for the invoice.
- Idle is the LARGEST part of the difference between them on a warehouse-centric workload, but it is not the whole difference. Documented behaviour is that the attribution calculation excludes warehouse idle time, data transfer, storage, cloud services, serverless features, and AI service tokens — six categories. Naming only idle makes the reconciliation look closeable when it is not, and it hides exactly the surfaces (serverless and AI) that a resource-monitor-only control design also misses.
- The practical consequence for a chargeback model: query attribution allocates **warehouse compute for query execution** and nothing else. Storage, transfer, serverless, and AI must be attributed by a separate mechanism or declared explicitly as unallocated. A model that silently omits them charges teams for a shrinking share of the bill while the unallocated remainder grows.
- Where the total cost of a query is needed, query acceleration credits are a separate component and must be added to attributed compute rather than assumed included.
- Attribution therefore never sums to the bill, and it under-sums by more than idle. State the full exclusion list before presenting a breakdown, or the first question in the room is why the numbers do not match — and the honest answer will be longer than anyone expects.

## Allocating shared idle honestly

- A defensible model distributes shared idle proportionally to each tag's attributed usage on that warehouse. State it as an allocation method with an assumption, not as a measurement.
- The test of a chargeback model is whether the team being charged can reproduce the number from evidence they can see. A model they cannot reproduce will be disputed and then abandoned, and the programme restarts a year later.
- Report untagged workload as its own line — 'untagged' — rather than distributing it silently. Its size is the honest measure of how much the attribution can be trusted.
- Query tags are set by the client, so tagging coverage is a workload-engineering problem as much as a FinOps one. Route the coverage gap to the teams that own the workloads, with the specific untagged warehouses and users named.

## Time-sensitive claims

Each row is volatile: re-verify against the cited primary source before encoding it in a recommendation. A status that has moved silently converts a safe recommendation into an unsafe one.

| Claim | Status / constraint | Verified | What the source proves | What it does NOT prove |
|---|---|---|---|---|
| The QUERY_ATTRIBUTION_HISTORY credit calculation excludes warehouse idle time, data transfer, storage, cloud services, serverless features, and AI service tokens; it covers warehouse usage for query execution, accounting for resizing and autoscaling by a weighted average, with concurrent queries attributed by relative resource consumption. | Current documented behaviour — re-verify before building a chargeback model on it | 2026-08-17 via Context7 `/websites/snowflake_en` (cost-attributing, account-usage/query_attribution_history usage notes) | That query attribution covers warehouse compute only, so the gap to the invoice is six categories wide rather than one | How large each excluded category is in this account — that requires the metering and storage views, and it is the number that decides whether the omission matters |

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
