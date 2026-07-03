# Stopping Rules and Peeking

Use this reference when evaluating whether an experiment's statistical significance claim or stop/continue decision is actually valid, given how it was monitored.

## What people get wrong

The naive story is:

> "The dashboard shows p < 0.05, so the result is significant — let's ship it."

Wrong. A p-value computed under a fixed-sample-size assumption is only valid if the experiment was actually analyzed that way — once, at a pre-determined sample size or duration. Checking the dashboard daily and stopping the moment it crosses a significance threshold ("peeking") inflates the true false-positive rate far above the nominal 5%, often dramatically, because each look is an additional opportunity for noise to cross the threshold by chance.

## Officially grounded shape

There is no single universally-mandated statistical framework across experimentation platforms — some use fixed-horizon frequentist testing, some use sequential testing frameworks (e.g. always-valid p-values, mSPRT-based methods) or Bayesian frameworks specifically designed to tolerate continuous monitoring. The review's job is not to mandate one specific framework, but to verify that **the stopping behavior actually used matches the statistical framework the experiment claims to be using.** A fixed-horizon test monitored continuously and stopped early on a significant read is invalid regardless of which platform produced the number.

## Non-negotiable design rules

1. **A primary metric and a minimum detectable effect (MDE) must be pre-registered before the experiment starts.** If the "significant" result was found by scanning a dashboard of many metrics after the fact and picking whichever moved, that is not a valid experiment result — it is multiple-comparisons-inflated noise. Treat the absence of a pre-registered primary metric as a blocking finding.
2. **The required sample size/duration must be computed from the pre-registered MDE, baseline rate, and desired power *before* the experiment launches**, not decided retroactively based on how the data looks partway through.
3. **If the experiment uses a fixed-horizon test, it must not be stopped early based on an interim significant read** — continuous or frequent peeking with a fixed-horizon test invalidates the significance claim. Verify the actual monitoring cadence against the analysis method actually used.
4. **If continuous monitoring genuinely is required (e.g. to catch a severe regression fast), the experiment must use a sequential-testing-aware method** built for that purpose — confirm the platform/analysis actually implements one rather than assuming a standard p-value naturally tolerates repeated looks.
5. **A minimum runtime should be enforced even for a positive early read** to cover known cyclical effects (day-of-week traffic mix, novelty effects that fade, weekly business cycles) — a result significant after two days is not the same evidence as the same result sustained across a full business cycle.
6. **Secondary/guardrail metrics must be declared in advance too**, distinct from the primary metric, so that a regression on a guardrail metric (e.g. latency, error rate, unsubscribe rate) is caught by design rather than discovered only if someone happens to check.

## Minimal safe review flow

1. Confirm a primary metric and MDE were declared before the experiment launched — ask for the pre-registration artifact (ticket, doc, experiment-platform config) rather than accepting a post-hoc description.
2. Identify which statistical framework is actually in use (fixed-horizon frequentist, sequential/always-valid, Bayesian) from the platform's documented behavior — verify via Context7/current docs rather than assuming.
3. Check the actual monitoring/stop history: was the experiment checked once at the planned endpoint, or repeatedly with a stop-on-significance pattern? If the platform's dashboard was checked daily and the experiment stopped as soon as it crossed significance, and the underlying method is fixed-horizon, flag this as invalid regardless of the reported p-value.
4. Confirm a minimum runtime (commonly at least one to two full business cycles, e.g. two weeks, adjusted for the app's actual traffic pattern) was respected even if an early read looked positive.
5. Confirm guardrail/secondary metrics were declared and checked, not just the metric that happened to move favorably.

## Adversarial checklist

Before accepting a "statistically significant" result as ship-worthy, answer these:

- What was the pre-registered primary metric, and does the "significant" result match that metric — or is it a different metric that happened to move?
- What analysis method is this platform's significance number actually based on (fixed-horizon, sequential, Bayesian), and was the monitoring behavior (single look vs. continuous dashboard checking) consistent with that method's assumptions?
- Was there a pre-declared minimum sample size or runtime, and was the experiment stopped before, at, or after that point?
- Were guardrail metrics (latency, errors, revenue-adjacent metrics not directly targeted) checked, or only the primary metric that moved favorably?
- If the same experiment had been stopped a few days earlier or later, is there reason to believe the result would look materially different (day-of-week effects, novelty effects)?

If any of these cannot be answered, the significance claim is unverified, not confirmed.

## High-risk assumptions to kill

- "It says p < 0.05 so it's real" — without knowing the analysis method and monitoring history, a p-value alone proves nothing.
- "We checked it every day and stopped as soon as it looked good" — this is the textbook peeking failure mode, not diligence.
- "We'll just ship whichever metric moved" — post-hoc metric selection without pre-registration is multiple-comparisons noise, not a finding.

## When to push back

Push back if the user asks to:

- ship a result based on a metric that was not the pre-registered primary metric,
- stop a fixed-horizon experiment early because an interim dashboard check crossed a significance threshold,
- skip a minimum-runtime requirement because an early read "already looks clearly positive,"
- run an experiment with no declared guardrail metrics on latency, errors, or adjacent business metrics.

Those are not efficiency gains. They are how false positives become shipped product decisions.
