# Workflow and output contract

Use this reference only when performing the full Success by Design governance review, implementation phase gate assessment, or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Phase gate completeness: artifacts produced and reviewed per phase (Strategize, Initiate, Implement, Prepare, Operate)
- Solution Blueprint Review: conducted, findings documented, Implementation Reviews triggered for high-risk areas
- Fit-to-standard analysis: Business Process Catalog used, standard adoption documented, deviations justified
- Fit-gap discipline: gaps logged, extension/customization decisions documented with risk and roadmap assessment
- Customization sprawl: count and scope of extensions, ISV solutions, and custom code relative to standard product footprint
- Implementation reviews: data model, security, integration, ALM, testing strategy reviews conducted where indicated by SBR
- Go-live readiness: SIT, UAT, performance testing, data migration dry runs, cutover plan, support plan, license confirmation
- Stakeholder sign-off: written go/no-go decision with named approvers
- Post-go-live: stabilization plan, hypercare team readiness, lessons-learned review scheduled

## Safe workflow

1. **Frame scope**
   - Implementation phase currently in scope:
   - Dynamics 365 workloads (e.g., Finance, Supply Chain Management, Customer Service, Sales):
   - FastTrack engagement: active / self-serve / none:
   - Required outcome (phase gate review / SBR readiness / go-live approval / post-go-live):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer documented artifacts: SBR output, fit-gap log, implementation review findings, go-live checklist, UAT sign-off, cutover plan.
   - Otherwise inspect sanitized user-provided summaries or official Success by Design documentation.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What phase gates have been skipped or have missing artifacts?
   - What SBR findings remain unresolved or without a triggered Implementation Review?
   - What customizations lack a documented fit-gap justification or roadmap alignment assessment?
   - What go-live readiness checklist items are incomplete or have no sign-off?
   - What evidence is missing that would change the verdict?
   - What rollback path exists if the go-live fails within the cutover window?

4. **Recommend the smallest safe action**
   - Prefer standard product adoption over customization, phased rollout over big-bang go-live, and SBR completion before implementation reviews.
   - If the safest action is to stop and complete a missing phase gate (e.g., conduct the SBR before proceeding), say that plainly.
   - Production deployment and go/no-go decisions require live-guard escalation. Do not recommend go-live without explicit stakeholder approval.

## Output contract

Return this structure:

```markdown
# D365 Success by Design Governance Review: <scope>
## Executive verdict
- Status: COMPLIANT / COMPLIANT WITH RISKS / NON-COMPLIANT / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Artifacts or checks to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
