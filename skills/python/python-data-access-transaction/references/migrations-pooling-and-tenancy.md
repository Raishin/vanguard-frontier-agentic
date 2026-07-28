# Migrations, Connection Pooling, And Multi-Tenancy

Safe schema migrations, pool sizing, and tenant scoping.

- A schema migration on a large table can hold a lock: adding a NOT NULL column with a default, building an index non-concurrently, or rewriting a type can block reads/writes for the duration — the safe pattern is expand-then-contract (add nullable, backfill in batches, then enforce) with a reversible downgrade.
- The connection pool size plus max overflow bounds concurrent database connections; it must fit under the database's connection limit and match the worker/concurrency model, and every connection must be returned (context-managed) or the pool leaks and exhausts.
- In a multi-tenant schema, every query must carry the tenant/row scope; applying it centrally (a default filter or a required predicate) prevents a single unscoped query from leaking or mutating another tenant's data.

## Sources

- https://docs.sqlalchemy.org/en/20/orm/queryguide/relationships.html
- https://alembic.sqlalchemy.org/en/latest/
