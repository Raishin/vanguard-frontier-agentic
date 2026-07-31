---
name: core-web-vitals-triage
description: Decomposes LCP, INP, and CLS regressions into their documented sub-phases using lab and field evidence, and refuses to declare a metric fixed without a field-data or CI-budget verification path.
allowed-tools: Read Grep Glob Bash(lighthouse:*) WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: operational
---

# Core Web Vitals Triage

## Purpose

Teams routinely shotgun-fix Core Web Vitals: compress an image, preload a font, and split a bundle in the same PR, then declare victory off one green Lighthouse run. That never isolates the actual cause, frequently doesn't move the field percentile Google's page-experience signal and real users actually experience, and burns engineering cycles on the wrong sub-phase. This skill decomposes an LCP, INP, or CLS regression into its documented sub-phases (per web.dev's LCP four-phase model, INP three-phase model, and the W3C Largest Contentful Paint / Event Timing specs), attributes each phase to a concrete verifiable cause, and refuses to close the loop without a field-data or CI-budget verification path — because a single lab run proves nothing about the field percentile that ranking and users actually see.

## When to use

Use this skill when the user asks to:

- diagnose a Core Web Vitals regression (LCP, INP, or CLS) reported by Lighthouse, PageSpeed Insights, CrUX, or Search Console's page-experience report,
- explain why a page "feels slow" or "feels janky" in terms of a specific, verifiable rendering or interaction sub-phase,
- reconcile a lab score change with (or without) a corresponding field percentile shift,
- decide which downstream skill should implement a Core Web Vitals fix (bundle/code-splitting vs. caching vs. server/CDN).

## When NOT to use

- Pure server/infra latency tuning with no client-rendering component (TTFB-dominant LCP with no client-side contribution) — hand off to an infra/CDN-focused review; this skill identifies that TTFB is the dominant phase but does not own backend latency remediation.
- Native mobile app performance — different metric model, not Core Web Vitals.
- Implementing the fix. This skill diagnoses and specifies the fix, then hands off implementation-class changes to `bundle-budget-code-splitting-review` or `service-worker-cache-strategy-review` (or the closest equivalent asset in scope) — it does not write the fix code itself.

## Context7 Documentation Protocol

Framework rendering internals that affect LCP/INP attribution (image loading priority hints, hydration/streaming boundaries, transition scheduling) change between major versions, and memorized behavior goes stale fast. Before attributing a delay to a specific framework mechanism:

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded this session.
2. Call `mcp__Context7__resolve-library-id` for the framework in scope (e.g. Next.js, React, Vite) before making any claim about its current rendering/loading behavior.
3. Call `mcp__Context7__query-docs` for the specific mechanism — e.g. "Image component priority/preload prop", "useTransition / startTransition scheduling", "manualChunks / codeSplitting build output" — before ruling on it. Do this per review; do not reuse a prior session's memory of framework internals.
4. Known version-sensitive traps verified via Context7 as of this skill's `updated` date: Next.js deprecated the `<Image priority>` prop in Next.js 16 in favor of `preload` (same LCP-eager-load intent, new prop name — do not tell a Next.js 16+ user to add `priority` without checking their major version first). Vite deprecated the object form of `build.rollupOptions.output.manualChunks` in Vite 8+ in favor of `rolldownOptions.output.codeSplitting` — do not prescribe `manualChunks` config to a Vite 8+ project without verifying the installed version.
5. web.dev is the primary source for Core Web Vitals *thresholds* and *phase definitions* and is not currently indexed in Context7; treat threshold/phase-boundary numbers pulled from `official_docs`/`WebFetch` as `documentation-based`, not `Context7-verified`, and say so explicitly.
6. If Context7 is unavailable or returns no relevant match, fall back to `official_docs` / `references/*.md` and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
7. Never invent a framework prop, build-config option, or performance-entry field that no queried source confirms.

## Lean operating rules

- Classify the evidence tier before decomposing anything: field RUM data present (CrUX API or `web-vitals` library export) vs. lab-only (Lighthouse/PSI JSON). Cap the verdict ceiling at `lab evidence only` when field data is absent — never phrase a lab-only finding as a field-validated user-experience claim.
- Decompose LCP into its four documented sub-phases (TTFB, resource-load delay, resource-load duration, render delay) per web.dev — never leave a finding at "LCP is high," always name the dominant phase.
- Decompose INP into its three documented sub-phases (input delay, processing time, presentation delay) per web.dev — never attribute INP to generic "JS is slow" without naming which phase and which task/handler.
- Decompose CLS by naming the specific shifting element and its trigger (late-loading web font, unsized image/embed, late-injected content, animation-triggered reflow) — a CLS score alone is not a diagnosis.
- Distinguish lab CLS (bounded to a single automated load, per Lighthouse) from field CLS (session-long, includes user-triggered layout shifts within 500ms of input, per the field CLS definition) — do not generalize one to the other.
- Attribute every phase to a concrete, verifiable artifact: a network-waterfall entry, a main-thread long-task entry, or a specific unsized DOM element. Never accept or emit "JS is slow" or "images are heavy" as a terminal diagnosis.
- State the correct owning skill for any implementation-class fix (bundle/code-splitting for JS-weight causes, service-worker caching for repeat-visit causes) rather than attempting to specify the implementation here.
- Never declare a metric "fixed" from a single lab run. State the CrUX 28-day rolling-window lag explicitly whenever recommending field-data confirmation.
- Never recommend removing accessible loading-state semantics (`aria-live`, `role="status"`, visible focus indicators) as a CLS or INP fix — that trades a compliance/usability regression for a metric number.
- Always account for device class: mobile field data (not desktop lab data) is what page-experience ranking and most real users actually see, per CrUX/web.dev methodology.

## References

Load these only when needed:

- [LCP phase decomposition](references/lcp-phase-decomposition.md) — use when the regression or complaint is about page-load speed / the largest visible element painting late.
- [INP phase decomposition](references/inp-phase-decomposition.md) — use when the regression or complaint is about interaction responsiveness / a page "feeling janky."
- [CLS attribution](references/cls-attribution.md) — use when the regression or complaint is about visible layout jumping/shifting.
- [Evidence tiers, verification, and handoff](references/evidence-tiers-and-handoff.md) — use for every triage to classify evidence tier, specify the re-verification path, and route implementation-class fixes to the correct owning skill.

## Response minimum

Return, at minimum:

- per-metric verdict (LCP/INP/CLS as applicable) with its evidence tier (`field evidence` vs. `lab evidence only` vs. `documentation-based` vs. `inference`),
- the sub-phase decomposition table for each metric in scope,
- the root-cause attribution per phase, tied to a concrete artifact (waterfall entry, long-task entry, DOM element),
- the minimal targeted fix per phase and which skill owns implementing it,
- the exact re-verification command/tool and the field-data confirmation window (CrUX's 28-day rolling window, stated explicitly) before the metric can be declared fixed.
