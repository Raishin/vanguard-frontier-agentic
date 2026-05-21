# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized source files (no connection strings, no database credentials, no tenant identifiers, no customer data — replace with placeholders):
- The `DbContext` class(es) and `OnModelCreating` / `IEntityTypeConfiguration` entity configuration.
- The DI registration where the `DbContext` is added (`AddDbContext`, `AddDbContextPool`, `AddDbContextFactory`, or a manual registration).
- The migration files and the model snapshot, if available.
- Repository, service, or query code that reads and writes entities.
- Optional: the entity classes for any multi-tenant or contended aggregates under review.

If migrations or the model snapshot are not provided, model-vs-migration findings are stated as `assumption (source absent)` — say so and ask for them.

### Step 2 — DbContext lifetime and registration audit

Confirm the `DbContext` has a safe lifetime.

- `DbContext` registered as a singleton, or resolved once and shared across requests → CRITICAL. `DbContext` is not thread-safe; concurrent use corrupts the change tracker.
- Expect `Scoped` registration (the `AddDbContext` default), or a pooled/factory pattern (`AddDbContextPool`, `AddDbContextFactory`) where each unit of work gets its own instance.
- A `DbContext` captured by a singleton service → CRITICAL (captive dependency).

### Step 3 — Raw SQL injection-surface audit

Scan every `FromSqlRaw`, `ExecuteSqlRaw`, `SqlQueryRaw`, and ADO.NET command for user input concatenated or string-interpolated into the SQL text.

- Raw SQL built by concatenating or `$"..."`-interpolating user input → CRITICAL SQL-injection surface.
- Recommend parameterized `FromSql` / `FromSqlInterpolated` / `ExecuteSql`, or `{0}` placeholder parameters on the `Raw` variants — never string concatenation.

### Step 4 — Multi-tenant query-filter audit

For each entity that carries a tenant discriminator (`TenantId` or equivalent):

- No global query filter (`HasQueryFilter`) scoping reads to the current tenant → CRITICAL tenant-isolation failure: every query can return other tenants' rows.
- A query filter present but bypassed with `IgnoreQueryFilters` on a user-facing path → CRITICAL.
- Recommend a `HasQueryFilter` keyed to an ambient tenant accessor, applied in `OnModelCreating`.

### Step 5 — Query-shape audit

Review query patterns for performance defects.

- Lazy loading inside a loop, or a per-row query issued on a request path → HIGH N+1. Recommend eager loading (`Include`, `ThenInclude`, or projection to a DTO) or a single batched query.
- `.ToList` / `.ToArray` with no `Skip`/`Take` or keyset bound on user-facing data → HIGH unbounded result set. Recommend pagination.
- Tracking queries on read-only paths → LOW. Recommend `AsNoTracking` for reads only — never on write paths.
- Consider split vs. single queries where a `Include` produces a large cartesian product.

### Step 6 — Concurrency-token audit

For contended aggregates (rows updated by multiple concurrent writers):

- No concurrency token (`RowVersion` / `IsRowVersion` / `IsConcurrencyToken`) → HIGH lost-update risk: the last writer silently overwrites the others.
- Recommend a `RowVersion` token and a `DbUpdateConcurrencyException` handling path.

### Step 7 — Migration-discipline audit

- Pending model changes not captured in a migration (model-vs-migration drift) → HIGH: the schema and the model disagree, and the next deploy may fail or run against a stale schema.
- Destructive migration operations (column drops, type narrowing) with no stated backfill or rollback plan → HIGH.
- Recommend regenerating the migration and verifying the model snapshot matches.

### Step 8 — Connection-resiliency audit

- No `EnableRetryOnFailure` (or an equivalent execution strategy) configured against a cloud database → MEDIUM reliability gap: transient faults surface as hard failures.
- A retry strategy combined with a manually managed transaction without `CreateExecutionStrategy` → MEDIUM (retries can replay a partial transaction).
- Never recommend a retry to mask a transaction-boundary bug.

### Step 9 — Produce the output

Format findings using the Output contract section below.

---

## Evidence checklist

Before writing findings, confirm which inputs were actually provided:
- [ ] `DbContext` class and entity configuration
- [ ] DI registration of the `DbContext`
- [ ] Migration files and model snapshot
- [ ] Query / repository / service source
- [ ] Multi-tenant entity definitions

Each unchecked item downgrades the related findings to `inference (partial source)` or `assumption (source absent)`.

---

## Findings rubric

| Severity | Criteria |
|----------|----------|
| critical | String-interpolated raw SQL with user input; missing global query filter on a multi-tenant entity; singleton/captive `DbContext`. |
| high | N+1 query patterns; unbounded user-facing queries; missing concurrency token on contended aggregates; model-vs-migration drift; destructive migration with no rollback plan. |
| medium | Missing connection resiliency against a cloud database; retry strategy without an execution-strategy-wrapped transaction. |
| low | Tracking queries on read-only paths. |

Every finding carries an evidence-basis label: `confirmed (source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

---

## Output contract

Return findings in this structure:

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<full source provided | partial source | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding> — <evidence basis> — <description> — <remediation>

### HIGH
- [H1] <finding> — <evidence basis> — <description> — <remediation>

### MEDIUM
- [M1] <finding> — <evidence basis> — <description> — <remediation>

### LOW
- [L1] <finding> — <evidence basis> — <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request or accept connection strings, database credentials, tokens, tenant identifiers, or customer data. Ask for source files with placeholders.
- This is a static review: never run migrations, open a database connection, execute SQL, or contact a live database.
- A string-interpolated raw SQL call with user input is the highest-impact finding possible — lead with it and tell the user to stop shipping that path until it is parameterized.
- A missing multi-tenant query filter is a silent cross-tenant data leak; treat it as CRITICAL and tell the user every query on that entity is unsafe until the filter is in place.
- Never recommend disabling a failing gate or check as the fix.
