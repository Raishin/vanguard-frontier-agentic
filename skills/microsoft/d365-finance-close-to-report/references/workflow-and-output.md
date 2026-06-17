# Workflow and output contract

Use this reference only when performing the full period-close or financial controls review, implementation guidance, audit evidence gathering, or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- GL configuration: chart of accounts, fiscal calendar, financial dimensions, ledger parameters
- Sub-ledger reconciliation: AR, AP, Inventory, Fixed Assets balances vs. GL summary accounts
- Period-end close task coverage: Financial period close workspace template completeness, task assignments, dependency chains, evidence of completion
- Posting profile configuration: customer, vendor, inventory, and fixed asset posting profiles; correct account mapping; absence of bypass patterns
- Foreign currency revaluation: revaluation parameters, exchange rate sources, unrealized gain/loss posting
- Ledger settlement and allocations: settlement completion before period close, allocation rule accuracy
- Year-end close configuration: year-end close parameters, balance transfer settings, permanently-closed risk
- Financial consolidation: consolidation company setup, intercompany elimination rules, minority interest handling
- Financial reporting access: Management Reporter / Financial reporting security, report definition access, output distribution controls
- Compensating controls: journal approval workflows, period-status restrictions, reconciliation sign-off evidence

## Safe workflow

1. **Frame scope**
   - Legal entities in scope:
   - Period type (month-end, quarter-end, year-end):
   - Compliance driver (SOX, IFRS, local GAAP, internal audit):
   - Required outcome (close readiness review / posting-config review / reporting controls audit):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported trial balance reports, financial period close workspace screenshots, reconciliation exports, and journal posting reports for current-state claims.
   - Otherwise inspect sanitized user-provided evidence, configuration screenshots, or official Dynamics 365 Finance documentation.
   - Label each finding as `live evidence`, `report evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What sub-ledger balances are unreconciled to the GL?
   - What journals are unposted as of the close date?
   - What posting profiles map to incorrect or catch-all accounts?
   - What period-end close tasks are incomplete or undocumented?
   - What evidence is missing that auditors or regulators would expect?
   - What year-end close parameters increase the risk of permanent period lockout?

4. **Recommend the smallest safe action**
   - Prefer reconciliation completion, journal posting, and task sign-off before advancing the period status.
   - If the safest action is to stop and gather evidence (run a trial balance or reconciliation report first), say that plainly.
   - Production period-close operations and posting-configuration changes require live-guard escalation. Do not recommend live changes without explicit human approval.

## Output contract

Return this structure:

```markdown
# D365 Finance Close-to-Report Review: <scope>
## Executive verdict
- Status: READY TO CLOSE / CLOSE WITH CONDITIONS / NOT READY / NEEDS EVIDENCE
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
