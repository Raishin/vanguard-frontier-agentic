# Azure Cost Budget and Quota Operations

Use this reference for current, source-grounded service behavior and the hard live-operation gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Assuming a budget alert stops Azure spend.
- Approving GPU/HPC quota because current spend is low, while ignoring delayed cost data.
- Creating action groups with no owner or runbook.
- Raising budget thresholds without financial authority and expiry.
- Using quota increase as capacity planning rather than explicit spend-risk acceptance.

## Officially grounded service shape

Microsoft Learn evidence says Cost Management budgets monitor spending and trigger notifications or action groups, but budget alerts do not stop resources or consumption. Cost and usage data is typically delayed, and budgets are evaluated on a schedule. Budgeting guidance recommends alerts, anomaly detection, stakeholder ownership, and automation only with clear accountable response paths.

- Budgets can be scoped and filtered, and can alert on actual or forecast cost.
- Budget alerts notify stakeholders and can integrate action groups at supported scopes.
- Cost data is not real time; budget evaluations lag consumption.
- Quota controls capacity but is not a budget by itself.
- Emergency spend-stop actions can be disruptive and need owner approval and rollback criteria.

## Non-negotiable design rules

- Confirm billing/subscription/resource-group scope and financial owner before changes.
- State that budgets do not stop consumption unless paired with separate approved automation.
- Review actual, forecast, data freshness, threshold, recipients, action groups, and anomaly coverage.
- Treat GPU/HPC quota and high-cost scale-up as high-risk financial mutations.
- Prefer lower-risk alerts and temporary quotas before permanent threshold increases.

## Minimal safe implementation flow

- Scope cost owner, subscription/resource group, workload, SKU, quota, and budget action.
- Collect current budget definitions, alert thresholds, action groups, current/forecast costs, data freshness, and quota state.
- Classify proposed change as alert-only, automation, quota increase, deallocation, or spend-stop.
- Gate mutation on explicit financial approval and rollback/restore plan.
- Verify alert/action configuration and document residual spend risk.

## Safe verification targets

- Budget thresholds and recipients match accountable stakeholders.
- Action group behavior and automation runbooks are tested and reversible where possible.
- Cost data freshness and forecast uncertainty are explicitly stated.
- Quota increase request has business justification, expiry/review, and spend ceiling.
- Emergency deallocation or quota reduction has owner approval and service-impact assessment.

## When to push back

- The user treats budget alerting as a hard spending stop.
- Financial approval is missing or vague.
- High-cost quota request lacks workload, duration, and maximum-spend boundary.
- Automation could deallocate production without an incident commander and rollback plan.
