# CSRF, Open-Redirect, and OAuth/OIDC Flow Review

Use this reference when the review scope includes a state-changing request's CSRF defenses, a redirect/return-URL parameter, or an OAuth/OIDC authorization flow. The OAuth/OIDC subsection applies only when an OAuth/OIDC flow is actually present in scope — do not load or apply it otherwise.

## CSRF defenses

### What people get wrong

The common bad assumption is:

> "We use cookies for sessions, so CSRF isn't really our problem — that's an old attack."

CSRF is specifically an attack against cookie-based (or any auto-attached-credential) session mechanisms: the browser automatically attaches the session cookie to a request the attacker's site triggers, without the user's knowledge. It remains directly relevant to any cookie-based session architecture, and is *not* automatically neutralized just because `SameSite` exists — `SameSite` is one layer, not a complete substitute for a token-based defense when the target action is sensitive.

### Grounded defense patterns (per OWASP CSRF Prevention Cheat Sheet)

- **Synchronizer token pattern** — server generates a unique, unpredictable, session-bound token; the client includes it in every state-changing request (hidden form field or custom header for AJAX/fetch); server validates it matches the session before processing. Requires server-side state. Preferred for traditional stateful (cookie-session) applications.
- **Double-submit cookie pattern** — a stateless alternative: the token is set as a cookie and also sent in the request body/header; the server compares the two. The *signed* double-submit variant, which cryptographically binds the token to the session, is the recommended variation — an unsigned token without session binding offers minimal protection and is vulnerable to cookie-injection attacks. Do not accept an unsigned double-submit implementation as adequate.
- **`SameSite` cookie attribute** — `Strict` or `Lax` prevents the browser from attaching the session cookie to most cross-site requests, providing meaningful CSRF mitigation as a primary layer per current OWASP guidance, but treat it as defense-in-depth alongside a token-based pattern for sensitive state-changing operations (password change, payment, account-deletion, permission grants), not as the sole control for those.
- **Custom request headers for API-only surfaces** — for a pure JSON API with no HTML forms, requiring a custom header (e.g., `X-Requested-With`) that only same-origin JS can set (browsers block cross-origin scripts from setting arbitrary headers without a permissive CORS policy) is an accepted lightweight defense, contingent on the API's CORS configuration not being permissive (`Access-Control-Allow-Origin: *` with credentials defeats this).

### Non-negotiables

- Never use `GET` requests for state-changing operations — GET requests are trivially triggerable cross-site (an `<img>` tag, a bare link) with no token protection possible in the same way, and are logged/cached/exposed via browser history and Referer headers.
- Do not transmit CSRF tokens inside cookies for the synchronizer token pattern — tokens for that pattern belong in the response payload (hidden form field, JSON body) and are returned via form submission or a custom header, not round-tripped through a cookie (that would defeat the pattern's separation from the auto-attached credential it is meant to validate).
- Do not accept "we validate the Origin/Referer header" as a complete substitute for a token or `SameSite` defense unless you have confirmed the app also has a documented fallback for the (rare but real) cases where those headers are stripped by proxies or privacy tools — treat header-checking as defense-in-depth, not sole control, unless the app explicitly documents this as its chosen primary defense with that tradeoff acknowledged.
- XSS bypasses CSRF protections entirely (a script running in-origin can read a CSRF token and submit it). If a known, unremediated XSS finding exists in the same review scope, state explicitly that the CSRF defenses reviewed here are undermined by it — do not present the CSRF findings as if they stand alone.

## Open-redirect prevention

### Grounded pattern (per OWASP Unvalidated Redirects and Forwards Cheat Sheet)

- Best: avoid using user input to determine the destination URL at all. Use a short name, ID, or token that the server maps to a full target URL server-side (with care to avoid enumeration issues if the mapping itself is guessable).
- If user input for the destination is unavoidable, validate it server-side against an allow-list of trusted hosts/paths before redirecting. A relative-path-only allow-list (rejecting any input containing a scheme or `//` prefix that would make it absolute/protocol-relative) is a common safe pattern for "return to this page after login" flows.
- If using a regex or string-prefix check for validation, it must be anchored and must account for scheme and protocol-relative URLs (`//evil.example.com` is parsed by browsers as `https://evil.example.com` when used as a redirect target) — an unanchored or naive `startsWith`/`includes` check is bypassable.
- Ideally, force a user-confirmation interstitial ("You are leaving [app] and going to [external site]") for any redirect to a genuinely external destination, even an allow-listed one, for sensitive flows.

### Non-negotiables

- A client-side-only check (JavaScript validation before calling `window.location.assign`/`.href =`) is not a control — an attacker can hit the server-side redirect endpoint directly with the malicious parameter, bypassing any client-side JS entirely. The validation must exist server-side to count as a mitigation.
- Do not accept "we only redirect within our own app" as verified without checking what "within our own app" means in the actual validation code — a check for a substring match (e.g., `url.includes('myapp.com')`) is bypassable by an attacker-controlled URL like `https://myapp.com.evil.example.com` or `https://evil.example.com/?x=myapp.com`.

## OAuth/OIDC flow review (load only when an OAuth/OIDC flow is in scope)

### What people get wrong

The common bad assumption is:

> "OAuth is OAuth — any grant type is fine as long as we're using a real identity provider."

Grant-type choice materially changes the app's attack surface. The implicit grant returns the access token directly in the URL fragment, exposing it to browser history, Referer leakage (in older browsers/misconfigurations), and any script with access to the URL — with no refresh-token issuance. OAuth 2.1 (the current consolidated guidance, an IETF draft at the time of writing — verify against the current draft revision via Context7 before citing exact section numbers) removes the implicit grant and hybrid flows entirely, and requires PKCE for public clients using the authorization code flow.

### Non-negotiable design rules

1. **Public clients (SPA, mobile app — anything that cannot hold a client secret) must use the authorization code flow with PKCE.** Verify both `code_challenge`/`code_challenge_method` on the authorization request and `code_verifier` on the token exchange are present. A code flow without PKCE for a public client is a finding.
2. **Never recommend or approve `response_type=token` (implicit grant) for new work.** If found in an existing app, flag as HIGH and recommend migration to authorization code + PKCE — do not treat it as merely legacy-acceptable.
3. **The `redirect_uri` must be an exact match against a pre-registered value at the authorization server**, not a pattern/wildcard match validated only client-side. Confirm this is enforced server-side (at the identity provider / authorization server config), not assumed.
4. **`state` parameter must be present, unique per authorization request, and validated on callback** to prevent CSRF against the OAuth flow itself (an attacker tricking a victim into completing an auth flow bound to the attacker's session).
5. **Tokens received from the authorization server must be stored per the token-storage guidance** in `references/token-storage-and-cookies.md` — an otherwise-correct PKCE flow that then stores the resulting access/refresh token in `localStorage` still carries the storage-location finding.

### Verification targets

- The actual authorization-request URL construction (grep for `response_type=`, `code_challenge`) to confirm PKCE is wired, not merely available in the SDK but unused.
- The token-exchange (token endpoint) call to confirm `code_verifier` is sent and matches what generated the `code_challenge`.
- The `state`-parameter generation and validation code path.

## When to push back

Push back if the user asks to:

- skip CSRF token implementation because "`SameSite=Lax` covers it" for a sensitive operation (payment, password change, account deletion, permission/role grant) — recommend defense-in-depth for those specifically, per the non-negotiables above,
- add a redirect allow-list check only in client-side JavaScript "to keep it simple" — restate that this is not a control and must exist server-side,
- keep an implicit-grant OAuth flow because "migrating is a bigger lift than this review's scope" — flag it at HIGH regardless of migration-effort framing; effort is a planning input, not a reason to downgrade a structural finding,
- treat Origin/Referer header validation as a complete CSRF defense with no documented fallback for header-stripping scenarios.
