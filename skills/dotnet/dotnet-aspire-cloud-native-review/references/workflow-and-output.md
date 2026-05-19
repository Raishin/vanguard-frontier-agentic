# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no secrets, no connection strings, no tokens, no tenant identifiers, no customer data — replace with placeholders):
- The Aspire AppHost project: the `AppHost` `Program.cs` declaring resources, services, and their dependencies.
- The ServiceDefaults project: the shared extension methods that register telemetry, health checks, service discovery, and resilience handlers.
- The Aspire manifest (`aspire-manifest.json`), if generated.
- Sanitized `appsettings.json` / `appsettings.{Environment}.json` for the AppHost and the service projects, with placeholder values.
- Any `Dockerfile` or container build evidence for services claimed container-ready.

If the AppHost or ServiceDefaults project is not provided, state the affected findings as `assumption (config absent)` and ask for it.

### Step 2 — Secret-hygiene audit

Confirm no secrets live in committed configuration.

- Connection strings, API keys, tokens, or passwords with real-looking values in `appsettings.json` or `appsettings.*.json` (instead of user-secrets, environment variables, or a secret store) → CRITICAL.
- Lead with this finding when present, and tell the user to rotate any exposed credential.

### Step 3 — AppHost-boundary audit

Confirm the team understands what Aspire is.

- The AppHost described, scripted, or documented as the production runtime or deployment target → HIGH: Aspire orchestration is a development-time and composition model, not a deploy platform. The production system must run on a real platform (containers, a managed service, an orchestrator) — route the specific platform to its board.

### Step 4 — Health-check audit

- Declared service dependencies (databases, caches, message brokers, downstream services) with no corresponding health check registered → HIGH: the dependency's state is invisible.
- Health checks present but not mapped to a readiness endpoint → MEDIUM.

### Step 5 — Resiliency audit

- A service dependency wired with no resiliency policy — no `HttpClient` standard resilience handler (`AddStandardResilienceHandler`) or equivalent retry/timeout/circuit-breaker policy → HIGH: a transient downstream failure cascades.
- Resilience handler present but with no timeout, or with a retry policy that could amplify load → MEDIUM.

### Step 6 — Configuration-drift audit

- Configuration keys, connection names, or service names that differ between the AppHost declaration and the consuming service project → MEDIUM: the value wired in development does not match what the service reads.
- ServiceDefaults registered in some service projects but not others → MEDIUM.

### Step 7 — Service-discovery and container audit

- Service discovery assumed to resolve identically in production with no handoff note (Aspire injects discovery configuration for local development; production discovery is platform-specific) → MEDIUM.
- A service claimed container-ready with no `Dockerfile`, no container build target, and no published-container evidence → MEDIUM.

### Step 8 — Produce the output

Format findings using the Output contract below.

---

## Evidence checklist

Before finalizing, confirm:
- [ ] The AppHost resource declarations have been read from actual `Program.cs` source, not assumed.
- [ ] Every health-check and resiliency claim is tied to a registration line or its absence.
- [ ] Secret findings cite the actual `appsettings` key (with the value redacted).
- [ ] Each finding carries an evidence-basis label.
- [ ] No secret, connection string, token, tenant identifier, or customer data was requested or echoed.
- [ ] Cloud-target deployment questions were routed to the AWS/Azure/GCP boards, and generic API review to the API skill, not answered here.

## Findings rubric

| Severity | Examples |
|----------|----------|
| CRITICAL | Secrets committed in `appsettings.json` or `appsettings.*.json` instead of user-secrets or a secret store. |
| HIGH | The Aspire AppHost treated as the production runtime or deployment target; missing health checks on declared service dependencies; a service dependency with no resiliency policy. |
| MEDIUM | Configuration drift between AppHost and service projects; service discovery assumed identical in production with no handoff note; no container or Dockerfile evidence for a service claimed container-ready. |
| LOW | Minor naming inconsistencies; cosmetic manifest nits with no correctness impact. |

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
- This is a static review: never run builds, tests, or the AppHost, never deploy, and never contact a live system.
- A secret committed to `appsettings` is the highest-impact finding possible in this scope — lead with it and tell the user to rotate the exposed credential.
- Never recommend treating Aspire orchestration as a production deployment platform. A failing gate is a signal to fix the gate, not to remove it.
- The cloud target itself, exporters, and managed-service topology are out of scope — route those to the AWS, Azure, or GCP boards. Note that .NET Aspire APIs evolve quickly; confirm against current official docs.
