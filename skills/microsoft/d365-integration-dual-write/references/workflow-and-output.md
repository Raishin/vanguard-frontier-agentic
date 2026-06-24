# Workflow and output contract

Use this reference only when performing the full dual-write integration review, or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Dual-write connection setup: Finance & Operations and Dataverse environment linking, solution installation completeness, health check results
- Table map selection: required maps for the integration scenario, Separated Dual-write Application Orchestration package usage, custom map design
- Dependency order: dependent table maps identified and enabled in correct sequence before the primary map
- Integration key mapping: all integration key fields mapped, lookup field expansion handled, bidirectional key mapping for bidirectional field maps
- Master-data ownership: authoritative source declared per entity map, conflict resolution strategy documented before initial sync
- Initial sync planning: data volumes estimated, sync duration estimated, skip vs. run decision, post-sync validation plan
- Error handling configuration: alert settings configured per error type, thresholds set, notification recipients defined, auto-pause or auto-stop rules set
- Error monitoring posture: error log review cadence, retry and dismiss workflow, 24-hour queue compliance window awareness
- Power Platform integration boundary: dual-write scope defined, no duplicate integration paths via Power Automate for the same entity
- Rollback and recovery: reset dual-write connection steps documented, rollback trigger criteria, rollback owner named
- Stakeholder sign-off: integration lead and data governance owner sign-off, dated

## Safe workflow

1. **Frame scope**
   - Finance & Operations workloads in scope (Finance, Supply Chain Management, Commerce, Human Resources):
   - Dataverse / customer engagement apps in scope (Sales, Customer Service, Field Service, other):
   - Entity maps in scope (customers, vendors, products, chart of accounts, other):
   - Integration direction concern (F&O-to-Dataverse, Dataverse-to-F&O, or bidirectional):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer documented artifacts: dual-write health check results, table map status exports, dependency list screenshots, master-data ownership decision log, sync error dashboard exports, alert configuration records, integration lead sign-off.
   - Otherwise inspect sanitized user-provided summaries or official dual-write documentation.
   - Label each finding as `live evidence`, `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - Have all dependent table maps been identified and enabled in the correct dependency order?
   - Has a master-data owner been declared per entity map before initial sync begins?
   - Have integration key fields been fully mapped including lookup field expansion for bidirectional maps?
   - Has the dual-write health check been run and passed before production map operations?
   - Are error alert thresholds configured with appropriate auto-pause or auto-stop rules?
   - Is the 24-hour queue compliance window understood and monitored for paused maps?
   - Is there a rollback plan with a named owner and documented trigger criteria for production map operations?
   - Has the integration lead and data governance owner signed off on master-data ownership decisions?
   - What happens if initial sync fails at 50% through a large entity map?

4. **Recommend the smallest safe action**
   - Prefer running the health check and resolving prerequisites over enabling maps, declaring master-data ownership before initial sync, and pausing maps with a resume plan over stopping maps.
   - If the safest action is to stop and declare master-data ownership or run the health check first, say that plainly.
   - Enabling/disabling production dual-write maps and initial sync runs require live-guard escalation. Do not recommend production map operations without explicit human approval from the integration lead and data governance owner.

## Output contract

Return this structure:

```markdown
# D365 Dual-Write Integration Review: <scope>
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
