# Workflow and Output Contract

> Static review only. Read entity classes, mappings, repositories, queries, and sanitized configuration. Never open a database connection, run a query, or execute a migration. Ask for source with placeholders — never connection strings, credentials, tenant identifiers, or customer data.

## Workflow

### Step 1 — Collect inputs

Ask the user for whichever apply, sanitized:
- Entity classes with their association mappings (`@OneToMany`, `@ManyToOne`, `@ManyToMany`, `fetch`/`cascade`, `@BatchSize`).
- The repositories / query sites (Spring Data method names, `@Query` JPQL, Criteria, `@EntityGraph` usage).
- The call sites that traverse associations (services, serializers/mappers, controllers).
- Relevant configuration: `spring.jpa.open-in-view`, `spring.jpa.properties.hibernate.*` (batch size, default_batch_fetch_size), HikariCP `maximumPoolSize`/`connectionTimeout`/`leakDetectionThreshold`, and the database's `max_connections` + instance count if a pool question is in scope.

If the mapping or the query site is missing for an association under discussion, downgrade that finding to `inference (partial source)` or `assumption (source absent)` and say so.

### Step 2 — Map associations and their fetch types

For each association, record: type, declared `FetchType`, whether it is a collection (bag/list vs set), and whether any query overrides the default with `JOIN FETCH`/`@EntityGraph`/`@BatchSize`.

### Step 3 — Trace access shape

For each association, determine how it is actually accessed: once per aggregate, per row in a loop, across a paginated result, or only for a few scalar fields. The access shape — not the mapping alone — determines the correct remedy (see `fetch-strategy-and-pool-evidence.md`).

### Step 4 — Detect the pathologies

- N+1 (lazy access per row / walked derived-query results).
- Eager collections.
- Pagination combined with a collection `JOIN FETCH` (`HHH000104`, in-memory pagination).
- Two bag `JOIN FETCH`es (`MultipleBagFetchException`).
- `open-in-view` leaned on to resolve lazy state in the view.
- DTO-projection opportunities on read-only paths.

### Step 5 — Assess connection reliability

If pool config is provided, check `maximumPoolSize` against database `max_connections`/instance count and per-request hold time; check for missing `connectionTimeout`/`leakDetectionThreshold`. Remember the most common cause of "pool exhaustion" is a long-held connection (over-broad transaction or open-in-view), not a small pool.

### Step 6 — Produce the output

Format using the Output contract below. Pick each remedy by access shape; never recommend blanket EAGER, global lazy-off, or re-enabling open-in-view.

## Evidence checklist

- [ ] Entity association mappings
- [ ] Repository / query sites
- [ ] Association access/call sites
- [ ] `open-in-view` and Hibernate batch settings
- [ ] Pool config + database `max_connections`/instance count (if pool is in scope)

Each unchecked item downgrades the related findings to `inference` or `assumption`.

## Findings rubric

| Severity | Criteria |
|----------|----------|
| high | N+1 on a request path; eager collection; pagination + collection fetch (in-memory pagination); `MultipleBagFetchException` risk; open-in-view leaned on to hide lazy access. |
| medium | Mis-sized/unbounded HikariCP pool relative to database capacity; missing `connectionTimeout`/`leakDetectionThreshold`; entity hydration where a projection suffices. |
| low | `hibernate.jdbc.batch_size` unset on a bulk path; tracking/first-level-cache overhead on a large read. |

Every finding carries an evidence-basis label: `confirmed (source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

## Output contract

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<full source | partial source | inference>

## Findings

### HIGH
- [H1] <finding> — <evidence basis> — <access shape> — <chosen remedy: JOIN FETCH / @EntityGraph / @BatchSize / DTO / two-query>

### MEDIUM
- [M1] <finding> — <evidence basis> — <description> — <remediation>

### LOW
- [L1] <finding> — <evidence basis> — <description> — <remediation>

## Safe next actions
1. <action>

## Open questions
- <mapping/query/config the user must supply>
```

## Security notes

- Never request or accept connection strings, database credentials, tenant identifiers, or customer data. Ask for source with placeholders.
- Static review only: never open a database connection, run a query, or execute a migration.
- Never recommend blanket `EAGER`, disabling lazy loading globally, or re-enabling `open-in-view` to silence `LazyInitializationException`.
- Never recommend disabling a failing gate as the fix.
