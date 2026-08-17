# Account Parameters and Resolution

Why a parameter's effective value is not the value you read at the account level, and how to establish the real one. Load whenever a setting is asserted.

## The resolution problem

- Snowflake parameters resolve through a hierarchy — account, then object (warehouse, database, schema, user, and so on), then session. A lower level overrides the higher one.
- The consequence: an account-level standard can be perfectly configured while the workload that matters runs under a session or object override that defeats it. Reading only `SHOW PARAMETERS IN ACCOUNT` produces a confident wrong answer.
- Always report both the effective value and the level it resolved at. `SHOW PARAMETERS` output includes the level, which is the whole point of using it.
- Parameters that are silently load-bearing for correctness deserve a written justification: timezone and timestamp handling, week and date semantics, statement timeouts, data retention, and anything governing how a query interprets or truncates a value.
- A parameter nobody can justify becomes a parameter nobody dares change. That is how a workload stops being maintainable — the finding is the missing justification, not the value.

## Evidence queries

Read parameters with their resolution level rather than assuming the account value is effective.

```sql
-- The LEVEL column is the point of this query, not the VALUE column.
SHOW PARAMETERS IN ACCOUNT;
SHOW PARAMETERS IN WAREHOUSE my_wh;
SHOW PARAMETERS IN DATABASE my_db;
SHOW PARAMETERS FOR USER my_service_user;

-- Anything whose LEVEL is not ACCOUNT is an override worth a written reason.
SELECT "key", "value", "level"
  FROM TABLE(RESULT_SCAN(LAST_QUERY_ID()))
 WHERE "level" <> 'ACCOUNT';
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/sql-reference/parameters — The parameter hierarchy, the levels at which each parameter may be set, and their defaults
