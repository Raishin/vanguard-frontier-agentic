# Migration Integrity and Ordering

> Static review only. This note covers applied-migration immutability, out-of-order hazards, repeatable-migration idempotency, DDL lock-risk on large tables, and the rollback/backfill requirement. Sources: the Flyway documentation and the Liquibase documentation (see the skill's `official_docs`). Any claim tied to a specific database engine's locking behavior, a specific Flyway/Liquibase edition's feature set, or a specific version's default behavior is time- and version-sensitive — verify it against the current official documentation rather than asserting it from memory, and mark it `unknown` if the user has not stated engine, version, or edition.

## Applied migrations are immutable

Both tools track what has run: Flyway's schema-history table stores a checksum per versioned migration; Liquibase's `DATABASECHANGELOG` table keys each changeSet by `id` + `author` + file path and also stores a checksum (`MD5SUM`). Once a migration has been recorded as applied in *any* environment, editing its body changes what the tool will compute on the next validate/update, which surfaces as:

- Flyway: a `validate` failure (checksum mismatch) — or, worse, silent drift if validation is skipped.
- Liquibase: a checksum warning or failure depending on configuration, or, if the changeSet id/author/path also changed, a *second, unintended* run of what looks like new content.

The only safe correction is a new migration that changes the schema further — never an edit to the old file, and never a checksum-repair command used to make the mismatch disappear. `flyway repair` and Liquibase checksum-clearing commands exist for recovering from legitimate tooling issues (e.g. a genuinely failed migration), not for absorbing a hand-edited already-applied file; treat a PR that reaches for either to resolve an edited-migration mismatch as compounding the problem.

## Out-of-order hazards

Versioned migrations are meant to apply in strictly ascending version order. A new file numbered lower than one already applied in a target environment is out-of-order. This commonly happens when two branches each add a migration off the same base version and merge without renumbering. Both tools support explicitly allowing out-of-order application, but that setting only makes the *symptom* (a validate failure) go away — it does not verify that the out-of-order migration is semantically safe to run after migrations that, in that environment, already ran ahead of it. Require: renumbering to preserve strict order where practical, or, if out-of-order is deliberately enabled, evidence that the ordering was checked against every target environment's actual applied-migration history (not just the developer's local database, which may have a different applied set).

## Repeatable migrations must be idempotent

Flyway repeatable migrations (`R__` prefix) and Liquibase changeSets marked to run `runOnChange`/`runAlways` are, by design, re-executed whenever their checksum changes or on every run respectively. DDL inside them must tolerate reapplication:

```sql
-- Unsafe repeatable migration: fails on second run in any environment
-- where it already applied once.
CREATE VIEW active_orders AS SELECT * FROM orders WHERE status = 'ACTIVE';
```

```sql
-- Safe: idempotent by construction.
CREATE OR REPLACE VIEW active_orders AS SELECT * FROM orders WHERE status = 'ACTIVE';
```

The same applies to Liquibase repeatable changeSets — prefer `preConditions` with `onFail: MARK_RAN` or idempotent statements (`CREATE OR REPLACE`, `IF NOT EXISTS` guards where the dialect supports them) over unconditional CREATE/DROP.

## Lock-risk DDL on large tables

Whether a given ALTER TABLE blocks reads/writes for its duration, or completes near-instantly via metadata-only change, is entirely dependent on the database engine, its version, and the specific operation — this is exactly the kind of fact that changes across releases and must not be asserted from memory. What generalizes across engines:

- Adding an index without an online/concurrent build path is a common source of long locks on large tables; most mainstream engines offer some non-blocking or reduced-blocking index-build mode — cite the target engine's current documentation for the exact syntax and guarantees rather than assuming behavior.
- A column type change that forces a full table rewrite scales with table size and can hold locks proportionally long.
- The safe default when table size or engine/version is unknown is to ask, and in the meantime to flag the DDL as lock-risk `inference (partial source)` rather than asserting a specific lock duration.

## Rollback and backfill plan requirement

Flyway's official documentation places automatic undo-migration support behind specific edition tiers, and Liquibase relies on either an authored `rollback` block in the changelog or its automatic rollback generation for a subset of change types — both the availability and the exact supported-tier/change-type set are the kind of vendor detail that changes over time. Do not state a specific edition capability or feature-availability claim from memory: cite the official documentation page by name and instruct the user to verify current support, or mark the capability `unknown` and require the PR to state its recovery plan explicitly (a compensating forward migration, a documented manual procedure, or a verified working `rollback`/undo mechanism).

## Escalation conditions

- The integrity question turns into "is this backfill query's isolation level correct" — hand off to `java-transaction-and-consistency-agent`.
- The lock-risk question requires an actual `EXPLAIN`, lock-wait measurement, or live table-size query — out of scope for static review; describe what to measure and who should run it.
- The migration is fine but the entity mapping consuming the new column is suspect — hand off to `java-jpa-hibernate-performance-agent`.
