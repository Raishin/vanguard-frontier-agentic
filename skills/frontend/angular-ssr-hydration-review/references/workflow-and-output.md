# Review workflow and findings contract

Use this reference for the full SSR/hydration review procedure and the required output shape.

## What people get wrong

The naive story is:

> Angular SSR just server-renders the HTML and then "boots up" on the client. Any DOM manipulation is fine as long as the app works.

Wrong. Once `provideClientHydration()` is configured, Angular does not throw away and re-render the server-sent DOM — it **reconciles** against it, walking the existing DOM tree and matching it node-for-node against what the client-side render would have produced, then attaching event listeners and internal state to the existing nodes instead of replacing them. Angular's own error documentation is explicit about the failure mode: when a native DOM API creates or moves a node inside that reconciled tree, "Angular has no information about" the new node, so it looks for the wrong node at that position and throws a hydration mismatch (NG0500). Treating "the app still renders correctly in the end" as proof hydration is safe misses that Angular may have already thrown, logged an error, or silently fallen back to a client-side re-render of the affected subtree — defeating the SSR investment even when the visible symptom is subtle (a flash, a layout shift, a slower Largest Contentful Paint).

The second common mistake: assuming a hydration review applies to any Angular app with server-rendering. It does not — SSR without `provideClientHydration()` configured still full-remounts on the client (Angular's pre-hydration behavior), so there is no reconciliation to break. Reviewing DOM-manipulation patterns as "hydration risks" in that context produces findings that don't apply; the actual finding is that the app isn't hydrating and is losing the SSR performance benefit entirely.

## Workflow

1. **Confirm hydration is actually configured**
   - Search the bootstrap entry point (`app.config.ts`, `main.server.ts`, or equivalent `bootstrapApplication` call) for `provideClientHydration(...)` in the providers array.
   - If absent, do not proceed with a mismatch-pattern review. Report the larger finding: SSR is configured but hydration is not, so the app fully re-renders client-side on every load. Stop the hydration-mismatch-specific checks; this is a different, higher-severity finding than any individual mismatch would be.
   - Note the Angular major version from `package.json` (`@angular/core`) — hydration feature availability (`withI18nSupport`, `withEventReplay`, incremental hydration) differs by version.

2. **Inventory native DOM API usage**
   - Grep for `document.createElement`, `document.querySelector` combined with mutation, `.insertBefore`, `.appendChild`, `.removeChild`, `.replaceChild`, `innerHTML =`, `outerHTML =`, and direct `ElementRef.nativeElement` property/method access that mutates the DOM, across components, directives, and any structural (not purely cosmetic-class-toggle) DOM work.
   - For each hit, capture the enclosing lifecycle hook or method (`ngOnInit`, `ngAfterViewInit`, a directive host binding, an event handler) — the lifecycle context determines whether the call can execute during the SSR-reconciled render path.

3. **Classify each hit: SSR-path vs browser-guarded**
   - If the call is wrapped in (or its containing method is only invoked from) an `isPlatformBrowser(...)` check, or is registered via `afterNextRender`/`afterRender` (Angular's documented browser-only render callbacks), and there is no reachable path for it to run during the reconciled initial render, classify it as **safe / post-hydration**. Do not flag it as a mismatch risk.
   - If the call has no such guard and executes in a lifecycle hook that runs during SSR (e.g. unguarded `ngOnInit`, unguarded constructor logic), classify it as **SSR-path**. Continue to step 4.

4. **Confirm the SSR-path hit actually touches the reconciled tree**
   - Determine whether the mutated node is inside the component's own template-rendered subtree (the tree Angular reconciles) versus a node genuinely outside Angular's knowledge (e.g. a detached, never-inserted-into-the-DOM canvas element used purely as an offscreen buffer). Angular's own NG0500 example shows the failure mode precisely: inserting a native node as a sibling/child within the host element Angular is walking during reconciliation.
   - If the mutated node is inside the reconciled tree → HIGH finding, cite the NG0500 mechanism, and check whether the mutation also injects dynamic/user-influenced string content (see the sanitization-bypass rule below).
   - If the node is genuinely outside Angular's reconciled tree → not a hydration finding; note it only if relevant to a different concern (e.g. memory-leak risk from an unmanaged node).

5. **Check for dynamic-content-plus-hydration double risk**
   - If the flagged SSR-path native DOM mutation also sets `innerHTML`/`outerHTML` (or an equivalent native string-injection call) with data that is user-influenced (query params, form input, API response echoing user data, route params), report it as two findings: the hydration-mismatch finding from step 4, and a separate HIGH-severity sanitization-bypass finding — Angular's template bindings (`[innerHTML]`) sanitize by default, but direct native DOM API calls do not pass through that pipeline.

6. **Audit existing `ngSkipHydration` usage**
   - Grep for `ngSkipHydration` (as a host binding, template attribute, or `[ngSkipHydration]`/`host: {ngSkipHydration: 'true'}`).
   - For each usage, look for an adjacent comment or a referenced tracked issue explaining why hydration is being skipped for that component. No justification → MEDIUM tech-debt finding. A justification tied to a known limitation (e.g. a third-party DOM-manipulating library — see `references/i18n-event-replay-and-third-party-libs.md`) → note it as acceptable but still worth a tracked-removal follow-up if the underlying library constraint might be resolved.

7. **Check i18n hydration configuration**
   - If the app uses `i18n` template attributes or `$localize`, confirm `withI18nSupport()` is present in the `provideClientHydration(...)` call. Its absence means i18n-block components silently skip hydration and re-render from scratch by Angular's documented default — flag as MEDIUM.

8. **Check pre-hydration accessibility**
   - Compare what the server actually sends (the pre-hydration markup) against what the fully-hydrated DOM exposes, for accessible names, landmark roles, and focus order on interactive elements in scope. If pre-hydration markup is missing accessibility structure that only appears after a client-side patch, flag it — assistive technology and slow-network users interact with the pre-hydration state, not just the final one.

9. **Produce ranked findings**
   - Order by blast radius: SSR-not-configured-at-all (if found in step 1, report this alone and stop), then SSR-path native DOM mutations touching the reconciled tree (HIGH, split into hydration + sanitization findings when both apply), then missing i18n hydration support and pre-hydration accessibility gaps (MEDIUM), then unjustified `ngSkipHydration` usage (MEDIUM tech-debt).

## Decision tree

- `provideClientHydration()` **absent** from bootstrap providers → single overriding finding: app is not hydrating, full client remount on every load. Stop mismatch-specific review.
- Native DOM mutation **guarded to browser-only execution** (`isPlatformBrowser`, `afterNextRender`/`afterRender`) with no reachable SSR-path call → not a finding.
- Native DOM mutation **runs during the SSR-rendered path** and **mutates a node inside the component's reconciled template tree** → HIGH finding: hydration-mismatch risk matching the NG0500 mechanism. Fix: refactor to template-driven DOM creation (bind the content through the template instead of imperative node insertion); do not recommend `ngSkipHydration` as the fix.
- Same as above, **and** the mutation injects dynamic/user-influenced string content via `innerHTML`/`outerHTML` or equivalent → report both the HIGH hydration finding and a separate HIGH sanitization-bypass (XSS-class) finding.
- `ngSkipHydration` present with **no linked justification** → MEDIUM finding: request a comment or tracked issue; do not treat presence alone as disqualifying.
- i18n blocks present, hydration enabled, **`withI18nSupport()` absent** → MEDIUM finding per Angular's documented default behavior.
- Pre-hydration markup **missing accessible structure** present only post-hydration → finding at a severity proportional to the affected interaction (focus-order or landmark loss on a primary interactive flow → HIGH; a secondary/decorative region → MEDIUM).

## Output contract

Return:

1. Component(s)/bootstrap file(s) in scope, and whether `provideClientHydration()` is confirmed present (with file:line)
2. Ranked findings, each with:
   - file:line evidence
   - hydration-mechanism category (SSR-path native DOM mutation / unjustified ngSkipHydration / missing i18n hydration support / pre-hydration accessibility gap / sanitization-bypass-plus-hydration-risk)
   - concrete fix sketch defaulting to template-driven DOM (never `ngSkipHydration` as the fix)
   - severity (HIGH / MEDIUM / LOW)
   - evidence level (`repo evidence`, `documentation-based`, `inference`)
3. Verdict: approve / approve-with-notes / block
4. Open questions or explicitly out-of-scope items (e.g. Signals/change-detection concerns deferred to `angular-architecture-signals-review`, hydration not configured so mismatch review does not apply, unconfirmed Angular major version)

## Validation gates

- Every hydration-mismatch claim names the specific native DOM call and the specific NG0500 mechanism it matches, version-matched to the repo's confirmed Angular major — no bare "this may cause a mismatch."
- Every `ngSkipHydration` finding states explicitly whether a justification (comment or tracked issue) is present.
- No finding recommends `ngSkipHydration` as the resolution to a hydration-mismatch finding — the fix recommendation defaults to template-driven DOM refactoring.
- No finding claims a mismatch "will happen" without confirming the mutation executes on the SSR-rendered path (not behind an `isPlatformBrowser`/`afterNextRender` guard).

## Common failure modes

- Flagging DOM manipulation that only executes inside an `isPlatformBrowser(...)` guard or an `afterNextRender`/`afterRender` callback as a hydration risk — those are Angular's documented browser-only execution points and do not participate in SSR reconciliation.
- Missing that `provideClientHydration()` itself is entirely absent, and reviewing the app for mismatch patterns anyway — when hydration isn't configured, there's nothing to reconcile, and the real finding (full client remount, no SSR benefit realized) is larger and different from any individual mismatch.
- Recommending `ngSkipHydration` as a fix rather than a last-resort workaround — it silently disables reconciliation for the component and its children, which is the SSR-defeating outcome the review exists to prevent.
- Assuming WCAG/accessibility requirements apply only to the final, fully-hydrated DOM — the server-rendered, pre-hydration markup is what users on slow connections or assistive technology querying early actually receive.
- Treating a hydration-mismatch finding and a sanitization-bypass finding as the same finding when both apply to one `innerHTML`-equivalent native call — they are independent risks (DOM-reconciliation correctness vs. XSS) and both must be reported.

## Adversarial checklist

Before finalizing a finding, answer these:

- Is `provideClientHydration()` actually present in the bootstrap providers, or is "hydration" being reviewed on an app that never configured it?
- Does the flagged native DOM manipulation occur inside an SSR-rendered code path, or is it guarded to browser-only execution (`isPlatformBrowser`, `afterNextRender`/`afterRender`)?
- Does every `ngSkipHydration` usage in scope have a linked justification (comment or tracked issue)?
- Is dynamic content ever inserted via a native `innerHTML`/`outerHTML`-equivalent call on user-influenced data — and if so, has both the hydration risk and the XSS risk been reported as separate findings?
- Is the NG0500 mechanism claim matched to this repo's actual confirmed Angular version, not assumed from the latest docs?
- Does the pre-hydration (server-sent) markup, not just the final hydrated DOM, retain the accessibility structure being evaluated?

If any answer is "not sure," lower the finding's confidence and label the evidence level accordingly — do not present it as a confirmed defect.
