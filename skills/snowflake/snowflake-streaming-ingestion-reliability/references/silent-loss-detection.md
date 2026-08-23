# Silent Loss Detection

How to make ingestion completeness observable, since no component-level health check will reveal it. Load when designing observability or investigating a suspected gap.

## The three counts

- Reconciliation needs three numbers for the same window: what the producer emitted, what the connector or client reported as accepted, and what the landing table contains. Any two agreeing while the third differs localizes the loss.
- Windows must be defined on a field that does not move — usually an event time carried by the record, not the ingest time. Reconciling on ingest time hides exactly the lateness you are trying to measure.
- Where the producer count is unavailable, say so and label completeness `UNKNOWN`. A two-number reconciliation between the connector and Snowflake cannot detect loss that happened before the connector.
- Run the reconciliation continuously on a rolling window, not as an incident-response activity. Its value is time-to-detect.

## Signals that reveal a partial stop

- **Throughput per channel or pipe**, alerting on an unexpected drop rather than on zero. A channel that carried 10,000 records an hour and now carries 40 is the case a zero-threshold alert misses.
- **Ingest lag at the landing table**, measured as the gap between event time and arrival time at p95. Rising lag precedes loss often enough to be the earliest signal available.
- **Offset progression per channel.** A stalled offset is the most direct evidence of a stopped channel; a regressed offset is evidence of a reopen and a probable replay.
- **Pending count and last-message timestamps** for a pipe, which show whether notifications are still arriving and whether they are being forwarded.
- **Rejected-record count**, with an owner. A rising reject rate is a schema-drift alarm as well as a loss alarm.
- Liveness of the connector process is not on this list. It is the check that gives false confidence, and its presence in a monitoring design is a finding.

## Replay without making it worse

- Before any replay: establish the exact window to replay, whether the landing table has an idempotent key, and what will deduplicate.
- A landing table with no natural key and no ingest-side deduplication cannot be safely replayed into. The fix is to replay into a staging table and merge, not to re-run and hope.
- State the ordering consequence. A replay that interleaves with live ingestion can produce out-of-order records that a downstream incremental process handles differently from the original order.
- Reconcile after the replay against the same three counts. A replay declared successful because it completed is the same error the whole domain is about.
- Snowpipe load history deduplicates by file within its own retention behaviour; that is not a general record-level guarantee and must not be relied on as one for a replay.

## Evidence queries

Detect a partial stop — per-pipe throughput over time, where a drop matters more than a zero.

```sql
SELECT pipe_name,
       DATE_TRUNC('hour', start_time) AS hour,
       SUM(files_inserted)            AS files,
       SUM(bytes_inserted)            AS bytes,
       SUM(credits_used)              AS credits
  FROM SNOWFLAKE.ACCOUNT_USAGE.PIPE_USAGE_HISTORY
 WHERE start_time >= DATEADD(day, -7, CURRENT_TIMESTAMP())
 GROUP BY pipe_name, hour
 ORDER BY pipe_name, hour;
-- Alert on a drop against the trailing baseline, not on zero. A pipe at 4%
-- of its normal rate is the failure a zero-threshold alert never fires on.
```

Check a pipe's live state — whether notifications are arriving and being forwarded.

```sql
SELECT SYSTEM$PIPE_STATUS('my_db.my_schema.my_pipe');
-- Read pendingFileCount together with lastReceivedMessageTimestamp and
-- lastForwardedMessageTimestamp. Received-but-not-forwarded and
-- nothing-received-recently are different failures with different owners.
```

Measure ingest lag at the landing table on event time, which is what the consumer experiences.

```sql
SELECT DATE_TRUNC('hour', ingest_ts) AS ingest_hour,
       COUNT(*)                                                       AS records,
       MEDIAN(DATEDIFF('second', event_ts, ingest_ts))                AS p50_lag_s,
       APPROX_PERCENTILE(DATEDIFF('second', event_ts, ingest_ts), 0.95) AS p95_lag_s,
       MAX(DATEDIFF('second', event_ts, ingest_ts))                   AS max_lag_s
  FROM my_db.my_schema.landing_table
 WHERE ingest_ts >= DATEADD(day, -1, CURRENT_TIMESTAMP())
 GROUP BY ingest_hour
 ORDER BY ingest_hour;
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/data-load-snowpipe-intro — Snowpipe auto-ingest behaviour, load history, and the file-level duplicate handling that is not a record-level guarantee
- https://docs.snowflake.com/en/user-guide/kafka-connector-overview — Connector behaviour, table and column naming, and the ingestion modes whose differences make version establishment mandatory
