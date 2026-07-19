# Workflow and Output Contract

> Static review only. Read source, JVM flag configuration, and user-supplied GC logs/JFR/heap-dump analysis output. Never open a live process, attach a profiler or debugger, invoke a JDK tool against a running JVM, or request/open a live heap dump. Ask for evidence with placeholders — never credentials, connection strings, tenant identifiers, or customer data.

## Workflow

### Step 1 — Classify the request

Determine which of the three review shapes applies (a request may span more than one):
1. **Collector-selection review** — a proposed or already-made GC-collector switch.
2. **Allocation/heap review** — source-level allocation pressure, heap-sizing flags, container-memory alignment.
3. **OOM/leak triage** — root-cause analysis from a supplied `OutOfMemoryError`, heap-dump analysis output, or JFR data.

### Step 2 — Collect inputs

Ask for whichever apply, sanitized:
- Current and (if changed) proposed JVM flags, in full — collector flag, heap flags, GC-tuning flags, logging flags.
- The JDK vendor/version in use (affects collector availability and default behavior — do not assume).
- For collector-selection review: GC logs, a JFR recording, or documented measured pause/throughput SLA data.
- For allocation/heap review: the relevant source (hot-path code, loops, stream pipelines), and any container manifest/Dockerfile with memory limits.
- For OOM/leak triage: the exact `OutOfMemoryError` message, heap-dump analysis output (dominator tree, retained-heap ranking, leak-suspects report), or JFR allocation/old-object-sample data.

If evidence for a specific claim is missing, downgrade that finding to `inference (partial source)` or `assumption (source absent)` and say so — do not wait to disclose this only at the end.

### Step 3 — Apply the refusal contract (collector-selection requests only)

If the request is (or includes) a collector-switch justification and the required pause-time/allocation evidence (GC logs, JFR, or a measured SLA breach) is not present, refuse the positive recommendation explicitly. State what evidence is missing, what capturing it would show, and route the capture itself to a live-telemetry/incident-response role — do not proceed to rule on fitness without it. Configuration-level findings (mis-set flags, cargo-cult signals) can still be reported even when the fitness verdict is refused.

### Step 4 — Review allocation, heap, and container alignment

For allocation/heap requests: trace hot-path allocation patterns in source, check `-Xms`/`-Xmx` spread and Metaspace bounds, and check container memory-limit alignment against the flags and JDK version shown.

### Step 5 — Triage OOM/leak evidence

For OOM/leak requests: classify the `OutOfMemoryError` message type, trace the retained-object path from the supplied dominator-tree/leak-suspects evidence to a specific retaining reference, and name the fix target from that evidence — not from the leaf object type alone.

### Step 6 — Produce the output

Format using the Output contract below. Never let an allocation/heap/OOM finding substitute for a GC-collector fitness verdict, and never let a collector-fitness refusal block reporting configuration-level findings that are independently confirmed.

## Evidence checklist

- [ ] Current (and proposed, if applicable) JVM flags in full
- [ ] JDK vendor and version
- [ ] GC logs / JFR recording / measured SLA data (required for any collector-switch fitness verdict)
- [ ] Relevant source for allocation-pressure findings
- [ ] Container manifest/Dockerfile memory limits (if container alignment is in scope)
- [ ] Heap-dump analysis output or JFR allocation data (required for any OOM/leak root-cause finding)

Each unchecked item downgrades the related findings to `inference` or `assumption`, or — for a collector-switch fitness verdict specifically — triggers the refusal contract rather than a downgraded finding.

## Findings severity rubric

| Severity | Criteria |
|----------|----------|
| critical | A positive collector-switch recommendation about to be issued without required pause-time/allocation evidence (refuse instead); a recommendation to disable a failing GC/allocation/leak gate. |
| high | Cargo-cult GC switch; mis-set collector-specific flag; fixed heap sizing ignoring a container memory limit; allocation-pressure pattern on a hot path; OOM/leak root cause asserted without heap-dump/JFR evidence. |
| medium | `-Xms`/`-Xmx` spread risking resize pauses; unbounded Metaspace on a dynamic-classloading service; GC logging absent while a pause claim is made; heap/GC change recommended to fix what is actually an algorithmic/structural issue. |
| low | Minor allocation inefficiency off the hot path; missing but non-blocking tuning flag (e.g. absent `-XX:+HeapDumpOnOutOfMemoryError` for future triage). |

Every finding carries an evidence-basis label: `confirmed (source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

## Output contract

```
## Verdict
<pass | pass-with-conditions | block | refused-pending-evidence>

## Evidence level
<full source | partial source | inference> — flags / GC logs / JFR / heap-dump analysis supplied: <list>

## Collector-selection findings
<justified | cargo-cult | refused-pending-evidence> — <evidence basis> — <reasoning>

## Findings

### CRITICAL
- [C1] <finding> — <evidence basis> — <why> — <required evidence to proceed>

### HIGH
- [H1] <finding> — <evidence basis> — <source location or flag> — <remediation>

### MEDIUM
- [M1] <finding> — <evidence basis> — <description> — <remediation>

### LOW
- [L1] <finding> — <evidence basis> — <description> — <remediation>

## Safe next actions
1. <action>

## Open questions
- <flag, log, JFR, or heap-dump evidence the user must supply>
```

## Security notes

- Never request or accept credentials, connection strings, tenant identifiers, or customer data; ask for sanitized excerpts with placeholders.
- Static review only: never open a live process, attach a profiler/debugger, invoke a JDK tool against a running JVM, or open/request a live heap dump.
- Never issue a positive GC-collector-switch recommendation without supplied pause-time/allocation evidence.
- Never recommend disabling a failing gate as the fix.
