# Official Sources

Primary SQLAlchemy and Alembic documentation and Context7 provenance.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.sqlalchemy.org/en/20/orm/session_basics.html
- https://docs.sqlalchemy.org/en/20/orm/session_transaction.html
- https://docs.sqlalchemy.org/en/20/orm/queryguide/relationships.html
- https://alembic.sqlalchemy.org/en/latest/

## Provenance notes

- docs.sqlalchemy.org (2.0) and alembic.sqlalchemy.org are the authoritative upstreams; Django ORM behaviour must be confirmed against docs.djangoproject.com when the code uses Django.
- Context7 MCP provenance — library ID `/websites/sqlalchemy_en_20` (source reputation High), retrieved 2026-07-26. Queries: Session transaction lifecycle (autobegin, commit/rollback, per-request scope) and avoiding N+1 with selectinload/joinedload. Confirmed: commit-as-you-go autobegin with explicit commit/rollback; per-request Session scope; `selectinload`/`joinedload` eager loading to eliminate N+1. Limitation: pool and lock behaviour depend on the specific database and driver, which must be confirmed from the user's environment.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
