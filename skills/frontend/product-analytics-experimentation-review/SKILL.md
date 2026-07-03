---
name: product-analytics-experimentation-review
description: Review frontend analytics instrumentation and A/B or multivariate experiment configurations for event-schema correctness, sample-ratio-mismatch risk, statistically valid stopping rules, and consent-gated privacy compliance before shipping a tracking or experiment change.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: data
---

# Product Analytics & Experimentation Review

## Purpose

Analytics and experimentation code looks low-risk (it doesn't change what users see) but is exactly where silent, expensive failures accumulate: schema drift that zeroes a KPI dashboard for weeks, sample-ratio mismatch that invalidates a whole test, and unconsented tracking that creates real compliance exposure. This skill exists to apply a measurement-integrity and privacy review before ship, not after a stakeholder notices the dashboard looks wrong.

## When to use

Use this skill when the user asks to:

- review new or changed analytics event instrumentation for schema correctness,
- validate an A/B or multivariate experiment's bucketing logic and statistical plan before launch,
- audit whether tracking calls are properly consent-gated for privacy compliance,
- diagnose a suspected sample-ratio mismatch or an experiment result that looks statistically implausible.

## When NOT to use

- Reviewing Core Web Vitals or general RUM/tracing instrumentation with no experiment or event-schema angle — hand off to `frontend-observability-rum-instrumentation`.
- Reviewing generic security/XSS/CSP posture of a page — hand off to `frontend-dom-xss-csp-review`; this skill only reviews the analytics/experimentation-specific privacy surface (consent gating and PII-in-events), not the broader page security model.
- Choosing a state-management or component-architecture pattern with no analytics or experiment angle — out of scope.

## Context7 Documentation Protocol

Analytics vendor SDKs (GA4/gtag.js, feature-flag/experimentation platforms) change event-parameter names, consent-mode signal shapes, and SDK method signatures across versions, and memorized snippets go stale fast. Before making any platform-specific claim:

1. Call `ToolSearch` with query `"context7"` (or `"select:mcp__Context7__resolve-library-id,mcp__Context7__query-docs"`) to load the Context7 tools if not already loaded this session.
2. Call `mcp__Context7__resolve-library-id` for the specific analytics/experimentation library actually imported in the code under review (e.g. the GA4/`gtag.js` SDK, a specific feature-flag/experiment SDK) before describing its event or bucketing API. Do not assume GA4 by default — verify the platform from the actual `import`/script-tag evidence first.
3. Call `mcp__Context7__query-docs` for the specific mechanism in scope — e.g. "GA4 recommended event parameters for purchase event", "Google Consent Mode v2 signal defaults", "GA4 measurement protocol event schema limits" — before ruling on it. Verified library ID for web.dev platform guidance as of this skill's `updated` date: `/websites/web_dev_articles`.
4. Known facts verified via Context7/web.dev as of this skill's `updated` date: web.dev documents sending `web-vitals` metrics to GA4 via `gtag('event', 'web_vitals', {...})` with `name`/`value`/`delta`/`id`/`label` fields, and shows GA4 BigQuery-export event tables keyed by `event_name`/`event_params`/`event_timestamp`/`user_pseudo_id` — treat any schema claim about GA4's exported event shape as needing this structure, not an invented one. web.dev's Permissions API guidance (`permissions-best-practices`) documents `navigator.permissions.query({name: ...})` returning a `state` of `granted`/`denied`/`prompt`, and states permission grants are scoped per-origin (a grant on one origin does not transfer to a subdomain/different origin) — apply the same non-transferability logic when reasoning about consent scope across subdomains.
5. If Context7 is unavailable or returns no relevant match, fall back to `official_docs` / `references/*.md` and mark the claim `documentation-based (Context7 unavailable)` rather than presenting it as freshly verified.
6. Never invent an analytics event-parameter name, consent-mode signal name, or experimentation-platform API that no queried source confirms.

## Lean operating rules

- Identify the actual analytics/experimentation platform in use from the imported SDK or script tag before citing platform-specific behavior — do not assume GA4 or any specific vendor by default.
- Verify that bucketing/assignment logic is deterministic per user (stable hash/seed keyed to a persistent identifier), not re-randomized on refresh, session change, or page reload — this is the single most common cause of invalid experiment results and a leading cause of sample-ratio mismatch.
- Verify consent gating is enforced at the call site of the tracking function itself, not merely present somewhere else on the page — a consent banner existing does not mean a specific event call respects it; trace the actual conditional guarding the SDK call.
- Flag any event schema field that could carry PII (free-text fields, email, precise geo/lat-long, payment data, raw URLs with query strings, user-typed search terms) for hashing/redaction before approving.
- Require a pre-registered primary metric and minimum detectable effect (MDE) for any experiment reviewed; treat their absence as a blocking finding, not a nice-to-have — an experiment analyzed after the fact against whichever metric moved is not a valid test.
- Treat any observed sample split materially off the configured ratio (e.g. configured 50/50 showing as 46/54 or further at meaningful volume) as a sample-ratio-mismatch candidate requiring a chi-squared check, not a rounding artifact to wave off.
- Load `references/srm-and-bucketing-integrity.md` only when auditing assignment/bucketing logic or diagnosing a suspected sample-ratio mismatch.
- Load `references/consent-and-pii-in-events.md` only when reviewing privacy/consent compliance of tracking calls or event payload PII exposure.
- Load `references/stopping-rules-and-peeking.md` only when evaluating whether an experiment's statistical significance claim or stop/continue decision is valid.
- This skill performs static review only; it does not execute experiment code, query a live analytics backend, or flip a feature flag / experiment configuration in production.

## Privacy & Consent Depth for Analytics

The generic consent/PII posture above (banner presence is not compliance, check the call site) covers the baseline. Some tracking changes need standard-specific depth: IAB TCF v2.2 purpose-granular consent, Google Consent Mode v2's default-denied timing requirement, and adjacent surfaces (Global Privacy Control/Do-Not-Track, cookie categorization, analytics-endpoint data residency) that a generic consent check can miss.

- **Consent Mode v2 defaults must be synchronous and denied-by-default** (doc-based, Google Consent Mode v2 docs): `gtag('consent', 'default', {analytics_storage: 'denied', ad_storage: 'denied', ad_user_data: 'denied', ad_personalization: 'denied'})` must be set at the top of the page, before the gtag.js/GTM snippet loads and before any `gtag('event', ...)` call — not inside a CMP callback or an async-loaded script. A later `gtag('consent', 'update', ...)` call once the user answers the CMP does not retroactively fix a missing or async default; tags may already have fired under an undefined/permissive state.
- **IAB TCF v2.2 consent is granular per purpose and per vendor, not a single flag** (standard-based inference from the TCF spec): a compliant check validates a specific `(vendorId, purposeId)` grant from the decoded TC string — "a TC string cookie exists" is presence, not scope. A vendor consented for one purpose (e.g. measurement) is not automatically consented for another (e.g. personalized ads).
- **Any tracking call — pixel, `gtag`, `sendBeacon`, `fetch` — that fires before a consent signal exists is unrecoverable exposure**: unlike a suppressed JS event, an HTTP request (and any PII in its query string or body) cannot be un-sent once it leaves the client.
- **PII in event properties is a violation independent of consent state**: consent governs whether tracking may happen, not what may be sent once it does — a consented-but-PII-laden event (raw email, full name, unhashed user ID) is still a data-minimization failure.
- **Cookies set without a declared category or an explicit expiry cannot be honored by a CMP** — flag any analytics/marketing cookie missing `Max-Age`/`Expires` or a category mapping.
- **Global Privacy Control (`navigator.globalPrivacyControl`) and Do-Not-Track (`navigator.doNotTrack`) must be checked as an opt-out signal alongside explicit CMP consent**, not replaced by it.
- **Analytics-endpoint data residency must be identified, not assumed** — note the destination host/region for each analytics call and flag payloads sent to a default/global endpoint when a residency-scoped endpoint is expected.
- Load `references/privacy-consent-depth-for-analytics.md` when a review needs this standard-specific depth (Consent Mode v2 timing, TCF purpose/vendor granularity, GPC/DNT, cookie categorization, data residency) rather than the general consent/PII check alone.

## References

Load these only when needed:

- [SRM and bucketing integrity](references/srm-and-bucketing-integrity.md) — use to verify deterministic, unbiased user assignment and to diagnose a suspected sample-ratio mismatch.
- [Consent and PII in events](references/consent-and-pii-in-events.md) — use to verify tracking calls are consent-gated and event payloads do not leak PII.
- [Stopping rules and peeking](references/stopping-rules-and-peeking.md) — use to evaluate whether an experiment's significance claim is valid given its actual monitoring/stopping behavior.
- [Privacy and consent depth for analytics](references/privacy-consent-depth-for-analytics.md) — use for IAB TCF v2.2 purpose/vendor granularity, Google Consent Mode v2 default-timing requirements, GPC/Do-Not-Track honoring, cookie categorization/expiry, and analytics-endpoint data residency.

## Response minimum

Return, at minimum:

- the analytics/experimentation platform identified and the docs used to verify its behavior,
- schema-correctness verdict against the documented data contract,
- SRM/bucketing-integrity verdict,
- consent-gate and PII findings,
- statistical-validity verdict (pre-registered metric/MDE present, stopping rule sound) with evidence level.
