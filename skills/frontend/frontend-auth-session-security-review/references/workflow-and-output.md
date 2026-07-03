# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure, the architecture-classification decision tree, and the required output shape. Load the other two references only for the specific defect class the auth/session code under review actually raises.

## Prerequisites

- Classify the app architecture before anything else:
  - **Cookie-based** — the server sets a session cookie on login and the browser sends it automatically on subsequent requests; no bearer token is manually attached to API calls.
  - **SPA/bearer-token** — the client receives a token (access token, ID token) after authentication and manually attaches it to API requests (typically an `Authorization: Bearer` header), whether via `fetch`/`axios` interceptor or a client SDK.
  - **Hybrid** — a bearer token is used for API calls but a refresh token or session anchor lives in an `HttpOnly` cookie (the current recommended pattern for browser-based apps per OAuth 2.1 guidance).
  - State this classification explicitly in the output before any other finding — cookie-flag findings do not apply to a pure bearer-token flow with no cookies, and token-storage findings about `localStorage` do not apply to a pure cookie-based flow with no client-readable token.
- Read `package.json` and any auth-config files first to confirm the actual auth library/pattern wired up (session middleware, an OAuth/OIDC SDK, a hand-rolled fetch-based flow) before recommending a fix.

## Workflow

1. **Classify the architecture** per the Prerequisites above and state it first.
2. **Locate every token/session-ID storage site.** Grep for `localStorage`, `sessionStorage`, `document.cookie`, cookie-library calls (`Set-Cookie`, `cookie.set`, session-middleware config), and in-memory storage (a JS variable/closure/module-scope singleton). For each, identify what is stored (session ID, access token, refresh token, ID token) and where. See `references/token-storage-and-cookies.md`.
3. **For every session cookie found, extract its attribute set** (`HttpOnly`, `Secure`, `SameSite`, `Domain`, `Path`, and whether a `__Host-`/`__Secure-` prefix is used). Compare against the compliance table in `references/token-storage-and-cookies.md`.
4. **Enumerate every state-changing request path** (form POST, fetch/axios mutation call) reachable from an authenticated session. For each, determine whether a CSRF defense is present (synchronizer token, double-submit cookie, `SameSite=Strict`/`Lax` reliance, or a custom-header-based defense for API-only surfaces). See `references/csrf-redirect-and-oauth.md`.
5. **Enumerate every redirect/return-URL parameter** (post-login redirect, OAuth `redirect_uri` handling, logout redirect, "return to" deep links). For each, trace whether validation happens server-side against an allow-list, client-side only, or not at all.
6. **If an OAuth/OIDC flow is present**, identify the `response_type` and grant type in use. Flag implicit grant (`response_type=token`) as a finding. Confirm PKCE (`code_challenge`/`code_verifier`) is present for the authorization code flow. Load the OAuth/OIDC subsection of `references/csrf-redirect-and-oauth.md` only in this case.
7. **Check logout behavior.** Confirm logout triggers a server-side session/token invalidation (session-store deletion, token-revocation endpoint call), not only a client-side storage clear.
8. **Produce ranked findings** using the output contract below.

## Decision tree

- Session ID or access/refresh token stored in `localStorage`/`sessionStorage` → **HIGH** finding (XSS-exposure risk: any injected script reads it via `document`/`window` APIs with no mitigation). Note whether a known XSS sink exists in the same codebase (raises severity further) or whether none was found in the scope reviewed (still HIGH — absence-of-observed-XSS is not evidence of safety).
- Session cookie missing `HttpOnly` → **HIGH** finding (defeats the cookie's primary XSS-mitigation purpose).
- Session cookie missing `Secure` → **HIGH** finding (cookie can be sent over plaintext HTTP; also blocks safe use of `__Host-`/`__Secure-` prefixes).
- Session cookie with `SameSite=None` and no `Secure` → **HIGH** finding (browsers increasingly reject this combination outright, but code relying on it is broken-by-design regardless).
- Session cookie with `SameSite` unset (relying on browser default) → **MEDIUM** finding — do not rely on browser defaults, which vary by browser and version; require an explicit value.
- State-changing request with no CSRF token, no `SameSite=Strict`/`Lax` cookie reliance, and no custom-header-based defense → **HIGH** finding.
- State-changing request relying solely on `SameSite=Lax`/`Strict` with no token-based defense-in-depth → **MEDIUM** finding — acceptable as a primary defense per current OWASP guidance, but flag the absence of defense-in-depth for sensitive operations (e.g., password change, payment).
- Redirect/return-URL parameter validated only client-side (a JS check before navigation, with no equivalent server-side check) → **HIGH** finding — bypassable by calling the server endpoint directly.
- Redirect/return-URL parameter with no validation at all → **HIGH** finding, open redirect.
- Redirect/return-URL parameter validated server-side against an allow-list of trusted hosts/paths → not a finding.
- OAuth/OIDC flow using `response_type=token` (implicit grant) → **HIGH** finding — current guidance requires authorization code flow with PKCE for public clients.
- OAuth/OIDC authorization code flow present but missing `code_challenge`/`code_verifier` (no PKCE) for a public client (SPA, mobile app with no client secret) → **HIGH** finding.
- OAuth/OIDC authorization code flow with PKCE correctly present → not a finding on flow choice (still check `redirect_uri` validation and token-storage location separately).
- Logout clears client-side storage only, with no server-side session/token invalidation call → **MEDIUM-to-HIGH** finding depending on token lifetime (a short-lived access token narrows the window; a long-lived or non-expiring token makes this HIGH).

## Output contract

Every response from this skill must return:

1. **Architecture classification** — cookie-based, SPA/bearer-token, or hybrid, stated first and driving which finding categories apply.
2. **Ranked findings** — each with file:line, defect category (`token-storage` / `cookie-flags` / `csrf` / `open-redirect` / `oauth-flow` / `logout-invalidation`), and a fix sketch grounded in the cited OWASP/OAuth guidance.
3. **Cookie-flag compliance table** — if session cookies are in scope, a table of cookie name → `HttpOnly`/`Secure`/`SameSite`/prefix status.
4. **Sanitized examples only** — every finding illustrated with placeholder/redacted values, never a real token, cookie, or secret observed in the reviewed code.
5. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `inference`.
6. **Explicit no-live-testing statement** — this skill performs static review only; it never attempts session hijacking, token replay, or CSRF exploitation against a live system.
7. **Verdict** — approve / approve-with-notes / block.
8. **Open questions or out-of-scope items** — e.g., "confirming this CSRF gap is exploitable end-to-end requires a live request against a running instance, not static review," or "JWT signature/algorithm choice is out of scope for this skill — recommend a token-issuance/backend review."

## When to push back

Push back if the user asks to:

- accept `localStorage` token storage because "we don't have any XSS right now" — absence of a known XSS finding in this review's scope is not proof none exists; the storage choice itself is the risk being flagged,
- skip cookie-flag validation because "the framework sets sane defaults" — verify the actual `Set-Cookie` header or session-middleware config; frameworks vary and defaults change across versions,
- treat a client-side-only redirect check as sufficient because "the server also has auth on that route" — authentication on the target route does not validate that the redirect *destination* itself was vetted; these are separate controls,
- approve an implicit-grant OAuth flow because "it's simpler to implement" — current guidance has removed implicit grant from OAuth 2.1 specifically because of its token-leakage-via-URL-fragment and no-refresh-token exposure; simplicity does not offset this,
- treat "we cleared localStorage on logout" as equivalent to server-side session invalidation — a still-valid token can be replayed from a captured request or a second device/tab until it naturally expires.
