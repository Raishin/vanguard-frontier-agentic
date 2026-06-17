# Safety checklist

Use this reference before any recommendation involving production period-close operations, posting-configuration changes, year-end close runs, ledger-period status updates, or compliance-impacting financial configuration changes in Dynamics 365 Finance.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, environment URLs, client secrets, certificates, or customer personally identifiable financial information into chat.
- Use exported financial reports or sanitized user-provided evidence for live-state claims; otherwise use documentation and label the evidence level.
- Do not invent account numbers, posting profile configurations, trial balance values, reconciliation results, or live environment state.
- Require explicit human approval before recommending any production period-close operation, period-status change, posting profile modification, or year-end close run.
- Use current official Microsoft Learn documentation for Dynamics 365 Finance general ledger and period-close behavior.
- Keep recommendations least-disruptive, reversible where possible, and scoped to the domain in question.
- Production period-close operations and posting-configuration changes are live-guard gated. Always escalate to a qualified Dynamics 365 Finance controller or system administrator with environment access before execution.

## Stress checks

- What sub-ledger balances are unreconciled to the GL that could cause a materially misstated financial statement?
- What journals are unposted or unapproved that would affect the period's closing balances?
- What posting profiles map to incorrect accounts or bypass required financial controls?
- What period-end close tasks are incomplete or lack documented sign-off that auditors would expect?
- What year-end close parameters increase the risk of permanent period lockout or incorrect balance transfer?
- What rollback path exists if a period is incorrectly set to On Hold or Permanently Closed?
- What audit evidence is missing that internal or external auditors would require?

## Evidence labels

Use `live evidence`, `report evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Dynamics 365 Finance ledger configuration, posted balances, period-close task completion state, or reconciliation results.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Setting a ledger period status to On Hold or Permanently Closed in any legal entity
- Running or reversing a year-end close process in any environment
- Modifying posting profiles in a production environment
- Changing fiscal calendar definitions or period date ranges in production
- Running financial consolidation or intercompany elimination processes in production
- Modifying tax code or tax group configurations in a production environment
- Changing foreign currency revaluation parameters or exchange rate sources in production
