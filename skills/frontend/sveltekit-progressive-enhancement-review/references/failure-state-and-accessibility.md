# Failure-State and Accessibility UX

Use this reference only when reviewing how a failed, errored, or cancelled form submission is surfaced to sighted and screen-reader users — not for pure native-fallback presence checks (see `references/workflow-and-output.md` for that).

## What people get wrong

The naive assumption is:

> "There's an `if (form?.error)` block in the template, so failures are surfaced to the user."

That is incomplete in two ways:

1. **Rendered is not the same as announced.** A conditionally-rendered error `<p>` inserted into the DOM after a client-side (`use:enhance`) form update does not automatically get read by a screen reader unless it is associated with a live region (`aria-live`, `role="alert"`/`role="status"`, or equivalent) or receives programmatic focus. WCAG 2.2's status-messages guidance exists precisely because DOM insertion alone does not guarantee an assistive-technology announcement.
2. **A variable existing in state is not proof it renders at all.** Component state (`form?.error`, a local `let errorMessage`) can be set by a callback and never referenced anywhere in the template — verify by reading the markup, not by trusting that a well-named variable implies it's displayed.

## Non-negotiables for this sub-review

- Do not accept "the error is in the `form` prop" as sufficient. Trace whether the template actually reads that prop into visible markup.
- Do not accept "there's a red error box in the design" as sufficient for accessibility. Confirm the error container has an appropriate live-region role/attribute (`aria-live="polite"` or `"assertive"`, `role="alert"`, or `role="status"`) or that focus is programmatically moved to it on failure — one of those two mechanisms is required for screen-reader users to reliably learn about an async/no-reload failure.
- For forms that fall back to a full-page reload (no `use:enhance`, or `use:enhance` with `update()`/`applyAction()` invoked), server-re-rendered failure content read on page load is generally sufficient without a live region, because the new page load itself resets assistive-technology reading context — do not demand `aria-live` on a full-reload failure path; that is a false-positive pattern specific to no-reload (client-side-updated) failure states.
- A `cancel()`-triggered client-validation message must follow the same rendering + announcement bar as a server-originated failure — a locally-cancelled submission is still a failed submission from the user's point of view.
- Do not conflate "the form has a `required` attribute" with adequate failure messaging for server-side validation failures. Native HTML validation and server-side `ActionResult` `failure` responses are different failure paths; both need coverage, and a HIGH/MEDIUM finding about one does not resolve a gap in the other.

## Minimal safe pattern to expect

1. Server action returns `fail(422, { error: '...', ...values })` (or similar) on validation/processing failure.
2. Template renders `{#if form?.error}` (or equivalent) into a container that either:
   - has `aria-live="polite"` (or `role="alert"`/`role="status"` as appropriate to urgency), or
   - receives programmatic focus (`element.focus()`) when the failure state first appears.
3. If a custom `use:enhance` callback is present, it either calls `update()` so the default `form`-prop-driven rendering above still fires, or manually replicates equivalent state-setting and rendering for every `ActionResult` branch it intercepts.
4. Client-side `cancel()` paths render into the same or an equivalently accessible container, not a separate, unannounced one.

## Adversarial checklist

- If you disabled CSS and squinted only at DOM order, would a screen reader announce this failure, or would it silently sit in the DOM until the next unrelated navigation?
- Does the live-region/focus mechanism differ between the server-originated failure path and the client-`cancel()` path? If so, is that difference justified or is it an accidental gap?
- Is the failure container conditionally *removed from the DOM* between attempts (which can suppress re-announcement of the same message on a second identical failure) rather than updated in place?
- For a full-page-reload fallback path, did you correctly treat the absence of `aria-live` as expected rather than flagging it as a gap?

## When to push back

Push back if the user asks to:

- close a review of a public-facing form's failure UX solely because "there's an error variable" without confirming it renders,
- skip the accessible-announcement check because "the design has a red box" — visual-only signaling is not sufficient for screen-reader users,
- treat a full-page-reload failure path and a no-reload (`use:enhance`-intercepted) failure path as needing identical `aria-live` treatment — they don't; judge each by its actual reload behavior.

Those are not shortcuts. They leave a portion of real users — those on assistive technology, and separately those with degraded/blocked JavaScript — with no way to know their submission failed.
