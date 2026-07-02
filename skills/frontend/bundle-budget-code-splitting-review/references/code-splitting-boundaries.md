# Code-Splitting Boundaries

Use this reference when deciding where to add route- or component-level splitting, or when diagnosing a duplicated-dependency-across-chunks finding or an over-splitting regression.

## What people get wrong

The naive story is:

> "More `import()` calls always make the bundle better."

Wrong. Splitting trades parse/execute weight on one chunk for request-count and coordination overhead across chunks. Per web.dev's code-splitting guidance, the goal is to defer weight that is not needed for the current route/interaction — not to fragment the bundle for its own sake. Every additional chunk is an additional request the browser must schedule, and on a constrained connection or a cold cache, that overhead is real and can offset the byte savings the split was meant to deliver.

## Classification per contributor

For each large module identified in the analyzer report, classify it into exactly one path:

1. **Needed on the critical path (keep, minimize).** The module is required to render the current route's above-the-fold content or its primary interaction. Do not split it away; instead look for a lighter alternative or confirm it is already tree-shaken (hand off to `tree-shaking-dead-code-review` if unused exports are suspected within an otherwise-necessary dependency).
2. **Needed but deferrable (route/component-split candidate).** The module is required somewhere in the app but not on the current route's critical render/interaction path — e.g., a charting library only used on a `/reports` route, a modal's rich-text editor only needed after a user action. Split via dynamic `import()` at the route or component boundary where it is actually invoked.
3. **Duplicated across chunks.** The same dependency (or a near-duplicate version) appears in the byte weight of more than one chunk. This is a chunk-grouping configuration defect, not an application-code defect — fix it via the bundler's vendor-chunk / `cacheGroups` / `codeSplitting.groups` configuration (see [Bundler chunking APIs](bundler-chunking-apis.md)), not by rewriting application imports.
4. **Heavier than necessary for its purpose.** The dependency itself is oversized relative to what the app uses from it (e.g., importing a full date-utility library for one format call). Recommend a lighter alternative and state the size delta; do not default to "just split it" when the real fix is not depending on the heavy library at all.

## Route-level vs. component-level splitting

- **Route-level splitting** is the default first move: split at the router boundary so an entire route's code (including its route-specific dependencies) loads only when that route is navigated to. This is the highest-leverage split because it aligns with actual user navigation and typically yields one request per route transition, not per component.
- **Component-level splitting** is for a specific heavy component *within* an already-loaded route that is not needed immediately — e.g., a modal, a below-the-fold widget, a rarely-used settings panel. Reach for this only after route-level splitting is exhausted; splitting every individual component inside an already-necessary route multiplies request count without changing what must load before the route is usable.
- Do not recommend component-level splitting for something the user will interact with immediately on route entry — that just moves the same byte weight into a second network round trip with no benefit, and adds a loading-state flash that can itself become a CLS or perceived-jank complaint (hand off wording/attribution disagreements to `core-web-vitals-triage`).

## Over-splitting: a real regression, not just a missed optimization

Before endorsing any new split point, require a request-count comparison alongside the byte comparison:

- If a split reduces the shared/critical-path bundle by N KB but adds M new chunks that are each fetched on the same route transition (e.g., several components all lazy-loaded on the same screen, each producing its own round trip), the net effect can be worse: more connection/scheduling overhead, more parse/compile invocations, and a longer time to fully interactive — even though the "critical path" number looks smaller in isolation.
- HTTP/2+ multiplexing reduces but does not eliminate this cost: each chunk still has its own parse, compile, and module-registration cost on the main thread, which is the more INP-relevant number per `core-web-vitals-triage`'s execution-cost framing.
- The correct test is: does total time-to-interactive (or the relevant INP-adjacent metric) improve, not just does the critical-path byte number shrink. State this comparison explicitly in the finding; do not accept a smaller entry-chunk number alone as proof of improvement.

## Duplicated dependency across chunks — diagnosis path

1. Confirm via the analyzer report (most bundle-analyzer tools flag this directly, e.g. webpack-bundle-analyzer's treemap showing the same module path under multiple chunk nodes) that the same module resolves into more than one output chunk.
2. Check for version skew first — two different semver ranges of the same dependency resolving to two separate copies is an application-level (`package.json`/lockfile) defect, not a chunking-config defect; recommend a dependency dedupe/resolution fix, not a chunking change.
3. If it is genuinely one version pulled into multiple chunks because multiple entry points or route chunks each import it, this is the textbook case for a shared vendor chunk (webpack `cacheGroups`) or an equivalent shared-group rule (Vite `codeSplitting.groups`) — see [Bundler chunking APIs](bundler-chunking-apis.md).

## Non-negotiables

- Do not recommend a component-level split for anything visible immediately on route entry.
- Do not endorse a split without the request-count comparison alongside the byte comparison.
- Do not treat a duplicated-dependency finding as an application-code problem before checking for version skew.
