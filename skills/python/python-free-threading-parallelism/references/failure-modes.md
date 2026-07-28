# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A shared counter incremented from multiple threads without a lock silently corrupts under the free-threaded build after years of being safe under the GIL.
- A native dependency with no `Py_mod_gil` declaration is imported, the GIL silently re-enables, and the expected parallelism speedup never materializes — with no error to explain why.
- An extension iterating a shared list without a critical section crashes intermittently once the GIL is removed.
- A team force-disables the GIL with `PYTHON_GIL=0` against an unsupported extension and ships a crash that never appeared in testing on the standard build.
- An I/O-bound service is moved to free-threading expecting a speedup, sees none, and pays the migration cost for zero benefit because the workload was never profiled.
