# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no secrets, no connection strings, no tokens, no signing keys, no tenant identifiers — replace with placeholders):
- The application bootstrap: `Program.cs` and/or `Startup.cs`, including the middleware pipeline and the service-registration block.
- Controller or minimal-API endpoint files for the public surface under review.
- Sanitized `appsettings.json` / `appsettings.{Environment}.json` with placeholder values.
- Any CORS, rate-limiter, API-versioning, or health-check registration extracted into helper extension methods.

If the bootstrap or configuration is not provided, state the affected findings as `assumption (config absent)` and ask for it.

### Step 2 — Middleware ordering audit

Confirm the pipeline is ordered correctly.

- `UseAuthorization` registered before `UseAuthentication` → CRITICAL: authorization evaluates without an authenticated principal.
- Authentication or authorization middleware registered after terminal/endpoint middleware (`MapControllers`, `MapGet`, `UseEndpoints`) → CRITICAL: the auth middleware never runs for those routes.
- Exception-handling middleware not registered first (or near-first) → MEDIUM: downstream failures bypass the handler.
- This skill only flags the presence and ordering of auth middleware. Whether the auth scheme and policies are correct is out of scope — defer to the identity-authz agent.

### Step 3 — Dependency-injection lifetime audit

Review service registrations against their consumers.

- A singleton that resolves a scoped or transient service (a captive dependency) → HIGH: the scoped service is pinned for the application lifetime and leaks state across requests.
- A scoped service injected into a singleton via constructor → HIGH (same defect).
- `DbContext` or other scoped infrastructure captured by a singleton → HIGH.
- Transient services holding disposable resources without disposal ownership → MEDIUM.

### Step 4 — CORS audit

- `AllowAnyOrigin` combined with `AllowCredentials` → CRITICAL. Never recommend wildcard CORS as a fix; recommend an explicit allow-list of origins.
- A permissive default policy applied globally with no per-endpoint narrowing → MEDIUM.

### Step 5 — Validation, versioning, and error-response audit

- Bound models with no validation (no data annotations, no `FluentValidation`, no `MinimalApis` validation filter) reaching handlers → HIGH.
- A public API with no versioning strategy (`Asp.Versioning` or an explicit route/header scheme) → HIGH.
- Developer exception page enabled, or unhandled-exception detail / stack traces returned, outside the Development environment → HIGH.
- Inconsistent error shape across endpoints (no `ProblemDetails` or equivalent) → MEDIUM.

### Step 6 — Rate limiting and health/readiness audit

- No rate limiting on public mutating endpoints (POST/PUT/PATCH/DELETE) → MEDIUM.
- No distinction between a liveness/health endpoint and a readiness endpoint → MEDIUM: orchestrators cannot tell "alive" from "ready to serve".
- Health checks that probe dependencies on the liveness path → MEDIUM: a dependency blip restarts a healthy process.

### Step 7 — Produce the output

Format findings using the Output contract below.

---

## Evidence checklist

Before finalizing, confirm:
- [ ] The middleware pipeline order has been read from actual `Program.cs` / `Startup.cs` source, not assumed.
- [ ] Every service lifetime claim is tied to a registration line and a consumer.
- [ ] CORS findings cite the actual policy builder calls.
- [ ] Each finding carries an evidence-basis label.
- [ ] No secret, connection string, token, signing key, or tenant identifier was requested or echoed.

## Findings rubric

| Severity | Examples |
|----------|----------|
| CRITICAL | `UseAuthorization` before `UseAuthentication`; auth middleware after endpoint middleware; `AllowAnyOrigin` with `AllowCredentials`. |
| HIGH | Captive dependency (singleton holding scoped/transient); unversioned public API; exception detail leaked outside Development; missing model validation. |
| MEDIUM | Missing rate limiting on public mutating endpoints; no health/readiness distinction; inconsistent error shape; permissive global CORS policy. |
| LOW | Minor pipeline ordering nits with no correctness impact; cosmetic configuration inconsistencies. |

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

- Never request or accept secrets, connection strings, tokens, signing keys, tenant identifiers, or customer data. Ask for sanitized `appsettings` with placeholders.
- This is a static review: never run builds, tests, or migrations, and never contact a live application or call its endpoints.
- A pipeline ordering defect that puts authorization before authentication is the highest-impact finding possible in this scope — lead with it.
- Never recommend `[AllowAnonymous]` or wildcard CORS as a fix. A failing gate is a signal to fix the gate, not to remove it.
