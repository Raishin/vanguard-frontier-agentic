# Review Workflow And Output Contract

The performance-and-memory review workflow and the required output shape.

## Workflow

1. Identify what performance or memory claim is being made and what evidence (profile, benchmark, tracemalloc snapshot) supports it.
2. Check the evidence type matches the claim — a profiler identifies the hot path, a benchmark measures wall-time of a representative workload — and flag any conflation.
3. For a memory concern, require a tracemalloc-evidenced growing allocation before accepting a leak diagnosis.
4. Check for algorithmic-complexity issues before any constant-factor optimization, and require gc evidence before blaming or disabling garbage collection.
5. Check import/startup cost and serialization overhead are measured, not assumed, and record every claim still needing evidence.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the profiling/benchmarking artifacts and environment assumed.
- Profiling/benchmarking-rigor, memory-growth/leak, complexity/GC-pressure, and import/serialization-cost findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any claim still needing a profile/benchmark/tracemalloc snapshot.
