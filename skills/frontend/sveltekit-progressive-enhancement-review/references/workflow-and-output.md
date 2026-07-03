# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure, the decision tree for classifying progressive-enhancement and `use:enhance` findings, and the required output shape.

> Version note: `use:enhance`, `ActionResult`, and `applyAction`/`deserialize` behavior are documented for current SvelteKit releases. Verify the installed `@sveltejs/kit` version in `package.json` before asserting exact default behavior applies unchanged; form-action APIs have shifted across majors.

## What people get wrong

The naive assumption is:

> "The form has `use:enhance`, so it's progressively enhanced and handles errors."

That is wrong in two independent ways:

1. `use:enhance` is not what makes a form work without JavaScript — the native `method="POST"` + resolvable action target does that, and it works with **zero** `use:enhance` at all. `use:enhance` only improves the *with-JS* experience (no full reload, focus management, etc.) on top of a fallback that must already exist.
2. Adding `use:enhance` with a **custom** `SubmitFunction` callback does not inherit any of the documented default behavior (form reset, `invalidateAll`, redirect/error/focus handling) — supplying a callback opts out of all of it unless the callback explicitly calls `update()` or reimplements the equivalent via `applyAction(result)`. A callback that only handles the success path silently drops `failure`, `error`, and `redirect` results.

## Step-by-step workflow

1. **Inventory forms in scope.** For every `<form>` in the reviewed files, record its `method`, `action` (or lack thereof), and whether it resolves to a real form action exported from a `+page.server.js`.
2. **Classify native-fallback status.** A form is native-POST-capable only if it has `method="POST"` and a resolvable action. A form driven solely by a client `onclick`/`onsubmit` JS handler with no native `method`/`action` pair has **no** fallback — note this explicitly, it is the highest-severity category below.
3. **Classify `use:enhance` usage per form**, one of:
   - absent (native-only; correct baseline if that's the intended UX, not a defect by itself),
   - bare/no-argument (gets full documented default behavior for free),
   - custom `SubmitFunction` with no returned callback (pre-submission-only customization, e.g. loading state; post-submission handling still defaults),
   - custom `SubmitFunction` with a returned callback (post-submission handling is now entirely the callback's responsibility).
4. **For every returned callback, check `result.type` branching.** Read the callback body. Confirm it inspects `result.type` and handles (directly or via `applyAction(result)`/`update()`) each of `success`, `failure`, `error`, and `redirect` that the corresponding form action can actually produce. A callback that reads `result.data` unconditionally, with no `result.type` check, is a silent-failure defect.
5. **Check every `cancel()` call site.** For each `cancel()` invoked in the pre-submission `SubmitFunction` body, confirm the surrounding code also produces a user-visible signal (e.g., sets a validation-message variable that is rendered) before or at the point of cancellation. A `cancel()` with no paired visible feedback is a silent failure from the user's perspective, identical in effect to a swallowed `ActionResult`.
6. **Trace failure-state rendering**, not just its existence in state. Confirm the variable/prop holding an error or failure message is actually referenced in the template output, and check whether it is delivered via an accessible status-message pattern (`aria-live`, `role="alert"`, or equivalent) — see `references/failure-state-and-accessibility.md` for that sub-review when it's in scope.
7. **Flag any hand-rolled `fetch()` bypass.** If a custom `SubmitFunction` (or its returned callback) calls `fetch()` directly against a URL other than the form's own `action`, or reimplements submission without going through the form's native `action`/`method`, flag it for security review — this can bypass origin-check behavior SvelteKit applies to form-action submissions. Do not treat it as pure style.
8. **Rank and report findings** per the output shape below.

## Decision tree

- Public-facing, unauthenticated-reachable form (signup, contact, checkout, etc.) has no `method="POST"` and no resolvable action — submission is JS-only via a click handler → **HIGH, hard stop**. Zero-JS users cannot submit at all; this blocks approval for conversion-critical public forms specifically.
- Same JS-only pattern on an internal/authenticated tool **that the user has explicitly confirmed is JS-required by design** → note as informational, not a finding; do not assume this exemption, ask for it.
- Custom `SubmitFunction` returns a callback that does not branch on `result.type` (assumes success) → **HIGH**. Identify which of `failure`/`error`/`redirect` is unhandled and cite the exact missing branch.
- `cancel()` called with no user-facing reason surfaced anywhere in the same code path → **HIGH** for a public-facing form, **MEDIUM** for an internal tool — silent cancellation is a UX dead end either way.
- Native `<form method="POST">` with no `use:enhance` at all, and the fallback works correctly (submits, server redirects/re-renders with `form` prop on failure) → **not a finding**; this is the correct, working baseline.
- Bare (no-argument) `use:enhance` on a form that would benefit from avoiding full-page reloads for inline validation UX, but the native fallback still works correctly → **LOW/informational** enhancement suggestion, not a defect.
- Error/failure state exists in a variable but is never rendered in the template, or rendered without any accessible status-message association → **MEDIUM** (visual users can still see nothing) escalating to **HIGH** if the form is public-facing and conversion-critical.
- Custom `SubmitFunction` or its callback calls `fetch()` directly instead of relying on native submission or `deserialize()`/`applyAction()` → **HIGH, flag for security review** (potential origin-check bypass), independent of whether error handling is otherwise correct.

## Adversarial checklist

Before closing a review with no findings, confirm:

- Does this form submit correctly via a plain HTTP POST with JavaScript entirely disabled — i.e., does it have a native `method="POST"` and a resolvable action, verified by reading the markup, not assumed?
- Does every `use:enhance` customization branch on all `ActionResult` types it can realistically receive, or does it assume happy-path only?
- Is every validation-triggered `cancel()` paired with a user-visible reason, not just a silent early return?
- Would a screen-reader user actually be informed of a failed submission (rendered, accessibly-associated status message), or does the failure exist only as unread component state or a purely visual cue?
- Did you flag every direct-`fetch()` bypass of native form submission for security review, even if its error handling looked otherwise correct?
- Is a form flagged only for missing `use:enhance` when its native fallback in fact works fine? If so, that is a false positive — remove it; absence of `use:enhance` is not itself a defect.

## Output shape

Every review response must include:

1. **Scope** — forms and form actions reviewed, each labeled by native-fallback status (present/absent) and `use:enhance` usage category (absent / bare / custom-no-callback / custom-with-callback).
2. **Findings** — ranked HIGH → MEDIUM → LOW, each with `file:line`, the classification (missing-fallback vs. unhandled-`ActionResult`-branch vs. silent-`cancel()` vs. unrendered/inaccessible failure state vs. fetch-bypass), and a concrete fix sketch matching the documented `use:enhance`/`applyAction`/`deserialize` patterns.
3. **Evidence level** per finding: `repo evidence` (read the actual markup/action code) or `inference` (plausible but unverified, e.g., an action target that could not be statically resolved).
4. **Verdict** — approve / approve-with-notes / block. Any public-facing, unauthenticated-reachable form with no native fallback is an automatic block, not a judgment call.
5. **Open questions** — anything unverified, including whether an internal tool exempted from progressive enhancement was actually confirmed JS-required by the user rather than assumed.
