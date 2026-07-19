# CPU Sizing and GC-Pause / Probe Interaction

> Static review only. Applies to any JVM under a Kubernetes CPU request/limit or an equivalent Docker `--cpus` constraint. Conclusions about processor-count-driven sizing need the CPU limit **and** thread-pool/GC flag evidence; conclusions about probe interaction need the GC configuration/pause evidence **and** the probe YAML. Missing either side downgrades the finding to `inference` or `assumption`. Sources: the Kubernetes probe-configuration and resource-management docs, and the JDK's Garbage Collection Tuning Guide (see the skill's official docs). Collector-specific pause-time characteristics are described here qualitatively, not as guaranteed numeric bounds — treat any specific pause-time claim as needing the user's own GC logs as evidence, not vendor marketing figures.

## Why this decision matters

Two failure modes are specific to running a JVM under a Kubernetes CPU limit and a probe: the JVM sizing its internal parallelism off the wrong core count, and a stop-the-world GC pause outliving the liveness probe's patience. Both produce symptoms (slow requests, restart loops) that look like application bugs but are container-fit problems.

## ActiveProcessorCount and CPU-driven sizing

A container-aware JVM resolves `Runtime.availableProcessors()` from the cgroup CPU quota/period (roughly `ceil(quota / period)`), not the host's physical core count, when `UseContainerSupport` correctly detects the limit. This value drives, by default: GC worker thread counts (parallel/concurrent GC threads), `ForkJoinPool.commonPool()` parallelism (used directly by application code and transitively by parallel streams and many reactive/async libraries), and any application or framework thread pool sized from `availableProcessors()` (Netty/reactor event-loop groups, some HTTP client connection pools).

`-XX:ActiveProcessorCount=<n>` can override this detection explicitly. Review it in both directions:

- **Not container-aware / detection fails.** A JVM that sizes these pools off the host's core count on a node with many more cores than the container's CPU limit over-subscribes CPU-bound work; the symptom is CPU throttling (cgroup CFS quota exhaustion) and tail-latency spikes under load rather than a clean error — often mistaken for a slow downstream dependency.
- **`ActiveProcessorCount` set above the actual CPU limit.** Reproduces the same over-subscription deliberately; flag it unless the reviewer can see a documented reason (e.g. deliberately allowing burst above a soft limit that the platform actually permits).
- **Very low CPU limits (fractional cores).** Ergonomics round the processor count up to at least 1; a workload assigned e.g. a quarter-core limit still gets pools sized for at least one full core's worth of parallelism, which can be more concurrency than the CPU budget actually supports — flag when GC thread count or pool sizing looks disproportionate to a sub-1-core limit.

## GC-pause vs probe-timeout interaction

A Kubernetes liveness probe's failure budget is `timeoutSeconds × failureThreshold` measured from the last successful probe (plus `periodSeconds` between attempts). Any stop-the-world pause — a young or full GC, or a long safepoint stall for another reason — that exceeds this budget makes the process fail to respond to the health endpoint in time, and kubelet kills the container. Because the GC behavior that caused one pause is usually still true on restart, this reads to an operator as an unexplained crash loop rather than the transient pause it is.

Collector choice changes the *shape* of this risk, not whether it can happen at all:

- Fully stop-the-world collectors (Serial, and Parallel's major collections) pause for the whole collection; larger live-set sizes mean longer pauses.
- G1 is designed for mostly-incremental, bounded young/mixed pauses via a pause-time goal (`-XX:MaxGCPauseMillis`), but a full GC (evacuation failure, allocation failure under pressure) is still fully stop-the-world and can exceed the pause-time goal significantly.
- Mostly-concurrent collectors (ZGC, Shenandoah) are designed to keep individual pauses very short even for large heaps, but still have brief stop-the-world phases (e.g. root scanning) — do not assert an absolute pause-time guarantee; ask for the user's own GC logs (`-Xlog:gc*`) if a specific pause figure is load-bearing for the finding.

Review the pairing, not either side alone: ask for the GC algorithm/flags (and pause evidence if available) together with `livenessProbe.timeoutSeconds`, `periodSeconds`, and `failureThreshold`. A probe timeout tight relative to the collector's plausible worst-case pause, on a heap sized large enough for that pause to matter, is the finding — not the collector choice by itself. Never fabricate a p99 pause number to close the finding: if one is needed, name it as the user's GC logs to supply.

## startupProbe for slow JVM cold start

JVM cold start can be dominated by class loading and verification, framework bootstrap (dependency-injection container wiring, component scanning), and — for a large heap — OS-level memory commit, none of which are bounded by the same timing envelope as steady-state health. A `livenessProbe`/`readinessProbe` sized for steady-state response time will kill a slow-starting pod before it ever reports ready if there is no `startupProbe` to hold off the other probes during startup. Flag the absence of a `startupProbe` whenever the evidence shows cold-start-heavy characteristics (large `-Xmx`, no CDS/AppCDS archive, a large dependency graph, or documented slow-start behavior) paired with liveness/readiness probes alone.

## Dangerous patterns (flag these)

- CPU limit set without checking GC thread count / common-pool parallelism against it, on a JVM whose container-CPU-awareness is unconfirmed.
- `ActiveProcessorCount` pinned above the CPU limit.
- A liveness probe timeout budget not compared against the GC configuration and heap size in scope.
- A cold-start-heavy JVM with no `startupProbe`.
- Recommending a probe timeout increase or probe removal as the fix for a pause-triggered restart loop, without also addressing the pause itself (GC tuning, heap sizing, or moving to a collector with a shorter pause profile) — masks the symptom rather than the cause.

## Escalation conditions

- The probe or resource fields themselves (missing `resources`, wrong QoS tier, no `startupProbe` field at all regardless of JVM behavior) are the generic finding with no JVM angle → hand to `kubernetes-pod-spec-review-agent`.
- The user wants to actually measure GC pause times or profile CPU usage on a live workload → out of scope for static review; describe what to capture (`-Xlog:gc*`, `kubectl top`, cgroup CPU throttling metrics) and who runs it.
