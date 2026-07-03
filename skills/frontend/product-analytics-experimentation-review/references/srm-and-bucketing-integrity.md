# SRM and Bucketing Integrity

Use this reference when auditing user-assignment/bucketing logic for an A/B or multivariate experiment, or when diagnosing a suspected sample-ratio mismatch (SRM).

## What people get wrong

The naive story is:

> "We split traffic 50/50 in the config, so the experiment is randomized correctly."

Wrong. A correct config value does not guarantee correct *realized* assignment. SRM — where the observed split between variants diverges from the configured split by more than chance — is one of the most common silent killers of experiment validity, and it is caused by implementation bugs, not by the experimentation platform's randomization algorithm.

## Common root causes of SRM

- **Non-deterministic bucketing**: assignment computed from `Math.random()` or an unseeded RNG on each page load/render instead of a stable hash of a persistent identifier (user ID, device ID, or a consistently-stored anonymous ID). This causes the same user to bounce between variants across requests, corrupting both the split and the per-user experience.
- **Redirect/loading-time asymmetry**: one variant triggers a client-side redirect or has a slower paint path, so users on slow connections/devices disproportionately bounce before the exposure event fires, undercounting that variant.
- **Bot/crawler contamination**: bot traffic is not filtered before assignment and is unevenly distributed across variants (e.g. one variant's URL gets crawled more).
- **Caching bypass**: a CDN or browser cache serves a cached response for one variant to users who should have been freshly assigned, without going through the bucketing code path at all.
- **Post-assignment filtering that isn't symmetric**: an error-handling or feature-guard path silently excludes users from one variant's exposure logging but not the other's (e.g. a try/catch around only the treatment code path).
- **Multiple exposure events per user**: a user is bucketed once but the exposure/"experiment viewed" event fires multiple times (e.g. once per re-render), inflating one variant's logged count without inflating actual unique users.

## Non-negotiable design rules

1. **Assignment must be deterministic per stable identifier.** Given the same user identifier and experiment ID, the bucketing function must return the same variant every time, computed via a consistent hash (not re-rolled per request). If the review cannot point to the exact hash/seed input, treat this as a blocking finding.
2. **The identifier used for bucketing must survive the user's session.** An identifier that resets on page reload, tab close, or unauthenticated-to-authenticated transition will fragment the same real user across variants — verify what the identifier actually is (cookie, local storage, logged-in user ID) and its persistence.
3. **The exposure/assignment log event must fire exactly once per unique assigned user**, at the point the user is committed to see the variant-specific experience — not on every render, not before the variant is actually applied, and not skipped on an error path.
4. **Bot and internal-traffic filtering must run before or as part of assignment**, and must be applied identically to all variants — an asymmetric filter is itself an SRM source.
5. **The chi-squared goodness-of-fit test is the standard tool for detecting SRM** on the ratio between logged variant counts and the configured allocation ratio; a low p-value (commonly p < 0.001 is used as a conservative SRM-alert threshold given how much SRM checks are run) on that test at meaningful sample volume is a signal to halt and debug before trusting any of the experiment's results, not something to explain away as "probably fine."

## Minimal safe review flow

1. Identify the exact code path that computes variant assignment. Confirm it is a pure function of `(stable_user_id, experiment_id)` via a documented hash — not `Math.random()`, not time-based, not re-evaluated per render.
2. Confirm the identifier's lifetime matches or exceeds the experiment's intended duration and survives the user journeys the experiment covers (e.g. anonymous → logged-in transition, cross-device if applicable).
3. Trace the exposure-logging call site: confirm it fires once per assigned user, only after the variant is actually rendered/applied (not merely "assigned" in memory before an early return/error).
4. Check for asymmetric guards: any `try/catch`, feature flag, cache rule, or redirect that could apply to one variant's code path but not the equivalent point in the other variant's path.
5. If actual counts are available, compute the chi-squared statistic against the configured ratio; treat a materially significant deviation as a blocking finding requiring root-cause before results are trusted.

## Adversarial checklist

Before approving an experiment's bucketing as sound, answer these:

- What exact value is hashed to produce the variant assignment, and where is it read from?
- Does that value exist and stay stable before the user has logged in, accepted cookies, or completed onboarding?
- Is there any code path (error handler, feature gate, redirect, cache rule) that can prevent the exposure event from firing for one variant but not the other?
- If a user's assignment identifier changes mid-experiment (e.g. anon ID merges into logged-in ID), what happens to their variant — do they get reassigned, and if so, does that reassignment apply identically across variants?
- Has anyone actually looked at the realized split of exposure-event counts, or is "50/50" only ever been checked in the config file?

If any of these cannot be answered, the SRM risk is unverified, not absent.

## High-risk assumptions to kill

- "The experimentation platform's randomization is trustworthy, so the split will be fine" — the platform's RNG is rarely the source of real-world SRM; the surrounding application code is.
- "We looked at the config and it says 50/50" — config intent is not realized assignment; only logged exposure counts prove realized assignment.
- "A small split deviation doesn't matter" — at low absolute counts a deviation may be noise, but the correct response is to run the chi-squared test at the actual observed volume, not to eyeball it.

## When to push back

Push back if the user asks to:

- launch an experiment where bucketing logic cannot be traced to a deterministic, persistent-identifier hash,
- treat an experiment's result as valid without ever having checked the realized split against the configured ratio,
- ship an experiment where the exposure-logging event fires before the variant is actually applied (this inflates "assigned" counts independent of any real exposure).

Those are not shortcuts; they are how invalid experiment results ship as product decisions.
