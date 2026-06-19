# Financial close controls guide

Use this reference for Dynamics 365 Finance period-close and financial controls domain-specific failure modes, safe close workflow, verification targets, and pushback criteria.

## What people get wrong

The lazy story is:

> Run the financial period close workspace tasks and the close is complete.

Wrong. Completing workspace tasks tracks task execution but does not guarantee underlying financial accuracy. Sub-ledger to GL reconciliation gaps, unapproved journals, stale foreign currency revaluation, and misconfigured posting profiles can all produce materially misstated financials even after workspace tasks are marked complete.

Common bad assumptions:

- Marking all Financial period close workspace tasks complete means the GL is accurate.
- Foreign currency revaluation is optional if the amounts seem small.
- Posting a journal in the next period is an acceptable substitute for posting it in the correct period.
- Setting a period to Permanently Closed is reversible.
- The consolidation company automatically picks up all sub-ledger adjustments without reconciliation.
- Tax posting configurations are validated automatically when tax codes are changed.

## Period-close failure modes

- Sub-ledger transactions posted after the sub-ledger-to-GL reconciliation check produce undetected variances in the closing balance.
- Year-end close run with the **Set fiscal year status to permanently closed** parameter set to Yes, locking the year against post-close adjustments required by auditors.
- Foreign currency revaluation not run before period close, leaving unrealized gains and losses unrecorded in the reporting period.
- Intercompany transactions not eliminated before consolidation, inflating consolidated revenue and expenses.
- Posting profiles referencing suspense or clearing accounts that were never cleared, producing phantom balances in financial statements.
- Journal approval workflows bypassed under time pressure, removing an internal control that auditors will expect evidence of.
- Mass financial period close used to set modules to None without per-module review, inadvertently blocking month-end postings in open modules.

## High-risk close control gaps (examples from record-to-report)

- Sub-ledger to GL reconciliation not run before period-close sign-off (undetected variances become audit findings)
- Foreign currency revaluation run after the period-close date rather than before (FX gains/losses in wrong period)
- Journals approved and posted by the same user (journal entry + approval in one role — SoD gap)
- Consolidation run without eliminating intercompany receivables and payables (inflated consolidated balance sheet)
- Posting profiles mapping inventory receipts to catch-all accounts instead of dedicated inventory accounts (distorts cost of goods sold)
- Trial balance not reviewed before setting period to On Hold (errors locked in without detection)
- Financial reporting distribution list includes unauthorized recipients (confidential financial data exposure)

These gaps represent the highest-risk close-process scenarios per SOX, IFRS, and local GAAP internal control guidance. Verify that each gap is addressed before approving close readiness.

## Minimum safe close review workflow

1. Confirm scope: legal entities, period type (month-end, quarter-end, year-end), and compliance drivers.
2. Verify all sub-module tasks are complete: AR invoicing, AP invoicing, inventory postings, fixed asset depreciation.
3. Confirm all journals in scope are posted and approved per the journal approval workflow.
4. Run foreign currency revaluation and confirm unrealized gain/loss is posted.
5. Run sub-ledger to GL reconciliation for AR, AP, Inventory, and Fixed Assets; resolve all variances.
6. Run trial balance and compare to prior period; investigate material unexplained movements.
7. Complete Financial period close workspace tasks and confirm all dependencies are satisfied.
8. For year-end: review year-end close parameters, confirm permanently-closed setting is No, run close, and verify opening balances in the new year.
9. For consolidation: confirm intercompany elimination rules are current, run consolidation, review consolidated trial balance.
10. Set period to On Hold after all tasks are complete and reconciliation evidence is documented.
11. Require live-guard escalation for any production change.

## Verification targets

- Sub-ledger reconciliation: AR, AP, Inventory, Fixed Assets balances reconciled to GL summary accounts
- Journal completeness: all expected journals posted and approved in the period
- Foreign currency revaluation: confirmation run completed and unrealized amounts posted
- Financial period close workspace: all tasks marked complete with documented evidence
- Period status: On Hold set only after reconciliation and sign-off
- Year-end close: opening balances in new year match year-end closing balances
- Consolidation: elimination entries posted; consolidated trial balance reviewed
- Reporting access: financial statements generated and distributed only to authorized recipients

## When to push back

Push back if the user asks to:

- approve close readiness without sub-ledger to GL reconciliation evidence
- skip foreign currency revaluation because amounts seem immaterial without formal assessment
- set a period to Permanently Closed without confirming that no post-close adjustments are anticipated
- approve journal postings without evidence of the journal approval workflow being followed
- accept a consolidation result without eliminating intercompany transactions
- make production posting-profile or period-close configuration changes without live-guard escalation and explicit human approval
- rely on Financial period close workspace task completion alone as close-readiness evidence without underlying financial validation
