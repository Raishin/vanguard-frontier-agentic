# Session Scope, Transactions, And N+1

SQLAlchemy 2.0 session lifecycle, autobegin transactions, and eager loading.

- The SQLAlchemy 2.0 guidance for web applications is to create a Session at the start of a request, commit on write operations, and close it at the end of the request; a Session is a unit-of-work boundary, not a long-lived shared object.
- In commit-as-you-go style, the Session autobegins a transaction on first database access and starts a new one after each commit/rollback; `with session.begin():` commits on success and rolls back on exception, making the boundary explicit.
- Lazy loading of a relationship emits a query on first access, so reading it across a collection produces N+1 queries; `selectinload` (a second batched SELECT) or `joinedload` (a JOIN) eager-loads the relationship in the initial query and eliminates the per-row queries.

## Sources

- https://docs.sqlalchemy.org/en/20/orm/session_basics.html
- https://docs.sqlalchemy.org/en/20/orm/session_transaction.html
