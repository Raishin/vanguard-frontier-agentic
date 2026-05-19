# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no secrets, no signing keys, no client secrets, no tokens, no connection strings, no tenant identifiers, no customer data — replace with placeholders):
- The authentication wiring: `AddAuthentication`, `AddJwtBearer`, `AddCookie`, `AddOpenIdConnect`, and the `TokenValidationParameters` block.
- The authorization wiring: `AddAuthorization`, named policy definitions, and any custom `AuthorizationHandler` / `IAuthorizationRequirement` types.
- Controller and minimal-API `[Authorize]` / `[AllowAnonymous]` attributes for the surface under review.
- Any code that reads a tenant, organization, or role identity from claims, headers, or the request.
- Authorization-related test files, if available, especially negative tests.

If the auth wiring or test coverage is not provided, state the affected findings as `assumption (config absent)` and ask for it.

### Step 2 — Token validation audit

Confirm tokens are fully validated.

- `ValidateIssuer`, `ValidateAudience`, `ValidateIssuerSigningKey`, or `ValidateLifetime` set to `false` → CRITICAL: forged, mis-issued, or expired tokens are accepted.
- `RequireHttpsMetadata = false` outside loopback / local development → CRITICAL: metadata and keys can be fetched over plaintext and tampered with.
- Hand-rolled token parsing or signature checking instead of the framework JWT handler → HIGH: subtle algorithm-confusion and validation gaps.
- An overly large `ClockSkew` masking lifetime problems → MEDIUM.

### Step 3 — Endpoint protection audit

Confirm state-changing endpoints are not anonymous.

- `[AllowAnonymous]` on any POST/PUT/PATCH/DELETE action or a mutating minimal-API handler → CRITICAL.
- A controller or endpoint group with no `[Authorize]` and no global fallback authorization policy → HIGH (or `inference` if the fallback policy is not shown).
- Recommended: a fallback authorization policy that requires an authenticated user by default, with `[AllowAnonymous]` reserved for genuinely public reads.

### Step 4 — Tenant and claims-trust audit

Confirm tenant and organization identity is verified server-side.

- A tenant or organization identifier taken from a client-supplied claim, header, or query/route value, used without server-side verification against the authenticated principal → CRITICAL privilege-escalation surface. The caller can set it to any value and act across tenants.
- Trusting a role or permission claim minted by an untrusted issuer → CRITICAL.
- Recommended: derive tenant from the verified principal, or verify the requested tenant is one the principal is authorized for before any data access.
- EF Core query-level tenant filters are out of scope here — defer global query filter review to the EF Core agent, but still flag a missing server-side tenant check at the auth boundary.

### Step 5 — Cookie and session audit

- An authentication cookie missing `Secure`, `HttpOnly`, or an appropriate `SameSite` → HIGH.
- No sliding-expiration or absolute-expiration strategy on the auth cookie → MEDIUM.
- Session fixation: the session or auth cookie not regenerated on privilege change (sign-in, elevation) → MEDIUM.

### Step 6 — Authorization-model audit

- Authorization decided solely by role membership where the operation acts on a resource the caller must own → HIGH: any role-holder can act on another user's resource. Recommend resource-based authorization via an `AuthorizationHandler` that checks ownership.
- Scattered inline role-string checks (`User.IsInRole("...")` sprinkled through controllers) instead of named policies → MEDIUM.
- Recommended: named, centrally defined authorization policies and resource-based handlers for owned resources.

### Step 7 — Negative-test audit

- No tests that assert an unauthorized request is rejected with 401/403 → HIGH: nothing proves the boundary denies. Positive tests alone confirm allowed paths, not denied ones.
- Recommended: for each protected operation, a negative test for the unauthenticated caller and for the authenticated-but-unauthorized caller.

### Step 8 — Produce the output

Format findings using the Output contract below.

---

## Evidence checklist

Before finalizing, confirm:
- [ ] Every `TokenValidationParameters` claim is read from actual source, not assumed.
- [ ] Each `[AllowAnonymous]` finding cites the actual attribute and the HTTP method of the endpoint.
- [ ] Each tenant-trust finding traces the identifier from its client-supplied source to the data access it gates.
- [ ] Cookie-flag findings cite the actual cookie options.
- [ ] Negative-test findings cite the test files reviewed, or state that tests were not provided.
- [ ] Each finding carries an evidence-basis label.
- [ ] No secret, signing key, client secret, token, connection string, tenant identifier, or customer data was requested or echoed.

## Findings rubric

| Severity | Examples |
|----------|----------|
| CRITICAL | `Validate*` set to false; `RequireHttpsMetadata = false` outside loopback; `[AllowAnonymous]` on a state-changing endpoint; client-supplied tenant claim used with no server-side verification. |
| HIGH | Auth cookie missing `Secure`/`HttpOnly`/`SameSite`; role-only authorization on an owned resource; missing negative authorization tests; hand-rolled token or signature validation. |
| MEDIUM | Scattered inline role-string checks instead of named policies; oversized `ClockSkew`; missing cookie expiration strategy; session not regenerated on privilege change. |
| LOW | Cosmetic policy-naming inconsistencies; minor structural nits with no bypass impact. |

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

- Never request or accept secrets, signing keys, client secrets, tokens, connection strings, tenant identifiers, or customer data. Ask for sanitized configuration with placeholders.
- This is a static review: never run the application, mint or inspect tokens, run builds, tests, or migrations, or contact an identity provider or any live system.
- Disabled token validation and a client-supplied tenant claim used without server-side verification are the highest-impact findings in this scope — lead with them.
- Never recommend `[AllowAnonymous]`, disabling validation, weakening cookie flags, or broad role grants to "unblock" a flow. A failing gate is a signal to fix the gate, not to remove it.
