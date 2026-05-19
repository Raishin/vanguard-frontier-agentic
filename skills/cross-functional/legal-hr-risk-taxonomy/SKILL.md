---
name: legal-hr-risk-taxonomy
description: Use this skill to assign consistent risk labels to a Legal or HR matter — severity ratings, privilege and privacy sensitivity labels, retaliation and discrimination risk labels, matter-type classes, escalation-gate triggers, and the audit-log schema. It standardizes the vocabulary every Legal and HR agent and case capsule uses so risk is rated the same way across the ecosystem. It does not give legal or HR advice and never concludes that a matter is safe or compliant.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-18"
  category: compliance
  lifecycle: experimental
---

# Legal-HR Risk Taxonomy

## Purpose
This skill is the shared risk vocabulary for the Legal and HR agent ecosystem.
It defines the severity scale, sensitivity labels, matter-type classes,
escalation-gate triggers, and the audit-log schema, so every agent and every
case capsule rates and labels risk the same way. It does not give legal or HR
advice and never concludes that a matter is safe, compliant, or approved.

## When to use
- An agent must assign a `risk_rating` or sensitivity label to a matter.
- An agent must decide whether an escalation gate is triggered.
- A capsule or audit-log entry must be filled in with consistent labels.

## Severity scale
| Rating | Meaning |
|---|---|
| Critical | Immediate legal or regulatory exposure; do not proceed without counsel sign-off. |
| High | Material litigation, regulatory, or financial exposure; escalation strongly indicated. |
| Medium | Manageable with documented controls; monitor and document. |
| Low | Limited exposure on current evidence; note and monitor. |
| Unknown | Jurisdiction or material facts missing; cannot rate. Mandatory when documentation is incomplete. |

`Unknown` is mandatory, not a fallback. An agent never upgrades a matter to a
ratable severity to avoid an escalation.

## Sensitivity labels
- `privilege_sensitivity`: `none` / `possible` / `likely-privileged`.
- `privacy_sensitivity`: `low` / `moderate` / `high` / `special-category`.
- `retaliation_risk`, `discrimination_or_harassment_risk`, `regulatory_risk`:
  `none-observed` / `possible` / `elevated` / `unknown`.
- `litigation_hold_needed`: `no` / `recommended` / `yes` / `unknown`.

## Escalation-grade matter types
The following are escalation-grade by default — they always reach a qualified
human owner regardless of severity rating: harassment, discrimination,
retaliation, whistleblower, workplace safety, wage/hour, worker classification,
union/labor, immigration, medical leave, disability accommodation, pay equity,
executive misconduct, mass layoff or reorganization, employee data breach, and
litigation-hold or discovery matters.

## Escalation gates
A matter must be paused and escalated when any gate is true:
- The matter is an escalation-grade matter type (above).
- A claim, complaint, charge, grievance, or subpoena has been filed or
  threatened.
- Protected activity, protected characteristics, or whistleblower status are in
  play.
- Attorney-client privilege or work-product protection may be implicated.
- Financial or reputational exposure is material.
- A board, audit-committee, or regulatory-reporting trigger may apply.
- The matter crosses Legal and HR and no documented controls exist.
- Legal and HR agents disagree.

See [references/risk-labels.md](references/risk-labels.md) for the full
`matter_type` value list and the audit-log schema.

## Audit-log schema
Every handoff and escalation produces one audit-log event with the minimum
necessary fields: `event_id`, `case_id`, `timestamp`, `initiating_agent`,
`receiving_agent`, `human_owner`, `matter_type`, `risk_rating`,
`escalation_status`, `data_sensitivity`, `privilege_sensitivity`,
`action_recommended`, `action_prohibited`, `evidence_summary`,
`open_questions`, `decision_status`, `retention_category`. Field rules are in
the reference file.

## References
- [Risk labels and audit-log schema](references/risk-labels.md) — full
  `matter_type` enumeration, label definitions, and audit-log field contract.

## Security notes
- A rating is an opinion on exposure, never a clearance. Never record "this is
  compliant" or "safe to proceed"; use the severity scale only.
- The audit log is minimum-necessary. It carries labels and summaries, never
  raw medical, privileged, credential, or protected-class content.
- When facts are missing, rate `Unknown` and trigger the escalation gate rather
  than guessing a lower severity.
