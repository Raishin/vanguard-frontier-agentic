---
name: angular-ssr-hydration-review
description: Statically review Angular SSR bootstrap configuration and component templates for hydration-mismatch risk (NG0500-class errors), unjustified ngSkipHydration usage, and direct-DOM-manipulation patterns that bypass Angular's template-owned DOM model, grounded in Angular's own hydration guide and error catalog.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: architecture
---

# Angular SSR & Hydration Review

## Purpose

Review Angular server-side-rendered bootstrap configuration and component templates for hydration-mismatch risk without re-litigating Signals/change-detection architecture, RxJS-only legacy code, or a full zoneless-migration plan in every response. This skill exists so hydration correctness — whether the DOM Angular produces on the client actually matches the DOM the server sent, per Angular's own reconciliation model — stays a focused, verifiable review instead of a vague "SSR looks fine" pass.

## When to use

Use this skill when the user asks to:

- review an SSR-enabled Angular app before or after enabling `provideClientHydration()`,
- diagnose a reported flash-of-unstyled-content / re-render-on-load symptom consistent with a hydration mismatch,
- assess whether a component's direct DOM manipulation is hydration-safe,
- review existing `ngSkipHydration` usage for justification and tech-debt tracking,
- confirm i18n-block hydration is correctly configured (`withI18nSupport()`).

Do not use this skill for:

- a purely client-rendered (non-SSR) Angular app with no SSR bootstrap/`provideClientHydration()` configuration — hydration concerns do not apply,
- general Signals/change-detection/DI architecture review with no SSR dimension — use `angular-architecture-signals-review` instead.

## Source priority (official Angular skills first)

Consult sources in this fixed order, and label findings by which layer they draw from:

1. **The official Angular team's `angular-developer` skill** ([github.com/angular/skills](https://github.com/angular/skills/tree/main/angular-developer)) is the AUTHORITATIVE primary source for idiomatic Angular SSR/hydration — `provideClientHydration()`, `withI18nSupport()`, `withEventReplay()`, incremental hydration, and template-driven (Angular-owned) DOM. Prefer its idioms over memory.
2. **Context7 `angular_dev` docs** (`/websites/angular_dev`, or the version-pinned `/websites/v20_angular_dev`) confirm hydration-feature availability and the NG0500 mechanism against the repo's pinned `@angular/core` major (read `package.json` first) — hydration support landed and matured incrementally, so the official skill states the modern shape and the versioned docs confirm it applies to THIS repo's major.
3. **This skill's static-review judgment** (hydration-mismatch detection, SSR-path-vs-browser-guarded classification, `ngSkipHydration` tech-debt rules, sanitization-bypass detection, pre-hydration accessibility checks) is the review layer applied ON TOP of 1 and 2.

Where guidance conflicts: the official Angular `angular-developer` skill + version-matched `angular_dev` docs WIN on "what is idiomatic Angular" (correct hydration setup and the recommended fix shape). THIS skill governs "what to flag in review" (which DOM-mutation, `ngSkipHydration`, i18n, or sanitization patterns rise to a HIGH/MEDIUM finding and why). Never let this skill's flagging override an idiom the official skill and pinned docs endorse; conversely, an idiom being official does not exempt a concrete hydration-mismatch or sanitization-bypass defect from being flagged.

## Context7 Documentation Protocol

- Resolve the Angular library ID with `resolve-library-id` (matched result: `/websites/angular_dev` or the version-pinned `/websites/v20_angular_dev` when the repo's confirmed major is known) before citing any hydration-mechanism or error-code claim.
- Before classifying a DOM-manipulation pattern as hydration-breaking, call `query-docs` against the repo's confirmed Angular major (read `package.json` — `@angular/core` — first) for `hydration`, `NG0500`, and `provideClientHydration`, and cite the doc section. The set of hydration-incompatible patterns is documented precisely by Angular (native `insertBefore`/`createElement`/`innerHTML`/`outerHTML` inside the reconciled tree) and is not safely inferable from general React/Vue hydration knowledge — the mechanisms differ.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.
- Do not assume a hydration feature (`withI18nSupport`, `withEventReplay`, incremental hydration) is available or behaves identically across Angular majors without confirming the version — hydration support landed and matured incrementally.

## Lean operating rules

- First confirm SSR/hydration is actually configured: find `provideClientHydration(...)` in the bootstrap providers (`app.config.ts` / `main.server.ts` or equivalent). If it is absent, do not review the app as a hydration-mismatch problem — flag the larger finding that the app is not hydrating at all (full client remount on load), which is a different and more consequential issue than a mismatch.
- Classify every native DOM API call found (`document.createElement`, `.insertBefore`, `.appendChild`, `.removeChild`, `innerHTML`, `outerHTML`, direct `ElementRef.nativeElement` mutation) by whether it executes inside an SSR-rendered code path or is guarded to browser-only execution. A call guarded by `isPlatformBrowser(...)` (or an equivalent browser-only lifecycle boundary) that only runs after hydration completes is not a hydration-mismatch risk — do not flag it as one.
- Any native DOM mutation of a node inside the component's own reconciled template tree that runs during the SSR-rendered path is a hydration-mismatch risk matching Angular's documented NG0500 mechanism: Angular loses track of what the server actually rendered at that position and looks for the wrong node during reconciliation. Cite the specific call site and the specific NG0500 mechanism it matches — never assert "this may cause a mismatch" without naming the mechanism.
- Never recommend `ngSkipHydration` as the fix for a hydration-mismatch finding. Angular's own guidance frames it as a last-resort/temporary workaround (e.g. for third-party DOM-manipulating libraries), not a solution — it causes the component and its children to skip reconciliation and re-render from scratch, which is exactly the SSR-defeating behavior the review exists to catch. The default fix recommendation is refactoring the mutation to be template-driven (Angular-owned DOM creation) or, when the mutation is genuinely display-only DOM Angular does not need to reconcile, isolating it behind a browser-only guard.
- Treat every existing `ngSkipHydration` usage found in scope as a finding requiring a linked justification (comment or tracked issue reference). An unjustified `ngSkipHydration` is a MEDIUM tech-debt finding, not automatically a blocker — it silently disables hydration reconciliation for that component and its children with no record of why.
- If the app renders i18n blocks (`i18n` template attributes, `$localize`) and hydration is enabled, confirm `withI18nSupport()` is present in the `provideClientHydration(...)` call per Angular's documented default (Angular skips hydration for i18n-block components without it, causing an unrecorded re-render-from-scratch). Flag its absence as a MEDIUM hydration finding.
- SSR-rendered, pre-hydration content must remain accessible on its own — do not treat WCAG conformance as a post-hydration-only concern. If pre-hydration markup (what the server actually sends) drops accessible names, landmark structure, or focus order that the post-hydration DOM restores, flag it: users on slow connections or with assistive tech querying the DOM before hydration completes are affected by the pre-hydration state, not just the final one.
- Treat any native DOM insertion of dynamic, user-influenced string content (`innerHTML`, `outerHTML`, or an equivalent native-API string-injection call) as a HIGH-severity finding on two independent axes simultaneously: the hydration-mismatch risk described above, and a bypass of Angular's built-in sanitization pipeline (Angular's template bindings sanitize `[innerHTML]`; direct native DOM API calls do not) — an XSS-class finding. Report both; do not fold one into the other or drop either.
- Never execute, build, or run application code as part of this review; this is a static-review skill (Read/Grep/Glob only).

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the SSR-rendered-vs-browser-guarded decision tree, and the required output shape.
- [i18n, event replay, and third-party DOM libraries](references/i18n-event-replay-and-third-party-libs.md) — load only when the diff includes i18n blocks, `withEventReplay()` configuration, or a third-party library known to perform direct DOM manipulation (e.g. D3, chart libraries, non-Angular widget wrappers).

## Response minimum

Return, at minimum:

- the component(s)/bootstrap file(s) in scope, and whether `provideClientHydration()` is confirmed present,
- ranked findings with file:line evidence, hydration-mechanism category (SSR-path native DOM mutation / unjustified ngSkipHydration / missing i18n hydration support / pre-hydration accessibility gap / sanitization-bypass-plus-hydration-risk), and a concrete fix sketch defaulting to template-driven DOM per finding,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- verdict (approve / approve-with-notes / block),
- open questions or explicitly out-of-scope items (e.g. Signals/change-detection concerns deferred to `angular-architecture-signals-review`, hydration not configured at all so mismatch review does not apply).
