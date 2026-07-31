---
name: frontend-auth-session-security-review
description: Review client-side authentication and session-management code for token-storage location, cookie-flag correctness, CSRF/open-redirect exposure, and OAuth/OIDC flow choice for browser-based apps against OWASP ASVS and Session Management Cheat Sheet guidance, with the OAuth-for-browsers reference loaded only when an OAuth/OIDC flow is in scope.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: security
---

# Frontend Auth & Session Security Review

## Purpose

Most client-side account-takeover incidents come from session-management shortcuts, not cryptographic flaws: tokens in `localStorage` exposed to any XSS, missing `HttpOnly`/`Secure`/`SameSite` cookie flags, client-only redirect validation enabling open redirects, or implicit-grant OAuth flows that current guidance has superseded. This skill reviews against OWASP ASVS session-management requirements and the current browser-based-app OAuth best practice, not outdated tutorial patterns. It exists so the review stays anchored to these four documented defect classes — token storage, cookie flags, CSRF/open-redirect, OAuth flow choice — instead of drifting into a general "auth code review."

## When to use

Use this skill when the user asks to:

- review where and how auth tokens/session identifiers are stored client-side,
- audit cookie attributes (`HttpOnly`, `Secure`, `SameSite`, `Domain`, `Path`) on session cookies,
- review a login/logout/session-refresh/redirect flow for CSRF or open-redirect exposure,
- review or design an OAuth 2.0/OIDC flow for a single-page or browser-based app,
- triage a session-fixation or account-takeover bug report.

Do not use this skill for:

- server-side authorization/access-control logic review (role checks, object-level permission enforcement) with no client-side session-handling angle — that is a backend authorization review, not a frontend session-security review,
- a DOM XSS or injection root-cause review with no session/auth angle — use a dedicated XSS/injection review skill; this skill treats "is there an XSS sink reachable from this token" only insofar as it changes the token-storage risk verdict, it does not hunt XSS sinks exhaustively,
- cryptographic algorithm or JWT signature-implementation review (choosing HS256 vs RS256, key-rotation mechanics) — that is a token-issuance/backend concern, not a client-side session-management concern,
- confirming that a session-fixation or CSRF bug has already been exploited in production — static review proves the structural risk, not confirmed exploitation; that requires live traffic analysis or a penetration test.

## Context7 Documentation Protocol

- Resolve the OWASP Cheat Sheet Series library ID with `resolve-library-id` (matched result: `/owasp/cheatsheetseries`) before citing any session-cookie-flag, CSRF-defense, or open-redirect-prevention claim; use `query-docs` against it to ground the exact flag/pattern being recommended (e.g., `__Host-` prefix requirements, `SameSite=Strict` vs `Lax` tradeoffs, synchronizer-token vs double-submit-cookie pattern).
- For OAuth/OIDC flow-choice claims (PKCE requirement for public clients, implicit-grant removal), resolve and query `/websites/datatracker_ietf_doc_draft-ietf-oauth-v2-1` (OAuth 2.1, which formalizes the browser-based-app guidance: implicit and hybrid flows removed, PKCE required for public clients). This is an IETF draft and its exact section numbers/text shift between draft revisions — label version-specific text as `documentation-based, verify against current draft revision`, not as ratified RFC text.
- Do not conflate the OAuth 2.1 draft's PKCE-for-public-clients requirement with a guarantee that a specific SDK or framework already implements it correctly — verify the actual library/SDK in use (via its own docs or Context7 entry, if one exists) before asserting the app's flow is compliant.
- If Context7 is unavailable for either library, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.
- Read `package.json` (and any auth-library config) first to confirm which auth pattern is actually wired up (cookie-session middleware, an OAuth/OIDC client SDK, a hand-rolled fetch-based token flow) before recommending a fix — do not prescribe a pattern the app's architecture cannot support without a larger refactor being made explicit.

## Lean operating rules

- First classify the app architecture: traditional server-rendered app using cookies for session state, vs. SPA/browser-based app calling an API with bearer tokens. The correct token-storage and CSRF-defense pattern differs by architecture — a cookie-flag finding does not apply to a pure bearer-token SPA, and vice versa. State this classification explicitly before any other finding.
- Never bless `localStorage`/`sessionStorage` for session or access tokens as a default recommendation. If the codebase already uses it, treat it as a finding (XSS-exposure risk: any injected script can read it) and only accept it as a deliberate, justified tradeoff if the user explicitly argues the tradeoff and no unresolved XSS-sink concern exists in the reviewed scope — this skill does not itself clear that bar, it flags it.
- Verify every session cookie has `HttpOnly`, `Secure`, and an explicit `SameSite` value appropriate to the flow. `SameSite=None` is only acceptable paired with `Secure` and a documented cross-site necessity (e.g., a third-party embed); never accept `SameSite=None` without `Secure`, and never accept an unset `SameSite` (browser defaults vary and should not be relied on).
- For OAuth/OIDC in browser-based apps, flag the implicit grant (`response_type=token`) as a finding — current guidance requires the authorization code flow with PKCE for public clients. Do not recommend implicit grant for new work under any circumstance.
- Verify redirect/return-URL parameters (post-login redirect, OAuth `redirect_uri`, logout redirect) are validated against a server-side allow-list, not merely checked client-side. A client-only check (e.g., a JS regex before `window.location.assign`) is bypassable by directly hitting the server endpoint with the malicious parameter and is not a valid control on its own.
- Never ask for or print real tokens, cookies, session IDs, or OAuth client secrets during review; use placeholder or redacted values in every example and finding.
- Check that logout actually invalidates the session/token server-side (revocation call, server-side session-store deletion), not just clears client-side storage — a client-only logout leaves the token valid for replay until natural expiry.
- Load only the reference needed for the concern in scope; never load the OAuth reference when no OAuth/OIDC flow is present in the reviewed code.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the architecture-classification decision tree, and the required output shape.
- [Token storage and cookie-flag review](references/token-storage-and-cookies.md) — load when the review scope includes where tokens/session IDs are stored or how session cookies are configured.
- [CSRF, open-redirect, and OAuth/OIDC flow review](references/csrf-redirect-and-oauth.md) — load when the review scope includes a state-changing request's CSRF defenses, a redirect/return-URL parameter, or an OAuth/OIDC authorization flow. Its OAuth/OIDC subsection applies only when an OAuth/OIDC flow is actually present in scope.

## Response minimum

Return, at minimum:

- the app architecture classification (cookie-based vs SPA/bearer-token) driving the recommendation,
- token-storage location finding and its XSS-exposure implication,
- cookie-flag compliance table (`HttpOnly`/`Secure`/`SameSite`) if cookies are in scope,
- CSRF-defense assessment (token pattern present/absent, `SameSite` reliance) for state-changing requests in scope,
- redirect/return-URL validation finding (server-side allow-list present or absent) if a redirect parameter is in scope,
- OAuth/OIDC flow assessment (PKCE-based authorization code vs deprecated implicit) only if an OAuth/OIDC flow is in scope,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- explicit statement that no live session hijacking, token replay, or CSRF exploitation was performed — this is a static review,
- verdict (approve / approve-with-notes / block) and open questions the review could not resolve statically.
