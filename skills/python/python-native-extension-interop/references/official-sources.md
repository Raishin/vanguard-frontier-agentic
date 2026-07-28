# Official Sources

Primary CPython C-API documentation for native-extension review.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.python.org/3/c-api/intro.html
- https://docs.python.org/3/c-api/stable.html
- https://docs.python.org/3/c-api/refcounting.html
- https://docs.python.org/3/c-api/buffer.html

## Provenance notes

- docs.python.org/3/c-api (intro, stable ABI, refcounting, buffer) is the authoritative upstream for CPython C-API ownership, buffer-protocol, and stable-ABI semantics in this skill.
- Context7 NOT separately used — the C-API ownership/buffer/stable-ABI semantics are quoted from the docs.python.org C-API reference (primary upstream); free-threaded extension specifics are cross-referenced with the free-threading how-to. PyO3/Cython specifics must be confirmed against their own documentation.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
