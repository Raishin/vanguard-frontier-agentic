# Safety checklist — SAP Order-to-Cash Review

Use before making any OTC remediation recommendation, especially for findings involving zero-price sales order paths, credit block release SoD gaps, unbilled delivery revenue, billing plan slippage, or DSO drivers.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP S/4HANA Sales (SD) system, SAP S/4HANA Finance (FI) system, Fiori launchpad, SAP GUI session, backend RFC, or production OTC database. This skill reviews artifacts only.
- Do not accept or request SAP logon credentials, RFC connection details, or direct OTC database access.
- Do not create, modify, release, or cancel any sales order, delivery document, billing document, customer credit limit, or customer payment. There is no OTC document creation or mutation in this skill's execution path. Recommendations always describe configuration and process design changes, not direct document transactions.
- Do not approve, close, or recommend closing an unmitigated critical OTC finding (zero-price revenue path; revenue account misposting at material level; credit block SoD gap with active exposure) without documented audit team and finance team authorization.
- Do not recommend implementing OTC configuration changes (pricing procedure edits, credit check parameter changes, billing document type modifications, SD–FI account determination changes) directly in a production SAP S/4HANA system. All recommendations must first be tested in a non-production environment.
- Do not use memory alone to assert what pricing condition types are active, what credit block aging looks like, whether billing plans are on schedule, or what the DSO drivers are in the user's OTC process. All findings must be grounded in user-provided artifacts or official SAP S/4HANA documentation.
- Do not conflate a credit block (a system-generated hold pending credit management review) with a delivery block (a manually applied hold for operational reasons). They are governed by different authorization objects and represent different control risks. Distinguish clearly when classifying findings.

## What people get wrong

- **Treating pricing error indicators as self-correcting**: When a SAP SD pricing error indicator (pricing status I) is set on a sales order because a condition type could not be found, the order is not automatically blocked from delivery and billing. Unless a specific incompletion procedure or user exit enforces a block on pricing-error orders, they can proceed to fulfillment and invoice at zero or incorrect values. Reviews that check for pricing errors but do not verify that pricing-error orders are blocked miss the actual control gap.
- **Overlooking the static vs. dynamic credit check distinction**: A static credit check (credit exposure at order creation vs. credit limit) does not update as deliveries and invoices are posted. A customer that passes the credit check at order creation may have their credit limit consumed by the time of delivery and billing. Organizations with high-value or long-cycle orders and a static-only credit check are monitoring a point-in-time exposure, not a real-time credit position.
- **Conflating DSO calculation with DSO management**: DSO as a metric measures the average days to collect receivables. DSO as a management problem requires a driver analysis (billing cycle, dispute volume, dunning gaps, cash application delays). A review that only reports DSO elevation without a driver analysis does not provide actionable remediation direction.
- **Missing the billing plan milestone slippage risk**: Milestone billing plans for project-based or multi-deliverable contracts require that billing dates are tied to project milestones, contract acceptance events, or delivery completions. If project delays push milestone dates without updating the billing plan, revenue recognition is deferred even when delivery value has been created. Billing plan milestone slippage is a common and underreported OTC revenue timing risk.
- **Treating goods issue posting timing as a logistics issue only**: The timing of goods issue (GI) posting in SAP S/4HANA determines when inventory is relieved and when billing can be triggered for delivery-based billing documents. Delayed GI posting (posting GI days after physical shipment) adds preventable days to the billing cycle and to DSO. It is both a logistics execution and a revenue recognition process design issue.
- **Overlooking unapplied cash in DSO analysis**: Customer payments sitting unapplied in the bank clearing or customer clearing account do not reduce the outstanding receivables balance in the aging report. High unapplied cash volumes systematically overstate DSO and can mask a well-performing collections process with a cash application process gap. DSO reviews that do not include clearing account aging miss this driver.
- **Assuming order incompletion blocks downstream processing**: The SAP SD incompletion log flags missing data but does not automatically block fulfillment and billing unless a specific block is configured in the incompletion procedure. A review that confirms the incompletion procedure is configured but does not verify that missing required fields result in a delivery or billing block may overestimate the control effectiveness.

## When to push back

- Push back (and escalate to the audit and finance team) when a zero-price sales order path without a block is identified, or when revenue accounts are confirmed to be posting incorrectly at a material level — do not proceed with other recommendations until this is escalated.
- Push back when a credit block SoD gap (order entry user can release their own credit block) is found — escalate to the GRC and audit team before any further credit decisions are made.
- Push back when the user asks to confirm OTC control compliance from memory alone without providing order block aging reports, pricing procedure documentation, billing schedule adherence data, or credit management configuration summaries.
- Push back when the request requires live SAP S/4HANA system access (SAP GUI session, RFC call, Fiori OData API) — state clearly that live inspection is out of scope and ask the user to supply the relevant exports or summaries.
- Push back when asked to create sales orders, release order blocks, post billing documents, modify credit limits, or post customer payments — this is an absolute boundary and must be refused in all circumstances.
- Push back when asked to approve or close an audit finding or revenue integrity risk without audit team and finance team authorization — this skill is advisory only.
- Push back when the user presents only one OTC domain (e.g., only pricing or only DSO) as a complete OTC review — a meaningful OTC assessment requires coverage across order management, pricing, credit, billing, blocks, fulfillment, and DSO together.

## Evidence labels

- `documentation-based` — grounded in SAP S/4HANA Sales and Distribution, SAP S/4HANA Finance Accounts Receivable, SAP FSCM Dispute Management, or SAP Help Portal OTC documentation, or SAP billing and revenue recognition guidance
- `user-provided evidence` — order block aging reports, billing schedule adherence reports, pricing procedure documentation, credit management configuration summaries, DSO analytics, dispute management reports, dunning procedure documentation, unbilled delivery reports, or written descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
