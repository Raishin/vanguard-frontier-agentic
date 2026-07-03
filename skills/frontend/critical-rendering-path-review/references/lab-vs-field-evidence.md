# Lab vs. Field Evidence Reconciliation

> Core Web Vitals thresholds (the good/needs-improvement/poor cutoffs for LCP/CLS/INP) live in web.dev, which is not currently indexed in Context7. Treat specific threshold numbers as `documentation-based` and re-verify against `https://web.dev/vitals` and the metric-specific web.dev articles before asserting a pass/fail budget verdict — do not assert them from memory or from this file, which does not restate them for that reason.

## What people get wrong

The naive story is:

> "I ran Lighthouse, the score is green, performance is fine."

Wrong, and it is wrong in a specific, well-documented way: Lighthouse is a **lab** tool — a single synthetic run, on a simulated or fixed device/network profile, usually with a cold cache, on whatever machine ran it (a fast CI runner or an engineer's laptop are both faster and more consistent than most real user devices/networks). It cannot see what real users on real hardware and real networks actually experience. A change can score green in a CI Lighthouse run and simultaneously regress Core Web Vitals in the Chrome UX Report (CrUX) for the real user population, and the reverse is equally possible — a lab regression that never shows up in field data because it falls below the threshold real users' devices are sensitive to.

## Officially grounded distinction

- **Lab data** (Lighthouse, WebPageTest, a local Chrome DevTools trace): synthetic, reproducible, deterministic-ish (modulo system noise), useful for **diagnosing** *why* something is slow because it gives you a full trace/waterfall — but it reflects one simulated environment, not your actual user population's device mix, network conditions, cache state, or extensions/interference.
- **Field data** (Chrome UX Report / CrUX, or a first-party Real User Monitoring / RUM setup using the `web-vitals` JS library or the underlying Performance APIs directly): aggregated measurements from real page loads by real users, on their actual devices and networks — this is what Core Web Vitals thresholds are actually evaluated against for things like Google Search's page-experience signal, and it is the only data source that can tell you what users actually experienced.

Neither one is a substitute for the other:

- Lab data without field data → you don't know if your synthetic environment represents your real users at all (e.g. testing on fast fiber when 40% of your traffic is mobile on 4G).
- Field data without lab data → you know something regressed, but you have no trace/waterfall to diagnose *why*, and CrUX is aggregated over a rolling 28-day window, so it lags a deploy by design and can't attribute a regression to a specific commit.

## Non-negotiable review rules

### 1. Never present a Lighthouse score as a user-experience claim

A sentence like "LCP is 1.8s, this is fast for users" is not supportable from a Lighthouse run alone — the correct claim is "LCP is 1.8s in this lab run under [specified throttling profile]; field verification via CrUX/RUM is needed to confirm real-user experience." Say which one you have.

### 2. Label every performance number by its evidence source

Use one of: `lab (Lighthouse/CI)`, `lab (local DevTools trace)`, `field (CrUX)`, `field (RUM)`, `inference (not measured, reasoned from code review)`. Do not present an inferred estimate ("this should improve LCP by roughly X") as if it were measured.

### 3. CrUX has structural limitations — know them before citing it

CrUX data: is a rolling aggregate (commonly a 28-day window) so it cannot attribute a regression to a specific deploy without waiting; only reports origins/URLs with sufficient real-world traffic (low-traffic pages may have no CrUX data at all); reports field percentiles (commonly the 75th percentile) rather than a single number, so "the field LCP" is actually a distribution, not a point value — always specify which percentile a cited number represents. Flag a claim that treats CrUX as instantly reflecting today's deploy, or as covering a low-traffic page it may not have data for.

### 4. A single Lighthouse run is not enough even as lab evidence

Lab measurement variance (system load, thermal throttling, background processes) means a single run of Lighthouse is not reliable evidence on its own — multiple runs (or a controlled tool that already runs multiple iterations and reports median/percentile) are needed before treating a lab number as stable evidence, not just a single sample.

### 5. Reconciling disagreement between lab and field

When lab says "fine" and field says "regressed" (or vice versa), the disagreement itself is the finding — investigate rather than picking whichever number supports the desired conclusion. Common causes: the lab throttling profile doesn't represent the real device/network mix (e.g. desktop-only lab testing for a mostly-mobile audience); the lab run has a warm cache the CI step didn't clear, but real first-time visitors don't; the regression is specific to a code path/interaction that the lab script's test URL/flow doesn't exercise (e.g. INP on a specific interactive element the Lighthouse run never clicks, since Lighthouse's synthetic run cannot fully simulate INP the way field RUM data — which requires observed real interactions — can).

## Minimal safe review flow

1. Identify what evidence is actually available for the claim being reviewed (lab-only, field-only, both, or neither/inference).
2. If only lab data exists for a production-impacting claim, state that field verification is outstanding as a residual risk rather than asserting a final verdict.
3. If lab and field data disagree, investigate the throttling-profile/cache-state/traffic-mix explanations above before concluding either number is "wrong."
4. Cite the specific percentile (e.g. p75) and time window for any field number, and the specific throttling/device profile for any lab number.
5. Re-verify current LCP/CLS/INP threshold cutoffs against `https://web.dev/vitals` before stating a pass/fail budget verdict — do not reuse a remembered threshold from a prior review.

## When to push back

Push back if the user says:

- "the Lighthouse score is green, ship it" — for anything with real production traffic, ask whether field data (CrUX or RUM) exists or is planned to confirm it,
- "CrUX shows we're fine" for a page/route that just shipped a change days ago — CrUX's rolling window means it may not yet reflect that change,
- "just average the field numbers" — Core Web Vitals are evaluated at specific percentiles (commonly p75), not the mean; averaging hides the tail experience that the metric is specifically designed to surface,
- "we don't need RUM, Lighthouse in CI is enough" for a production application with a real, diverse user base — lab-only measurement cannot see device/network heterogeneity that field data is specifically designed to capture.
