# Implementation Governance Guide

Use this reference for Dynamics 365 Success by Design phase governance, Solution Blueprint Review discipline, fit-gap failure modes, safe review workflow, verification targets, and pushback criteria.

## What people get wrong

The lazy story is:

> Run the SBR once at kickoff, then proceed with development. The framework is optional ceremony.

Wrong. The Solution Blueprint Review is a mandatory, findings-driven review that triggers Implementation Reviews for every high-risk area identified. Skipping it or treating it as a one-time checkbox means that data model risks, integration gaps, security blind spots, and customization sprawl are not surfaced until late in the project when they are expensive to fix.

Common bad assumptions:

- The SBR is a one-time kickoff meeting with no follow-on obligations.
- Fit-to-standard analysis means accepting all standard Dynamics 365 processes without review.
- Fit-gap results are optional; the implementation team can decide on customizations ad hoc.
- Phase gates are advisory; teams can proceed without completing artifacts.
- Implementation Reviews are only required if FastTrack is engaged.
- UAT sign-off from a single user representative is sufficient for go-live approval.
- Customizations introduced during implementation do not need re-assessment at go-live.

## Governance failure modes

- SBR conducted too late (after implementation has started), meaning findings trigger expensive rework.
- Fit-gap log not maintained, so customization decisions cannot be traced back to business requirements.
- Implementation Reviews skipped for security and data model domains, leaving audit and performance risks undetected.
- Customization sprawl: every legacy system requirement triggers a custom extension without challenge or fit-to-standard re-evaluation.
- Go-live readiness checklist items marked complete without documented evidence or stakeholder sign-off.
- Mock cutover not performed, making the real cutover the first rehearsal.
- Hypercare and post-go-live support plan not finalized before cutover begins.
- Project sponsor not included in go/no-go decision, leaving no accountable authority for go-live approval.

## High-risk governance gaps (examples)

- Missing SBR → Implementation Reviews cannot be scoped; risk accumulates silently.
- No fit-gap log → Customizations are undocumented, untraceable, and unmaintainable.
- Skipped security Implementation Review → Role design, SoD, and data access controls not validated before production.
- No data migration dry run → First migration attempt is production; data loss or corruption risk is unmitigated.
- No cutover plan sign-off → Cutover sequence is improvised; rollback criteria and owners are unknown.
- UAT not completed → Business process validation gaps reach production.
- Hypercare plan missing → Post-go-live issues have no escalation path or support SLA.

## Minimum safe governance workflow

1. Confirm the project scope: Dynamics 365 workloads, legal entities, go-live date, FastTrack engagement status.
2. Verify SBR completion: workshop conducted, findings documented, Implementation Reviews triggered for all high-severity findings.
3. Review fit-gap log: every customization or extension has a documented business justification, cost estimate, and roadmap alignment assessment.
4. Confirm Implementation Reviews conducted: data model, security, integration, ALM, testing strategy as triggered by SBR findings.
5. Review go-live readiness checklist: SIT, UAT, performance testing, data migration dry runs, cutover plan, support plan, license confirmation — all items completed and signed off.
6. Verify mock cutover results: at least one mock cutover completed, issues documented, plan updated.
7. Confirm written go/no-go decision with named project sponsor and implementation lead sign-off.
8. Provide a minimum-safe-action recommendation scoped to the highest-severity governance gaps.
9. Require live-guard escalation for any production deployment authorization.

## Verification targets

- SBR status: conducted, findings documented, Implementation Reviews triggered
- Fit-gap log: all customizations traced to business requirements with documented justification
- Implementation Reviews: coverage of data model, security, integration, ALM, testing
- Go-live checklist: all items complete with stakeholder sign-off
- Mock cutover: completed, issues resolved, plan updated
- Cutover plan: sequenced tasks, owners, durations, rollback criteria, go/no-go checkpoint
- Support plan: hypercare team named, escalation path defined, coverage outside business hours
- Go/no-go decision: written, named approvers, dated

## When to push back

Push back if the user asks to:

- approve go-live without a completed SBR and go-live readiness checklist
- waive a mandatory Implementation Review without documented compensating controls
- approve customizations without a fit-gap log entry and architectural review
- accept UAT sign-off from a single user without business stakeholder approval
- proceed to production without a mock cutover or documented rollback plan
- bypass the go/no-go decision gate without project sponsor involvement
- mark a phase gate complete without documented artifacts
