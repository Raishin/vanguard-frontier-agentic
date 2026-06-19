---
name: erp-crm-cutover-protocol
description: Use this skill when a Dynamics 365 ERP or CRM implementation is approaching go-live and must progress through mock migration, data reconciliation, cutover runbook execution, rollback testing, and the Success by Design go-live gate before production switchover. Orchestrates d365-data-migration-cutover-agent as primary, d365-success-by-design-governance-agent for SbD gate sign-off, and d365-security-sod-governance-agent for separation-of-duties validation. Gates include reconciliation sign-off, rollback readiness, and SbD go-live gate. Never makes the go/no-go decision autonomously; all production-impacting steps require human owner and Microsoft FastTrack confirmation where applicable.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-16"
  category: data
  lifecycle: experimental
---

# ERP / CRM Cutover Protocol

## Purpose
This skill defines the controlled sequence for taking a Dynamics 365 ERP or CRM
implementation from pre-production readiness through mock migration, data
reconciliation, runbook execution, rollback validation, and the Success by
Design (SbD) go-live gate to production go-live. It exists so that no cutover
proceeds without tested rollback paths, reconciled data, and explicit stakeholder
sign-off at each gate. It does not execute migration scripts or production
deployments; it structures the process so human owners can make well-informed
go/no-go decisions.

## When to use
- A Dynamics 365 Finance, Supply Chain Management, Customer Engagement, or
  Business Central implementation is entering the Prepare phase and a cutover
  plan must be validated.
- A mock cutover has been completed and reconciliation findings must be reviewed
  before the real cutover is authorised.
- An organisation needs to confirm SbD go-live gate criteria before requesting
  Microsoft FastTrack go-live readiness review.
- An existing production environment must be refreshed or migrated to a new
  release and cutover risk must be assessed.

## When NOT to use
- The go-live has already completed and this is post-go-live hypercare support
  — use the hypercare support runbook instead.
- The migration involves a third-party ERP not on the Dynamics 365 platform —
  this protocol is scoped to Dynamics 365.
- You need live production database or environment access to execute migration
  scripts — that requires the infrastructure owner; this protocol is
  recommendation-only.

## Participating agents
- `d365-data-migration-cutover-agent` (primary — mock migration, data reconciliation, cutover runbook, rollback plan)
- `d365-success-by-design-governance-agent` (SbD gate review, go-live readiness sign-off)
- `d365-security-sod-governance-agent` (separation of duties validation, security role sign-off)

## Inputs required
- Cutover plan document (task list, owners, timing, rollback steps)
- Data migration strategy and scripts (for review, not execution)
- Mock cutover results from sandbox environment (including error log)
- Go-live checklist status from the SbD Prepare phase
- Security role assignments and SoD analysis for go-live users
- Integration test results and external dependency sign-off

## Evidence required
- At least one successful mock cutover has been completed in a sandbox
  environment that mirrors production
- UAT has been completed and signed off by business stakeholders
- Performance testing is complete and SLAs are met
- Production environment is provisioned and matches the version in
  development/test environments
- Rollback plan has been tested and timing confirmed

## Workflow

1. **Mock cutover review** — Review mock cutover results. Identify all errors,
   timing overruns, and unresolved issues. Confirm the mock was run with
   production-representative data and all integration steps.
2. **Data reconciliation** — Validate that migrated data matches source records
   within agreed tolerance thresholds. Identify exceptions and require sign-off
   from the data owner on accepted variances.
3. **Gate 1 — Reconciliation sign-off** — Data owner and project lead must
   formally sign off that reconciliation thresholds are met. d365-data-migration-
   cutover-agent produces the reconciliation report. Do not proceed without
   this sign-off.
4. **Rollback plan validation** — Confirm the rollback plan is documented,
   tested, and timed. Confirm who has authority to invoke rollback and the
   criteria that trigger it.
5. **Gate 2 — Rollback tested** — Confirm rollback has been rehearsed in the
   sandbox. d365-data-migration-cutover-agent attests rollback is feasible
   within the cutover window. Do not proceed without this confirmation.
6. **SoD validation** — d365-security-sod-governance-agent reviews production
   security role assignments for separation-of-duties violations. Flag any
   violations for remediation before go-live.
7. **SbD go-live gate** — d365-success-by-design-governance-agent runs the SbD
   go-live readiness review against the full go-live checklist: solution scope,
   UAT sign-off, performance, integrations, data migration, change management,
   support readiness. Produce a go/conditional-go/no-go recommendation.
8. **Gate 3 — SbD go-live gate sign-off** — Project steering committee and
   Microsoft FastTrack (where applicable) must confirm the go-live gate.
   This protocol never issues the final go/no-go autonomously.
9. **Cutover execution readiness** — Confirm all people, environments, and
   communication plans are in place. Verify the cutover window is realistic
   given mock timing plus buffer.
10. **Post-cutover validation** — After go-live, confirm data accuracy in
    production, integration health, and user access. Invoke hypercare plan.

## Decision gates

| Gate | Condition | Action |
|---|---|---|
| Reconciliation sign-off | Data variances exceed agreed thresholds | Stop cutover; require data owner resolution |
| Rollback tested | Rollback not rehearsed or timing exceeds window | Stop cutover; require rollback rehearsal |
| SoD violations | Production roles contain SoD conflicts | Escalate to d365-security-sod-governance-agent; remediate before go-live |
| SbD go-live gate | Any SbD blocking issues unresolved | Stop cutover; require issue owner sign-off |
| Mock cutover errors | Critical errors unresolved from mock | Stop cutover; require second mock |

## Refusal triggers
- Stop if no successful mock cutover has been completed — do not recommend
  production go-live without at least one successful rehearsal.
- Stop if reconciliation sign-off has not been obtained from the data owner.
- Stop if the rollback plan is untested or the rollback window exceeds what
  the business can tolerate.
- Stop if SbD blocking issues are unresolved — do not substitute judgement
  for the SbD gate process.
- Stop if credentials, production org IDs, or customer data are requested
  as inputs to this protocol — refuse and escalate.

## Handoff rules
- All handoffs carry: project_id, skill_id, skill_version, invoked_by,
  gate_status, open_issues, rollback_status, sbd_verdict, do_not_do_list.
- Human go/no-go decisions are always documented with decision-maker name,
  date, and statement of accepted risk.
- SbD gate sign-off from d365-success-by-design-governance-agent is the
  minimum required before any production cutover recommendation.

## KPIs
- Mock cutover success rate (errors per rehearsal)
- Data reconciliation pass rate (% within tolerance)
- Rollback rehearsal completion (yes/no with timing)
- SbD go-live gate outcome (go / conditional-go / no-go)
- Actual vs. planned cutover window duration

## References
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-to-go-live
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-go-live-checklist
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-go-live-cutover-strategy
- https://learn.microsoft.com/dynamics365/guidance/fasttrack/go-live-workshops
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/success-by-design
