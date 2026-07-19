# Expand-Contract and Destructive DDL

> Static review only. This note covers the phasing decision for column/table drops and renames, and the same-release destructive-DDL rule, under a rolling or blue-green deploy where two application versions can run against the same schema simultaneously. Sources: the Flyway documentation's migrations concept page and the Liquibase documentation (see the skill's `official_docs`). Engine-specific locking behavior (MySQL/InnoDB, PostgreSQL, etc.) is not covered here in generalized form because it varies by engine and version — flag it as `inference` unless the user states engine and version.

## Why phasing exists

In a rolling deploy, for some window both the old and the new application binary serve traffic against the *same* database schema. A migration that a single application version could apply safely in isolation can still break the deploy if the *other*, not-yet-redeployed version depends on the schema shape the migration just changed. The review's job is to find every place a migration assumes single-version deployment and require it to instead assume mixed-version deployment for the duration of the rollout.

## The expand-contract sequence

For any column or table drop, and for any rename, require this sequence across releases — never inside one:

| Phase | What happens | Release |
|---|---|---|
| Expand | Add the new column/table. If replacing an existing one, dual-write to both old and new. | N |
| Migrate | Backfill existing rows into the new shape. Switch reads to the new shape. Old writes may still be dual-written. | N (or N+1) |
| Contract | Remove all application reads/writes of the old shape. Ship and let this version roll out to 100%. | N+1 (or N+2) |
| Drop | Migration drops the old column/table. Safe now because no running version references it. | N+2 (or later) |

A migration is a defect if it performs the Drop-phase DDL in the same release as the Contract-phase code change, because the previous version — still live during rollout — has not yet stopped using the old shape.

```sql
-- Unsafe: same release adds NOT NULL constraint on a column
-- the old (still-deploying) version does not populate yet.
ALTER TABLE orders ADD COLUMN fulfillment_status VARCHAR(32) NOT NULL DEFAULT 'PENDING';
-- ...and the PR also removes the last read of orders.legacy_status,
-- then drops it, in the same migration set.
ALTER TABLE orders DROP COLUMN legacy_status;
```

```sql
-- Safe: expand only. Contract (removing legacy_status reads) and the
-- drop are separate, later releases after this one has fully rolled out.
ALTER TABLE orders ADD COLUMN fulfillment_status VARCHAR(32) NOT NULL DEFAULT 'PENDING';
```

## Renames are drop-and-add in disguise

`ALTER TABLE ... RENAME COLUMN`, Liquibase's `renameColumn`/`renameTable`, or a manual DROP+ADD all present the same hazard: the old name stops resolving the instant the DDL runs, but the not-yet-redeployed application version still references the old name. Treat a bare rename exactly like a drop for phasing purposes — it needs the same expand (new name, dual-write or a compatibility view), migrate, contract, cleanup sequence. There is no single-statement rename that is safe across a rolling deploy.

## Destructive DDL inventory

Treat each of these as destructive and subject to the same-release rule above:

- `DROP COLUMN`, `DROP TABLE`.
- Type-narrowing `ALTER COLUMN` (e.g. `VARCHAR(255)` to `VARCHAR(50)`, `BIGINT` to `INT`, a widening-precision-loss numeric change) — data that fits today may not fit, and the old version may still write values that no longer fit.
- `NOT NULL` added to a column on a table that already has rows, without a `DEFAULT` and without a prior backfill — rows written by the old version before the constraint existed may violate it, and depending on engine/version this can also be a long-locking full-table operation (see the integrity-and-ordering reference for lock-risk).

## Additive-first as the default posture

When there is no explicit business reason to remove something in this release, default every migration to additive: new columns nullable or defaulted, new tables independent of any drop. If a PR description or commit message says "add X and remove old X" in one release, that is a same-release destructive-DDL violation regardless of how small the old column looks.

## Escalation conditions

- The reviewed migration is correct in isolation but the application code paths that would make it safe (dual-write, backfill job, code removal) are not provided — downgrade the phasing verdict to `inference (partial source)` and ask for the code, not just the migration.
- The question becomes "is this ORM mapping correct for this new column" rather than "is this migration deploy-safe" — hand off to `java-jpa-hibernate-performance-agent`.
- The question becomes "does the backfill job need to run inside a transaction with what isolation level" — hand off to `java-transaction-and-consistency-agent`.
