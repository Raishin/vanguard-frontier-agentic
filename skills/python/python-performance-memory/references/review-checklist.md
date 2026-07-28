# Performance-And-Memory Review Checklist

The per-concern checklist applied to every performance/memory review.

- Evidence: every performance claim cites a profile or benchmark identifying the hot path; no claim rests on intuition.
- Rigor: profiler time (cProfile) is never quoted as benchmark wall-time; a micro-benchmark is never generalized to production without a representative workload.
- Memory: a suspected leak is evidenced by a growing tracemalloc snapshot, not asserted.
- Complexity: an O(n^2) or worse hot-path pattern is flagged before any constant-factor tuning is considered.
- GC: garbage collection is never disabled as a fix without reference-cycle evidence from gc.
- Startup: expensive top-level imports and unmeasured serialization costs are flagged, with lazy import recommended only where proven.
