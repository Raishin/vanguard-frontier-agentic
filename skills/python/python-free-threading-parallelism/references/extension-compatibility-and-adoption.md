# Extension Compatibility And Adoption

Declaring free-threaded support in a C-extension and grounding the adopt/pilot/defer verdict.

- A C-extension must be built for the free-threaded build and declare support via `Py_mod_gil` (`Py_MOD_GIL_NOT_USED`) or `PyUnstable_Module_SetGIL`, else importing it re-enables the GIL.
- The free-threaded build (3.13, `t` suffix) is experimental and needs pip 24.1+.
- The adopt/pilot/defer verdict rests on workload parallelism, dependency support, and thread-safe test coverage.

## Sources

- https://docs.python.org/3/howto/free-threading-extensions.html
- https://docs.python.org/3/whatsnew/3.13.html
