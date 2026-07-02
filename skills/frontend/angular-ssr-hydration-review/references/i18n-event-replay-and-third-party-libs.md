# i18n, event replay, and third-party DOM libraries

> Load this reference only when the diff under review includes i18n blocks, `withEventReplay()` configuration, or a third-party library known to perform direct DOM manipulation (e.g. D3, chart libraries, non-Angular widget wrappers).

## What people get wrong

The naive story on i18n:

> Hydration is hydration — if `provideClientHydration()` is configured, every component hydrates, including ones using i18n blocks.

Wrong. Angular's own hydration guide states its default plainly: "By default, Angular skips hydration for components using i18n blocks, causing them to re-render from scratch." `provideClientHydration()` alone does not cover i18n content — it requires the explicit `withI18nSupport()` feature. A review that confirms `provideClientHydration()` is present and stops there, without checking i18n-block components specifically, will miss that those components are silently falling back to full client re-render even though hydration "looks" enabled.

The naive story on third-party libraries:

> If a third-party library breaks hydration, that's a library bug — nothing to do at the integration layer except wait for a fix.

Incomplete. Angular's own guidance names this exact class of problem and gives the sanctioned mitigation: "Third-party libraries that rely on direct DOM manipulation, such as D3 charts, might cause DOM mismatch errors with hydration enabled. If you encounter such issues, applying the `ngSkipHydration` attribute to the component rendering the library can serve as a workaround." This is the one context where `ngSkipHydration` is Angular's own documented recommendation — but it is still scoped as a workaround for a specific, named class of problem (uncontrolled third-party DOM manipulation), not a general-purpose fix for any hydration mismatch, including ones caused by the application's own code.

## i18n hydration review rules

- Confirm `withI18nSupport()` is present in the `provideClientHydration(...)` call whenever the codebase uses `i18n` template attributes or `$localize` anywhere in scope. Its absence is a MEDIUM finding: those components silently re-render from scratch, which is a real (if less severe) SSR-benefit loss, not a hard error.
- Do not assume `withI18nSupport()` is a no-op addition — confirm the Angular major version supports it (check `package.json` for `@angular/core`, then verify against Context7 `query-docs` for the confirmed version) before recommending it as a one-line fix.
- If `withI18nSupport()` is present, do not separately flag i18n-block components as hydration-risky purely for using i18n — the feature exists specifically to make that safe.

## Third-party DOM library review rules

- When a component wraps a library known to perform direct DOM manipulation outside Angular's control (charting libraries, D3-based visualizations, non-Angular widget wrappers, map libraries), and that component shows an `ngSkipHydration` usage, treat the usage as the Angular-sanctioned workaround for this specific case — do not flag it with the same severity as an unjustified `ngSkipHydration` elsewhere in the app. Still require the justification comment/issue link (per the main workflow's tech-debt rule) so the workaround is tracked and revisited if the library adds hydration-safe integration later.
- Do not recommend removing `ngSkipHydration` from a genuine third-party-DOM-manipulation component without first confirming the library itself no longer performs uncontrolled native DOM mutation in the version pinned in the repo — removing the guard prematurely reintroduces the exact NG0500-class mismatch it exists to prevent.
- If a component wraps a third-party library but the library's DOM manipulation is confined to a container element genuinely outside Angular's reconciled tree (e.g. the library owns a `<div>` marked with `ngSkipHydration` and Angular does not attempt to reconcile anything inside it), do not also flag the library's internal DOM calls as separate hydration-mismatch findings — the `ngSkipHydration` boundary already scopes that correctly, per Angular's own guidance.
- Distinguish a third-party library's DOM manipulation from the *hosting component's own* unguarded native DOM calls. `ngSkipHydration` on the wrapper does not excuse unrelated unguarded DOM manipulation the application's own code performs elsewhere in the same component outside the library-owned region — evaluate those against the main workflow's SSR-path rules independently.

## Event replay review rules

- `withEventReplay()` captures user interactions (clicks, and other supported native events) that occur before hydration completes and replays them once hydration finishes, per Angular's documented behavior. Its presence does not change hydration-mismatch risk — it changes interaction responsiveness during the hydration window. Do not conflate a missing `withEventReplay()` with a hydration-mismatch finding; it is a UX/responsiveness consideration, not a correctness one.
- If incremental hydration is configured, note that event replay is automatically active per Angular's documented behavior — do not flag `withEventReplay()` as separately required in that configuration.
- If the review scope includes a reported "clicks are lost/ignored right after page load" symptom (as opposed to a visual mismatch/flash symptom), treat that as a `withEventReplay()` gap, not an NG0500-class hydration-mismatch finding — the two symptom classes map to different mechanisms and different fixes.

## When to push back

Push back if the user asks to:

- add `ngSkipHydration` to a component whose native DOM manipulation is the **application's own** unguarded code (not a third-party library boundary) "to make the error go away" — that suppresses the mismatch symptom while leaving the app fully re-rendering that component on every load, which is the outcome the review exists to prevent,
- skip confirming `withI18nSupport()` because "hydration is already enabled" — the i18n feature is opt-in and separate from the base `provideClientHydration()` call, per Angular's own documented default,
- treat every third-party-library `ngSkipHydration` usage as pre-approved without a justification link — the mitigation being Angular-sanctioned for this class of problem does not exempt it from the tracking requirement the main workflow applies to all `ngSkipHydration` usage.

That is not a workaround. It is silently disabling the SSR benefit the review exists to protect.
