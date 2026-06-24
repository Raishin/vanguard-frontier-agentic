# Revenue Operations Domain Guide

Use this reference for Dynamics 365 Sales revenue operations failure modes, safe review workflow, verification targets, and pushback criteria.

## What people get wrong

The lazy story is:

> If opportunities are in the pipeline and sellers are active, forecasting will be accurate.

Wrong. CRM pipeline accuracy depends entirely on data discipline: close dates must be current, forecast categories must be correctly assigned, and probabilities must reflect real deal health rather than defaults. A pipeline full of stale, never-touched opportunities produces confident but meaningless forecast numbers.

Common bad assumptions:

- The default opportunity probability from the sales stage is accurate enough for forecasting.
- Sellers updating their own forecast adjustments is a reliable substitute for a structured forecast review cadence.
- A high pipeline volume means the forecast is healthy.
- Sales accelerator sequences are in use just because they are configured.
- CRM data hygiene is a reporting problem, not a forecasting problem.
- Predictive opportunity scoring replaces the need for a human pipeline review.

## Revenue operations failure modes

- Opportunities remain open past their close date with no update, inflating pipeline and distorting forecast-period accuracy.
- Forecast categories (Pipeline, Best Case, Committed) are not consistently defined and trained across the team, leading to sellers submitting "Committed" with very different conviction levels.
- Premium AI forecasting is enabled but not calibrated to the organization's historical win patterns, producing AI predictions that diverge from manager expectations.
- Sales accelerator sequences are configured for one scenario type but applied broadly, creating irrelevant activity suggestions that sellers ignore.
- Lead scoring models are trained on insufficient historical data, producing scores that do not correlate with actual conversion rates.
- Duplicate accounts and contacts degrade relationship intelligence and cause Copilot for Sales to surface stale or incorrect context during customer interactions.
- Seller activity data is sparse because sellers work through email or phone outside of Dynamics 365, leaving the CRM without signal for AI models.

## High-risk revenue leakage patterns

- Opportunities stalled in mid-funnel stages (Develop, Propose) for longer than the average sales cycle with no recent seller activity.
- Opportunities closing in the current quarter with no activity recorded in the last 14 days.
- Leads assigned to sellers with a response time exceeding 24 hours (research shows response rates drop sharply after the first hour).
- Forecast category "Committed" applied to opportunities with predictive scores below 40 (high overcommit risk).
- Sequences with step completion rates below 50% (sellers are skipping steps or disengaging).
- Accounts with no contact activity in 90+ days in expansion or renewal pipeline.

## Minimum safe review workflow

1. Confirm the scope: teams, territories, fiscal period, and primary concern (pipeline trust, forecast accuracy, hygiene, seller productivity).
2. Export or review the opportunity pipeline view filtered by close date, stage, and last activity date.
3. Review the forecast grid for the current period — compare Pipeline, Best Case, and Committed columns against quota.
4. Check sales accelerator work list metrics — step completion rates, skip rates, and overdue activities.
5. Review predictive opportunity scores distribution — flag high-value opportunities with low scores.
6. Identify the top three pipeline hygiene gaps (stale close dates, missing required fields, duplicate records).
7. Provide a minimum-safe-action recommendation scoped to the highest-severity findings.
8. Require live-guard escalation for any production configuration change.

## Verification targets

- Pipeline currency: percentage of open opportunities with a close date in the past or with no activity in 30+ days
- Forecast category discipline: percentage of "Committed" opportunities with predictive score above 60
- Sequence health: step completion rate, skip rate, overdue activity count
- Lead responsiveness: median lead response time by seller and team
- CRM data completeness: required field completion rate on opportunity and contact records
- Predictive scoring adoption: percentage of opportunities with a score assigned and viewed by the seller
- Copilot for Sales usage: percentage of sellers using AI-generated summaries, email drafts, and meeting briefs

## When to push back

Push back if the user asks to:

- accept a pipeline report as accurate without confirming close dates and last activity dates are current
- approve a committed forecast number from a seller without reviewing the underlying opportunity evidence
- bulk-update close dates or forecast categories in production without a documented review and rollback plan
- treat sales accelerator sequence configuration as complete without validating step completion rates
- make production forecast configuration changes (column definitions, hierarchy changes, quota resets) without live-guard escalation and explicit human approval
- use predictive scores as the sole basis for opportunity go/no-go decisions without human judgment
