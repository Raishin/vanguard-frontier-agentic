# Official Sources

Primary CPython asyncio documentation and Context7 provenance for the version-sensitive claims.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.python.org/3/library/asyncio-task.html
- https://docs.python.org/3/library/asyncio-eventloop.html#asyncio.loop.run_in_executor
- https://docs.python.org/3/library/asyncio-task.html#timeouts
- https://docs.python.org/3/library/asyncio-task.html#task-groups

## Provenance notes

- docs.python.org (CPython asyncio) is the authoritative upstream for every claim in this skill; version-gated features are labelled with the Python version that introduced them.
- Context7 MCP provenance — library ID `/python/cpython` (version `v3.13.9`, source reputation High), retrieved 2026-07-26. Queries: asyncio.timeout cancellation semantics; asyncio.TaskGroup exception propagation; loop.run_in_executor for blocking calls. Confirmed: `asyncio.timeout()` cancels only its enclosed block and raises `TimeoutError`; `TaskGroup` aborts siblings on first error and raises an `ExceptionGroup`; `run_in_executor(None, fn)` offloads blocking/CPU-bound work. Limitation: Context7 indexes the documented behaviour, not the user's installed interpreter version — the applicable version must be confirmed from the user's environment.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
