# Data-Access Review Checklist

The per-concern checklist applied to every data-access review.

- Session: scoped to a unit of work (per request), never shared across threads or held for the process lifetime.
- Transaction: every write path commits on success and rolls back on error; no error path leaves a poisoned transaction.
- N+1: relationships read across a collection use eager loading (`selectinload`/`joinedload`) or an explicit join.
- Reads: user-facing lists are paginated/bounded; no full-table load into memory.
- Pool: pool+overflow matches concurrency and the database limit; connections are context-managed (no leak).
- Migrations: reversible, non-blocking (expand-then-contract, `CONCURRENTLY` indexes); multi-tenant queries are always scoped.
