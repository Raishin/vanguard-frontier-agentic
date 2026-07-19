# Allocation Pressure, Heap Sizing, and OOM/Leak Triage

> Static review only. Allocation-pressure findings come from source; OOM/leak root-cause findings come only from user-supplied heap-dump analysis output or JFR data — never from a live process, an attached profiler, or a dump this board opens itself. Sources: the Oracle HotSpot GC Tuning Guide (docs.oracle.com/en/java/javase/), OpenJDK JEP 158 (Unified JVM Logging), and general JDK Flight Recorder documentation under docs.oracle.com/en/java/javase/. Container/cgroup default-behavior version boundaries are vendor- and release-specific and are flagged `unknown` here rather than asserted — verify against the JDK actually in evidence.

## Allocation pressure from source patterns

Allocation pressure is a source-level, statically reviewable concern: every avoidable short-lived object is GC work the collector must later reclaim, regardless of which collector is chosen. Flag these when the evidence (source) shows them on a hot or frequently-invoked path:

- **Autoboxing in a hot loop.** A primitive repeatedly boxed (e.g. an `Integer`/`Long` accumulator, a boxed type used as a `Map` key/value inside a per-element loop) allocates one wrapper object per iteration.
- **String concatenation via `+` inside a loop.** Each iteration allocates a new `String` (and often an intermediate `StringBuilder`); use an explicit `StringBuilder` reused across iterations.
- **Stream/lambda churn on a hot path.** A `Stream` pipeline re-created per request/iteration, or intermediate boxed-stream operations (`IntStream.boxed()`, etc.) on a path invoked at high frequency, adds allocation and lambda-capture overhead the equivalent imperative loop would not.
- **Avoidable defensive copies.** Copying a collection or array on every call where the original is never mutated by the caller.
- **Finalizers.** `Object.finalize()` overrides are deprecated for removal and add GC overhead (finalizable objects require an extra GC cycle to reclaim); a `java.lang.ref.Cleaner`-based or explicit `close()`/try-with-resources pattern is the evidence-appropriate alternative to recommend.

```java
// Flagged: allocates a String and (often) a StringBuilder every iteration
String out = "";
for (Order o : orders) {
    out += o.id() + ","; // HIGH: allocation pressure, cite this line
}

// Preferred: single reused StringBuilder
StringBuilder sb = new StringBuilder();
for (Order o : orders) {
    sb.append(o.id()).append(',');
}
```

Every allocation-pressure finding must cite the specific source location; a generic "this code allocates a lot" is not a reviewable finding.

## Heap-sizing and logging flags

- **`-Xms`/`-Xmx` spread.** A wide gap between initial and maximum heap forces the JVM to resize the heap under load, which itself pauses; for latency-sensitive services, recommend `-Xms == -Xmx` once the evidence (sizing data, not a guess) supports a specific value. Do not recommend a specific number without sizing evidence — that is `assumption (source absent)`.
- **Metaspace bounds.** `-XX:MetaspaceSize` and `-XX:MaxMetaspaceSize` left unbounded on a service with dynamic class loading (e.g. heavy proxy/bytecode-generation frameworks, frequent classloader churn) is a leak-shaped risk: unbounded Metaspace growth manifests as `OutOfMemoryError: Metaspace`, not heap exhaustion.
- **Container memory-limit alignment.** Fixed absolute `-Xmx` values in a Dockerfile/K8s manifest, set without regard to the container's memory limit, risk either OOM-killer termination (heap too large for the limit) or waste (heap far under the limit). Flag the interaction as unverified unless both the JDK version and any container-awareness flags are shown in evidence; the JDK-version boundary at which such awareness defaults to enabled is vendor/release-specific and must not be asserted from memory.
- **GC logging enablement.** `-Xlog:gc*` (unified logging, JDK 9+, JEP 158) or the deprecated `-XX:+PrintGCDetails -XX:+PrintGCDateStamps` pair on JDK 8 must be present before any pause-time or allocation-rate claim can be evaluated; its absence makes the GC picture `unknown`.

## OOM/memory-leak triage from static evidence

This board never opens a live process or a raw heap-dump binary. Triage works from **user-supplied analysis output**: a dominator-tree summary, retained-heap ranking, or leak-suspects report produced by a tool the user ran (e.g. Eclipse MAT-style output), or JFR data containing allocation/old-object-sample events. If only a stack trace or a single log line is supplied, label the root cause `assumption (source absent)` and ask for the dump/JFR artifact before concluding.

`OutOfMemoryError` messages point at different root causes and different remedies — do not treat them interchangeably:

| Message suffix | Typical root cause | What to ask for |
|---|---|---|
| `Java heap space` | Heap too small for live-set, or a genuine leak (growing retained set) | Dominator tree / retained-heap ranking across two or more dumps over time |
| `GC overhead limit exceeded` | JVM spending excessive CPU on GC with little heap reclaimed — near-full heap, thrashing | Same as above; also check `-XX:GCTimeLimit`/`-XX:GCHeapFreeLimit` if tuned away from defaults |
| `Metaspace` | Class-loading leak (repeated classloader creation without unloading) or bound set too low | Metaspace flags, and evidence of dynamic class generation/proxying |
| `Direct buffer memory` | Off-heap `ByteBuffer.allocateDirect` usage not released/GC'd promptly, or `-XX:MaxDirectMemorySize` set too low | Source sites allocating direct buffers, and their lifecycle/release pattern |
| `unable to create native thread` | OS thread-count/ulimit exhaustion, not a Java-heap problem at all | Thread-count evidence, OS limits — likely out of this board's evidence entirely |

When a dominator tree or leak-suspects report is supplied, look for a retained-object graph rooted in a long-lived reference (static field, thread-local not cleared, unbounded cache/collection, listener/callback never deregistered) — that root, not the leaf object type, is the fix target. Recommend scoping the reference, clearing it explicitly, or switching to a bounded/weak reference structure, and name the specific retaining path from the evidence rather than a generic "reduce object retention."

## Escalation conditions

- The requester wants a live dump captured, a profiler attached, or ongoing memory-growth monitored → out of scope; route to a live-telemetry/incident-response role.
- The allocation pattern traces back to ORM entity hydration or fetch shape rather than plain source-level allocation → hand to `java-jpa-hibernate-performance-agent`.
- The retained-object root is a security-relevant object (e.g. sensitive data cached unbounded) → note it, but the security classification itself belongs to the security agents.
