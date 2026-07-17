# Collector Selection and the Refusal Contract

> Static review only, board-agnostic across JDK LTS lines unless a specific version is named in the evidence. This board never measures live pause telemetry — every conclusion here is conditioned on the GC-log/JFR/SLA evidence actually supplied. Sources: the Oracle HotSpot Virtual Machine Garbage Collection Tuning Guide (docs.oracle.com/en/java/javase/, version-appropriate `gctuning` section for the JDK in evidence), OpenJDK JEP 248 (Make G1 the Default Garbage Collector), JEP 377 and JEP 439 (ZGC, Generational ZGC), and the OpenJDK Shenandoah project page. JEP numbers and their shipping JDK versions below are historical, completed facts (already released) — this differs from time-sensitive facts like support end dates, which this board never states from memory.

## Why this decision needs a refusal contract

A GC-collector switch changes throughput, pause behavior, CPU/memory overhead, and (for ZGC/Shenandoah) heap-region layout — trade-offs that are only visible in measured pause and allocation data. This board reads source and configuration; it cannot run a JVM, attach a profiler, or observe live p99/p99.9 pause telemetry. That asymmetry is the reason for a hard rule: **never issue a positive recommendation to switch collectors without the requester supplying pause-time or allocation evidence.** Without it, a recommendation is not tuning advice — it is a guess dressed as a verdict.

What counts as evidence:
- GC logs captured with unified logging (`-Xlog:gc*:time,uptime,level,tags` or a narrower `-Xlog:gc` selector, available JDK 9+ per JEP 158, Unified JVM Logging) or, on JDK 8, the deprecated `-XX:+PrintGCDetails -XX:+PrintGCDateStamps` pair.
- A JDK Flight Recorder (JFR) recording containing GC and/or object-allocation-sample events.
- A documented, measured pause-time or throughput SLA breach with actual percentiles (p99/p99.9), not an estimate.

What does **not** count as evidence: developer intuition ("G1 pauses feel long"), a blog-post benchmark run on unrelated hardware and heap shape, "it's the newer collector," or a switch already merged in a PR with no logs attached. Any of these, alone, is a cargo-cult switch — flag it as unjustified regardless of which collector is the destination, including a switch *back* to an older collector.

When the requester wants a verdict but has not supplied the evidence: state what the current collector's behavior implies structurally (e.g. "G1's adaptive sizing is defeated by a fixed `-Xmn`" is a configuration fact, not a pause-time claim), require the missing GC-log/JFR/SLA evidence before ruling on fitness, and route the live measurement itself — capturing the logs, running the JFR recording, or measuring the SLA — to a live-telemetry/incident-response role outside this board. This board can tell the requester *what to capture and why*; it cannot capture it.

## Collector fit, by workload

| Collector | Status | Fits when evidence shows | Notes |
|---|---|---|---|
| G1 | Default general-purpose collector since JDK 9 (JEP 248) | No unusual pause or heap-size constraint; the default is correct until evidence says otherwise | Region-based, adaptive pause-time goal via `-XX:MaxGCPauseMillis`; fixing `-Xmn` on G1 defeats that adaptive sizing |
| Parallel | Throughput-first, older default (pre-JDK 9) | Batch/offline/throughput-first workloads with no pause-time SLA on the request path | Never appropriate for a user-facing request path regardless of throughput gains; stop-the-world pauses scale with live-set size |
| ZGC | Low-pause; concurrent (JEP 377, JDK 15+); generational since JEP 439 (JDK 21+) | Evidenced sub-10ms-class pause requirements or very large heaps where G1 evidence shows pause times are insufficient | Confirm the JDK version in evidence actually includes the generational mode if that specific benefit is being claimed |
| Shenandoah | Low-pause; concurrent (JEP 189 experimental JDK 12, JEP 379 production JDK 15) | Same low-pause profile as ZGC, when the specific JDK **distribution** in evidence ships it | Not shipped by every OpenJDK build/vendor (originated at Red Hat) — confirm from the build actually in use, never assume availability from JDK version alone |
| Serial | Single-threaded, minimal footprint | Small heaps, constrained environments (evidence: heap size and CPU allotment), not covered further here | Out of scope unless the evidence specifically raises it |

Do not treat this table as a ranking. "Newer" is not "better" absent an evidenced pause or throughput problem the current collector demonstrably cannot solve.

## Mis-set flags to flag on sight

These are configuration defects visible in static evidence — no live telemetry required to flag them:

- A collector-specific flag applied to the wrong collector (e.g. `-XX:MaxGCPauseMillis` alongside `-XX:+UseParallelGC`, which silently ignores it — Parallel has no pause-time goal).
- `-Xmn` (fixed young-generation size) combined with `-XX:+UseG1GC` — this fights G1's adaptive region sizing rather than tuning it.
- ZGC or Shenandoah flags present alongside a JDK version too old to support the specific mode being invoked (e.g. generational ZGC flags on a pre-21 JDK).
- Fixed absolute `-Xmx`/`-Xms` byte values in a Dockerfile or Kubernetes manifest that do not account for the container's memory limit — flag as unverified container-awareness, not as a confirmed defect, unless the JDK version and any `-XX:MaxRAMPercentage`/`-XX:+UseContainerSupport`-family flags are also shown; the exact JDK-version boundary at which container awareness defaults to enabled is vendor/version-specific and must be checked against the JDK actually in evidence, not asserted from memory.
- GC logging absent entirely from the startup flags while a pause-time claim is being made in the same conversation — this makes the claim `unknown`, not favorable.

## Escalation conditions

- The requester wants the collector switch *measured*, not just reasoned about — capturing GC logs, running a JFR session, or watching live p99/p99.9 → hand to a live-telemetry/incident-response role; out of scope for this static board.
- The slow path traces to JPA/Hibernate fetch shape rather than GC/allocation → hand to `java-jpa-hibernate-performance-agent`.
- The blocker is a JDK version/vendor support boundary rather than a GC/performance question → hand to `java-jdk-lifecycle-and-upgrade-agent`.
