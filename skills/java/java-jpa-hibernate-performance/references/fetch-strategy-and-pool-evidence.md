# Fetch-Strategy and Pool Evidence

> Static review only. Every conclusion here needs the mapping *and* the query site as evidence; a fetch claim without both is `inference` or `assumption`. Sources: Hibernate ORM User Guide, Jakarta Persistence spec, Spring Data JPA reference, HikariCP pool-sizing notes (see the skill's `official_docs`).

## Why this decision matters

The same association can be correct or catastrophic depending on how it is fetched *for a given access shape*. There is no globally right fetch strategy — the review picks the remedy that matches the query, and rejects reflexive fixes (blanket EAGER, global lazy-off, re-enabling open-in-view) that trade one pathology for a worse one.

## The N+1 remedy decision

When a lazy association is accessed per row (a loop over a list, a serializer walking a graph, a derived query whose results are then traversed), you have N+1. Choose the remedy by shape:

| Situation | Remedy | Why |
|---|---|---|
| Single association needed, small fan-out, no pagination | `JOIN FETCH` (JPQL) or an `@EntityGraph` on the repository method | One query, association materialized; `@EntityGraph` keeps it declarative and query-specific |
| Association needed across **many** parents, or pagination required | `@BatchSize` (or `hibernate.default_batch_fetch_size`) | Loads children in `IN (…)` batches; composes with pagination because the root query is unaffected |
| Only a few scalar fields needed, read-only | DTO / interface **projection** (constructor expression or Spring Data projection) | Skips entity hydration entirely; no lazy state to manage |
| Multiple collections needed at once | Separate queries or `@BatchSize`, **not** two `JOIN FETCH`es | Avoids cartesian explosion and `MultipleBagFetchException` |

`@EntityGraph` vs `JOIN FETCH`: prefer `@EntityGraph` when the same entity needs different fetch plans on different methods (declarative, per-query); use `JOIN FETCH` for one-off JPQL. Both collapse N+1 to one query for the single-collection case.

## Dangerous patterns (flag these)

- **Pagination + collection `JOIN FETCH`.** `setMaxResults`/`Pageable` over a query that `JOIN FETCH`es a collection makes Hibernate paginate **in memory** after loading the full cartesian product (log signature `HHH000104: firstResult/maxResults specified with collection fetch; applying in memory`). Remedy: fetch page of root IDs first, then a second query fetching the collection for those IDs; or `@BatchSize`.
- **`MultipleBagFetchException`.** Two `List`/bag collections `JOIN FETCH`ed in one query. Remedy: model as `Set`, or fetch the collections in separate queries.
- **`FetchType.EAGER` on collections.** Forces the association on every load path, defeats query-specific tuning, and hides N+1 behind "it just works." Default collections LAZY; fetch explicitly.
- **Leaning on `open-in-view`.** `spring.jpa.open-in-view=true` (Spring Boot's default when unset in a web app) keeps the persistence context — and the JDBC connection — open through view rendering so lazy access "works." It masks N+1 and holds a pooled connection for the entire request. Remedy: set `open-in-view=false` and fetch what each endpoint needs explicitly. Treat any `LazyInitializationException` "fix" that just re-enables OIV as a regression, not a fix.
- **Second-level cache as an N+1 band-aid.** Recommending 2LC to hide N+1 without stating the read-consistency and invalidation trade-off is not a remedy — name the trade-off or don't recommend it.

## Connection-pool (HikariCP) sizing evidence

Pool size is not a round number; it follows from concurrency and connection **hold time**.

- **Over-provisioned** pools (e.g. `maximumPoolSize` far above the database's `max_connections` divided by instance count) exhaust the database's connection budget and can starve other services. `maximumPoolSize × instanceCount` must fit inside the database's `max_connections` with headroom.
- **Under-provisioned** pools queue requests behind `connectionTimeout`; the symptom is `Connection is not available, request timed out` under load, not slow SQL.
- HikariCP guidance is that a **small** pool sized to actual concurrency usually outperforms a large one; size from measured mean connection hold time and target throughput, not from thread count.
- Flag missing `connectionTimeout` (fail-fast bound) and `leakDetectionThreshold` (surfaces connections held too long — often the real cause of "pool exhaustion" is a leak or an over-long `@Transactional`/open-in-view scope, not pool size).

State pool findings as `inference` unless the user provides both the pool config *and* the database's `max_connections`/instance count; recommend measuring hold time rather than asserting a number.

## Escalation conditions

- The slow path is a `@Transactional` boundary/propagation/isolation problem rather than fetch shape → hand to `java-transaction-and-consistency-agent`.
- The evidence points at raw SQL built from user input → hand to the security agents (injection is not this agent's verdict).
- The user asks to actually run `EXPLAIN`/profiling against a live database → out of scope for static review; describe what to measure and who runs it.
