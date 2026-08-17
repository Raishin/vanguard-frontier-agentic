# Streams, Tasks, and Dynamic Tables

The semantics that decide whether a Snowflake batch pipeline silently loses or duplicates data, and how to measure lag through a chain. Load when reviewing any of the three.

## Stream offset semantics

- A stream tracks changes since its offset. Consuming it in a DML statement inside a transaction advances that offset when the transaction commits.
- The consequence engineers miss: once the offset advances, those changes are gone from the stream. If a later step in the same pipeline fails after that commit, the changes are not re-delivered — they must be recovered from the base table, and the design must say how.
- Multiple consumers of a single stream compete for the same offset. Where two processes need the same changes, each needs its own stream; sharing one is a silent-loss design.
- Streams become stale if not consumed within the retention of their source. A stale stream is a data-loss event that presents as an error much later.
- `SYSTEM$STREAM_HAS_DATA` tells you whether there is anything to consume. It does not tell you whether the last consumption was processed successfully downstream.

## Task graph behaviour

- A scheduled task runs on its schedule regardless of whether upstream data arrived, unless the design gates it. That is how a downstream table gets recomputed from stale inputs and reports success.
- Establish overlap behaviour: what happens when a run is still going and the next schedule fires. A long-running run plus a short schedule is either skipped executions or concurrent ones, and both have correctness consequences.
- Failure propagation must be explicit. In a graph, decide and document whether a failed predecessor stops the branch, and whether a partial graph completion is an acceptable state to leave the data in.
- Task history is the evidence for all of the above. Read run durations against the schedule interval, not just the success flag.

## Dynamic tables and lag chaining

- Target lag is a declared contract: the table aims to be no staler than the target. Achieved lag is what the refresh history shows. Report both, always.
- Lag chains. A dynamic table with a one-minute target lag reading from an hourly source is at best hourly; the declared target on the leaf object tells you nothing about end-to-end freshness.
- Refresh mode matters to both cost and lag. Where the query cannot be refreshed incrementally, the refresh becomes a full recomputation — which changes the cost model and often makes the target lag unachievable. Confirm the actual mode from the object rather than the intent from the DDL.
- Optimizing the query for incremental refresh is the lever that makes a demanding target lag affordable. That is a design change, not a setting.
- The refresh consumes credits continuously. A tighter target lag is a cost decision as much as a freshness decision — pair it with FinOps before committing to it.

## Evidence queries

Compare achieved refresh behaviour against the declared target lag, and see whether refreshes are incremental.

```sql
SELECT name,
       schema_name,
       state,
       refresh_action,
       refresh_trigger,
       data_timestamp,
       refresh_start_time,
       refresh_end_time,
       DATEDIFF('second', data_timestamp, refresh_end_time) AS lag_seconds
  FROM SNOWFLAKE.ACCOUNT_USAGE.DYNAMIC_TABLE_REFRESH_HISTORY
 WHERE refresh_start_time >= DATEADD(day, -7, CURRENT_TIMESTAMP())
 ORDER BY refresh_start_time DESC;
-- refresh_action distinguishes an incremental refresh from a full one.
-- Compare lag_seconds against the declared TARGET_LAG from SHOW DYNAMIC TABLES.
```

Find tasks whose run duration approaches or exceeds their schedule interval — the overlap and staleness risk.

```sql
SELECT name,
       database_name,
       schema_name,
       state,
       scheduled_time,
       completed_time,
       DATEDIFF('second', query_start_time, completed_time) AS run_seconds,
       error_message
  FROM SNOWFLAKE.ACCOUNT_USAGE.TASK_HISTORY
 WHERE scheduled_time >= DATEADD(day, -7, CURRENT_TIMESTAMP())
 ORDER BY run_seconds DESC
 LIMIT 50;
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/streams-intro — Stream offset advancement on transaction commit, staleness, and multi-consumer behaviour
- https://docs.snowflake.com/en/user-guide/tasks-intro — Task scheduling, graph dependencies, overlap handling, and failure behaviour
- https://docs.snowflake.com/en/user-guide/dynamic-tables/target-lag — That target lag is a declared objective, that it is monitored against achieved refresh behaviour, and that refresh mode and query shape determine whether it is attainable
