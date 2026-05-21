# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as source files (no secrets, no connection strings, no tokens, no signing keys — replace any embedded values with placeholders):
- The C# source files under review (`*.cs`).
- The project file(s) (`*.csproj`) — needed to confirm `<Nullable>`, `<PublishAot>`, `<PublishTrimmed>`, target framework, and `LangVersion`.
- Optional: the build warning list, if the user wants warnings such as CS4014 cross-checked.
- Optional: a short description of which methods sit on a request path or hot path, so allocation findings can be prioritized.

If only a fragment of source is provided, say so and downgrade affected findings to `inference (partial source)` or `assumption (source absent)`.

### Step 2 — Async and concurrency audit

Confirm async code does not block threads and observes its faults.

```csharp
// HIGH — sync-over-async blocks a thread; on a request path this risks thread-pool starvation
var data = GetDataAsync.Result;
GetDataAsync.Wait;
var x = GetDataAsync.GetAwaiter.GetResult;

// HIGH — fire-and-forget: the returned task is dropped, faults are unobserved (CS4014)
DoWorkAsync;
```

- Sync-over-async (`.Result`, `.Wait`, `.GetAwaiter.GetResult`) on a request or hot path → HIGH. Recommend awaiting the call through an async path end to end.
- A task-returning call left un-awaited (CS4014) → HIGH. Recommend `await`, or an explicit, justified `_ =` with fault handling if fire-and-forget is truly intended.
- An async public API that does not accept and honor a `CancellationToken` → MEDIUM. Recommend threading a token through and passing it to inner async calls.
- Mutable `static` fields or shared instance state mutated from concurrent paths without a lock, `Interlocked`, or a concurrent collection → HIGH.

### Step 3 — Exception-handling audit

```csharp
// HIGH — exception swallowed: neither logged, handled, nor rethrown
try { DoWork; }
catch { }
catch (Exception) { /* nothing */ }
```

- An empty `catch {}`, or a catch that neither logs, handles meaningfully, nor rethrows → HIGH. Failures vanish and the system looks healthy while broken.
- Never recommend a broad catch-all as a way to "stabilize" code — that converts a known fault into an invisible one. Recommend handling the specific exception or letting it propagate.

### Step 4 — Resource-lifetime audit

- An `IDisposable` / `IAsyncDisposable` resource created and not disposed, or disposed only on the success path while an exception path leaks it → HIGH. Recommend `using` / `await using` so disposal is guaranteed.
- A resource disposed while still in use (disposed inside a loop that reuses it, or returned after disposal) → HIGH.

### Step 5 — Allocation and hot-path audit

- Per-request LINQ chains, repeated `string` concatenation in loops, or avoidable boxing on a hot path → MEDIUM. Recommend caching, `StringBuilder`, spans, or pooling where the path is genuinely hot.
- Flag allocation findings as `inference` when the user has not confirmed the method is on a hot path.

### Step 6 — Correctness and nullability audit

- `DateTime.Now` or culture-sensitive parsing/formatting (`Parse`/`ToString` without `CultureInfo.InvariantCulture`) in domain logic → MEDIUM. Recommend `DateTimeOffset.UtcNow` and explicit invariant culture.
- Nullable reference types disabled or warnings suppressed with `#nullable disable` or `!` null-forgiving operators used to silence real warnings → MEDIUM to HIGH depending on exposure. Never recommend `#nullable disable` to clear warnings.

### Step 7 — AOT and trimming audit

- Reflection (`Type.GetType`, `Activator.CreateInstance`, member lookup) without `DynamicallyAccessedMembers` annotations in a project with `<PublishAot>` or `<PublishTrimmed>` enabled → HIGH. The trimmer removes the members and the code fails at runtime.
- Flag as `inference` when the project file is not provided and AOT/trimming status is unknown.

### Step 8 — Produce the output

Format findings using the Output contract below.

---

## Evidence checklist

Before writing the verdict, confirm:
- [ ] The C# source under review was provided (not just a description).
- [ ] The `*.csproj` was provided, so `<Nullable>`, `<PublishAot>`, `<PublishTrimmed>`, and target framework are known.
- [ ] Each async finding cites the specific call site.
- [ ] Each allocation finding states whether the method is confirmed on a hot path or assumed.
- [ ] Each finding carries an evidence-basis label.

---

## Findings rubric

| Severity | Use for |
|----------|---------|
| critical | A runtime defect certain to cause data loss, a hang, or a crash in normal operation with confirmed source |
| high | Sync-over-async on a request path, swallowed exceptions, fire-and-forget, undisposed resources, unsynchronized shared state, unannotated reflection under AOT/trimming |
| medium | Missing `CancellationToken`, allocation-heavy hot paths, culture-sensitive domain logic, nullability suppression |
| low | Idiom, naming, and readability issues with no runtime impact |

Each finding also carries an evidence-basis label:
- `confirmed (source provided)` — the defect is visible in source the user supplied.
- `inference (partial source)` — likely a defect, but only a fragment was provided.
- `assumption (source absent)` — raised from description alone; source needed to confirm.
- `unknown` — cannot be assessed without more input.

---

## Output contract

Return findings in this structure:

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<full source + project file provided | source only | partial source | description only>

## Findings

### CRITICAL
- [C1] <finding> — <evidence-basis label>: <description> — <remediation>

### HIGH
- [H1] <finding> — <evidence-basis label>: <description> — <remediation>

### MEDIUM
- [M1] <finding> — <evidence-basis label>: <description> — <remediation>

### LOW
- [L1] <finding> — <evidence-basis label>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Static review only: never compile, run, or instrument code, and never contact live systems.
- Never request or accept secrets, connection strings, tokens, signing keys, tenant identifiers, or customer data — ask for source with placeholders.
- Never recommend `.Result` / `.Wait` to "fix" async — that introduces the deadlock and starvation risk this skill exists to catch.
- Never recommend `#nullable disable` to clear warnings, and never recommend a broad catch-all to "stabilize" code.
- Never recommend disabling a failing gate (a compiler warning promoted to an error, an analyzer rule) as the fix — fix the underlying defect.
