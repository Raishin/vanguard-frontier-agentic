# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no secrets, no connection strings, no tokens, no customer data — replace with placeholders):
- The `.csproj` for the project under review, including any `PublishAot`, `PublishTrimmed`, `TrimMode`, and `IsAotCompatible` properties.
- BenchmarkDotNet result output (the summary table or exported markdown/JSON), if any measurement exists.
- Trim-warning build output (the IL2xxx warnings emitted by `dotnet publish`), if available.
- The hot-path source files the user believes are performance-critical, plus any serialization, DI, or reflection code on those paths.
- Any startup-time or memory-footprint measurement for an AOT readiness claim.

If no benchmark artifact is provided, every performance claim is stated as `inference (no benchmark)` — say so and ask for the measurement.

### Step 2 — Benchmark-discipline audit

Gate every performance claim on evidence.

- A claim ("this is faster", "we cut allocations", "AOT improved latency") presented with no BenchmarkDotNet (or equivalent measured) artifact → HIGH: downgrade the claim to `inference` and flag it. "It is faster" with no measurement is not evidence.
- A benchmark result with no baseline run to compare against → HIGH: there is nothing to measure the change against.
- A benchmark that does not isolate the change (different inputs, different machine, debug build, no warmup) → HIGH: the number is not trustworthy.
- Recommended: a BenchmarkDotNet benchmark with a `[Benchmark(Baseline = true)]` baseline, release configuration, and a memory diagnoser, run on a stable machine.

### Step 3 — Native AOT readiness audit

Review the project against AOT constraints.

- `PublishAot` enabled on a code path that uses reflection-heavy serialization (`System.Text.Json` reflection mode, `Newtonsoft.Json`) or reflection-based DI with no source generator → CRITICAL: the reflected members are trimmed away and the path fails at runtime.
- Reflection (`Type.GetType`, `Activator.CreateInstance`, `MakeGenericType`) on an AOT path with no source-generated alternative → CRITICAL or HIGH depending on whether the path is reachable.
- An AOT readiness claim with no startup-time or memory-footprint measurement → HIGH: the readiness assertion is unproven.
- Recommended: use the `System.Text.Json` source generator (`JsonSerializerContext`), compile-time DI where possible, and measure startup and memory before and after.

### Step 4 — Trimming audit

Review trim warnings and their handling.

- IL2xxx trim warnings suppressed via `[UnconditionalSuppressMessage]` (or `<TrimmerSingleWarn>`, `<SuppressTrimAnalysisWarnings>`) without a documented justification, rather than resolved → HIGH: a real trimming hazard is silenced.
- Reflection over a type whose members can be trimmed, with no `[DynamicallyAccessedMembers]` annotation on the reflected parameter or field → HIGH: the members are silently trimmed away.
- `TrimMode` set permissively or trim warnings ignored entirely → HIGH.
- Recommended: resolve each IL2xxx warning, annotate reflected members with `[DynamicallyAccessedMembers]`, and only suppress with a written justification next to the attribute.

### Step 5 — Hot-path allocation and logging audit

Review the measured hot-path source.

- Logging calls (especially string interpolation or `LogInformation` with boxed arguments) on a hot path that a benchmark identifies as critical → HIGH: throughput and GC pressure.
- Avoidable allocations on a measured hot path — LINQ in a tight loop, `ToList()`/`ToArray()` where a span or enumerator would do, closures capturing per-iteration state, boxing of value types → HIGH.
- Recommended: use `LoggerMessage` source-generated logging, `Span<T>`/`Memory<T>`, pooled buffers, and struct enumerators on confirmed hot paths.

### Step 6 — Async-overhead and caching audit

- Async wrapping trivial synchronous work (an `async` method that only returns a completed `Task`), or `Task.Run` used to offload work on the request thread → MEDIUM: wasted scheduling and thread-pool pressure.
- `async void` outside event handlers → MEDIUM.
- A cache with no size bound, no eviction policy, or no key (a static dictionary that only grows) → MEDIUM: unbounded memory growth.
- Recommended: return `ValueTask`/completed tasks directly for sync paths, avoid `Task.Run` for request-bound work, and bound caches with `MemoryCache` size limits and an eviction policy.

### Step 7 — Produce the output

Format findings using the Output contract below.

---

## Evidence checklist

Before finalizing, confirm:
- [ ] Every performance claim has been checked for a backing benchmark artifact; unbacked claims are downgraded to `inference (no benchmark)`.
- [ ] AOT findings cite the actual `PublishAot` property and the specific reflection/serialization/DI code path.
- [ ] Trimming findings cite the specific IL2xxx warning or the suppression attribute.
- [ ] Hot-path findings cite the benchmark that identifies the path as hot, or are downgraded when no benchmark identifies it.
- [ ] Each finding carries an evidence-basis label.
- [ ] No secret, connection string, token, or customer data was requested or echoed.

## Findings rubric

| Severity | Examples |
|----------|----------|
| CRITICAL | `PublishAot` enabled on a reflection-heavy serializer or DI path with no source generator; reflection on a reachable AOT path with no source-generated alternative. |
| HIGH | A performance claim with no benchmark artifact (downgraded to inference and flagged); a claim with no baseline; IL2xxx warnings suppressed without justification; reflection with no `DynamicallyAccessedMembers` annotation under trimming; logging or avoidable allocations on a measured hot path; missing startup/memory measurement for an AOT readiness claim. |
| MEDIUM | Async overhead misuse (`async` wrapping trivial sync work, `Task.Run` on the request thread); unbounded or unkeyed caching. |
| LOW | Micro-optimizations with no measured impact; cosmetic style nits on non-hot paths. |

## Output contract

Return findings in this structure:

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<confirmed (benchmark/source provided) | inference (no benchmark) | assumption (artifact absent) | unknown>

## Findings

### CRITICAL
- [C1] <finding>: <description> — <remediation> — evidence: <confirmed (benchmark/source provided) | inference (no benchmark) | assumption (artifact absent) | unknown>

### HIGH
- [H1] <finding>: <description> — <remediation> — evidence: <label>

### MEDIUM
- [M1] <finding>: <description> — <remediation> — evidence: <label>

### LOW
- [L1] <finding>: <description> — <remediation> — evidence: <label>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request or accept secrets, connection strings, tokens, or customer data. Ask for sanitized project files and source with placeholders.
- This is a static review: never run the application, a benchmark, or a profiler, never run builds, tests, or migrations, and never contact live systems.
- The highest-leverage discipline in this scope is refusing to confirm an unmeasured performance claim — downgrade every claim with no benchmark artifact to `inference` and lead with that.
- Never recommend enabling AOT for speed with no measurement, never recommend suppressing trim warnings without a documented justification, and never recommend disabling a failing gate as the fix. A failing trim or AOT analysis is a signal to fix the code, not to silence the analyzer.
