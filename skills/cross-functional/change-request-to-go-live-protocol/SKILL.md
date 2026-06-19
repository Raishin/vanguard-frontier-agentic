---
name: change-request-to-go-live-protocol
description: Use this skill when a Dynamics 365 change request must be structured and progressed through impact assessment, fit-gap analysis, UAT sign-off, go/no-go decision, go-live execution, and hypercare using the Success by Design framework. Defines the full change-to-go-live flow — change request intake, solution blueprint alignment, testing gates, cutover plan verification, go/no-go decision, deployment, and hypercare. Does not authorize go-live, cutover execution, or production deployments; all production-impacting actions require human approval from the project sponsor, release manager, and Microsoft FastTrack for Dynamics 365 team (where engaged). Does not replace a qualified Dynamics 365 architect or implementation partner.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-16"
  category: delivery
  lifecycle: experimental
---

# Change Request to Go-Live Protocol

## Purpose
This skill defines how a Dynamics 365 change request is taken from intake through impact assessment, fit-gap analysis, UAT sign-off, go/no-go decision, go-live execution, and hypercare, aligned to the Microsoft Success by Design framework. It applies the Success by Design gate structure — Solution Blueprint Review, Implementation Reviews, and Go-live Readiness Review — to ensure that only well-tested, stakeholder-approved solutions reach production. No agent authorizes go-live, cutover execution, or production deployments; those are human decisions by the project sponsor, release manager, and (where engaged) the Microsoft FastTrack for Dynamics 365 team.

## When to use
- A new Dynamics 365 change request (feature, configuration, integration, or data migration) must be assessed for impact before development or deployment.
- A solution is approaching the Prepare phase and a go-live readiness gate review is needed.
- UAT sign-off or go/no-go criteria must be structured and tracked.
- A cutover plan must be reviewed for completeness before a go-live window.
- A hypercare plan must be confirmed and a support owner identified before go-live.

## When NOT to use
- The change is a minor hotfix that does not require a full Solution Blueprint Review — use the environment-to-production-release-protocol with expedited gates.
- The implementation is not a Dynamics 365 project — this protocol is specifically aligned to the Dynamics 365 Success by Design framework.
- The go-live has already occurred and the project is in the Operate phase — escalate post-go-live issues to the hypercare support owner.
- The matter involves a regulatory or contractual commitment that requires legal review before the change can proceed — escalate to legal counsel first.

## Participating agents
- `d365-success-by-design-governance-agent` — primary: assesses Solution Blueprint Review readiness, go-live readiness checklist completion, and Success by Design gate compliance
- `d365-data-migration-cutover-agent` — secondary: assesses data migration plan completeness, cutover plan, mock go-live execution, and rollback readiness

## Inputs required
- Change request description and scope (feature, configuration change, integration, or data migration)
- Current Success by Design phase (Discover, Initiate, Implement, Prepare, Operate)
- Solution Blueprint Review findings (if available)
- Target go-live date and cutover window
- Testing status (SIT, UAT, performance testing)
- Open issues list from prior reviews

## Evidence required
- Solution Blueprint Review outcomes (risks, recommendations, open items)
- System integration testing (SIT) results
- User acceptance testing (UAT) results and sign-off status
- Performance testing results
- Data migration plan and mock cutover results
- Go-live readiness checklist completion status
- Production support plan and hypercare plan

## Workflow

1. **Intake and phase alignment** — receive change request; confirm current Success by Design phase; confirm whether a Solution Blueprint Review has been completed.
2. **Impact and fit-gap assessment** — assess the functional and non-functional impact of the change against the approved solution design; identify fit-gap items (gaps between standard Dynamics 365 functionality and business requirements that require customization).
3. **Solution Blueprint Review gate (if not yet completed)** — confirm Solution Blueprint Review has been conducted; review findings and open risks; confirm that Implementation Reviews for critical areas (data model, security, integration, ALM, testing strategy) are scheduled or completed.
4. **Testing gate: SIT and performance** — confirm system integration testing and performance testing are complete; review results; flag any critical or blocking defects.
5. **Testing gate: UAT sign-off** — confirm UAT is complete and business stakeholders have signed off; flag any open UAT defects; confirm UAT sign-off authority has formal sign-off on record.
6. **Cutover plan verification** — confirm cutover plan is documented, includes task owners, durations, and dependencies; confirm mock go-live has been executed or is scheduled; confirm rollback criteria and rollback steps are defined.
7. **Escalation gate: impact and fit-gap** — if critical fit-gap items are unresolved or the solution design has material gaps, pause and escalate to the project sponsor and Dynamics 365 architect before proceeding.
8. **Escalation gate: UAT sign-off** — if UAT sign-off is not obtained or critical UAT defects are open, stop and escalate to the project sponsor; do not proceed to go/no-go without UAT sign-off.
9. **Go-live readiness review** — conduct or confirm completion of the Go-live Readiness Review (Success by Design mandatory review); assess go-live readiness against the checklist: solution acceptance, user training, performance, integrations, code management, configuration management, blocking issues, cutover plan, risk mitigation, and support plan.
10. **Escalation gate: go/no-go decision** — require explicit human go/no-go decision from project sponsor and release manager; record decision with timestamp and decision owner reference; do not initiate cutover without a confirmed Go decision.
11. **Go-live and cutover execution** — confirm production deployment and data migration cutover execution per the approved cutover plan; monitor go-live status; record each cutover task completion.
12. **Escalation gate: rollback** — if a blocking issue is detected during go-live that meets rollback criteria, stop and escalate to project sponsor and release manager for rollback decision; do not continue go-live over a rollback-trigger threshold without explicit authorization.
13. **Hypercare** — confirm hypercare period is active; confirm hypercare support owner and escalation path; schedule post-go-live review; track and resolve post-go-live issues.

## Decision gates

| Gate | Condition | Action |
|---|---|---|
| Impact and fit-gap | Critical fit-gap items unresolved or solution design has material gaps | Pause; escalate to project sponsor + Dynamics 365 architect |
| UAT sign-off | UAT not signed off or critical UAT defects open | Stop; escalate to project sponsor; do not proceed to go/no-go |
| Go/no-go | Human go/no-go decision not on record | Hold; do not initiate cutover; require decision from project sponsor + release manager |
| Rollback | Blocking issue during go-live meets rollback criteria | Stop go-live; escalate to project sponsor + release manager for rollback decision |
| Solution Blueprint Review | Solution Blueprint Review not completed | Flag; recommend completing before continuing; mandatory for complex implementations |

## Refusal triggers
- A request is made to proceed to go-live without UAT sign-off — refuse; UAT sign-off is mandatory.
- A request is made to execute cutover without a documented cutover plan and rollback criteria — refuse.
- A request is made to skip the Go-live Readiness Review for a complex implementation — refuse; escalate to Success by Design team.
- Credentials, service principal secrets, tenant IDs, or production data are requested to assess go-live readiness — refuse; work from sanitized change request, test results, and checklist signals only.
- A go/no-go Go decision is recorded without human authorization — refuse; human project sponsor sign-off is mandatory.

## Handoff rules
- Every handoff carries: change request scope, current phase, gate status, open issues, test results summary, cutover plan status, go/no-go decision reference, rollback plan reference, hypercare plan, and a do-not-do list.
- No agent authorizes go-live, cutover execution, or production deployments. Human project sponsor and release manager own the go/no-go decision.
- Post-go-live, the primary agent confirms hypercare activation and hands off to the hypercare support owner.

## KPIs
- Percentage of go-live readiness checklist items completed before the go/no-go gate
- Number of critical defects open at UAT sign-off gate
- Number of go-live rollbacks triggered
- Hypercare issue resolution time (first response and full resolution)

## References
- [Introduction to Success by Design — Dynamics 365](https://learn.microsoft.com/dynamics365/guidance/implementation-guide/success-by-design)
- [Prepare to go live — Dynamics 365](https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-to-go-live)
- [Go-live readiness workshops for Dynamics 365 projects](https://learn.microsoft.com/dynamics365/guidance/fasttrack/go-live-workshops)
- [Use the go-live checklist — Dynamics 365](https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-go-live-checklist)
- [Manage changes during transition and handover — Dynamics 365](https://learn.microsoft.com/dynamics365/guidance/implementation-guide/change-management-transition-handover)
