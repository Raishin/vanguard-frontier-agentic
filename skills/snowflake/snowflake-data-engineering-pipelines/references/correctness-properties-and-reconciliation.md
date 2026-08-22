# Correctness Properties and Reconciliation

The seven properties that get conflated into 'the pipeline works', and how to design reconciliation that actually proves the data. Load for any correctness question.

## Seven properties, seven pieces of evidence

- **The job ran** — task history shows success. This is the only thing a green pipeline proves.
- **The data arrived** — rows landed in the target for the expected window. Evidence: counts by window, not a total.
- **The data is complete** — every source record that should be present is present. Evidence: reconciliation against a source count or control total.
- **The data is valid** — values satisfy their constraints and domains. Evidence: data metric functions or explicit checks.
- **The data is semantically correct** — the values mean what the consumer thinks they mean. Evidence: a definition agreed with the consumer, which is the analytics agent's domain when contested.
- **The data reconciles** — the target agrees with the source on counts, control totals, and boundaries. Evidence: an executed comparison.
- **The data is fresh enough** — it is current at the point of consumption within the business requirement. Evidence: measured lag at consumption, not configured lag on an object.
- Never use one as proof of another. Almost every pipeline incident that reaches an executive is a case where someone did.

## Reconciliation design

- Reconcile on three things: row counts by window, control totals on the columns that matter (sums of amounts, distinct counts of keys), and boundary conditions (min and max of the time key, and the count in the first and last partition).
- Reconcile inside the run, not on a schedule afterwards. A check that runs later reports a defect after the data has already been consumed.
- Reconcile against the source, not against the previous run. Comparing to yesterday detects a change; it does not detect a systematic loss that has been happening since the first run.
- State the tolerance and the action. A variance threshold with no defined action is a dashboard, and a reconciliation that always fails at 0.01% teaches everyone to ignore it.
- Express reconciliation as counts, sums, and checksums — never as exported rows. The check must not become the exposure.

## Boundaries — where pipelines are actually wrong

- **Late-arriving records.** A window closed at midnight and a record that arrives at 00:05 with a prior-day timestamp: is it captured, dropped, or silently placed in the wrong window?
- **Time zones and day boundaries.** The most common source of a small, persistent, unnoticed count difference. Establish which time zone defines the business day and whether every step agrees.
- **Deletes and soft deletes.** A pipeline built on inserts and updates frequently ignores deletes entirely, so the target grows monotonically and quietly diverges.
- **Restatements.** A source that corrects prior periods requires the pipeline to reprocess them; an append-only design cannot and will not tell you.
- **The first and last run after a change.** Schema changes, lag changes, and warehouse changes all have an edge run. Check it explicitly rather than the steady state.

## Evidence queries

Check what a load actually loaded — the rows-parsed versus rows-loaded gap that a successful COPY can hide.

```sql
SELECT file_name,
       status,
       row_count,
       row_parsed,
       row_parsed - row_count AS rows_not_loaded,
       error_count,
       first_error_message,
       last_load_time
  FROM SNOWFLAKE.ACCOUNT_USAGE.COPY_HISTORY
 WHERE last_load_time >= DATEADD(day, -7, CURRENT_TIMESTAMP())
   AND (error_count > 0 OR row_parsed <> row_count)
 ORDER BY last_load_time DESC;
-- A permissive ON_ERROR setting turns a partial load into a success.
-- rows_not_loaded > 0 with status = LOADED is exactly that case.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/data-load-overview — COPY semantics, load metadata and its duplicate-file behaviour, and error-handling options
