# Safety checklist — SAP Procurement Ariba Value Leakage Review

Use before making any procurement value leakage remediation recommendation, especially for findings involving three-way match tolerance overrides, supplier sanctions screening, contract compliance bypass, or payments to unqualified suppliers.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP Ariba tenant, SAP Ariba Network, SAP S/4HANA Procurement system, Fiori launchpad, or production procurement database. This skill reviews artifacts only.
- Do not accept or request SAP Ariba login credentials, API tokens, or direct procurement database access.
- Do not create, approve, modify, or release any purchase requisition, purchase order, contract, invoice, or payment. There is no procurement document creation or mutation in this skill's execution path. Recommendations always describe configuration and process changes, not direct procurement transactions.
- Do not approve, close, or recommend closing an unmitigated critical finding (three-way match bypass without approval; payment to sanctioned supplier) without documented compliance and audit team authorization.
- Do not recommend implementing configuration changes (tolerance group edits, guided buying rule activation, contract linkage enforcement) directly in a production SAP Ariba tenant or SAP S/4HANA system. All recommendations must first be tested in a non-production environment.
- Do not use memory alone to assert what a specific SAP Ariba tenant's guided buying enforcement rules, contract consumption rates, or tolerance group settings are. All findings must be grounded in user-provided artifacts or official SAP Ariba documentation.
- Do not conflate a three-way match tolerance (a configured control parameter allowing defined variance) with a match bypass (a missing or disabled match requirement). A tolerance is a managed control; a bypass is a control failure. The distinction determines the risk classification.

## What people get wrong

- **Treating tolerance groups as a sufficient match control**: A tolerance group that allows automatic invoice release for price variances up to 10% or quantity variances up to 5% is a managed control — but only if tolerance overrides still require documented approval for releases above zero. A tolerance that releases invoices automatically without any approval step is a control gap, not a tolerance design.
- **Overlooking non-PO spend in leakage analysis**: Spend analysis that covers only PO-based spend misses a significant leakage channel. Non-PO invoices (direct-pay, emergency purchases, service-based invoices without PO reference) are frequently the largest maverick spend source. Procurement card spend adds a third channel. All three must be in scope.
- **Conflating contract existence with contract compliance**: An active contract in SAP Ariba Contracts does not prove that spend is flowing through the contract. Contract compliance requires both contract existence and enforcement that routes POs through the contract. Low contract consumption rates with high spend in the category signal contract existence without compliance.
- **Missing guided buying bypass in analytics**: SAP Ariba reporting may not surface the guided buying bypass rate directly — it requires comparing total requisition volume to guided-buying-sourced requisition volume per category. A review that only confirms guided buying is configured without measuring actual bypass rates misses the adoption gap finding.
- **Treating supplier qualification as a one-time event**: Supplier qualification completion at onboarding does not guarantee ongoing risk management. Supplier financial health, geographic concentration, and sanctions status change over time. A qualification framework without periodic re-screening and automated alerts for material risk changes is an incomplete supplier risk control.
- **Ignoring discount window timing in AP process design**: Early payment discount capture is not purely a configuration question — it is a process timing question. An AP process that routes invoices through multi-level approval before coding and payment release will systematically miss discount windows even when SAP Ariba Discount Management is correctly configured.
- **Assuming three-way match is active for all PO types**: In SAP S/4HANA, three-way match (invoice/PO/GR) is required for stock material POs but may not be enforced for service POs, blanket POs, or invoicing plan POs. A review that confirms three-way match is enabled for goods-based procurement but does not check service and blanket PO invoice verification may miss the largest match leakage channel.

## When to push back

- Push back (and escalate) when a critical finding is identified (three-way match bypass without approval enabling automated payment; payment to a supplier with active sanctions alert) — do not proceed with other recommendations until this is escalated to the audit and compliance teams.
- Push back when the user asks to confirm procurement compliance from memory alone without providing spend analysis reports, contract compliance metrics, exception reports, or supplier risk summaries.
- Push back when the request requires live SAP Ariba tenant access, Ariba API calls, or SAP S/4HANA system inspection — state clearly that live inspection is out of scope and ask the user to supply the relevant exports or summaries.
- Push back when asked to create purchase orders, approve invoices, release payments, or modify supplier records — this is an absolute boundary and must be refused in all circumstances.
- Push back when asked to approve or close a compliance finding or sanctions screening alert without audit team and legal team authorization — this skill is advisory only.
- Push back when the user presents a single-channel spend report (PO-based only) as a complete view of procurement value leakage — non-PO and procurement card channels must be included for a meaningful assessment.

## Evidence labels

- `documentation-based` — grounded in SAP Ariba, SAP S/4HANA Procurement, or SAP Help Portal source-to-pay documentation, SAP Ariba best practice guides, or SAP procurement compliance guidance
- `user-provided evidence` — spend analysis exports, contract compliance reports, three-way match exception reports, supplier risk summaries, discount capture rate data, guided buying adoption metrics, invoice aging reports, or written descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
