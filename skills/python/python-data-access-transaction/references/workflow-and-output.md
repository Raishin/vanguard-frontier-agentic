# Review Workflow And Output Contract

The data-access review workflow and the required output shape.

## Workflow

1. Identify the ORM/toolkit, the session/engine configuration, and the unit-of-work boundary assumed.
2. Check session scope and that every write path commits on success and rolls back on error (autobegin).
3. Trace relationship access for N+1 and confirm the eager-loading strategy; check for unbounded reads and missing pagination.
4. Check connection-pool sizing and that connections are context-managed; check multi-tenant query scoping.
5. Check each migration for blocking DDL, reversibility, and expand-then-contract, and record every claim needing a real database to confirm.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the ORM/toolkit and database assumed.
- Session/transaction, N+1/lazy-loading, connection-pool, and migration/multi-tenancy findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any query-count/lock claim the user must confirm against a real database.
