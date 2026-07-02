# CLS Attribution

Use this reference when the regression or complaint is about visible layout jumping/shifting, a CLS score drop, or a user report that "content jumps while loading."

## What people get wrong

The naive story is:

> "CLS is 0.3, so something isn't sized — check the images."

Unsized images are one cause among several, and CLS is a session-level accumulation, not a single-event score. Two different pages can hit the same CLS number for entirely different reasons, and lab CLS (a single automated Lighthouse load) systematically under-reports field CLS (a real session including user-triggered scroll/interaction shifts), because field CLS per web.dev's methodology is a windowed sum across the full page lifetime, not just initial load.

## Lab vs. field CLS — do not conflate them

- **Lab CLS** (Lighthouse/PSI): measured over one automated page load with no user interaction. It only captures load-time shifts.
- **Field CLS** (CrUX / `web-vitals` library): measured over the actual session, using the "maximum session window" methodology (windows of shifts gapped by <1s and spanning <5s, take the largest window) — per web.dev's CLS definition. It can and does include shifts triggered well after load, e.g. late-injected consent banners, lazy-loaded below-the-fold content, or shifts within 500ms of a user interaction (which the spec excludes from counting, but adjacent shifts outside that window still count).
- If lab CLS is low but field CLS (CrUX) is high, the cause is very likely something that only manifests during real sessions: late third-party script injection, ad-slot resize, or content that loads outside the automated tool's short observation window. Do not tell the user "Lighthouse says it's fine" as if that settles a field CLS complaint.

## Attribution catalog — name the specific trigger

For every CLS finding, name the specific shifting element and its trigger category. Do not report a bare CLS number as a diagnosis. Common trigger categories, per web.dev's optimize-CLS guidance:

1. **Unsized media** — images, videos, iframes, or embeds rendered without explicit `width`/`height` attributes or a reserved `aspect-ratio`, so the browser doesn't know the box size until the resource loads. Fix: explicit intrinsic dimensions or CSS `aspect-ratio` on the container.
2. **Web-font swap/FOIT-to-FOUT shift** — a fallback font renders at a different metric than the loaded webfont, causing reflow when the real font swaps in. Fix: `font-display` strategy tuned for the situation, and where supported, `size-adjust`/`ascent-override`/`descent-override` font-face descriptors to metric-match the fallback — verify current browser support before prescribing these descriptors as a guaranteed fix.
3. **Late-injected content above existing content** — a banner, ad slot, cookie-consent widget, or async-loaded element inserted above content the user is already viewing, pushing everything below it down. Fix: reserve space for the element up front (a placeholder with the known/estimated final size) rather than letting it inject with zero reserved height.
4. **Animation-triggered reflow** — CSS animations/transitions that animate layout-affecting properties (`width`, `height`, `top`, `left`, `margin`) instead of compositor-only properties (`transform`, `opacity`). Fix: rewrite the animation to use `transform`/`opacity` so it doesn't trigger layout at all.
5. **Dynamically injected/removed DOM nodes tied to loading states** — a skeleton/spinner removed and replaced with real content at a different size. This is the category most likely to overlap with an accessibility loading-state pattern — see the non-negotiable below before touching it.

## Non-negotiable: do not sacrifice accessible loading semantics for a CLS number

A loading skeleton or spinner frequently carries `aria-live`, `role="status"`, or manages focus during a load transition. Do not recommend removing these attributes, collapsing the announcement, or eliminating visible focus indicators as a way to "simplify" the DOM and reduce shift count. If the loading-state element itself is the CLS trigger, fix it by reserving its layout space (matching the skeleton's box size to the final content's box size) — not by removing the accessibility semantics that make the loading state perceivable to assistive technology users. A metric win that produces a WCAG regression is not a fix; flag it as a conflict and specify the size-reserving fix instead.

## Root-cause attribution — require an artifact, not an assertion

Require one of:

- A `LayoutShift` PerformanceObserver entry (per the W3C spec family CLS draws from) identifying the specific `sources` (nodes) and their `previousRect`/`currentRect`.
- A Lighthouse/PSI "Avoid large layout shifts" audit entry naming the specific DOM node.
- A DevTools Performance panel layout-shift region correlated to a specific paint frame and the element that moved.

If none of these are available, the finding is capped at `inference` and must say so.

## Verification target

Re-run the trace and confirm the named element no longer appears in the layout-shift audit / `LayoutShift` entries. For field confirmation, see `references/evidence-tiers-and-handoff.md` — a lab-only CLS fix does not confirm the field CLS percentile improved, especially for triggers (late-injected content, session-long shifts) that lab tooling under-samples by design.
