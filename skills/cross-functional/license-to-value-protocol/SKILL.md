---
name: license-to-value-protocol
description: Use this skill when Microsoft 365 or Dynamics 365 licenses have been assigned and the organisation needs to measure adoption, instrument value outcomes, identify waste, and reclaim inactive licenses before purchasing more. Orchestrates microsoft-business-impact-value-realization-agent as primary. Gates include baseline definition before adoption target is set, and inactive-license reclaim before any new license purchase is recommended. Does not make purchase decisions or modify license assignments autonomously. Never requests credentials, tenant IDs, or customer data; production-impacting steps escalate to the licence owner or IT procurement team.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-16"
  category: finance
  lifecycle: experimental
---

# License to Value Protocol

## Purpose
This skill defines how an organisation moves from license assignment through
adoption instrumentation, value measurement, and proactive waste reclaim. It
exists so licence investment is justified by measurable adoption outcomes, and
so inactive or redundant licenses are identified and reclaimed before additional
spend is approved. It does not make procurement decisions; it produces
structured evidence that finance, IT, and business owners use to make
well-informed decisions.

## When to use
- A Microsoft 365 or Dynamics 365 licence cohort has been deployed and adoption
  measurement has not yet been established.
- A periodic licence review is due and inactive-licence analysis is required.
- Leadership needs a value realisation report to justify renewal or expansion.
- A business unit has requested additional licences and the existing estate must
  be reviewed for reclaim opportunities before purchase.
- Adoption Score or Microsoft Viva Insights signals indicate low utilisation
  in a licensed cohort.

## When NOT to use
- The matter is a licensing compliance audit with legal implications — route to
  your software asset management (SAM) team and legal counsel.
- The organisation has no telemetry access (Adoption Score, usage reports) —
  prerequisites are not met; this protocol cannot produce evidence-based findings.
- You need to execute license assignment changes in the Microsoft 365 admin
  center — that requires the license owner; this protocol is recommendation-only.
- The matter is a cost-management optimisation across non-Microsoft cloud spend
  — use the appropriate FinOps protocol.

## Participating agents
- `microsoft-business-impact-value-realization-agent` (primary — adoption instrumentation, value measurement, inactive-licence identification, reclaim recommendation)

## Inputs required
- Microsoft 365 Adoption Score (current score and trend, per category including
  AI adoption if Microsoft 365 Copilot is licensed)
- Microsoft 365 Copilot usage report (if applicable)
- Assigned licence count vs. active user count per SKU
- Business outcomes or KPIs the licence cohort was intended to drive
- Inactive user threshold (days without activity) agreed with the business

## Evidence required
- Microsoft 365 admin center access is available to the licence owner
- Adoption Score reporting is enabled (not in GCC High/GCC/DOD tenants where
  Adoption Score is unavailable)
- Usage reports are enabled and privacy controls allow aggregate analysis
- An agreed inactive-user threshold exists (e.g. 30 days without any activity)
- A licence owner (IT or procurement contact) is identified

## Workflow

1. **Baseline definition** — Establish the adoption baseline: current Adoption
   Score per category, licensed user count, active user count, and the business
   KPIs the deployment was intended to drive. Document the baseline date.
2. **Gate 1 — Baseline defined** — Do not set adoption targets until a baseline
   exists. microsoft-business-impact-value-realization-agent attests baseline
   completeness. If baseline data is unavailable, stop and require the licence
   owner to enable reporting before proceeding.
3. **Adoption target setting** — Based on the baseline and peer benchmarks
   (Adoption Score peer benchmark), set time-bound adoption targets per user
   cohort and per value category (Communication, Meetings, Content
   Collaboration, AI Adoption, etc.).
4. **Value instrumentation** — Map Adoption Score signals and Copilot usage
   metrics to the organisation's stated business outcomes. Identify which
   metrics are leading indicators of value (e.g. AI Adoption Score ≥ 66 signals
   users on track for three-day weekly habit).
5. **Inactive licence identification** — Using usage reports and agreed
   inactive-user thresholds, identify users who have not performed any licensed
   activity within the threshold period. Segment by SKU, department, and
   licence tier.
6. **Gate 2 — Inactive-licence reclaim before purchase** — Before recommending
   any new licence purchase, confirm that inactive licences have been reviewed
   and a reclaim decision has been made by the licence owner. Do not recommend
   purchase without this gate.
7. **Reclaim recommendation** — For inactive licences: recommend suspension,
   downgrade, or reassignment. Provide estimated cost saving. Route recommendation
   to licence owner for confirmation.
8. **Value realisation report** — Produce a value realisation summary: adoption
   progress vs. targets, value category scores, estimated productivity impact,
   reclaim savings, and recommended next actions.
9. **Renewal / expansion recommendation** — If adoption targets are met and
   inactive licences are remediated, produce a data-supported expansion or
   renewal recommendation. If adoption targets are not met, recommend an
   adoption intervention before expansion.
10. **Human confirmation** — Route all purchase, reclaim, and adoption
    intervention recommendations to the licence owner, IT procurement, and
    finance team. This protocol never executes licence changes autonomously.

## Decision gates

| Gate | Condition | Action |
|---|---|---|
| Baseline defined | No adoption baseline exists | Stop; require licence owner to enable reporting |
| Inactive-licence reclaim | Inactive licences not reviewed before purchase request | Block purchase recommendation; complete reclaim review first |
| Adoption target not met | Adoption Score below peer benchmark with no improvement trend | Recommend adoption intervention before expansion |
| Reporting unavailable | Adoption Score or usage reports disabled | Stop; flag as prerequisite gap to licence owner |

## Refusal triggers
- Stop if adoption baseline data does not exist — do not produce value
  recommendations based on anecdote or assumption.
- Stop if a purchase recommendation is requested without first completing the
  inactive-licence reclaim gate.
- Stop if credentials, tenant IDs, or individual-level user data are requested
  — this protocol works with aggregate and anonymised signals only.
- Stop if the organisation is in a tenant type where Adoption Score is
  unavailable (GCC High, GCC, DoD) without an alternative telemetry source
  agreed with the licence owner.

## Handoff rules
- All handoffs carry: licence_cohort_scope, skill_id, skill_version,
  invoked_by, baseline_date, gate_status, open_questions, do_not_do_list.
- Reclaim recommendations always include the inactive threshold used, the
  review date, and the licence owner who confirmed the threshold.
- Purchase recommendations are never produced without the inactive-reclaim gate
  sign-off attached.

## KPIs
- Microsoft 365 Adoption Score (overall and per category)
- Copilot AI Adoption Score (if Microsoft 365 Copilot licensed)
- Active user rate (active / assigned licences per SKU)
- Inactive licence count and estimated annualised waste
- Time from licence assignment to adoption baseline establishment

## References
- https://learn.microsoft.com/microsoft-365/admin/adoption/adoption-score
- https://learn.microsoft.com/microsoft-365/admin/adoption/ai-adoption-score
- https://learn.microsoft.com/microsoft-365/copilot/microsoft-365-copilot-licensing
- https://learn.microsoft.com/microsoft-365/admin/activity-reports/microsoft-365-copilot-usage
- https://learn.microsoft.com/microsoft-365/copilot/microsoft-365-copilot-enablement-resources
