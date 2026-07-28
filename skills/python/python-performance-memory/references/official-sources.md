# Official Sources

Primary Python documentation for profiling, memory, and garbage-collection claims.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.python.org/3/library/profile.html
- https://docs.python.org/3/library/tracemalloc.html
- https://docs.python.org/3/library/gc.html
- https://docs.python.org/3/library/timeit.html

## Provenance notes

- docs.python.org (profile, tracemalloc, gc, timeit) is the authoritative upstream for the profiling, memory, and garbage-collection semantics in this skill.
- Context7 NOT separately used — the profiling/tracemalloc/gc semantics are stable stdlib behaviour quoted from docs.python.org.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
