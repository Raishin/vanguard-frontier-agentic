# Workflow and output contract

Use this reference only when performing the full data migration or cutover readiness review, or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Migration strategy: entity selection, dependency sequence, full vs. delta load approach, data volume estimates
- Data quality: legacy data cleansing status, known issues, deduplication, null handling, format alignment
- Data Management Framework setup: data entities mapped to legacy source fields, staging table configuration, error handling
- Mock migration results: number of dry runs completed, staging error counts per entity, elapsed time per entity, reconciliation totals
- Reconciliation controls: record count comparison, field-level sampling, financial balance reconciliation, business user validation sign-off
- Cutover runbook: task sequence, owners, durations, dependencies, system freeze timing, delta load steps, go/no-go checkpoint
- Rollback plan: rollback trigger criteria, rollback owner, rollback execution steps, rollback validation, rollback time estimate
- Stakeholder sign-off: named business data owner and implementation lead approvals, dated
- Post-migration hyper-care: data issue triage path, escalation owner, validation checklist for day-one operations

## Safe workflow

1. **Frame scope**
   - Dynamics 365 workloads in scope (Finance, Supply Chain Management, Customer Service, other):
   - Legacy systems being migrated from:
   - Data domains in scope (master data, reference data, open transactions, historical data):
   - Go-live date and cutover window duration:
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer documented artifacts: mock migration result logs, staging table error exports, reconciliation reports, cutover runbook, rollback plan document, business owner sign-off.
   - Otherwise inspect sanitized user-provided summaries or official Dynamics 365 documentation.
   - Label each finding as `live evidence`, `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - Has at least one mock migration been completed and documented?
   - Have staging table errors been resolved and re-validated?
   - Does reconciliation evidence go beyond record counts to include field-level sampling and balance checks?
   - Is the cutover window realistic given measured migration throughput and buffer?
   - Is there a tested rollback plan with named owner and documented trigger criteria?
   - Has the business data owner signed off on migration results?
   - What happens if migration fails at 60% through the cutover window?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest safe action**
   - Prefer additional mock migrations over proceeding to production, staged delta loads over big-bang migration, and staging validation resolution before production promotion.
   - If the safest action is to stop and complete another mock migration or resolve staging errors, say that plainly.
   - Production data migration requires live-guard escalation. Do not recommend production migration without explicit human approval from the implementation lead and business data owner.

## Output contract

Return this structure:

```markdown
# D365 Data Migration & Cutover Review: <scope>
## Executive verdict
- Status: READY / READY WITH CONDITIONS / NOT READY / NEEDS EVIDENCE
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
