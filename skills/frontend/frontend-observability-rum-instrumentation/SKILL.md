---
name: frontend-observability-rum-instrumentation
description: Design or review browser-side Real User Monitoring instrumentation for Core Web Vitals (LCP, INP, CLS) using the web-vitals attribution build and distributed tracing via OpenTelemetry Web, enforcing lab-vs-field evidence labeling, sampling/cardinality sizing, and PII-in-telemetry controls, with library-specific wiring references loaded only when instrumentation code is actually being written or reviewed.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-07-02"
  category: observability
---

# Frontend Observability RUM Instrumentation

## Purpose

Lab-only performance testing (Lighthouse, synthetic CI runs) systematically misses the device/network diversity of real users, and ad-hoc telemetry instrumentation routinely leaks PII into span attributes or blows up observability cost through unsampled high-cardinality export. This skill wires and reviews field RUM (web-vitals + OpenTelemetry Web) with explicit evidence labeling and privacy/cost guardrails baked in from the start.

## When to use

Use this skill when the user asks to:

- instrument Core Web Vitals (LCP, INP, CLS) field measurement in a web app,
- set up or review OpenTelemetry Web browser tracing (document load, fetch/XHR spans),
- review an existing RUM/telemetry pipeline for PII exposure or sampling/cost issues,
- interpret or report on Core Web Vitals data and needs a lab-vs-field distinction made explicit,
- set or validate a performance budget against field p75 data.

## When NOT to use

- Diagnosing *why* a specific LCP/INP/CLS number is high (phase decomposition of an existing regression) — hand off to `core-web-vitals-triage`; this skill instruments the pipeline that produces the data, it does not decompose an already-captured regression.
- Bundle size, code-splitting, or caching remediation once a cause is known — hand off to `bundle-budget-code-splitting-review` or `service-worker-cache-strategy-review`.
- Server-side or backend-service tracing with no browser-origin span — this skill is scoped to browser-side RUM and OpenTelemetry Web only.

## Context7 Documentation Protocol

`web-vitals` and OpenTelemetry Web packages ship breaking API changes across major versions (attribution-build option shapes, sampler class names, exporter constructor options), and memorized snippets go stale. Before writing or reviewing instrumentation code:

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded this session.
2. Call `mcp__Context7__resolve-library-id` for `web-vitals` and/or `opentelemetry-js` before making any claim about their current API shape. Verified IDs as of this skill's `updated` date: `/googlechrome/web-vitals` and `/open-telemetry/opentelemetry-js`.
3. Call `mcp__Context7__query-docs` for the specific mechanism in scope — e.g. "attribution build generateTarget option", "INPAttributionReportOpts durationThreshold", "WebTracerProvider spanProcessors config", "TraceIdRatioBasedSampler" — before ruling on it. Do this per review; do not reuse a prior session's memory of these APIs.
4. Known facts verified via Context7 as of this skill's `updated` date: the attribution build (`web-vitals/attribution`) accepts `AttributionReportOpts` (`reportAllChanges`, `generateTarget`) on `onCLS`/`onFCP`/`onLCP`/`onTTFB`, and `INPAttributionReportOpts` additionally exposes `durationThreshold` (default `40`) and `includeProcessedEventEntries` (default `true`) on `onINP`. `WebTracerProvider` (from `@opentelemetry/sdk-trace-web`) takes a `spanProcessors` array in its constructor — do not tell a user to call a separate `addSpanProcessor` method as the primary pattern without checking the installed SDK version. `TraceIdRatioBasedSampler` takes a single ratio argument (0–1) and is normally wrapped in `ParentBasedSampler({ root: ... })` so that downstream/parent sampling decisions are respected.
5. If Context7 is unavailable or returns no relevant match, fall back to `official_docs` / `references/*.md` and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
6. Never invent a `web-vitals` metric field, attribution property, OpenTelemetry exporter option, or sampler class that no queried source confirms.

## Lean operating rules

- Always label every performance number as lab evidence (Lighthouse/CI/synthetic) or field evidence (RUM, real users — state the percentile; p75 is the CWV standard) — never let the two blur into an unlabeled "the site is fast" claim.
- Default to the standard `web-vitals` build (`onLCP`/`onINP`/`onCLS`) for production reporting; only use the `/attribution` build's richer diagnostic payload when actively debugging a specific regression, and only send that extra payload to a dev/debug destination, not blanket production export.
- Do not enable `reportAllChanges: true` in production by default — it multiplies event volume for a debugging-only benefit.
- Before adding any custom span/metric attribute, run it through a PII check: no raw URLs with query strings, no user IDs, no free-text form input, no precise geolocation, unless explicitly scrubbed and justified.
- Size the OpenTelemetry sampling rate (e.g. `TraceIdRatioBasedSampler` wrapped in `ParentBasedSampler`) against the stated/estimated traffic volume and the backend's cost/cardinality limits; never recommend 100% unsampled export for a high-traffic app without an explicit cost review.
- Use OpenTelemetry Semantic Conventions attribute names instead of inventing ad-hoc span/attribute naming, so telemetry stays queryable/comparable across services.
- Treat any recommended performance budget threshold as grounded in web.dev's documented "good" ranges (LCP ≤2.5s, INP ≤200ms, CLS ≤0.1 at p75) unless the org has an explicitly stricter documented SLO.
- This skill performs static review and instrumentation-code authoring only; it does not deploy telemetry configuration to production or flip sampling/export settings on a live collector without explicit human sign-off logged outside this skill.

## References

Load these only when needed:

- [web-vitals attribution build wiring](references/web-vitals-attribution-wiring.md) — use when writing or reviewing the actual `onLCP`/`onINP`/`onCLS` instrumentation call, choosing between standard and attribution builds, or setting `generateTarget`/`durationThreshold`/`reportAllChanges`.
- [OpenTelemetry Web tracing wiring](references/opentelemetry-web-tracing-wiring.md) — use only when wiring or reviewing `WebTracerProvider`, `DocumentLoadInstrumentation`, `FetchInstrumentation`, exporter, or sampler configuration.
- [Sampling, cardinality, and PII controls](references/sampling-cardinality-pii-controls.md) — use when sizing sampling rates against traffic/cost, naming attributes via Semantic Conventions, or auditing an existing pipeline for PII leakage.

## Response minimum

Return, at minimum:

- the metric/trace component in scope and whether guidance concerns lab or field measurement,
- evidence level for any performance claim (lab-evidence, field-evidence with percentile, or documentation-based),
- instrumentation code or review findings with exact API options used,
- PII-in-telemetry check result for every attribute touched,
- sampling-rate rationale tied to stated traffic volume and cost constraints.
