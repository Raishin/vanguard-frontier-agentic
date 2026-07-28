# Free-Threading Review Checklist

The per-concern checklist applied to every free-threading adoption review.

- GIL assumptions: shared mutable state that relied on the GIL for safety is re-guarded with explicit synchronization on the free-threaded build.
- Extension declaration: every native dependency declares GIL-disabled support (`Py_mod_gil`/`PyUnstable_Module_SetGIL`); none silently re-enables the GIL.
- Read-modify-write: shared counters and non-atomic container mutations are synchronized, not assumed single-threaded.
- Critical sections: extension code iterating shared containers uses `Py_BEGIN_CRITICAL_SECTION`/`Py_END_CRITICAL_SECTION`.
- Maturity: the build is treated as experimental (3.13t, pip 24.1+), with adoption piloted, not defaulted.
- Workload fit: the workload profile (CPU-bound, low contention) is established before recommending adoption.
