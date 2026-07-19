> Static review only. Read Java source (`Thread.ofVirtual`/`Executors.newVirtualThreadPerTaskExecutor` call sites, `ExecutorService` construction, `synchronized` blocks, `ThreadLocal` usage), sanitized configuration (pool-size properties, rate-limiter config), and any JFR/trace text the user pastes as plain text. Never run a build, invoke a JDK, attach a profiler or JFR recorder, open a live DB/broker connection, or contact a live system. Never state a JDK EOL/support date or a JEP's finalization status from memory — see `carrier-pinning-and-jdk-version-gating.md`.

## Workflow

### Step 1 — Collect inputs

Ask for whichever of these apply, sanitized (no credentials, connection strings, or customer data):
- Executor/thread-creation call sites: `Thread.ofVirtual()`, `Executors.newVirtualThreadPerTaskExecutor()`, and any `Executors.new*ThreadPool` still in use.
- The JDK version in scope — build and, if different, runtime — required before any pinning verdict.
- Configuration for any downstream resource a virtual-thread task reaches into: connection-pool `maximumPoolSize` (or equivalent), rate-limiter settings, existing `Semaphore` sizes.
- Any JFR export of `jdk.VirtualThreadPinned` events, or `-Djdk.tracePinnedThreads` output, if pinning is in question.
- `synchronized` blocks/methods and native/FFM call sites reachable from virtual-thread tasks.
- `ThreadLocal`/`InheritableThreadLocal` declarations and their read/write sites.
- Use of `StructuredTaskScope`/`ScopedValue` and the `--enable-preview` status of the build.

### Step 2 — Establish the JDK version gate

Before evaluating any pinning claim, fix the JDK version(s) in scope. If build and runtime disagree, or the version is unstated, ask for it rather than defaulting to a JDK-21-era assumption. Record whether the codebase is pre- or post-JEP-491 (JDK 24+) — this classification governs Step 5.

### Step 3 — Review the thread-execution model

For every executor-construction site, classify it against `virtual-thread-lifecycle-and-resource-bounds.md`: correct (one virtual thread per task, unbounded creation), pooling anti-pattern, or capping anti-pattern.

### Step 4 — Trace resource bounds across the migration

For each downstream call a virtual-thread task makes into a limited resource (DB connection pool, rate-limited API, another bounded internal service), determine whether a bound existed pre-migration (implicit, via the old platform-thread pool's size) and whether an explicit bound (`Semaphore` or equivalent) now exists post-migration. A missing explicit bound where an implicit one used to exist is a critical finding regardless of whether the original pool size was intentional.

### Step 5 — Evaluate pinning risk

For each `synchronized` block/method or native/FFM call reachable from a virtual-thread task, apply the JDK-version gate from `carrier-pinning-and-jdk-version-gating.md`. Request JFR/trace evidence; without it, cap the finding's evidence basis at `inference` or `assumption` and its severity at medium or below.

### Step 6 — Review ThreadLocal and structured concurrency

Flag `ThreadLocal` state that assumes a small thread population. Confirm any `StructuredTaskScope`/`ScopedValue` usage against the preview/GA status for the JDK version in scope, and confirm `--enable-preview` is set wherever a preview API is used.

### Step 7 — Review classic concurrency

Apply standard checks: unbounded work queues, `ThreadLocal` leaks in pooled executors, visibility/atomicity gaps (missing happens-before edges), and non-atomic check-then-act or read-modify-write sequences on shared state.

### Step 8 — Produce the output

Format using the Output contract below.

## Evidence checklist

- [ ] JDK version(s) in scope (build + runtime), and pre-/post-JEP-491 classification
- [ ] Executor/thread-creation call sites reviewed for pooling/capping anti-patterns
- [ ] Downstream resource configuration (pool sizes, rate limits) for every resource reached by virtual-thread-spawned tasks
- [ ] JFR `jdk.VirtualThreadPinned` or `tracePinnedThreads` evidence, if a pinning finding is raised above medium
- [ ] `ThreadLocal`/`InheritableThreadLocal` declarations and usage sites
- [ ] `StructuredTaskScope`/`ScopedValue` usage and `--enable-preview` status

Each unchecked item downgrades the related findings to `inference (partial source)` or `assumption (source absent)`.

## Findings rubric

| Severity | Criteria |
|---|---|
| critical | Pooling or capping virtual threads; a migration that stripped a downstream resource bound without re-imposing it. |
| high | JFR-evidenced (confirmed) carrier pinning on a hot path for the JDK version in scope; unbounded `ExecutorService` queue; `ThreadLocal` leak in a pooled executor; missing happens-before edge on shared mutable state; a third-party/library floor or build/runtime JDK disagreement that changes the pinning answer. |
| medium | Source-level pinning risk without JFR confirmation; `ThreadLocal` sized for a small pool carried into a virtual-thread-per-task model without evidence of actual pressure; preview-API usage without version/flag confirmation. |
| low | Stylistic or clearly low-traffic `synchronized` usage with no evidence of blocking-while-held. |

Every finding carries an evidence-basis label: `confirmed (source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

## Output contract

```
## Verdict
<pass | pass-with-conditions | block>

## Scope
JDK (build): <version>   JDK (runtime): <version>   Pinning regime: <pre-JEP-491 | post-JEP-491 | unknown>

## Virtual-thread lifecycle findings
- [id] <finding> — <evidence basis> — <remediation>

## Downstream-resource bound findings
- [id] <finding> — <evidence basis> — <required Semaphore/bound + sizing source>

## Carrier-pinning findings
- [id] <finding> — <evidence basis> — <JFR evidence: yes/no> — <remediation>

## ThreadLocal / structured-concurrency findings
- [id] <finding> — <evidence basis> — <remediation>

## Classic concurrency findings
- [id] <finding> — <evidence basis> — <remediation>

## Safe next actions
1. <action>

## Open questions
- <JDK version, JFR evidence, or resource-capacity config the user must supply>
```

## Security notes

- Never request or accept connection strings, credentials, tenant identifiers, or customer data — ask for sanitized configuration (pool-size numbers, not secrets).
- This is a static review: never run a build, invoke a JDK, attach a profiler/JFR recorder, or contact a live system.
- Never state a JDK EOL/support date or a JEP's finalization status from memory; cite the primary source or mark it unknown.
- Never recommend disabling a failing gate, suppressing a warning, or deleting a test as the fix for a concurrency finding.
