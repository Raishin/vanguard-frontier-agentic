# Memory Headroom and Heap Sizing

> Static review only. Applies to a JVM running under a Linux cgroup memory limit (Kubernetes `resources.limits.memory`, a plain Docker `--memory`, or an equivalent runtime cap). Every conclusion here needs the container memory limit **and** the JVM heap/off-heap flags as evidence; a sizing claim with only one side is `inference` or `assumption`. Sources: the JDK's HotSpot Garbage Collection Tuning Guide and JVM ergonomics documentation, and the Kubernetes resource-management docs for how `limits.memory` is enforced (see the skill's official docs). JDK version thresholds for container-support defaults and cgroup v2 detection are not asserted here from memory — verify them against the release notes of the JDK actually in evidence.

## Why this decision matters

A container memory limit bounds the whole process — heap, metaspace, thread stacks, direct buffers, code cache, and every native allocation the JVM or its libraries make. A review that only checks `-Xmx` against the limit is checking one component of several and will pass configurations that get SIGKILLed the first time metaspace grows, a thread pool starts more threads, or a library allocates a large direct buffer.

## The container limit vs the JVM's view of memory

Modern JDKs detect the container's memory limit (rather than the host's total RAM) through `-XX:+UseContainerSupport`, which is enabled by default on supporting JDK versions and reads the limit from the cgroup the process is confined to (cgroup v1 `memory.limit_in_bytes` or cgroup v2 `memory.max`, and the equivalent CPU controllers for processor count). `-XX:MaxRAMPercentage` (and the related `MinRAMPercentage`/`InitialRAMPercentage`) compute the default heap ceiling as a percentage of whatever memory value the JVM resolves — the container limit when container support is active and detects one, the host's physical memory otherwise. A fixed `-Xmx` bypasses this ergonomics calculation entirely and must be set by the reviewer's own arithmetic against the limit.

Do not assume every JDK in scope is new enough, or configured correctly, to resolve cgroup v2 limits: verify the JDK version against its own release notes when the version is in evidence, and mark the conclusion `inference` or `assumption` when it is not — do not state a specific version threshold from memory.

## Off-heap components that must fit in the remaining headroom

| Component | Driven by | Evidence to ask for |
|---|---|---|
| Metaspace | Class metadata; grows with loaded classes | `-XX:MaxMetaspaceSize` (unset = unbounded up to native memory) |
| Thread stacks | Thread count × `-Xss` (default per-platform, commonly 512KB–1MB) | Expected peak thread count (web server, executor pools, GC threads) |
| Direct / NIO buffers | `ByteBuffer.allocateDirect`, Netty/reactor buffer pools, some serialization libraries | `-XX:MaxDirectMemorySize`, buffer-pool configuration |
| Code cache | JIT-compiled code | `-XX:ReservedCodeCacheSize` |
| GC native structures, JNI/native libraries | Collector internals, native agents, JNI code | Named native dependencies, GC algorithm in use |

A sizing conclusion is `confirmed` only when the reviewer can name each applicable component and its bound (explicit or a reasoned estimate from thread count and buffer usage); otherwise it is `inference` (some components bounded) or `assumption` (none named).

## Dangerous patterns (flag these)

- **`MaxRAMPercentage` set high (e.g. leaving little visible headroom) with no off-heap accounting.** A percentage in isolation says nothing about safety — a value that is fine for a low-thread-count batch job can OOMKill a high-concurrency service with large connection/thread pools and direct-buffer usage.
- **Fixed `-Xmx` set at or very near the container memory limit.** Leaves no room for metaspace, stacks, buffers, or code cache; any transient off-heap spike (a burst of new threads, a large response buffered off-heap) triggers a cgroup OOM kill, not a catchable `OutOfMemoryError`.
- **No explicit memory flag and no confirmed container-aware JDK.** Ergonomics defaults (25% of RAM for `-Xmx` historically, or `MaxRAMPercentage` defaults) resolve against whatever memory value the JVM detects; on a JDK/config that does not correctly detect the container limit, this can default the heap far too large for the container and guarantee an OOM under load.
- **Burstable QoS memory asymmetry.** When `requests.memory` is far below `limits.memory`, `MaxRAMPercentage` still computes off the **limit** (what `UseContainerSupport` resolves), so the JVM sizes itself for memory the pod is not guaranteed under node pressure — the observed failure under contention is node-level eviction, not a JVM-reported OOM, and needs to be diagnosed as such.
- **Off-heap growth invisible to heap flags.** Direct-buffer leaks, native-library allocations, or unbounded metaspace growth (e.g. dynamic proxy/classloader churn) are not capped by `-Xmx` at all and are a common cause of "heap looks fine, pod still OOMKilled" — ask specifically whether metaspace and direct-memory ceilings are set.

## Evidence needed and escalation

Required for a `confirmed` verdict: the container memory limit, the full heap-sizing flag set (`-Xmx`/`MaxRAMPercentage` and related), and either explicit off-heap ceilings or a reasoned estimate of thread count/buffer usage sufficient to bound the off-heap footprint. If the memory limit is missing entirely, the whole memory-sizing section is `assumption (source absent)` — say so and ask for it rather than reasoning about a fixed `-Xmx` in isolation. If the evidence points to a genuine in-application memory leak (unbounded heap growth under steady load, not a sizing/headroom problem), note it but do not attempt a heap-dump-based diagnosis — that requires a live JVM and is out of scope for static review.
