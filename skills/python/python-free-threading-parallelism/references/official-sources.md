# Official Sources

Primary CPython free-threading documentation and Context7 provenance.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.python.org/3/howto/free-threading-python.html
- https://docs.python.org/3/howto/free-threading-extensions.html
- https://peps.python.org/pep-0703/
- https://docs.python.org/3/whatsnew/3.13.html

## Provenance notes

- docs.python.org (free-threading how-to guides, What's New in 3.13) and peps.python.org (PEP 703) are the authoritative upstreams for free-threaded/no-GIL semantics in this skill.
- Context7 MCP provenance — library ID `/python/cpython` (version `v3.13.9`, source reputation High), retrieved 2026-07-26. Query: free-threaded build (PEP 703), Py_mod_gil, C-extension compatibility, shared-state thread safety. Confirmed: on Py_GIL_DISABLED builds a C-extension must declare GIL-disabled support via the `Py_mod_gil` slot (`Py_MOD_GIL_NOT_USED`) or `PyUnstable_Module_SetGIL`, and importing a non-declaring extension re-enables the GIL unless overridden by PYTHON_GIL=0 / -X gil=0; the free-threaded build uses a `t` suffix (python3.13t) and requires pip 24.1+; shared containers need `Py_BEGIN_CRITICAL_SECTION`. Limitation: free-threading is experimental and evolving across 3.13/3.14 — the applicable interpreter build must be confirmed from the user's environment.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
