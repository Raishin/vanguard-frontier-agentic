# Grain, Joins, and Analytical Traps

The specific ways Snowflake analytical SQL produces plausible wrong numbers, and how to check for each. Load for any correctness review.

## Grain is the whole discipline

- Declare the grain of every input, every join output, and the metric itself. A grain change that nobody declared is the root cause of most analytical defects, and it never raises an error.
- **Fan-out** — joining to a table whose key is not unique multiplies rows. Every subsequent SUM, AVG, and ratio is inflated; the query succeeds and the number looks like a good quarter.
- **Aggregation after fan-out** — the most damaging variant, because the aggregate hides the row multiplication that caused it. Check the key uniqueness, not the aggregate's plausibility.
- **Distinct-count as a false comfort** — `COUNT(DISTINCT id)` survives fan-out, so it is often correct while every other measure in the same query is wrong. Its correctness is frequently taken as evidence that the join is fine.
- **Outer joins converted to inner** — a predicate on the outer side in the WHERE clause eliminates the null-extended rows, silently dropping the very records the outer join was written to keep. The fix is a predicate in the join condition or a null-tolerant filter.
- **Window frame defaults** — a window with an ORDER BY and no explicit frame does not always mean what the author assumed. State the frame explicitly for any running total, ranking, or lag-based metric.
- **Null semantics** — nulls are excluded from most aggregates, are not equal to each other, and change the result of NOT IN. Each of these produces a quietly different number rather than an error.
- **Time zones and business calendars** — the metric's day must be the business's day. A UTC-truncated day against a business calendar in another zone produces a persistent small error that survives every review because it is small.

## Checking rather than trusting

- Uniqueness check for every declared many-to-one relationship: count rows against count of distinct keys on the 'one' side. A single duplicate invalidates every metric built on the relationship.
- Row-count check across a join: if the output row count differs from the driving table's row count in a join intended to preserve grain, the grain changed.
- Independent reconciliation: compute the metric a second way, ideally by someone who has not seen the first implementation, and compare. This finds the errors that reviewing the same SQL twice does not.
- Boundary check: the first and last period, the smallest and largest category, and the rows with nulls in the join key. Defects concentrate at the edges.

## Evidence queries

Verify a declared many-to-one relationship before trusting any metric built on it.

```sql
SELECT COUNT(*)                        AS rows_total,
       COUNT(DISTINCT airport_code)    AS distinct_keys,
       COUNT(*) - COUNT(DISTINCT airport_code) AS duplicate_rows
  FROM my_db.my_schema.airports;
-- duplicate_rows > 0 means every 'many-to-one' join to this table fans out,
-- and every SUM, AVG and ratio downstream of it is inflated.
```

Detect a grain change across a join intended to preserve it.

```sql
WITH driving AS (
  SELECT COUNT(*) AS n FROM my_db.my_schema.flights
),
joined AS (
  SELECT COUNT(*) AS n
    FROM my_db.my_schema.flights f
    JOIN my_db.my_schema.airports a
      ON a.airport_code = f.departure_airport
)
SELECT d.n AS driving_rows,
       j.n AS joined_rows,
       j.n - d.n AS rows_added_by_join
  FROM driving d, joined j;
-- rows_added_by_join <> 0 in a grain-preserving join is the defect.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/sql-reference/functions-analytic — Window function frame semantics and the defaults that differ from common author intent
