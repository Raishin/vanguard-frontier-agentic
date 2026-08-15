# Backpressure And Resource Bounds

How to bound concurrency against real downstream capacity, and guarantee resource release.

- An unbounded `Promise.all` over a collection sized by external input (a user-supplied list, an unbounded query result) is a concurrency-bounds defect independent of whether every individual promise eventually settles correctly.
- A stream or async-iterable consumer that does not honor the producer's backpressure signal buffers without limit under sustained load, which is a resource-exhaustion risk distinct from a simple correctness bug.
- Cleanup that runs in a `.then()` handler rather than a `.finally()` (or an equivalent guaranteed-run construct) is skipped whenever the preceding operation throws or rejects, which is the exact failure mode of a resource leak under error conditions.
