# Workflow and output contract

Use this reference only when performing the full pipeline review, forecast accuracy assessment, sales accelerator audit, or CRM hygiene review, or when formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Pipeline health: opportunity stage distribution, stale records, past-due close dates, probability accuracy, win/loss patterns
- Forecast configuration: forecast columns, category mapping, rollup hierarchy, quota assignment, AI predictive columns
- Lead process: lead qualification criteria, lead scoring configuration, lead-to-opportunity conversion rates, disqualification tracking
- Sales accelerator: sequence coverage of key sales scenarios, assignment rule accuracy, work list prioritization signals
- CRM data hygiene: duplicate detection rules, required field completion rates, data enrichment coverage, inactive record policies
- Sales insights: predictive scoring adoption, Copilot for Sales usage, conversation intelligence coverage
- Seller productivity: sequence adherence rates, activity completion against plan, response time to leads

## Safe workflow

1. **Frame scope**
   - Teams or territories in scope:
   - Sales process domains (e.g., new business, renewal, expansion, channel):
   - Business concern (pipeline trust / forecast accuracy / hygiene / seller productivity / revenue leakage):
   - Required outcome (point-in-time review / ongoing advisory / configuration recommendation):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported pipeline snapshots, forecast reports, sales accelerator usage reports, and activity history exports.
   - Otherwise inspect sanitized user-provided evidence, configuration screenshots, or official Dynamics 365 Sales documentation.
   - Label each finding as `live evidence`, `report evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What opportunities have no recent activity and a past close date?
   - What forecast categories are misconfigured or inconsistently applied across the team?
   - What sequences have low completion rates, signaling seller friction or poor sequence design?
   - What lead sources show low conversion rates that could indicate qualification gap or poor lead quality?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest safe action**
   - Prefer targeted hygiene fixes, sequence refinement, and forecast category clarification over bulk pipeline updates.
   - If the safest action is to gather evidence first (export pipeline report, run forecast comparison), say that plainly.
   - Production forecast configuration and sales-process changes require live-guard escalation. Do not recommend live changes without explicit human approval.

## Output contract

Return this structure:

```markdown
# D365 Sales Revenue Operations Review: <scope>
## Executive verdict
- Status: HEALTHY / AT RISK / NEEDS ATTENTION / NEEDS EVIDENCE
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
- Reports or checks to run:
- Expected result:
## Residual risk
- <risk or explicit none>
```
