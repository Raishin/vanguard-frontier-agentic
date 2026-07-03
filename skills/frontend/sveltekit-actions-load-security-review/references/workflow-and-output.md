# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the other two references only for the specific defect class the code under review actually raises.

## Prerequisites

- Read `package.json` and `svelte.config.js` to confirm the SvelteKit major version, adapter, and the current `kit.csrf` configuration (`checkOrigin`, `trustedOrigins`).
- Locate every `+page.server.js`, `+layout.server.js`, `+page.js`, `+layout.js`, `+server.js`, and `hooks.server.js` in scope, plus any `.svelte` templates with `{@html}`.

## Workflow

1. **Check the CSRF configuration.** Read `svelte.config.js`'s `kit.csrf` block. `checkOrigin: false` or `trustedOrigins` containing `'*'` both fully disable the origin check for form-content-type `POST`/`PUT`/`PATCH`/`DELETE` requests. Absence of the block, or `checkOrigin: true` with a named `trustedOrigins` list, is safe. See `references/csrf-and-auth-boundaries.md`.
2. **Trace every form action.** For each entry under `export const actions`, confirm any sensitive read/write is preceded by an auth check on that exact code path (not assumed from elsewhere in the route tree).
3. **Trace every `load()` function's auth boundary.** For each `load()`, determine: does this exact function call a verifying helper (`requireLogin()`-style, reading `event.locals` populated by `hooks.server.js`, or an explicit session-store lookup) before fetching/returning sensitive data? If the auth check lives only in a parent `+layout.server.js`, confirm the child explicitly calls `await parent()` and checks the result, or that `hooks.server.js`'s `handle` enforces the guard before any `load()` runs at all. See `references/csrf-and-auth-boundaries.md`.
4. **Trace every raw cookie value used as an identity claim.** Grep for `cookies.get(...)` and follow its result. If it flows directly into a database call, a trust decision, or a response body with no verification step, that is unguarded — not authenticated.
5. **Enumerate every `cookies.set()` call.** For each, check that `httpOnly`, `secure`, `sameSite`, and `path` are all explicit and set to secure values. See `references/cookies-and-html-bindings.md`.
6. **Enumerate every `{@html}` binding.** For each, trace its data source backward through props, `load()` return data, form data, and API responses to the origin. Determine whether the origin includes user-reachable input and whether a named sanitizer call sits on that exact path. See `references/cookies-and-html-bindings.md`.
7. **Produce ranked findings** using the output contract below.

## Decision tree

- `svelte.config.js` sets `checkOrigin: false`, or `trustedOrigins` contains `'*'` → **HIGH** finding, `csrf-bypass`. Cite SvelteKit's documented `csrf_check_origin` resolution directly.
- `csrf.checkOrigin` is left at its default (`true`) and `trustedOrigins` (if present) lists specific origins with no wildcard → not a finding.
- A `load()` or action fetches/returns sensitive data with no auth check on that exact function's code path → **HIGH** finding, `auth-leakage`.
- An auth check exists only in a parent `+layout.server.js` `load()`, and the child `+page.server.js`/`+page.js` does not call `await parent()` (or does but never inspects the result), and `hooks.server.js` does not independently enforce the guard → **HIGH** finding, `auth-boundary`. Cite SvelteKit's documented "Implications for authentication" note: layout loads do not always re-run on client-side navigation, and sibling loads run concurrently unless `parent()` is awaited.
- The guard is enforced in `hooks.server.js`'s `handle` function before `resolve(event)` runs any `load()` → not a finding regardless of what individual `load()` functions do downstream, provided the `handle` guard actually covers the route in scope.
- A raw `cookies.get(...)` value is passed directly into a database call, an authorization decision, or returned to the client with no verifying lookup → **HIGH** finding, `auth-leakage`.
- `cookies.set()` omits `httpOnly` or sets it to `false` on a session/identity cookie → **HIGH** finding, `cookie-policy`. `secure: false` on a non-`localhost` origin, or a missing `sameSite` → **HIGH**-to-**MEDIUM** depending on the cookie's sensitivity. A missing `path` → at least **MEDIUM** (SvelteKit requires an explicit `path` since v2 to avoid ambiguous scoping).
- `cookies.set()` has explicit `httpOnly: true`, `secure: true` (or the documented `localhost`-only relaxation), `sameSite`, and `path` → not a finding.
- `{@html}` binding's traced data source includes user-reachable input and no sanitizer call is present on that exact path → **HIGH** finding, `xss`. Do not accept "sanitized elsewhere" as clearing this.
- `{@html}` binding's traced data source is fully origin-controlled (static marketing copy, developer-authored content with no user-submission path) → not a finding, but state this explicitly rather than silently omitting it.

## Output contract

Every response from this skill must return:

1. **Scope** — the form action(s), `load()` function(s), `hooks.server.js` handle, `svelte.config.js` csrf block, cookie operations, and/or `{@html}` bindings reviewed.
2. **Ranked findings** — each with file:line, defect category (`csrf-bypass` / `auth-leakage` / `auth-boundary` / `cookie-policy` / `xss`), the concrete data-flow trace naming every hop, and a fix sketch matching SvelteKit's documented pattern.
3. **Sanitizer/guard status per finding** — for `{@html}` findings, an explicit statement of whether a sanitizer call is present on the traced path; for auth-boundary findings, an explicit statement of whether the guard is enforced in `hooks.server.js` or only in a specific `load()`.
4. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `inference`. Label structural risk findings as structural risk explicitly — do not imply confirmed exploitation without live evidence.
5. **Verdict** — approve / approve-with-notes / block.
6. **Open questions or out-of-scope items** — e.g., "confirming actual cross-site exploitation requires a live CSRF proof-of-concept, not static review," or "client-side reactivity/state-management review is out of scope for this actions-and-load-focused skill."

## When to push back

Push back if the user asks to:

- approve `trustedOrigins: ['*']` because "we'll tighten it before launch" — a wildcard origin is equivalent to disabling the check entirely, today,
- treat a `+layout.server.js` auth check as sufficient without confirming the specific child page calls `await parent()` and checks its result, or that `hooks.server.js` independently enforces the guard,
- approve a raw `cookies.get(...)` value as an identity claim because "it's a random-looking string" — randomness is not verification without a server-side lookup or signature check,
- downgrade an untraced `{@html}` finding to informational because "it's probably fine" — this skill's default is HIGH for exactly this class of unproven claim.
