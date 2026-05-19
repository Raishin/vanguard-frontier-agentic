# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no secrets, no connection strings, no tokens, no tenant identifiers, no customer data — replace with placeholders):
- The application bootstrap: `Program.cs` and/or `Startup.cs`, including the OpenTelemetry registration block (`AddOpenTelemetry`, `WithTracing`, `WithMetrics`, logging configuration).
- Logging configuration: the `ILogger` usage in handlers and services, and any logging extension methods.
- Instrumentation code: custom `Activity`/`ActivitySource` usage, `Meter`/instrument creation, and outbound `HttpClient` or messaging registration.
- Sanitized `appsettings.json` / `appsettings.{Environment}.json` with placeholder values, including any sampling configuration.

If the bootstrap or telemetry configuration is not provided, state the affected findings as `assumption (config absent)` and ask for it.

### Step 2 — PII-in-telemetry audit

Confirm no PII reaches spans or logs.

- Email, access token, password, payment card number, or a full request body written to a span attribute (`activity.SetTag(...)`, `AddTag(...)`) → CRITICAL.
- The same values interpolated or passed as structured properties into a log message → CRITICAL.
- Lead with this finding when present — telemetry is widely readable and often long-retained.

### Step 3 — Trace context propagation audit

Confirm traces cross service boundaries.

- Outbound `HttpClient` calls with no `AddHttpClientInstrumentation` (or equivalent) registered → HIGH: the downstream span is orphaned and the trace breaks at the boundary.
- Messaging producers/consumers with no context propagation (trace context not injected into or extracted from the message) → HIGH.
- ASP.NET Core inbound requests with no `AddAspNetCoreInstrumentation` → HIGH.

### Step 4 — Log correlation audit

- Log messages with no correlation or trace identifier (`TraceId`, `SpanId`, or an explicit correlation ID) attached → HIGH: logs cannot be joined to a trace or to each other.
- Correlation identifier present in some sinks but not others → MEDIUM.
- Recommended: enrich the logging scope with the active trace context so every log line carries it.

### Step 5 — Structured logging audit

- Exceptions logged via an interpolated string (`logger.LogError($"failed: {ex}")`) instead of the exception overload (`logger.LogError(ex, "...")`) → MEDIUM: the structure and stack trace are flattened into a string.
- Log messages built with string concatenation/interpolation instead of message templates with named properties → MEDIUM: the events are not queryable by property.

### Step 6 — Metrics and sampling audit

- No request-rate, latency, and error-rate metrics for the service surface → MEDIUM: the service cannot be monitored for the signals that matter.
- 100% trace sampling configured for production with no cost note or caveat → MEDIUM: unbounded telemetry volume and cost. Never recommend 100% sampling in production without a cost caveat.
- Sampling not configured at all (defaulting silently) with no note → MEDIUM.

### Step 7 — Health vs. readiness audit

- No distinction between a liveness/health endpoint and a readiness endpoint → MEDIUM: orchestrators cannot tell "alive" from "ready to serve".
- Health checks that probe dependencies on the liveness path → MEDIUM: a dependency blip restarts a healthy process.

### Step 8 — Produce the output

Format findings using the Output contract below.

---

## Evidence checklist

Before finalizing, confirm:
- [ ] The OpenTelemetry registration block has been read from actual `Program.cs` / `Startup.cs` source, not assumed.
- [ ] Every propagation claim is tied to a registration line or its absence.
- [ ] PII findings cite the actual span-attribute or log-message call.
- [ ] Each finding carries an evidence-basis label.
- [ ] No secret, connection string, token, tenant identifier, or customer data was requested or echoed.
- [ ] Collector, exporter, and dashboard topology questions were routed to the `opentelemetry` board, not answered here.

## Findings rubric

| Severity | Examples |
|----------|----------|
| CRITICAL | PII (email, access token, password, payment card number, full request body) written to span attributes or log messages. |
| HIGH | No trace context propagation across service boundaries (missing outbound `HttpClient` or messaging instrumentation); no correlation or trace identifier in logs. |
| MEDIUM | Exceptions logged as interpolated strings; missing request-rate/latency/error-rate metrics; 100% production sampling with no cost note; no health/readiness distinction. |
| LOW | Minor instrumentation naming nits; cosmetic logging-template inconsistencies with no correctness impact. |

## Output contract

Return findings in this structure:

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<confirmed (config provided) | inference (config partial) | assumption (config absent) | unknown>

## Findings

### CRITICAL
- [C1] <finding>: <description> — <remediation> — evidence: <confirmed (config provided) | inference (config partial) | assumption (config absent) | unknown>

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

- Never request or accept secrets, connection strings, tokens, tenant identifiers, or customer data. Ask for sanitized `appsettings` and source with placeholders.
- This is a static review: never run builds, tests, or the application, and never contact a telemetry backend or live system.
- PII written into span attributes or log messages is the highest-impact finding possible in this scope — telemetry is broadly readable and often long-retained. Lead with it.
- Never recommend "log everything" or 100% production sampling without a cost caveat. A failing gate is a signal to fix the gate, not to remove it.
- Collector topology, exporters, backends, and dashboards are out of scope — route those to the `opentelemetry` provider board.
