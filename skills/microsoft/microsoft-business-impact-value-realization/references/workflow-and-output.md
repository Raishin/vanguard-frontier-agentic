# Workflow and output contract

Use this reference only when performing the full Microsoft value-realization review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- License-to-value: assigned vs. active licenses, whitespace, inactive/under-utilized seats, SKU fit and downgrade/upgrade candidates
- Adoption measurement: Adoption Score, AI adoption score, Microsoft 365 Apps usage reports, readiness report
- Copilot value: Copilot Control System measurement/reporting, Copilot Analytics, Copilot Dashboard (assisted hours/value), business value/ROI reporting
- Rollout instrumentation: pilot → deploy → operate phases, early-adopter champions, defined success criteria
- Executive framing: leading vs. lagging indicators, baseline, target, kill criteria, cost-per-active-user
- FastTrack alignment: which adoption activities are in-scope vs. out-of-scope (e.g., change management, custom reports)

## Safe workflow

1. **Frame scope**
   - Product(s) in scope (Microsoft 365 apps, Copilot, Viva, specific SKUs):
   - Required outcome (license rationalization / adoption lift / ROI reporting / rollout plan):
   - Available evidence (admin center usage reports, Adoption Score, Copilot Analytics):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer Microsoft 365 admin center usage/readiness reports, Adoption Score, Copilot Dashboard exports.
   - Note metric latency (up to ~72h) and any documented known-issue windows before quoting figures.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test value**
   - Which assigned licenses are inactive (waste)?
   - Is the rollout instrumented with a baseline and target, or is "value" asserted without measurement?
   - Are success criteria defined before deployment, or retrofitted?
   - What is the cost-per-active-user, and is it trending the right way?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest measurable action**
   - Tie every recommendation to an indicator with a baseline, target, and kill criterion.
   - Recommend reclaiming or reassigning inactive licenses before buying more.
   - Never make a purchase commitment or guarantee a savings figure.

## Output contract

Return this structure:

```markdown
# Microsoft Value Realization Review: <scope>
## Executive verdict
- Status: ON TRACK / AT RISK / UNDERPERFORMING / NEEDS EVIDENCE
- Biggest value gap:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Business impact | Minimum measurable action |
|---|---|---|---|---|
## Recommended actions
1. <action> — indicator: <metric>, baseline: <x>, target: <y>, kill criteria: <z>
## Validation
- Reports or checks to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
