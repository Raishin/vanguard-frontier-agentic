# Operations Briefing Signal Quality Guide

Use this reference when preparing AWS daily or weekly operations briefings from CloudWatch, AWS Health, Trusted Advisor, cost, deployment, incident, and backlog signals.

## What people get wrong

The lazy story is:

> Summarize dashboards and call it an ops brief.

Wrong. A briefing is an evidence-weighted decision artifact. Dashboards show slices of reality, not ownership, impact, trend, or actionability.

Common bad assumptions:

- No active alarms means no operational risk.
- AWS Health events automatically map to impacted workloads.
- Trusted Advisor checks are always current and universally available.
- Cost spikes are operational incidents without business context.
- Deployment success means customer impact is zero.
- Open tickets can be summarized without owner, age, severity, and blocker state.

## Briefing-specific failure modes

- Mixing confirmed incidents, unverified alerts, and stale backlog as equal-priority items.
- Reporting alarm counts without affected service, owner, severity, duration, and customer/business impact.
- Ignoring deployment timeline around incident onset.
- Presenting cost variance without service, account, usage type, tag, or commitment context.
- Omitting AWS Health or support-case context during regional/service disruption.
- Producing action items without single owners or deadlines.

## Minimum safe workflow

1. Define audience and time window: executive daily, engineering handoff, weekly risk review, or incident standup.
2. Gather read-only signals: alarms, incidents, AWS Health, deployments, cost deltas, Trusted Advisor, open tickets, and known risks.
3. Label each item as confirmed, sampled, stale, inferred, or missing evidence.
4. Rank by customer impact, security/compliance risk, operational urgency, and business cost.
5. Convert noise into actions: owner, next step, due time, blocker, and escalation path.
6. Separate “watch” items from “act now” items.
7. State blind spots explicitly; do not imply the account is healthy from narrow evidence.

## Verification targets

- CloudWatch alarms and metric trends for the reporting window
- AWS Health events and account/organization visibility scope
- Trusted Advisor check status and support-plan/access limitations
- Cost Explorer or Budgets variance by service/account/tag/usage type
- deployment and change timeline from pipeline, change calendar, or release notes
- incident records, OpsItems, support cases, tickets, and unresolved postmortem actions
- stale data markers: last updated time, Region/account coverage, and missing telemetry

## When to push back

Push back if the user asks to:

- claim “all clear” from one dashboard or Region sample
- hide uncertainty, stale data, or missing account coverage
- turn briefing coordination into live remediation
- publish raw sensitive evidence in a broad audience brief
- assign action items without owners or deadlines
- merge security, cost, availability, and backlog risk into one vague priority
