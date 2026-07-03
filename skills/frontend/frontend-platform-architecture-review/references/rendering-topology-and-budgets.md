# Rendering Topology and Cross-Cutting Budgets

Use this reference only when the proposal changes rendering strategy (CSR → SSR, SSR → SSG/ISR/streaming/PPR) or when Core Web Vitals, accessibility, or security posture needs grounding against current framework and WCAG guidance. Do not load this for a pure module-boundary or dependency-adoption proposal that does not touch rendering.

## What people get wrong

The naive story is:

> We're moving from SSR to PPR/streaming, so this is purely a performance upgrade — it's strictly better.

Wrong. A rendering-topology change is a cross-cutting change to caching, hydration timing, focus/reading order, and the bundle's client-reachable surface — not an isolated performance knob. Officially, Next.js describes Partial Prerendering as combining a static prerendered shell with dynamic content streamed in via Suspense boundaries at request time; React's Suspense mechanism decides what streams and when. That means every dynamic region introduces a new hydration/streaming boundary that a11y (focus management, live-region announcements) and security (what's in the static shell vs. what's fetched dynamically, and with what auth context) both have to account for. Treating it as "just faster" skips the review this skill exists to force.

## Officially grounded rendering-strategy shape (Context7-verified)

- **Partial Prerendering (PPR)** — Next.js's glossary describes PPR as "a rendering optimization that combines prerendering and dynamic rendering in a single route. The static shell is served immediately while dynamic content streams in when ready." As of Next.js 16, PPR is enabled via `cacheComponents: true` in `next.config`, replacing the earlier `experimental_ppr` route-segment flag. Verify the repo's installed Next.js major before citing either mechanism — the config surface changed between versions.
- **Static-shell-plus-streaming pattern** — implemented via a `<Suspense fallback={...}>` boundary wrapping the dynamic component; the fallback is prerendered as part of the static shell, and the wrapped component streams once its data resolves. Every such boundary is a place where the visible-but-not-yet-interactive gap (for a11y) and the deferred-fetch trust boundary (for security) both need explicit review.
- **Component decomposition as the review's dependency, not its subject** — React's own "Thinking in React" guidance frames component splitting around separation of concerns ("a component should ideally only be concerned with one thing. If it ends up growing, it should be decomposed into smaller subcomponents"). This skill does not re-review component-level decomposition (that is `react-component-architecture-review`'s job); it uses this principle only to sanity-check that a proposed module/rendering boundary maps to a real separation of concerns rather than an arbitrary split.

## Non-negotiable design rules

1. **State the Core Web Vitals budget impact per route class, not per app.** LCP/INP/CLS impact differs by route (a marketing shell vs. an authenticated dashboard); a single app-wide estimate hides regressions in the routes that matter most.
2. **Separate lab data from field data explicitly.** Lab data (Lighthouse, local traces) proves a route *can* meet budget under controlled conditions; field data (CrUX, RUM) proves it *does* for real users. A proposal citing only lab numbers as evidence of production impact is incomplete.
3. **Every new streaming/hydration boundary must state its focus-management and live-region behavior.** Content that pops in after initial paint can silently move focus or fail to announce to assistive technology; this is not optional polish, it is the a11y consequence of the rendering choice.
4. **State what's in the static shell vs. fetched dynamically, and under what auth context.** A static shell is often cached and served without per-request auth; if a proposal moves user-specific or sensitive data into a cached shell to hit a performance target, that is a security defect, not a clever optimization.
6. **Treat framework version as load-bearing for every claim.** PPR's config surface, Suspense streaming semantics, and caching defaults have all changed across React and Next.js majors — a claim true for one major can be false or renamed in another. Confirm the installed version before citing a mechanism.

## Adversarial checklist

Before approving a rendering-topology change, answer these:

- What is the LCP/INP/CLS budget for the specific route(s) affected, and is it lab data, field data, or an unmeasured estimate?
- Which regions of the page are in the static shell, and which stream in dynamically?
- Is any user-specific or sensitive data present in the static shell (which may be cached/shared across requests)?
- What happens to keyboard focus and screen-reader announcements when a streamed region resolves after initial paint?
- What is the fallback/loading state, and does it meet the same a11y bar as the resolved content (not just visually, but semantically)?
- Does the target framework version actually support the mechanism being proposed, confirmed via Context7 against the installed version — not assumed from general familiarity with the framework?
- If this fails or partially fails in production, what is the rollback — a config flag, a route-level revert, or a full redeploy?

If these cannot be answered, the review is not ready for a verdict — send it back with the specific gaps named.

## High-risk assumptions to kill

- "Streaming is strictly additive — it can only make things faster, never worse."
- "The static shell has no sensitive data because we didn't put user data there on purpose."
- "PPR/ISR works the same way in this Next.js major as it did in the blog post I read."
- "A11y is a component-level concern, so it's out of scope for a rendering-strategy review."
- "We'll measure Core Web Vitals after launch" as a substitute for a stated budget before launch.

Those are lazy assumptions. Each one has caused a real production regression in review histories this skill's design pattern is meant to catch.

## When to push back

Push back if the proposal:

- states a performance goal with no route-level budget or measurement plan,
- moves rendering strategy without naming the resulting static/dynamic data split,
- treats accessibility of streamed content as a follow-up ticket rather than part of the design,
- cites a framework capability without the installed version being confirmed against Context7-verified docs,
- offers no rollback mechanism narrower than a full redeploy.

That is not "shipping faster." It is deferring the review to production incidents.
