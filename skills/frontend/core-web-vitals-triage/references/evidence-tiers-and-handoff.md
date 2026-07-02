# Evidence Tiers, Verification, and Handoff

Use this reference for every triage, regardless of which metric (LCP/INP/CLS) is in scope, to classify evidence, specify the re-verification path, and route implementation-class fixes to the correct owning skill.

## Evidence tiers — classify before decomposing anything

Label every claim in the output with one of these tiers. Do not let a lab-only trace produce a field-level verdict.

1. **Field evidence** — CrUX API p75 data for the specific origin/URL/device-class, or an aggregated export from a `web-vitals`-library RUM collector covering real user sessions. This is the only tier that can support a "this is what real users experience" verdict.
2. **Lab evidence only** — a single Lighthouse/PSI run, a synthetic CI trace, or a local DevTools trace. Useful for sub-phase decomposition and root-cause attribution, but caps the verdict at "reproduced in a synthetic environment" — never phrase it as a confirmed real-user-experience claim.
3. **Documentation-based** — a threshold, phase definition, or mechanism claim sourced from web.dev/W3C docs or Context7-queried framework docs, not from a trace the user provided. Correct for grounding definitions; not a substitute for evidence about this specific page.
4. **Inference** — a plausible attribution without a cited artifact (waterfall entry, long-task entry, `LayoutShift`/`PerformanceEventTiming` entry). Must be labeled explicitly and should prompt a request for the missing artifact before the finding is treated as actionable.

If field data is absent entirely, the ceiling for the whole triage is **lab evidence only** — recommend field instrumentation (the `web-vitals` JS library, or enabling/checking CrUX coverage for the origin) as a first-class next step, not an afterthought.

## Device class matters — do not generalize desktop lab data to mobile field reality

CrUX and the page-experience ranking signal are dominated by mobile field data for most origins. A desktop Lighthouse run that looks clean does not confirm the mobile field experience is clean, and the two frequently diverge (different CPU/network throttling profiles, different viewport-driven CLS triggers, different bundle-parse cost on slower mobile CPUs). Always state which device class a lab trace was captured against, and prefer mobile-throttled traces when field data specifically flags mobile.

## CrUX's rolling window — the confirmation lag that must be disclosed

CrUX reports a **28-day rolling window** of aggregated field data. This has two direct consequences for any "field-confirmed fixed" claim:

- A fix deployed today will not be visible in CrUX's public dataset as a clean, isolated before/after split for up to 28 days, and even then the reported window blends pre- and post-fix days until the older data ages out.
- Same-day or next-day CrUX checks after a deploy prove nothing about the field impact of that deploy. Do not let a user (or yourself) declare a metric "fixed" from a CrUX check taken within the rolling-window lag of the deploy.

If the user wants faster confirmation than CrUX's window allows, the correct answer is same-origin RUM instrumentation (a `web-vitals`-library collector reporting to the user's own analytics pipeline), which can show a near-real-time before/after split — but that requires the user to already have or add that instrumentation; do not assume it exists.

## Handoff table — this skill diagnoses, it does not implement

| Dominant cause | Owning skill / next step |
|---|---|
| TTFB-dominant LCP | Server/CDN/infra latency review — out of this skill's cluster entirely |
| JS bundle weight causing resource-load-delay, resource-load-duration, or INP input-delay/processing-time | `bundle-budget-code-splitting-review` (or the closest equivalent asset in the catalog for this provider) |
| Repeat-visit caching / stale-resource causes | `service-worker-cache-strategy-review` (or the closest equivalent caching-strategy asset in the catalog) |
| Single-element resource-hint fix (a missed preload/eager-load on the actual LCP element) | Specify directly in this skill's output; it's a targeted attribute change, not a build-pipeline change |
| Framework-specific rendering/scheduling mechanism (transitions, image-loading props, hydration boundaries) | Verify via Context7 first (see SKILL.md protocol), then specify directly if it's a single-component change; hand off if it implies a broader architectural pattern change |
| Accessibility-vs-metric conflict (loading-state semantics implicated in a CLS/INP finding) | Do not resolve by stripping accessibility semantics; specify the size/scheduling fix that preserves them, and flag the conflict explicitly |

Do not reinvent bundle-splitting or caching-strategy guidance inside this skill — name the dominant cause, cite the artifact, and route to the owning skill so the fix is specified once, correctly, in the asset that owns it.

## Adversarial checklist — answer before closing a triage

- Is every sub-phase attribution backed by a cited waterfall/long-task/`LayoutShift`/`PerformanceEventTiming` entry, or is it asserted?
- Is the evidence tier for the overall verdict correctly capped (lab-only claims never phrased as field-confirmed)?
- Is the device class of every lab trace stated, and does it match the device class the field complaint (if any) refers to?
- Is the CrUX 28-day lag disclosed if field re-confirmation is part of the recommended next step?
- Is the fix routed to the correct owning skill instead of being reinvented here?
- Does any proposed fix remove or weaken an accessible loading-state semantic? If so, it must be flagged as a conflict, not silently accepted.
- Would declaring this metric "fixed" right now be true, or just true-in-the-lab?

## Common failure modes to actively guard against

- Shotgunning multiple simultaneous changes (image compression + font preload + code-splitting in one PR) so the actual causal phase is never isolated — insist on isolating the dominant phase before recommending a bundle of fixes, or at minimum flag that a bundled fix will make attribution of the eventual field-data improvement ambiguous.
- Treating a single lab run, in either direction, as proof of anything about the field percentile.
- Generalizing desktop lab data to a mobile field complaint, or vice versa.
- Declaring victory the day after deploy based on a CrUX check that is still inside the 28-day blended window.
