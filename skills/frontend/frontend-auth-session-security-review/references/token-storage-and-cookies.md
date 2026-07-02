# Token Storage and Cookie-Flag Review

Use this reference when the review scope includes where tokens or session IDs are stored client-side, or how session cookies are configured. Grounded in the OWASP Session Management Cheat Sheet (`/owasp/cheatsheetseries` via Context7, or `official_docs` fallback).

## What people get wrong

The common bad assumption is:

> "We use HTTPS, so cookies/tokens are safe."

TLS protects data in transit between the browser and server. It does nothing about:

- a script running in the page's own origin reading `document.cookie` (no `HttpOnly`) or `localStorage`,
- a cookie being sent to the wrong origin or subdomain (`SameSite`/`Domain` misconfiguration),
- a token being replayed after logout because nothing invalidated it server-side.

Storage location and cookie attributes are a separate control surface from transport security. Both must be correct.

## Storage location: the tradeoffs, stated plainly

- **`HttpOnly` cookie** — not readable by JavaScript. The strongest default for a session identifier or refresh token. Cannot be attached manually to cross-origin API calls (the browser does that automatically only for same-origin/configured-domain requests), which is why bearer-token SPAs calling a separate API origin often cannot use this alone.
- **`localStorage`/`sessionStorage`** — readable by any JavaScript running in the page's origin, including any successfully injected script (stored or reflected XSS, a compromised third-party script/CDN dependency, a malicious browser extension with page access). No same-origin isolation between "your code" and "any script that got a foothold." Never the default recommendation for a session token or refresh token.
- **In-memory (JS variable/closure, not persisted)** — not readable via `document.cookie` or storage APIs, and does not survive a page reload without a re-fetch (typically via an `HttpOnly` refresh-token cookie). This is the pattern OAuth 2.1 / current SPA guidance converges on: short-lived access token in memory, refresh mechanism anchored in an `HttpOnly` cookie.

## Cookie-flag compliance table (build this for every session cookie in scope)

| Attribute | Required value | Why |
|---|---|---|
| `HttpOnly` | present | Blocks `document.cookie` read access from JS; the primary XSS-exfiltration mitigation for the cookie. Does not stop the cookie being *sent* during a combined XSS+CSRF attack — pair with CSRF defenses. |
| `Secure` | present | Cookie is only sent over HTTPS; without it, a network attacker on an insecure network segment can observe or (on `SameSite=None` without `Secure`, which browsers reject) inject it. |
| `SameSite` | `Strict` (preferred) or `Lax`; never unset, never `None` without `Secure` | Prevents the browser from attaching the cookie to cross-site requests, mitigating CSRF and cross-origin leakage. Do not rely on an unset value — browser default behavior has changed across versions and should not be the enforcement mechanism. |
| `Domain` | omitted, or scoped to the narrowest subdomain that needs it | An overly broad `Domain` (e.g., a session cookie set on the parent domain when only one subdomain needs it) widens the blast radius if any sibling subdomain is compromised. |
| `Path` | `/` or the narrowest path needed | Broad by convention for session cookies; flag only if a narrower path was clearly intended and not applied. |
| `__Host-` prefix | recommended when `Domain` is omitted and `Path=/` | Browser-enforced: only accepted if `Secure`, no `Domain` attribute, and `Path=/`. Prevents subdomain-forgery and downgrade attacks on the cookie name itself. |

Extract this table from the actual `Set-Cookie` header construction in code (session-middleware config, a manual `res.cookie(...)`/`Set-Cookie` call) — do not infer flags from framework defaults without checking the actual configured options, since defaults vary by framework and version.

## Non-negotiables

- Do not accept "the session cookie is `HttpOnly`" as sufficient on its own without also checking `Secure` and `SameSite` — all three are independent controls addressing different attack vectors (XSS-read, network interception, cross-site request attachment).
- Do not accept a refresh token or long-lived credential in `localStorage`/`sessionStorage` under any framing ("just for this internal tool," "we'll migrate later") without flagging it as a finding — a refresh token grants renewable access and is a higher-value target than a short-lived access token.
- Treat an access token held only in memory (a JS variable, not persisted storage) as acceptable for that token alone, but check separately how the app re-obtains a new access token after a page reload — if that mechanism reads a token from `localStorage` instead of an `HttpOnly` refresh-cookie flow, the finding moves to that mechanism.
- Session ID/token entropy and length are a server-side generation concern (cryptographically secure random generation, sufficient bit length per OWASP Session Management guidance) — verify it if the generation code is in scope, but do not assume insufficient entropy without reading the actual generation call; do not guess.

## Verification targets

- Actual `Set-Cookie` header value (from server code, middleware config, or a captured response header if the user provides sanitized evidence) — not the framework's documented default.
- The token re-acquisition path after page reload/tab reopen, to catch a `localStorage` fallback hiding behind an otherwise-correct in-memory primary storage.
- The logout code path, to confirm it triggers server-side invalidation (see `references/workflow-and-output.md` decision tree) rather than only clearing client-side state.
