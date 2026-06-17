---
name: close-to-report-protocol
description: Use this skill to coordinate the financial period close and reporting process across Dynamics 365 Finance and Microsoft Fabric / Power BI. It orchestrates subledger close, general ledger reconciliation, currency revaluation, consolidation, financial statement preparation, and report certification. The skill defines stage gates for subledger close, reconciliation sign-off, and report certification, and enforces structured handoffs between d365-finance-close-to-report-agent and fabric-power-bi-business-insights-governance-agent. It does not post journal entries, modify financial data, certify financial statements, or access live ledger credentials; all production-impacting steps are escalated to the relevant specialist or human finance owner.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-16"
  category: finance
  lifecycle: experimental
---

# Close-to-Report Protocol

## Purpose
This skill orchestrates the financial period close and reporting process in Microsoft Dynamics 365 Finance environments, with downstream coordination to Microsoft Fabric and Power BI for business insights and report distribution. It coordinates the transition from an open accounting period through subledger close, general ledger reconciliation, adjustments, consolidation, financial statement preparation, and certified report publication. The protocol exists so that closing tasks are sequenced correctly, no reconciliation step is skipped, and report data is not published before financial close is certified by the responsible finance owner.

## When to use
- A month-end, quarter-end, or year-end financial period close must be orchestrated across multiple modules.
- Subledger close, bank reconciliation, currency revaluation, and ledger reconciliation must be sequenced and tracked.
- Consolidated financial statements must be prepared across multiple legal entities.
- Power BI financial reports must not be refreshed with uncertified period data.
- A cross-module reconciliation discrepancy is detected and must be investigated before close certification.

## When NOT to use
- The matter is a single-module posting question — route to the Dynamics 365 Finance specialist.
- The ask involves tax return preparation or external audit coordination — escalate to the finance owner and external advisors.
- Budget vs. actual analysis is required without a period close event — use Fabric / Power BI governance skill directly.
- The matter involves procurement or sales order financial entries that are not yet posted — resolve upstream first.

## Participating agents
- d365-finance-close-to-report-agent — owns all period-close tasks in Dynamics 365 Finance: subledger posting, reconciliation, revaluation, consolidation, and financial statement preparation
- fabric-power-bi-business-insights-governance-agent — owns report certification, data refresh governance, and distribution of certified financial reports via Microsoft Fabric and Power BI

## Inputs required
- Period end date and fiscal period identifier
- Legal entity or consolidation group scope
- Close checklist template reference from Dynamics 365 Finance period close workspace
- Materiality threshold for reconciliation discrepancy escalation
- Report distribution list and certification sign-off owner

## Evidence required
- Subledger close confirmation for: Accounts Receivable, Accounts Payable, Fixed Assets, Inventory, and Project Operations (as applicable)
- Bank reconciliation sign-off
- Currency revaluation run confirmation with FX rates source
- Ledger-to-subledger reconciliation report showing zero unexplained differences
- Consolidation and elimination journal posting confirmation (multi-entity scenarios)
- Finance owner sign-off on financial statements before report publication

## Workflow
1. Open the financial period close workspace in Dynamics 365 Finance; load the applicable close template for the period type (month-end, quarter-end, year-end).
2. Execute subledger close tasks in sequence: Accounts Payable, Accounts Receivable, Fixed Assets, Inventory, and Project Operations.
3. Perform bank reconciliation for all bank accounts in scope; escalate any unexplained reconciling items above the materiality threshold.
4. Run currency revaluation for all foreign-currency accounts; validate source FX rates against an approved reference.
5. Perform ledger-to-subledger reconciliation; confirm zero unexplained differences. Any difference halts the process.
6. Subledger close gate: d365-finance-close-to-report-agent confirms all subledger modules are closed and reconciled.
7. Post period-end accruals, adjustments, and eliminations as required; document all manual journal entries with supporting evidence.
8. For multi-entity environments: run consolidation, intercompany eliminations, and minority interest calculations.
9. Prepare draft financial statements: balance sheet, income statement, and cash flow statement.
10. Reconciliation sign-off gate: finance owner reviews and approves the reconciliation package.
11. Invoke fabric-power-bi-business-insights-governance-agent to refresh certified financial data in Microsoft Fabric; lock the reporting period to prevent further edits.
12. Report certification gate: finance owner certifies financial statements before reports are distributed.
13. Distribute certified financial reports via Power BI; record distribution event with timestamp and recipient list.

## Decision gates
| Gate | Condition | Action if not met |
|---|---|---|
| Subledger close | All subledgers closed with zero unexplained differences | Halt general ledger close; escalate to subledger owner |
| Bank reconciliation | All bank accounts reconciled with documented items | Halt; escalate unexplained items above materiality threshold to finance owner |
| Currency revaluation | Revaluation run confirmed with approved FX rates | Hold close; validate rate source |
| Ledger-to-subledger match | Zero unexplained difference | Halt; investigate and resolve before proceeding |
| Reconciliation sign-off | Finance owner approved the reconciliation package | Do not prepare or publish financial statements |
| Report certification | Finance owner certified statements as accurate | Block fabric-power-bi-business-insights-governance-agent from publishing reports |

## Refusal triggers
- Financial statements are requested for distribution before reconciliation sign-off — refuse.
- A manual journal entry is requested without supporting documentation and human approval — refuse.
- A reporting period is requested to be re-opened after financial close certification — escalate; do not re-open without documented approval.
- Live ledger credentials, chart of accounts details, or intercompany elimination schedules with actual figures are requested by an agent — refuse; escalate.
- A consolidation adjustment is requested to be backdated — refuse; escalate to the finance controller.

## Handoff rules
- d365-finance-close-to-report-agent produces the reconciliation package and certified close confirmation before fabric-power-bi-business-insights-governance-agent may refresh or publish any financial data.
- Every handoff carries a capsule: matter_id, period_id, current_stage, gate_outcomes, open_questions, do_not_do_list.
- Report publication is blocked until the report certification gate is passed.
- All manual journal entries are logged separately with author, approver, business reason, and supporting document reference.

## KPIs
- Period close cycle time (period end date to final close certification)
- Reconciliation first-pass rate (% of accounts reconciled without human intervention)
- Number of post-close adjustments (target: zero material adjustments post-certification)
- Report certification lead time (time from close completion to certified report distribution)
- Audit finding rate related to period close (target: zero repeat findings)

## References
- [Close financial periods — Dynamics 365 Finance record-to-report](https://learn.microsoft.com/dynamics365/guidance/business-processes/record-to-report-close-financial-periods)
- [Financial period close workspace — Dynamics 365 Finance](https://learn.microsoft.com/dynamics365/finance/general-ledger/financial-period-close-workspace)
- [Record to report business process areas overview](https://learn.microsoft.com/dynamics365/guidance/business-processes/record-to-report-areas)
- [Configure and perform periodic processes in Dynamics 365 Finance (training)](https://learn.microsoft.com/training/modules/configure-periodic-processes-dyn365-finance/)
