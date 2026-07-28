# Official Sources

Primary framework documentation and Context7 provenance for the sync/async model.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://fastapi.tiangolo.com/async/
- https://www.starlette.io/
- https://docs.djangoproject.com/en/stable/topics/security/
- https://flask.palletsprojects.com/en/stable/

## Provenance notes

- fastapi.tiangolo.com, starlette.io, docs.djangoproject.com, and flask.palletsprojects.com are the authoritative upstreams; the framework-specific reference is loaded only when the framework is detected in the artifacts.
- Context7 MCP provenance — library ID `/websites/fastapi_tiangolo` (source reputation High), retrieved 2026-07-26. Query: def vs async def path operations and the external threadpool. Confirmed: a `def` path operation runs in an external threadpool while an `async def` runs on the event loop, so blocking calls belong in `def` or must be offloaded. Limitation: framework internals change across releases — the applicable framework version must be confirmed from the user's dependencies.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
