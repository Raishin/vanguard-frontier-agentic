# LCP Phase Decomposition

Use this reference when the regression or complaint is about page-load speed, a Lighthouse/PSI LCP score drop, or a CrUX LCP p75 shift.

## What people get wrong

The naive story is:

> "LCP went from 2.3s to 3.8s, so the page got slower — compress the images."

Wrong, or at least unverified. LCP is not one number produced by one cause. web.dev decomposes it into four sequential sub-phases, and the correct fix depends entirely on which phase grew. Compressing an image only helps if resource-load-duration is the dominant phase; if the regression is actually render-delay because the image is being lazy-loaded instead of eager-loaded, compression does nothing.

## The four documented sub-phases

Per web.dev's LCP guidance, decompose the LCP element's timeline into:

1. **Time to First Byte (TTFB)** — from navigation start to the first byte of the document response. Server/CDN/redirect-chain territory, not a client-rendering fix.
2. **Resource load delay** — from TTFB to when the browser starts loading the LCP resource (e.g. the hero image). Grown by: the resource being discovered late (not in the initial HTML, injected by JS, behind a render-blocking script), missing `fetchpriority="high"`/preload hints, or a redirect chain on the resource URL itself.
3. **Resource load duration** — the time actually spent downloading the LCP resource. Grown by: resource size, missing compression/modern format, slow origin, lack of CDN, render-blocking requests competing for bandwidth/connections ahead of it.
4. **Render delay** — from resource load complete to the element actually painting. Grown by: main-thread blocking (long tasks, large JS execution, web-font blocking text render), CSS/layout work queued ahead of the paint.

Every LCP finding in this skill's output must name which of these four phases is dominant, with a waterfall-entry citation, not just restate the total LCP number.

## Framework-specific attribution — verify via Context7, do not assume

Do not attribute an LCP delay to a framework mechanism from memory. Resolve the framework via Context7 and query current docs first (see SKILL.md's Context7 Documentation Protocol). Known traps as of this skill's `updated` date, confirmed via Context7 against `/vercel/next.js`:

- **Next.js `<Image>` eager-load prop is version-sensitive.** As of Next.js 16, the `priority` prop is deprecated in favor of `preload` (same intent — insert a `<link>` preload and skip lazy-loading for the LCP candidate — new prop name). Telling a Next.js 16+ user to add `priority={true}` is stale guidance; verify the installed Next.js major first, then recommend `preload` (16+) or `priority` (pre-16) accordingly. Either way: the LCP image must not be lazy-loaded by default (`loading="lazy"` is the `next/image` default) — that alone is a common render-delay root cause when the hero image regresses after an unrelated refactor accidentally drops the eager-load prop.
- **Do not assume `fetchpriority="high"` alone fixes resource-load-delay** if the resource is still discovered late (e.g. only referenced inside a client-side-rendered component that itself waits on hydration). Preload hints only help resources the browser can discover early; a resource injected after JS execution still incurs discovery delay regardless of priority hints.
- For non-Next.js stacks (Vite-based React/Vue/Svelte apps), verify current `vite-imagetools` / native `<img fetchpriority>` support against the installed Vite major via Context7 (`/vitejs/vite`) rather than assuming a plugin API that may have moved between majors.

## Root-cause attribution — require an artifact, not an assertion

For each phase you name as dominant, require one of:

- A network-waterfall entry (from the Lighthouse/PSI JSON `resourceSummary`/`network-requests` audit, or a browser DevTools export) showing the specific request's start/end offsets.
- A `LargestContentfulPaint` PerformanceObserver entry (`element`, `url`, `renderTime`, `loadTime`) per the W3C Largest Contentful Paint spec, if the user has instrumented field RUM.
- For render-delay specifically: a long-task entry (`PerformanceLongTaskTiming`) overlapping the window between resource-load-complete and `renderTime`.

If none of these are available, the finding is capped at `inference` and must say so — do not present a phase attribution as verified without a cited artifact.

## Minimal targeted fix per phase (and who owns it)

- **TTFB-dominant** → out of this skill's scope; hand off to server/CDN/infra review (redirect-chain removal, CDN edge caching, server response-time reduction). Do not prescribe a client-side fix for a TTFB problem.
- **Resource-load-delay-dominant** → eager-load / preload the correct hint for the actual LCP element (verify current framework prop via Context7 first), remove render-blocking requests ahead of discovery, eliminate resource redirect chains. Implementation of a broader preload/resource-hint strategy across a route is `bundle-budget-code-splitting-review`'s territory if it touches build output; a single-element preload hint can be specified directly here.
- **Resource-load-duration-dominant** → image/video compression, modern format (AVIF/WebP), responsive `sizes`/`srcset`, CDN/origin latency — hand off bundle-weight-class changes to `bundle-budget-code-splitting-review`.
- **Render-delay-dominant** → identify and reduce the specific long task or web-font-blocking render ahead of the LCP paint; if the cause is JS bundle weight/parse time, hand off to `bundle-budget-code-splitting-review`.

## Verification target

Re-run the trace (Lighthouse CI or PSI API) against the same route/device-class combination and diff the four-phase breakdown, not just the top-line LCP number, against the pre-regression trace. See `references/evidence-tiers-and-handoff.md` for the field-data confirmation window before declaring the metric fixed.
