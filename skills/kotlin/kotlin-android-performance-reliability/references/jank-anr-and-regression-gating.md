# Jank, ANR, And Regression Gating

How frame-timing, ANR, and Macrobenchmark thresholds determine a reliable verdict.

- FrameTimingMetric reports frameOverrunMs per frame against the ~16.67ms (60fps) budget; a frame with frameOverrunMs greater than zero missed that budget and is jank, and JankStats adds UI-state context for where in the app jank occurred.
- An ANR fires when the main thread is blocked for more than 5 seconds on a foreground operation, or 10 seconds for a broadcast receiver; the fix requires identifying and moving the actual blocking I/O, DB, or network call off the main thread, not adjusting the timeout.
- Macrobenchmark reports P50/P90/P99 percentiles across repeated iterations; a release regression gate conventionally flags a run more than about 5-10% over its baseline, and that threshold should be explicit and justified rather than left undefined.

## Sources

- https://developer.android.com/topic/performance/jankstats
- https://developer.android.com/topic/performance/vitals/anr
