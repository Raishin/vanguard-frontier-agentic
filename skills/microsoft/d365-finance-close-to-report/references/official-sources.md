# Official sources

Use this reference only when you need source grounding for Dynamics 365 Finance general ledger, period-close, financial consolidation, or financial reporting service behavior, or the detailed source list.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live environment state:

- https://learn.microsoft.com/dynamics365/finance/general-ledger/close-general-ledger-at-period-end
- https://learn.microsoft.com/dynamics365/finance/general-ledger/financial-period-close-workspace
- https://learn.microsoft.com/dynamics365/finance/general-ledger/tasks/close-fiscal-year
- https://learn.microsoft.com/dynamics365/finance/general-ledger/tasks/mass-financial-period-close
- https://learn.microsoft.com/dynamics365/finance/general-ledger/year-end-close
- https://learn.microsoft.com/dynamics365/guidance/business-processes/record-to-report-close-financial-periods
- https://learn.microsoft.com/training/paths/configure-use-general-ledger-dyn365-finance/
- https://learn.microsoft.com/training/modules/configure-periodic-processes-dyn365-finance/

## Grounding rule

Official documentation explains Dynamics 365 Finance service behavior. It does not prove the user's current environment configuration, ledger balances, posted journals, period-close task completion state, or posting profile assignments. Prefer read-only evidence from the environment (e.g., exported trial balance reports, financial period close workspace screenshots, ledger settlement exports, reconciliation outputs) over inference.

## Service facts (verified 2026-06-16)

General ledger model structure:
- The Dynamics 365 Finance GL is organized around a **chart of accounts** shared across legal entities, **financial dimensions**, and a **fiscal calendar** broken into **fiscal years** and **periods**.
- **Ledger periods** can be set to Open, On Hold, or Permanently Closed. On Hold restricts posting for specified modules or user groups. Permanently Closed cannot be reopened — use with extreme caution.
- **Posting profiles** define the GL accounts to which sub-ledger transactions (customer invoices, vendor invoices, inventory transactions, fixed asset transactions) are posted. Misconfigured posting profiles produce posting errors and GL imbalances.

Period-end close process:
- The **Financial period close workspace** organizes closing tasks into a template-driven schedule. Tasks can be assigned to individuals per legal entity, linked to system pages or external URLs, and tracked with dependencies.
- Recommended period-end steps (per Microsoft Learn): complete all module tasks (AR, AP, Inventory), verify all journals are posted, run foreign currency revaluation, settle ledger transactions, process allocations, post period-end adjustments, perform consolidation, generate financial statements, then set the period to On Hold.
- **Year-end close** transfers balances to a new fiscal year. The parameter **Set fiscal year status to permanently closed** should generally be left as No to preserve the ability to post adjustments.
- **Mass financial period close** allows updating period status and module-level posting access across multiple legal entities simultaneously.

Sub-ledger reconciliation:
- Sub-ledgers (AR, AP, Inventory, Fixed Assets) maintain their own transaction records that must reconcile to GL summary balances. Reconciliation gaps indicate either unposted sub-ledger transactions, posting profile misconfigurations, or manual GL journal entries that bypass the sub-ledger.
- The **Trial Balance** and **Trial Balance by Period** reports show GL account movements and closing balances per period; use them to detect unexplained variances.

Financial reporting:
- Dynamics 365 Finance integrates with **Financial reporting** (formerly Management Reporter) for financial statement generation. Reports use row and column definitions against the GL data.
- Financial reporting access is controlled separately from GL posting access; review report-level security alongside GL role assignments.

Review implications:
- Do not approve a close process from intent alone. Require reconciliation evidence, Financial period close workspace task completion screenshots, foreign currency revaluation run confirmation, and business owner sign-off.
- Documentation cannot prove the user's actual period-close task completion state, posted balances, or posting profile configuration.
