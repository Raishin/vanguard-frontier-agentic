# Workflow and Output Contract

> Static review only. Read source, Dockerfiles, JVM flags/env, GC or startup logs, and sanitized Kubernetes manifests/Helm values. Never open a JDK, run or profile the workload, or read live `/proc`/cgroup filesystem state. Ask for source and configuration with placeholders — never connection strings, credentials, tenant identifiers, kubeconfigs, or customer data.

## Workflow

### Step 1 — Collect inputs

Ask the user for whichever apply, sanitized:

- The container's memory and CPU `requests`/`limits` (Kubernetes pod spec or Helm values) or an equivalent Docker `--memory`/`--cpus`.
- JVM flags and environment (`JAVA_TOOL_OPTIONS`, `JDK_JAVA_OPTIONS`, entrypoint/`CMD` args, `-Xmx`/`-XX:MaxRAMPercentage`, `-XX:MaxMetaspaceSize`, `-XX:MaxDirectMemorySize`, `-XX:ReservedCodeCacheSize`, `-XX:ActiveProcessorCount`, GC algorithm flags).
- The JDK vendor/version in scope (base image tag, `java -version` output, or build metadata) — needed to reason about container-support and cgroup v1/v2 detection defaults.
- `livenessProbe`/`readinessProbe`/`startupProbe` configuration (`timeoutSeconds`, `periodSeconds`, `failureThreshold`, `initialDelaySeconds`).
- If available: GC logs (`-Xlog:gc*` output) or startup-time evidence, and expected peak thread count for the workload.

If the container memory/CPU limit or the JVM flags are missing, downgrade the related findings to `inference (partial source)` or `assumption (source absent)` and say so explicitly rather than reasoning as if a value were confirmed. Do not substitute a fabricated measurement (GC pause p99, throughput) for a missing one — name it as evidence to supply.

### Step 2 — Map memory ergonomics

Determine whether `UseContainerSupport` is active (default-on for a sufficiently recent, confirmed JDK version) and whether the JVM resolves the container's cgroup memory limit or the host's. Record the heap-sizing mechanism (`MaxRAMPercentage` or fixed `-Xmx`) and compute — or request evidence for — the off-heap headroom: metaspace, thread stacks (peak thread count × `-Xss`), direct/NIO buffers, and code cache, against the memory limit.

### Step 3 — Map CPU ergonomics

Determine whether `ActiveProcessorCount` (explicit or ergonomically detected) matches the CPU limit, and whether GC thread counts and JVM-managed thread pools (`ForkJoinPool.commonPool`, framework event-loop/connection pools defaulting to `availableProcessors()`) are sized consistently with that value rather than the host's core count.

### Step 4 — Check GC-pause vs probe-timeout interaction and startup fit

Compare the GC algorithm/configuration (and any pause evidence provided) against `livenessProbe.timeoutSeconds × failureThreshold`. Separately, assess whether cold-start characteristics (heap size, CDS/AppCDS use, framework bootstrap weight) warrant a `startupProbe`, and whether one is present. If a specific pause figure is load-bearing and no GC log was supplied, request the log rather than asserting a number.

### Step 5 — Produce the output

Format using the Output contract below. Pick each remedy by the evidence gathered; never recommend disabling container support, pushing heap toward the limit without naming the excluded headroom, or loosening a probe to hide a pause-vs-timeout mismatch.

## Evidence checklist

- [ ] Container memory limit
- [ ] Container CPU limit
- [ ] JVM heap-sizing flags (`-Xmx` / `MaxRAMPercentage` and related)
- [ ] Off-heap flags or thread-count/buffer-usage evidence (metaspace, direct memory, code cache, stacks)
- [ ] JDK vendor/version in scope
- [ ] `livenessProbe` / `readinessProbe` / `startupProbe` configuration
- [ ] GC algorithm/flags and, if available, GC pause evidence

Each unchecked item downgrades the related findings to `inference` or `assumption`.

## Findings rubric

| Severity | Criteria |
|----------|----------|
| critical | `UseContainerSupport` explicitly disabled under a confirmed container limit; a failing gate (CI memory check, admission resource-quota check, probe-timeout linter) recommended for removal instead of fixed. |
| high | Heap-to-limit ratio with no accounted off-heap headroom; CPU limit not reconciled with `ActiveProcessorCount`/thread-pool sizing; GC pause behavior not checked against the liveness-probe failure budget. |
| medium | Missing `startupProbe` on a cold-start-heavy JVM; cgroup v1/v2 detection asserted without JDK-version evidence; Burstable-QoS memory asymmetry not called out. |
| low | `UseContainerSupport` merely unverified (absent from flags, JDK version not confirmed) with no other risk signal. |

Every finding carries an evidence-basis label: `confirmed (source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

## Output contract

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<full source | partial source | inference>

## Memory ergonomics findings
- <heap-to-limit ratio, off-heap headroom, MaxRAMPercentage/-Xmx, UseContainerSupport/cgroup detection>

## CPU ergonomics findings
- <ActiveProcessorCount vs CPU limit, GC/thread-pool sizing>

## GC-pause vs probe-timeout findings
- <pause budget vs probe timeout, startupProbe adequacy>

## Findings

### CRITICAL
- [C1] <finding> — <evidence basis> — <remediation>

### HIGH
- [H1] <finding> — <evidence basis> — <remediation>

### MEDIUM
- [M1] <finding> — <evidence basis> — <remediation>

### LOW
- [L1] <finding> — <evidence basis> — <remediation>

## Safe next actions
1. <action>

## Open questions
- <limit/flag/probe evidence the user must supply>
```

## Security notes

- Never request or accept connection strings, credentials, tenant identifiers, kubeconfigs, or customer data. Ask for source and configuration with placeholders.
- Static review only: never open a JDK, run or profile the workload, or read live `/proc`/cgroup filesystem state.
- Never fabricate a measured runtime figure (GC pause p99, throughput/latency) to complete a finding — name the GC log or metric the user must capture.
- Never recommend disabling `UseContainerSupport`, pushing heap toward the container limit without naming the excluded off-heap headroom, or loosening/removing a probe to silence a restart-loop symptom instead of fixing its cause.
- Never recommend disabling a failing gate as the fix.
