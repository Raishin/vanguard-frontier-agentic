# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- An optimization shipped on intuition alone makes the hot path worse because the actual bottleneck was never profiled.
- A micro-benchmark on a synthetic input is quoted as a production win, and the real workload sees no improvement.
- An unbounded cache with no eviction grows until the process is OOM-killed in production.
- An O(n^2) membership check in a hot loop degrades a service from milliseconds to seconds as the input grows, while the team tunes an unrelated constant.
- GC is disabled to 'fix' a slowdown with no cycle evidence, and a genuine reference-cycle leak grows unchecked instead.
