# Workflow and Output Contract

> Static review only. Read migration scripts/changelogs, sanitized schema snapshots, and the application source that reads or writes the affected columns. Never open a database connection, run a migration, or query live schema state. Ask for source and schema snapshots with placeholders — never connection strings, credentials, tenant identifiers, or customer data.

## Workflow

### Step 1 — Collect inputs

Ask the user for whichever apply, sanitized:
- The migration file(s) under review: Flyway versioned (`V*`), undo (`U*`), or repeatable (`R__`) scripts (SQL or Java-based), or the Liquibase changelog file(s) (XML/YAML/JSON/formatted SQL) plus the master changelog reference if relevant.
- The current schema shape for the affected table(s) — a `CREATE TABLE`/`DESCRIBE` snapshot or equivalent, and, if lock-risk is in question, an approximate row count and the target database engine + version.
- The application code paths that read or write the columns/tables the migration touches, especially anything the migration would drop, rename, narrow, or constrain.
- The rollout mechanism (rolling vs blue-green) and, if known, whether the migration runs before, during, or strictly after traffic cutover.
- Any stated rollback or backfill plan.

If the migration is provided without the corresponding application code, or the schema/engine context needed for a lock-risk call, downgrade the affected findings to `inference (partial source)` or `assumption (source absent)` and say so explicitly.

### Step 2 — Classify every DDL statement

For each statement in the migration, classify it as: additive (new column/table, nullable or defaulted), destructive (drop, type-narrow, NOT NULL without default on a populated table), a rename, an index/constraint change, or bulk DML. Note whether it targets a table already known or plausible to be large.

### Step 3 — Check applied-migration integrity

Determine whether this migration is new or an edit to a file that has already been applied in any environment (ask if unclear — do not assume). Check version ordering against the highest version known to be applied. If the migration is repeatable, check every statement for idempotency.

### Step 4 — Trace destructive DDL against live usage

For every destructive or rename statement found in Step 2, search the provided application code for reads/writes of the old shape. If any are found and are not confirmed to be already fully rolled out to 100% in a prior release, this is a same-release destructive-DDL violation — require expand-contract phasing instead (see `expand-contract-and-destructive-ddl.md`).

### Step 5 — Assess lock risk and recovery plan

Using the table size/engine context from Step 1, flag DDL plausible to hold a long lock on a large table and check for an online/concurrent variant. Check for a stated rollback or backfill plan; if the PR relies on an undo/rollback capability, treat its availability as a claim to verify against current official docs, not to assume.

### Step 6 — Produce the output

Format using the Output contract below. Every remedy recommended must be the phased/additive path, never a same-release drop, an unphased rename, or a suppressed validation gate.

## Evidence checklist

- [ ] Migration file(s) under review, with clear applied/not-yet-applied status
- [ ] Current schema snapshot for affected table(s)
- [ ] Application code paths reading/writing affected columns
- [ ] Table size / database engine + version (if lock-risk is in scope)
- [ ] Rollout mechanism (rolling vs blue-green) and migration-timing relative to cutover
- [ ] Stated rollback/backfill plan

Each unchecked item downgrades the related findings to `inference` or `assumption`.

## Findings rubric

| Severity | Criteria |
|----------|----------|
| critical | Edited already-applied migration (checksum/history drift); destructive DDL shipped in the same release the application still uses the old shape; recommending disabling a failing validation gate. |
| high | Unphased column/table rename or drop (missing expand-contract phase); missing or unverified rollback/backfill plan; long-locking DDL on a plausible-large table without an online variant; out-of-order migration without verified cross-environment ordering. |
| medium | Non-idempotent repeatable migration; same-release add-and-remove pairing; NOT NULL added without a default on a populated table; mis-sequenced additive-first violation. |
| low | DDL mixed with unbatched bulk DML in one script; minor lock-duration/rollback-blast-radius risk from unbatched backfill. |

Every finding carries an evidence-basis label: `confirmed (source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

## Output contract

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<full source | partial source | inference>

## Findings

### CRITICAL
- [C1] <finding> — <evidence basis> — <affected migration/statement> — <required remediation>

### HIGH
- [H1] <finding> — <evidence basis> — <affected migration/statement> — <required phasing/remediation>

### MEDIUM
- [M1] <finding> — <evidence basis> — <description> — <remediation>

### LOW
- [L1] <finding> — <evidence basis> — <description> — <remediation>

## Safe next actions
1. <action>

## Open questions
- <migration/schema/code the user must supply>
```

## Security notes

- Never request or accept connection strings, database credentials, tenant identifiers, or customer data. Ask for migration files and schema snapshots with placeholders.
- Static review only: never open a database connection, run a migration, or query live schema state.
- Never recommend editing an already-applied migration, collapsing expand-contract phasing into one release, or dropping a column/table still in use by any deploying version.
- Never recommend disabling a failing validation gate (Flyway validate, Liquibase preConditions, a CI migration-safety check) as the fix.
