---
name: sveltekit-actions-load-security-review
description: Statically review SvelteKit form actions, load functions, hooks, and templates for CSRF origin-check bypass (checkOrigin/trustedOrigins), unauthenticated sensitive-data returns from load(), auth guards confined to +layout.server.js without an enforced parent()/hooks check, insecure cookies.set() options, and unsanitized {@html} bindings, grounded in SvelteKit's own CSRF, cookies, load, and authentication documentation.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-03"
  category: security
---

# SvelteKit Actions & Load Security Review

## Purpose

Review SvelteKit form actions (`export const actions`), `load()` functions (`+page.server.js`/`+page.js`, `+layout.server.js`/`+layout.js`), `+server.js` endpoints, and template bindings for the concrete, documented defect classes that recur in SvelteKit apps: CSRF protection disabled or weakened via `checkOrigin`/`trustedOrigins`, `load()` returning sensitive data with no auth check on that exact path, an auth guard that lives only in a parent `+layout.server.js` and is silently skipped by a child page that never calls `await parent()` (or by client-side navigation that does not re-run the layout load), cookies set without `httpOnly`/`secure`/`sameSite`/`path` explicitly locked down, and unsanitized `{@html}` rendering of user-reachable input. This skill exists so the review stays anchored to these five documented, security-critical sinks instead of drifting into a general "SvelteKit code review" of routing, reactivity, or component design.

## When to use

Use this skill when the user asks to:

- review a SvelteKit form action (`export const actions`) or a `load()` function for authentication/authorization correctness,
- assess whether `svelte.config.js`'s `csrf` block (`checkOrigin`, `trustedOrigins`) is safely configured,
- investigate whether an authenticated route is actually protected on every entry path (direct page load, client-side navigation, and the action itself),
- assess whether a `cookies.set()` call or an `{@html}` binding is safe,
- perform a pre-launch security review of a SvelteKit application's server-side data-loading and mutation surface.

Do not use this skill for:

- general SvelteKit routing, reactivity (`$state`/`$derived`), or component-composition review with no security angle — use a SvelteKit architecture-focused skill instead,
- a purely client-only Svelte component tree with no `+page.server.js`/`+layout.server.js`/`+server.js`/form-action code and no cookie or `{@html}` usage — there is no server-side sink in scope,
- a bug that requires live traffic reproduction (a captured cross-site request, a live CSRF proof-of-concept, session-replay capture) to confirm exploitation — static analysis proves the structural risk, not that it has already been exploited in production.

## Context7 Documentation Protocol

- Resolve the library ID with `resolve-library-id` (matched result: `/sveltejs/kit`) before citing any CSRF-mechanism, cookie-default, or `load()`/auth claim.
- `/sveltejs/kit` is SvelteKit's own repository (runtime source such as `respond.js`/`cookie.js` plus its documentation tree), so both source-level mechanics and prose guidance are queryable through `query-docs`. Use it to confirm exact runtime behavior — e.g., that `csrf_check_origin` only rejects same-origin-mismatched, form-content-type `POST`/`PUT`/`PATCH`/`DELETE` requests, that `trustedOrigins: ['*']` fully disables the origin check regardless of `checkOrigin`, and that `cookies.set()` defaults `httpOnly` and `secure` to `true` (with `secure` relaxed only on plain-HTTP `localhost`) and `sameSite` to `'lax'`.
- Before flagging a `+layout.server.js` auth guard as insufficient, confirm via `query-docs` (or the `official_docs` URLs in this skill's `metadata.json` if Context7 is unavailable) that layout `load()` functions do not re-run on every child-route navigation and that layout/page `load()` functions run concurrently unless a child explicitly calls `await parent()` — this is the documented mechanism behind the risk, not an assumption.
- Read `package.json` and `svelte.config.js` first to confirm the SvelteKit major version and adapter in use — cookie defaults (the `path`-required behavior) and CSRF option shape changed between SvelteKit 1 and 2; do not apply v2 requirements to a v1 codebase or vice versa.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- CSRF-bypass, auth-leakage, and XSS findings default to HIGH severity. This is a security-scoped skill: do not downgrade a `checkOrigin: false`, a wildcard `trustedOrigins`, an unguarded sensitive `load()` return, or an untraced `{@html}` sanitizer gap to MEDIUM just because it has not been observed exploited yet — the risk is in the structure, not in whether someone has already hit it.
- Trace every finding to a concrete file:line and a concrete data-flow path. A finding that says "this load() might leak data" or "this cookie might be insecure" without showing the specific `cookies.get()` call, the specific missing guard, or the specific `cookies.set()` options object is not a valid finding — it is a guess.
- For every `export function load()` (or `export const actions`), determine whether an auth check happens *inside that exact function* (e.g., via a `requireLogin()`-style helper reading `event.locals`/`cookies`) before any sensitive data is fetched or returned. Do not accept "the parent layout checks auth" as sufficient unless the specific child `load()` under review actually calls `await parent()` and that call's result is checked, or `hooks.server.js`'s `handle` function enforces the guard before any `load()` runs.
- Do not approve a raw `cookies.get(...)` value being passed directly into a database call or trusted as an identity claim. A session/user identity must be resolved through a verifying helper (session-store lookup, signature check, `requireLogin()`) — a lookup with no verification step is not authentication, it is an unguarded read keyed on attacker-controlled input.
- Check every `cookies.set()` call for explicit `httpOnly`, `secure`, `sameSite`, and `path`. Flag any call that sets `httpOnly: false` (or otherwise turns off a secure default) on a session/identity cookie as HIGH, and flag any call missing `path` as at least MEDIUM (SvelteKit requires an explicit `path` since v2 specifically to avoid ambiguous cookie scoping).
- Do not approve an `{@html}` binding whose data source includes any user-reachable input (route params, query strings, request bodies, form-submitted content, third-party API responses that themselves echo user input) unless a named sanitizer call (e.g., `DOMPurify.sanitize()`) is visibly present on that exact data-flow path in the template expression. A sanitizer import existing elsewhere in the codebase does not clear this bar.
- Treat `checkOrigin: false` and `trustedOrigins: ['*']` as equally severe: both fully disable SvelteKit's CSRF origin check for cross-site form submissions, even though only one of them touches the literal `checkOrigin` key.
- Never execute, build, or run application code, and never send live requests, as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the CSRF/auth/cookie/XSS decision tree, and the required output shape.
- [CSRF and auth-boundary review](references/csrf-and-auth-boundaries.md) — load only when the review scope includes `svelte.config.js`'s `csrf` block, a form action, or a `load()`/`+layout.server.js`/`hooks.server.js` auth-guard trace.
- [Cookies and {@html} review](references/cookies-and-html-bindings.md) — load only when the review scope includes a `cookies.set()` call or an `{@html}` template binding.

## Response minimum

Return, at minimum:

- the form action(s), `load()` function(s), `hooks.server.js` handle, `svelte.config.js` csrf block, cookie operations, and/or `{@html}` bindings in scope,
- ranked findings with file:line evidence, defect category (`csrf-bypass`, `auth-leakage`, `auth-boundary`, `cookie-policy`, or `xss`), the concrete data-flow trace (the exact `checkOrigin`/`trustedOrigins` value, the `cookies.get()`-to-sink path, the layout-to-child guard gap, the `cookies.set()` options object, or the `{@html}` origin-to-sink path), and a fix sketch matching SvelteKit's documented pattern,
- for every `{@html}` finding, an explicit statement of whether a sanitizer call is present on the traced path — never approve on the assumption one exists elsewhere,
- for every auth-boundary finding, an explicit statement of whether the guard is enforced in `hooks.server.js` (applies to every request) or only in a `+layout.server.js`/`+page.server.js` `load()` (applies only if that exact function runs and, for a layout, only if a child calls `await parent()`),
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`), with structural risk findings explicitly labeled as structural risk, not as confirmed-exploited,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "confirming actual cross-site exploitation requires a live CSRF proof-of-concept, not static review").
