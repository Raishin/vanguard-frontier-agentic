# Duplication, Loss, and Reconciliation

Which pipeline operations can silently change the dataset, and the reconciliation that proves whether they did. Load during preflight for any operation that touches data.

## Operations that change data silently

- **Stream recreation** resets the offset. Changes that occurred before the recreation are not re-delivered — this is data loss, and it is executed with a command that looks like maintenance.
- **Pipe refresh** can re-stage and re-load files. Snowflake's load metadata deduplicates by file within its own retention behaviour, which is not a record-level guarantee — a file re-staged under a new name is a new file.
- **Backfill and replay** re-insert records. Without an idempotent key and a merge path in the target, they duplicate, and the duplication then propagates into every aggregate computed downstream.
- **Suspension** creates a gap. The window during which the object was stopped is missing until someone backfills it, and no signal reports the absence.
- **Target lag changes** on a dynamic table alter refresh frequency and can change whether the refresh remains incremental, which alters both cost and the achievable freshness.
- For each of these, state before approval which risk applies — duplication or loss — and what in the target prevents it. 'It should be fine' is not an analysis.

## The reconciliation that closes the change

- Compare three things over the affected window: row counts, control totals on the columns that carry business meaning, and boundary values (min and max of the time key, and counts in the first and last partitions).
- Compare against two references: the pre-change baseline, which detects what the change did, and the source, which detects whether the result is right.
- Reconcile at the consumer's grain as well as the table's. A table-level match with a report-level mismatch is common, and the report is what the business sees.
- Agree the tolerance and the sign-off owner before execution. A tolerance decided after a discrepancy appears is a negotiation, and it always resolves in favour of proceeding.
- A duplication is invisible to a liveness check and to a row-count check that only looks at growth. Distinct-key counts against total counts is the check that finds it.
- Express reconciliation as counts, sums, and checksums — never as exported rows.

## Evidence queries

Capture the pre-change baseline — freshness at consumption and counts by window. This cannot be reconstructed afterwards.

```sql
-- Freshness at the consumption point, not the object's configured lag.
SELECT MAX(event_ts)                                        AS latest_event,
       DATEDIFF('minute', MAX(event_ts), CURRENT_TIMESTAMP()) AS minutes_stale
  FROM <DB>.<SCHEMA>.<TARGET_TABLE>;

-- Counts and control totals by window — the reconciliation baseline.
SELECT DATE_TRUNC('hour', event_ts) AS window_hour,
       COUNT(*)                     AS rows,
       COUNT(DISTINCT <business_key>) AS distinct_keys,
       SUM(<amount_column>)         AS control_total
  FROM <DB>.<SCHEMA>.<TARGET_TABLE>
 WHERE event_ts >= DATEADD(day, -7, CURRENT_TIMESTAMP())
 GROUP BY window_hour
 ORDER BY window_hour;
-- rows > distinct_keys after a replay is the duplication signal that a
-- growth-only row count would miss.
```

Capture the last successful processing state and offset position before the change.

```sql
SELECT name, state, scheduled_time, completed_time, error_message
  FROM SNOWFLAKE.ACCOUNT_USAGE.TASK_HISTORY
 WHERE name = '<TASK>'
   AND scheduled_time >= DATEADD(day, -7, CURRENT_TIMESTAMP())
 ORDER BY scheduled_time DESC
 LIMIT 20;

SELECT SYSTEM$PIPE_STATUS('<DB>.<SCHEMA>.<PIPE>');
SELECT SYSTEM$STREAM_HAS_DATA('<DB>.<SCHEMA>.<STREAM>');

SHOW DYNAMIC TABLES LIKE '<DT>' IN SCHEMA <DB>.<SCHEMA>;
```

Enumerate the downstream consumers the change propagates to.

```sql
SELECT referencing_database || '.' || referencing_schema || '.' || referencing_object_name AS dependent_object,
       referencing_object_domain
  FROM SNOWFLAKE.ACCOUNT_USAGE.OBJECT_DEPENDENCIES
 WHERE referenced_database    = '<DB>'
   AND referenced_schema      = '<SCHEMA>'
   AND referenced_object_name = '<TARGET_TABLE>';
-- Each dependent object inherits both the staleness and any duplication.
-- Name their owners in the approval; they did not approve this change.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/streams-intro — Stream offset semantics — the basis for treating a stream recreation as a data-loss operation
- https://docs.snowflake.com/en/sql-reference/functions/system_pipe_status — The pipe state signals used as prior state: pending count and last received and forwarded message timestamps
