# Profiling Versus Benchmarking

The distinction between a deterministic profiler and a wall-time benchmark, and what a valid performance claim states.

- cProfile is a deterministic profiler that reports per-call/cumulative time with measurement overhead, used to find the hot path, not to state absolute production latency.
- timeit measures small-snippet wall-time and must use a representative input to generalize.
- A valid performance claim states input size, method, environment, and repeats.

## Sources

- https://docs.python.org/3/library/profile.html
- https://docs.python.org/3/library/timeit.html
