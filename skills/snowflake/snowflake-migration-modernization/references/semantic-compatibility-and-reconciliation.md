# Semantic Compatibility and Reconciliation

The differences that translate cleanly and answer differently, and the reconciliation that catches them before cutover. Load for any compatibility assessment.

## Differences that produce wrong answers, not errors

- **Null handling and ordering.** Platforms differ in where nulls sort and how they compare. A migrated `ORDER BY` or a window function over a nullable column can rank rows differently with identical SQL.
- **Empty string versus null.** Some source platforms treat the empty string as null. Migrated data and migrated predicates then disagree about which rows match, and the difference is a row count nobody expected.
- **Numeric precision, scale, and rounding.** Division, aggregation, and implicit type promotion differ. Financial totals drift by small amounts that survive every review because they are small.
- **Date and timestamp arithmetic, and time zones.** Date differencing, week and quarter boundaries, and the session time zone all vary. This is the most common source of a persistent, unnoticed reporting difference after a migration.
- **Collation and case sensitivity.** Joins and grouping on text keys can match differently, which changes both counts and grain.
- **Implicit casting.** Comparisons between a string and a number, or between differing timestamp types, resolve differently across platforms, silently changing which rows qualify.
- **Aggregate and window semantics at the edges.** Behaviour with all-null inputs, empty partitions, and ties in ranking functions differ.
- The common property of this list: the SQL runs, the result looks reasonable, and the number is wrong. Only reconciliation on real data finds them.

## Reconciliation as the cutover gate

- Reconcile the same period on both platforms on three things: row counts by window, control totals on the columns that carry business meaning, and boundary values (min, max, and the counts in the first and last partitions).
- Reconcile at the consumer's grain, not at the table's. A table-level match with a report-level mismatch is the common outcome, and the report is what the business sees.
- Set the tolerance explicitly and state the action on breach. 'Close enough' decided in the cutover meeting is how a known difference becomes a permanent one.
- Run reconciliation for several periods before cutover, not once. A single passing run can hide a difference that only appears at month end, on a leap day, or across a daylight-saving boundary.
- A failed reconciliation stops the cutover. The date is not evidence, and the pressure to proceed is exactly why this rule is written down in advance.
- Express reconciliation as counts, sums, and checksums, never as exported rows from either platform.
