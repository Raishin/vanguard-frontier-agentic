# CI Enforcement and Verification

Use this reference for every review to specify the CI-enforced budget mechanism, the verification command, and the adversarial checklist before a size finding can be closed.

## Non-negotiable: no finding closes without a CI-enforced budget

A one-time manual fix — splitting a chunk, removing a dependency, adjusting `cacheGroups` — proves the bundle is smaller *today*. It proves nothing about tomorrow's dependency bump, an unreviewed import added six weeks from now, or a well-intentioned refactor that quietly reintroduces the same weight. Require one of the following, tied to the numeric budget established in [Budget methodology](budget-methodology.md), before marking any size finding resolved:

- A `bundlesize`-style check (or equivalent size-limit tool) wired into CI that fails the build/PR when a named entry or chunk exceeds its budget.
- A Lighthouse CI budget (`budgets.json` / `lighthouserc` budget assertions) that fails on `resource-summary` script/stylesheet byte-size thresholds.
- A bundler-plugin size check (e.g., a Rollup/Vite/webpack plugin that asserts output size at build time) that fails the build rather than just reporting.

If none of these exist in the project, state that as an explicit gap in the finding — do not let a review close as "fixed" when the only evidence is a single local `npm run build` byte count with no enforcement wired into CI.

## Verification command

State the exact command used to produce the before/after comparison, matching the project's actual tooling rather than assuming a generic one:

- `npm run build -- --report` (or the project's equivalent analyzer-invoking build script) to regenerate the analyzer report.
- For Vite projects, confirm whether `rollup-plugin-visualizer` (or an equivalent) is wired into the build config; if not present, state that as a prerequisite gap before a byte-level review can be evidence-backed rather than estimated.
- For webpack projects, confirm `webpack-bundle-analyzer` (or the `--json` stats output piped into an equivalent tool) is available.
- Always compare gzipped (or brotli, matching production `Content-Encoding`) sizes, not raw/parsed sizes, unless the budget was explicitly defined in a different compression form — see [Budget methodology](budget-methodology.md).

## Adversarial checklist

Before closing a bundle-size finding, answer these:

- Is the budget tied to a percentile and a device/network class, or is it vague ("keep it small")? If vague, it is not a budget yet — fix that first.
- Is the prescribed bundler chunking API confirmed against the installed major version via Context7, or is it a memorized snippet that might target a removed/deprecated API shape?
- Does the proposed split reduce byte weight without increasing request count enough to net-negative the change? Was that comparison actually made, or only the byte number?
- Is a CI-enforced budget check part of the deliverable, or only a one-time manual diff?
- Was the ranking done by byte size alone, or also by main-thread execution cost? A large but rarely-executed module and a small but hot-path module do not carry equal INP risk.
- If a dependency was flagged as "too heavy," was a lighter alternative actually identified with a size delta, or was "split it" offered as a substitute for that harder analysis?
- If a dynamic `import()` specifier is built from any user-controlled input (route param, query string, feature flag value sourced externally), was that flagged as a code-injection risk, not filed only as a performance note?

If any of these cannot be answered affirmatively, the finding is not ready to close — say so explicitly rather than presenting a partial analysis as complete.

## When to push back

Push back if the user asks for:

- a one-time size fix with no request to add or verify a CI budget check — explain why that guarantees regression.
- inlining a third-party script "to save a request" without acknowledging the CSP/SRI trade-off.
- splitting every component in a route "to be safe," without a request-count comparison — that is not caution, it is trading one unverified assumption for another.
- a chunking config copied from a different bundler or an unconfirmed version, because "it's probably still the same API" — that is exactly the failure mode this skill's Context7 protocol exists to prevent.

Those are not shortcuts. They convert a measurable problem into an unmeasured one.

## Handoff boundaries

- If the root cause is unused exports inside an otherwise-necessary, correctly-split dependency: hand off to `tree-shaking-dead-code-review`.
- If the user's starting complaint is a vague "it feels slow" with no analyzer report yet and JS weight has not been confirmed as the dominant contributor: hand off to `core-web-vitals-triage` first, then return here once bundle weight is confirmed as the cause.
- If the fix under discussion is caching strategy for repeat visits rather than first-load byte weight: hand off to `service-worker-cache-strategy-review` (or the closest equivalent asset in scope) rather than stretching this skill's budget framing to cover caching.
