---
name: critical-rendering-path-review
description: Review page-load resource sequencing, render-blocking CSS/JS, layout-shift sources, and Core Web Vitals (LCP/CLS/INP) budget adherence against the critical rendering path model, explicitly separating lab/synthetic measurement (Lighthouse) from field/real-user measurement (CrUX/RUM) so performance claims are evidence-graded rather than asserted from a single synthetic run.
allowed-tools: Read Grep Glob Bash(git diff:*) WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: operational
---

# Critical Rendering Path Review

## Purpose

Performance regressions in the critical rendering path (render-blocking resources, layout-shift-inducing patterns, oversized LCP candidates) are cheap to introduce and expensive to diagnose after the fact, and lab data (a single Lighthouse run) frequently disagrees with field data (real users on real networks/devices) in ways that matter — a change that looks fine in a fast CI-runner Lighthouse run can regress Core Web Vitals for the actual user population. This skill reviews resource-loading order and layout-shift risk against the browser's actual parse/style/layout/paint pipeline, and enforces the lab-vs-field evidence distinction so performance verdicts aren't overclaimed.

## When to use

Use this skill when the user asks to:

- review a diff for render-blocking CSS/JS, resource-loading order, or `<link>` resource-hint (preload/prefetch/preconnect) usage,
- diagnose or prevent Cumulative Layout Shift (CLS) sources (unsized images/embeds, late-injected content above existing content, web-font swap),
- assess Largest Contentful Paint (LCP) or Interaction to Next Paint (INP) budget adherence for a page or component,
- reconcile conflicting lab (Lighthouse) vs. field (CrUX/RUM) performance data,
- set or enforce a performance budget for a page/route.

## Context7 Documentation Protocol

Render-blocking semantics, `<link>` resource-hint behavior, and Core Web Vitals thresholds/definitions change as browser implementations and measurement methodology evolve (e.g. INP replaced FID as a Core Web Vital in May 2024) — never assert them from memory.

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if they are not already loaded in this session.
2. Call `mcp__Context7__resolve-library-id` for the relevant documentation set — for this skill that is almost always MDN (`/mdn/content`, or `/websites/developer_mozilla_en-us` if the former lacks coverage for the query).
3. Call `mcp__Context7__query-docs` for the specific mechanism in question — e.g. "render-blocking `<link>` and `<script>` behavior", "`rel=preload` `as` and `crossorigin` requirements", "LargestContentfulPaint / layout-shift performance entry semantics" — before ruling on it. Do this per review, not once from memory of a prior session.
4. web.dev is the primary source for Core Web Vitals *thresholds* (good/needs-improvement/poor cutoffs) and is not currently indexed in Context7; treat threshold numbers pulled from `official_docs`/`WebFetch` as `documentation-based` rather than `Context7-verified`, and say so explicitly. Use Context7/MDN to verify the underlying *mechanism* (what LCP/CLS/INP actually measure, what a `PerformanceObserver` reports) wherever possible.
5. Prefer the official spec/MDN wording over this skill's own paraphrase when the two could be read to disagree; cite the resolved doc URL in the finding.
6. If Context7 is unavailable or returns no relevant match, fall back to the URLs in `official_docs` / `references/*.md`, and explicitly mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
7. Never invent a `<link>` attribute, performance-entry field, or Core Web Vitals threshold that no queried source confirms.

## Lean operating rules

- Always separate lab data (synthetic, single-run, deterministic-ish, from Lighthouse/CI) from field data (real-user, aggregated, from CrUX or RUM tooling) explicitly in every finding — never present a Lighthouse score as if it were a field-validated user-experience claim.
- Trace resource-loading order against the actual critical rendering path (HTML parse → DOM/CSSOM construction → render tree → layout → paint → composite) rather than asserting 'this is render-blocking' without identifying which pipeline stage it blocks.
- Every image, video, or embed must have explicit width/height or `aspect-ratio` reserved space to prevent CLS; flag any that don't.
- Flag web-font loading without a `font-display` strategy or preload for the LCP-critical font, since font-swap/flash is a common unaddressed CLS and LCP-delay source.
- Query current web.dev/MDN Core Web Vitals thresholds before asserting a pass/fail budget verdict — LCP/CLS/INP thresholds and measurement methodology have changed (INP replaced FID as a Core Web Vital in May 2024); never assert current thresholds from memory.
- Do not recommend `preload`/`prefetch`/`preconnect` for third-party origins without weighing the connection-establishment cost against the information-leakage/timing tradeoff.
- Treat a single synthetic Lighthouse run as insufficient evidence for a production performance claim; flag the need for field data (CrUX/RUM) before asserting a real-user-experience verdict.
- Never suggest dropping Subresource Integrity (SRI) or bypassing CSP `script-src` allow-listing as a performance optimization.

## References

Load these only when needed:

- [Resource-loading order and render-blocking audit](references/resource-loading-order.md) — use when tracing which resources block first paint/LCP and how to reorder or hint them (preload/defer/async/module).
- [Layout-shift source catalog](references/layout-shift-sources.md) — use when auditing a page/component for CLS sources beyond unsized media (late web-font swap, injected banners/ads, animation-triggered reflow).
- [Lab vs. field evidence reconciliation](references/lab-vs-field-evidence.md) — use when Lighthouse (lab) and CrUX/RUM (field) data disagree, or when a performance claim needs to be evidence-graded for a stakeholder deliverable.

## Response minimum

Return, at minimum:

- the critical-rendering-path stage(s) affected by the change in scope (parse/style/layout/paint/composite),
- the render-blocking resource audit for any new/modified `<link>`, `<script>`, or `@import`,
- the CLS-source audit (unsized media, font-swap, late-injected content) for the page/component in scope,
- the LCP/CLS/INP budget verdict, explicitly labeled as lab evidence, field evidence, or inference,
- residual risk notes for anything requiring live Lighthouse/CrUX/RUM data beyond this static review.
