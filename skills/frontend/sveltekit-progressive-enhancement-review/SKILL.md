---
name: sveltekit-progressive-enhancement-review
description: Statically review SvelteKit forms and form actions for functional resilience without JavaScript (native method="POST" fallback) and for use:enhance customization correctness (ActionResult branch handling, cancel() feedback, redirect/invalidation behavior), flagging silent-failure and conversion-risk defects.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# SvelteKit Progressive Enhancement Review

## Purpose

Review SvelteKit forms and form actions for progressive-enhancement resilience — whether the form still functions via a native browser POST when JavaScript fails, is blocked, or hasn't finished loading — and for `use:enhance` customization correctness, without re-litigating server/client `load`-function placement, component styling, or routing precedence in every response. This skill exists because a form that "works fine" during manual testing with JavaScript enabled can still be a dead end for the measurable share of real users who hit slow/blocked/failed JS on real-world connections, and because a custom `use:enhance` `SubmitFunction` that only handles the happy path silently swallows every `failure`/`error`/`redirect` `ActionResult`, turning a broken submission into a UI that does nothing and tells the user nothing.

## When to use

Use this skill when the user asks to:

- review a new or changed `<form>` / form-action implementation before merge,
- confirm whether a form "works without JavaScript",
- investigate a report that a form submission silently does nothing (no error, no navigation, no feedback) when it fails,
- audit a custom `use:enhance` `SubmitFunction` for missing `ActionResult` branches or unexplained `cancel()` calls.

Do not use this skill for:

- internal admin tools explicitly scoped as JS-required — progressive enhancement may be intentionally out of scope there; confirm that scoping with the user before flagging anything,
- `+page.js` / `+page.server.js` / `+layout.js` universal-vs-server `load` placement review — use `sveltekit-routing-load-review` instead,
- pure visual/styling review of form markup with no functional or error-handling question in scope.

## Context7 Documentation Protocol

- Resolve the library ID with `resolve-library-id` (matched result: `/sveltejs/kit`) before citing any SvelteKit-specific claim about form actions or `use:enhance`.
- Before asserting what the **default, no-argument** `use:enhance` provides, call `query-docs` against `/sveltejs/kit` for "progressive enhancement use:enhance" and quote the precise rule: `use:enhance` only applies to a `<form method="POST">` that posts to an action defined in `+page.server.js`; used without arguments it emulates native browser behavior — it updates the `form` prop and page status on success, resets the form element, calls `invalidateAll()`, and handles redirects, error boundaries, and focus management automatically, all without a full-page reload. Do not assume feature parity with a generic SPA form-handling library; this behavior is SvelteKit-specific and documented, not inferred.
- Before asserting what a **custom** `SubmitFunction` must reimplement, call `query-docs` against `/sveltejs/kit` for "customising use:enhance SubmitFunction ActionResult" and confirm: the `SubmitFunction` receives `{ formElement, formData, action, cancel, submitter }` before submission, and may return an async callback receiving `{ result, update }` — where `result` is the `ActionResult` (`success` | `failure` | `redirect` | `error`) and `update` triggers the default post-submission logic that would otherwise run. If the callback is supplied at all, none of that default logic runs unless the callback calls `update()` or manually replicates it (e.g. via `applyAction(result)`).
- Verify the SvelteKit version installed in the repo (`package.json`) before asserting `use:enhance` or `ActionResult` shape details are unchanged from the queried docs — form-action APIs have evolved across SvelteKit majors.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.
- Never assume `applyAction` or `invalidateAll` is called inside a custom callback without reading the callback body — their absence is exactly the defect this skill exists to catch.

## Lean operating rules

- First classify every `<form>` in scope as native-POST-capable (has `method="POST"` and a real action target resolving to a `+page.server.js` export) or JS-only (driven solely by a click/`onclick` handler with no native fallback) — do this by reading the markup, never by assuming.
- A native `<form method="POST">` with **no** `use:enhance` at all already works without JavaScript by default; that is the baseline SvelteKit contract, not a defect. Do not flag its absence as a finding on its own.
- Bare (no-argument) `use:enhance` gets the full documented default behavior (form reset, `invalidateAll`, redirect/error/focus handling) for free — do not demand the caller reimplement anything for a bare `use:enhance` usage.
- Every custom `SubmitFunction` callback must branch on `result.type`. A callback that assumes `result` is always a success (e.g., only reads `result.data` with no type check) is a silent-failure defect: `failure`/`error`/`redirect` results are dropped on the floor and the user sees nothing.
- A `cancel()` call inside the pre-submission `SubmitFunction` body must be paired with a clear, user-facing reason (e.g., inline client-side validation message shown to the user) — a silent `cancel()` with no visible feedback is equivalent to a silent failure.
- Do not accept "an error variable exists in component state" as proof that failures are surfaced to the user. Trace whether that variable is actually rendered in the markup and, for accessibility, associated with the form via a status-message pattern (e.g., `aria-live` region or equivalent) — an unrendered or unannounced error state is still a silent failure for sighted users tracking visually and a silent failure for screen-reader users regardless.
- Treat any custom `SubmitFunction` that bypasses the form's native `action`/`method` to call a hand-rolled `fetch()` directly (rather than letting `use:enhance` submit natively or using SvelteKit's own `deserialize()`/`applyAction()` pattern) as a security-review flag, not a pure UX nit — it can skip origin-check behavior SvelteKit applies to form-action submissions; do not wave it through as a stylistic preference.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only). "Works without JS" claims must be verified by reading markup for `method="POST"` and a resolvable action, never assumed from a component name or comment.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the decision tree for classifying findings, and the required output shape.
- [Failure-state and accessibility UX](references/failure-state-and-accessibility.md) — load only when reviewing how a failed/error/cancelled submission is surfaced to sighted and screen-reader users, not for pure native-fallback presence checks.

## Response minimum

Return, at minimum:

- the form(s) and form action(s) in scope,
- evidence level (`repo evidence` from markup/action code vs `inference`) and Context7/docs grounding used,
- ranked findings (file:line, missing-fallback or unhandled-`ActionResult`-branch gap, fix sketch matching documented `use:enhance`/`applyAction` patterns),
- verdict on whether any public-facing, unauthenticated-reachable form (signup, contact, checkout) lacks a native fallback — this is a hard stop, not advisory,
- open questions (e.g., whether an internal tool is confirmed JS-required by design before exempting it).
