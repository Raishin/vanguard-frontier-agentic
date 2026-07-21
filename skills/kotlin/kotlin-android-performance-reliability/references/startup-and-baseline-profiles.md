# Startup And Baseline Profiles

How startup metrics and compilation mode determine whether a cold-start claim is representative.

- StartupTimingMetric distinguishes cold, warm, and hot start and must be paired with the CompilationMode (None/Partial/Full) used, since a None-compiled run is not representative of a Baseline-Profile-shipped release build.
- A Baseline Profile pre-compiles the classes and methods used on a hot path, such as startup or key journeys, ahead of time so they don't run interpreted or JIT-warm on first use, and a claimed cold-start improvement should be backed by a paired before/after StartupTimingMetric measurement under the same CompilationMode.

## Sources

- https://developer.android.com/topic/performance/benchmarking/macrobenchmark-overview
- https://developer.android.com/topic/performance/baselineprofiles/overview
