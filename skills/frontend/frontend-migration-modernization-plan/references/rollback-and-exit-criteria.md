# Rollback and Exit Criteria

Use this reference when defining or auditing what makes a migration phase safe to promote, and what makes legacy code eligible for decommission. Every phase in a migration plan produced by this skill must satisfy the rules below — a phase with a date-only gate and no measurable exit criteria is not a real gate.

## What people get wrong

The common bad assumption is:

> "We'll know it's done when it feels stable / when the sprint is over."

That is not an exit criterion, it is a shrug. A migration plan that cannot say, in advance, exactly what evidence promotes a phase and exactly what action reverts it is not a plan — it is hope with a Gantt chart.

## Non-negotiable design rules

### 1. Every phase needs a rollback action completable within one deploy cycle

Not "we could revert the commit eventually." A concrete, rehearsed action: flip a feature flag, revert a route-table entry (see `strangler-boundary-design.md`), or redirect the fallback proxy back to the legacy origin. If the rollback action requires a data migration to reverse, it is not a one-deploy-cycle rollback — flag that phase as higher risk and require an explicit sign-off before it ships, not an assumption that "we probably won't need to roll back."

### 2. Every phase needs a measurable exit metric, not a date

Acceptable exit metrics (pick what's relevant to the phase, be specific, not generic):

- **Correctness parity**: error rate / exception rate on the migrated surface within an agreed delta of the legacy baseline over a stated observation window (e.g. 7 days of production traffic, not "looked fine in staging").
- **Performance budget**: a named Core Web Vitals or custom timing metric within budget on real user traffic (field data), not lab-only Lighthouse runs. State the budget number and the percentile (e.g. p75 LCP) before the phase starts, not after.
- **Accessibility parity**: no WCAG 2.2 AA regressions introduced on the migrated surface versus the legacy baseline — verified, not assumed, because visual parity does not imply a11y parity (a modern component can look identical and still break keyboard nav or screen-reader semantics).
- **Functional coverage**: percentage of legacy behavior with an automated test (unit/E2E) proving parity before the legacy path is removed for that surface. "It looks right when I clicked around" is not functional coverage.

A phase gate that only says "ship by end of quarter" with no metric above attached is incomplete — say so explicitly rather than accepting a date-only gate.

### 3. Rollback and forward-progress must not both depend on the same broken assumption

If the rollback path assumes the legacy stack is still fully deployable and the forward path assumes the legacy stack can be safely decommissioned, decide explicitly which is true at each point in time. Common failure: teams decommission legacy build infrastructure or dependencies "since we're mostly migrated" while the rollback plan still assumes that infrastructure exists. State exactly when the legacy stack becomes non-reinstatable, and do not cross that line until the exit criteria for full decommission (below) are met.

### 4. The legacy-decommission step is mandatory and time-boxed

A migration plan that ends at "new code is live in production alongside the old code, indefinitely" is not finished — it has produced a permanent second stack to maintain, patch for security, and onboard new engineers into. Every plan produced by this skill must include an explicit decommission phase with:

- a stated time-box (a date or a trigger condition — e.g. "30 days after the correctness-parity metric holds"),
- the specific artifacts being removed (routes, build config, dependencies, feature flags, dual-stack CI jobs),
- a final rollback-of-last-resort note: once legacy code/dependencies are actually deleted (not just unrouted), rollback is no longer a redeploy — it is a restore from version control and a re-provisioning effort. State this transition point explicitly so nobody assumes rollback stays cheap forever.

### 5. Security/auth coexistence gates are checkpoints, not footnotes

If any phase requires legacy and modern stacks to share auth/session state (see `strangler-boundary-design.md`, route-table seam), that phase's exit criteria must include an explicit security review sign-off as a blocking gate — not a "should be fine" assumption. Do not let a convenience shortcut ("just share the cookie") skip this gate.

## Minimal safe phase-gate template

For each phase, state:

1. **Entry criteria** — what must be true before this phase starts (e.g. prior phase's exit metric held for N days).
2. **Rollback action** — the specific, one-deploy-cycle-or-less action, and who/what triggers it (automatic alert threshold vs. manual call).
3. **Exit metric(s)** — the specific measurable target(s) from the categories above, with number, percentile/window, and data source (field vs. lab, explicitly labeled).
4. **Decommission artifacts** (only for the final phase(s)) — exactly what gets deleted and the time-box/trigger for deletion.

## Adversarial checklist

Before finalizing a phase gate, answer these:

- If this phase fails its exit metric, what is the exact command/flag/route change that reverts it, and has anyone rehearsed it?
- What happens to in-flight user sessions or in-progress transactions at the moment of rollback?
- Is the exit metric based on field data (real user traffic) or only lab data (synthetic runs)? If only lab, say so — it does not prove field parity.
- Does any exit metric assume a sample size / observation window long enough to be statistically meaningful, or is it "looked fine after a day"?
- Who has authority to declare the decommission time-box met, and what happens if the trigger condition never fires (permanent stall)?

If you cannot answer these for a given phase, the phase gate is not ready to ship — say so rather than presenting an incomplete gate as final.

## When to push back

Push back if the user proposes:

- "We'll decide the rollback plan if we need it" — rollback design is not optional and not deferrable to incident time.
- "Let's skip the decommission step, we can clean up later" — "later" is how dual-stack maintenance becomes permanent.
- "The exit criteria is just 'ship it and see'" — that is not a gate, it is an unmonitored rollout.

Those are not pragmatic shortcuts. They are how a bounded migration plan turns into indefinite dual-stack technical debt.
