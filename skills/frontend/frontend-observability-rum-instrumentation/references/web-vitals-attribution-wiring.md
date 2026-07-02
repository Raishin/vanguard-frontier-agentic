# web-vitals Attribution Build Wiring

Use this reference only when writing or reviewing the actual `onLCP`/`onINP`/`onCLS` (or `onFCP`/`onTTFB`) instrumentation call.

> Version note: `web-vitals` API surface (attribution option shapes, default thresholds) is version-sensitive. Verify the installed package version against Context7-fetched docs (`/googlechrome/web-vitals`) before prescribing option names.

## What people get wrong

The naive story is:

> "I'll import `onLCP` from `web-vitals` and send everything to analytics with `reportAllChanges: true` so I never miss a value."

Wrong on two counts:

1. The **standard build** (`web-vitals`) only returns the metric name, value, rating, and delta. It does **not** tell you *why* LCP was slow or *which* element shifted for CLS. For root-cause work you need the **attribution build** (`web-vitals/attribution`), which is a separate import path, not a flag on the standard build.
2. `reportAllChanges: true` reports every intermediate value change (e.g., every CLS shift, every LCP candidate), not just the final value — this is a debugging tool that multiplies event volume in production, not a default-on setting.

## Standard build vs. attribution build

- **Standard build** (`import {onLCP, onINP, onCLS} from 'web-vitals'`): metric name, `value`, `rating` (`good`/`needs-improvement`/`poor`), `delta`, `id`. Use this for default production RUM export — it is the lower-cardinality, lower-payload choice.
- **Attribution build** (`import {onLCP, onINP, onCLS} from 'web-vitals/attribution'`): adds a `metric.attribution` object with phase/target detail (e.g., LCP's `attribution.target`, `attribution.timeToFirstByte`, `attribution.resourceLoadDelay`; INP's `attribution.interactionTarget`, `attribution.inputDelay`, `attribution.processingDuration`, `attribution.presentationDelay`; CLS's `attribution.largestShiftTarget`, `attribution.largestShiftTime`).
- Do not send the attribution build's full diagnostic payload to a blanket production analytics endpoint by default — route it to a debug/diagnostic destination or sample it separately, because its per-metric fields substantially increase payload size and can carry more surface area for accidental PII (e.g., a raw element attribute pulled into a target string).

## Non-negotiable design rules

### 1. Choose `generateTarget` deliberately, not by default

`AttributionReportOpts.generateTarget` is `(el: Node | null) => string | undefined`. It overrides how a DOM node is stringified in attribution fields (e.g., `attribution.target`, `attribution.largestShiftTarget`, `attribution.interactionTarget`). Do not leave the default CSS-selector-based stringification unexamined if the DOM contains sensitive `id`/`class`/`data-*` values that could leak into telemetry (e.g., a selector embedding a customer name). A safe pattern prioritizes an intentional tracking attribute over the raw selector:

```javascript
import {onCLS, onINP, onLCP} from 'web-vitals/attribution';

function generateTarget(el) {
  // Prefer an explicit, reviewed tracking attribute.
  if (el?.dataset.trackingId) {
    return el.dataset.trackingId;
  }
  // Fall back to default CSS-selector logic only if that selector
  // is known not to embed PII (audit before relying on this).
  return undefined;
}

onLCP((metric) => sendToAnalytics(metric), {generateTarget});
onCLS((metric) => sendToAnalytics(metric), {generateTarget});
onINP((metric) => sendToAnalytics(metric), {generateTarget});
```

### 2. Tune `INPAttributionReportOpts` explicitly, don't accept silent defaults

`onINP` in the attribution build accepts `durationThreshold` (default `40`ms — interactions shorter than this are not attributed) and `includeProcessedEventEntries` (default `true` — includes the full array of event-timing entries for the interaction frame). State both defaults explicitly in any review: a lower `durationThreshold` increases event volume by attributing more interactions; leaving `includeProcessedEventEntries: true` on a high-traffic page increases per-event payload size and should be weighed against export cost.

### 3. Keep `reportAllChanges` off in production by default

`reportAllChanges` (inherited from `ReportOpts`, default `false`) reports every intermediate metric change instead of just the final value. Only enable it for a scoped debugging session against a non-production or sampled destination — never as the default production wiring.

### 4. Match the reporting transport to the page-lifecycle guarantee you need

`web-vitals` callbacks can fire late in the page lifecycle (e.g., on visibility change or page unload for LCP/CLS finalization). Use a transport that survives page unload (e.g., `navigator.sendBeacon`, or `fetch` with `keepalive: true`) rather than a plain `fetch` call that can be cancelled when the page unloads mid-request.

## Minimal safe implementation flow

1. Decide standard vs. attribution build based on whether root-cause detail is needed for this rollout (default: standard build for blanket production RUM).
2. If attribution build: define and review a `generateTarget` function before shipping — do not accept the default selector logic unreviewed.
3. Explicitly set (or explicitly accept and document) `durationThreshold`/`includeProcessedEventEntries` for INP.
4. Leave `reportAllChanges` at its default `false` unless a scoped debug session requires it, and route that debug payload separately.
5. Choose an unload-safe transport (`sendBeacon` / `fetch` with `keepalive`).
6. Run every attribution field emitted through the PII check in `sampling-cardinality-pii-controls.md` before it ships.

## Adversarial checklist

Before shipping any `web-vitals` wiring, answer these:

- Which build (standard or attribution) is this, and does that match the stated purpose (production dashboard vs. active debugging)?
- If attribution build: has `generateTarget` been reviewed against the actual DOM, or is the default selector logic unaudited?
- Is `reportAllChanges` on, and if so, is that intentional and scoped to non-production/debug traffic?
- What percentile will this data be aggregated to before it's called "the site's LCP/INP/CLS" (per CWV convention, p75)?
- Does the reporting transport survive `visibilitychange`/unload, or will late-finalizing metrics be silently dropped?

## When to push back

Push back if the user asks for:

- shipping the attribution build's full payload to production analytics "just in case," with no PII review of `generateTarget` output,
- `reportAllChanges: true` as a permanent production default,
- reporting a single lab run's LCP/INP/CLS value as if it were the field p75 the org will be judged on.

Those are not shortcuts. They inflate cost, leak data, and misreport what users actually experience.
