# Sampling, Cardinality, and PII Controls

Use this reference when sizing a sampling rate against traffic/cost, naming attributes via OpenTelemetry Semantic Conventions, or auditing an existing RUM/tracing pipeline for PII leakage.

## What people get wrong

The naive story is:

> "Sampling is just a knob you turn down if the bill gets too high."

That treats sampling as a cost afterthought instead of a design input, and it ignores that unsampled, high-cardinality, or PII-bearing telemetry is a *data-governance* problem before it is a *cost* problem — a leaked attribute doesn't get cheaper at a lower sample rate, it gets leaked less often.

## Non-negotiable design rules

### 1. Sample deterministically by trace ID, not randomly per-request

`TraceIdRatioBasedSampler` (from `@opentelemetry/sdk-trace-web` / `@opentelemetry/sdk-trace-base`) samples a percentage of traces determined deterministically by the trace ID — any trace sampled at a given ratio is also sampled at any higher ratio. This is the documented building block for consistent, traffic-proportional sampling:

```javascript
const {
  WebTracerProvider,
  ParentBasedSampler,
  TraceIdRatioBasedSampler,
} = require('@opentelemetry/sdk-trace-web');

const provider = new WebTracerProvider({
  sampler: new ParentBasedSampler({
    root: new TraceIdRatioBasedSampler(0.1), // sample 10% of root traces
  }),
});
```

### 2. Wrap it in `ParentBasedSampler` unless you have a specific reason not to

`ParentBasedSampler` respects an incoming trace's sampling decision by default and only delegates the *root*-span sampling decision to the wrapped sampler (e.g., `TraceIdRatioBasedSampler`). This keeps a single distributed trace's sampling decision consistent across service boundaries — a browser span sampled independently of its downstream backend spans produces incomplete, misleading traces.

### 3. Size the ratio against stated traffic volume and the backend's cost/cardinality limits, not a guess

Before recommending a ratio:

- Get (or explicitly estimate and label as an estimate) the page's traffic volume (sessions/day or requests/day).
- Get the backend's pricing/cardinality model (per-span cost, per-attribute cardinality limits, retention window).
- Compute expected exported span volume at candidate ratios (e.g., 100k sessions/day × 5 spans/session × 0.1 ratio = 50k spans/day) and state that arithmetic explicitly in the recommendation — never hand back a bare percentage with no traffic math behind it.
- Never recommend 100% (unsampled) export for a stated high-traffic production app without an explicit, logged cost review — treat that as the exception requiring justification, not the default.

### 4. Name attributes with OpenTelemetry Semantic Conventions, not ad-hoc keys

Use the standard attribute namespaces (e.g., `http.request.method`, `http.response.status_code`, `url.path`, `user_agent.original`) documented in the OpenTelemetry Semantic Conventions rather than inventing project-specific keys like `httpMethod` or `req_status`. Ad-hoc naming breaks cross-service correlation and makes backend queries/dashboards non-portable. When no standard convention covers a genuinely custom business attribute, prefix it clearly (e.g., a documented internal namespace) so it's visually distinguishable from standard conventions, rather than colliding with or shadowing a standard key name.

### 5. Every custom attribute must pass a PII check before it ships

Before adding any span attribute, metric label, or `web-vitals` attribution `generateTarget` output, check it against this list. If any apply, the attribute must be scrubbed, hashed, truncated, or dropped before export:

- **Raw URLs with query strings** — query strings frequently carry session tokens, search terms, or user-identifying values. Capture path only (`url.path`), or explicitly strip known-sensitive query parameters before capturing `url.full`.
- **User identifiers** — no raw email, username, account ID, or customer name as a plain attribute value. If correlation is needed, use an opaque, rotated, non-reversible identifier and document the mapping's access control separately.
- **Free-text form input** — never capture form field values (search boxes, comment fields, addresses) as telemetry attributes.
- **Precise geolocation** — no raw lat/long or street-level location; use coarse-grained (e.g., country/region) if location is genuinely needed for the stated purpose.
- **DOM-derived strings that embed the above** — a CSS selector or `generateTarget` output can accidentally embed a customer name or ID if it's built from live `id`/`class`/`data-*` attributes that the DOM populates with user data; audit before trusting default stringification (see `web-vitals-attribution-wiring.md` rule 1).
- **Auth headers/tokens** — never mirror `Authorization`, cookies, or bearer tokens into span attributes, even truncated; treat any such capture as a credential leak, not a debugging convenience.

### 6. Cardinality is a cost multiplier independent of sampling

A single high-cardinality attribute (e.g., a raw user ID or full URL used as a span attribute) can multiply backend storage/indexing cost regardless of the trace sampling ratio, because most backends index per-unique-attribute-value. Prefer bounded-cardinality attributes (enum-like values, coarse buckets) for anything intended to be an indexed/filterable dimension; keep genuinely high-cardinality values (if needed at all) as non-indexed payload fields, and confirm the plan against the destination backend's own cardinality documentation.

## Minimal safe implementation flow

1. Get or explicitly estimate traffic volume for the page/app in scope.
2. Get the backend's pricing/cardinality/retention model (or state clearly that it's unknown and flag the gap).
3. Propose a `TraceIdRatioBasedSampler` ratio wrapped in `ParentBasedSampler`, with the traffic-volume arithmetic shown.
4. Enumerate every custom span attribute, metric label, and `generateTarget` output planned; run each through the PII check in rule 5.
5. Map custom attributes to OpenTelemetry Semantic Conventions where a standard key exists; namespace the rest clearly.
6. Flag any attribute that is both high-cardinality and unbounded (raw user input, raw URL) as an indexing/cost risk even if it passes the PII check.
7. State the sampling ratio, PII findings, and cardinality findings together as one review output — do not split cost sizing from privacy review into separate, disconnected passes.

## Adversarial checklist

Before approving a RUM/tracing pipeline, answer these:

- What is the actual (or explicitly estimated) traffic volume this sampling ratio is sized against?
- Does the sampler respect parent sampling decisions (`ParentBasedSampler`), or will browser spans and backend spans disagree on what got sampled?
- For every custom attribute: does it match a standard Semantic Convention key, or is it deliberately namespaced as custom?
- For every attribute sourced from user-controllable input (URL, form, DOM content, `generateTarget`): has it been explicitly scrubbed, or is it being trusted "because it looked fine in testing"?
- Which attributes are indexed/filterable in the destination backend, and are any of those unbounded-cardinality values?

If these cannot be answered, the review is incomplete — say so rather than approving.

## When to push back

Push back if the user asks for:

- 100% unsampled export "to not miss anything," on a stated high-traffic app, with no cost review,
- capturing full request URLs or form values "just in case it's useful later,"
- inventing custom attribute names that duplicate an existing Semantic Convention key under a different spelling,
- skipping the PII check because "it's just internal telemetry" — internal-only access does not eliminate the need for a PII/data-governance review; it changes who is affected, not whether it matters.

Those are not shortcuts. They are cost incidents and data-governance failures waiting to be found in an audit.
