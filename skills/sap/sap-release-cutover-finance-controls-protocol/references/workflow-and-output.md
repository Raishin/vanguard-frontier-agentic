# Workflow and output contract — SAP Release Cutover Finance Controls Protocol

Use this reference for all trigger classification, transport and cutover assessment sequencing, financial controls impact classification, decision rights application, and output formatting.

## Trigger classification table

| Trigger class | Primary signal | Activating agent(s) | Finance Controls involvement | Escalation risk |
|---|---|---|---|---|
| `transport-import` | Transport queue ready for production import | sap-release-change-collision-agent | Required if any transport has financial impact | Medium unless period-sensitive |
| `freeze-exception` | Change request submitted during declared freeze | sap-release-change-collision-agent | Mandatory for all freeze exceptions | High |
| `cutover-readiness` | Go-live cutover approaching; readiness checklist in progress | sap-data-migration-cutover-readiness-agent | Mandatory sign-off required | High |
| `period-close-impact` | Transport modifies financial configuration during open period | sap-finance-fico-controls-agent | Mandatory; may be blocking | Critical |
| `o2c-p2p-disruption` | O2C or P2P process interrupted by transport or migration | sap-release-change-collision-agent + sap-hypercare-incident-commander-agent | Required (business impact) | High |
| `inventory-valuation` | Transport affects material ledger or cost estimate configuration | sap-finance-fico-controls-agent | Mandatory; balance sheet impact | Critical |
| `revenue-recognition` | Transport affects RAR or SD billing configuration | sap-finance-fico-controls-agent | Mandatory; potential prior-period impact | Critical |
| `hypercare-escalation` | Production incident during hypercare requiring correction transport | sap-hypercare-incident-commander-agent | Required if financial process affected | High |

## Financial impact classification

`sap-finance-fico-controls-agent` classifies each transport or cutover item using the following taxonomy:

| Classification | Criteria | Approval gate |
|---|---|---|
| `no-financial-impact` | Transport contains no FICO, CO, SD billing, MM valuation, or RAR objects | Release Manager + Business Process Owner only |
| `financial-configuration-change` | Transport modifies FICO/CO configuration objects (document types, posting period variants, account determination, tax codes, controlling area settings) | Release Manager + Finance Controls lead required |
| `financial-posting-logic-change` | Transport modifies business logic affecting financial postings (pricing procedures, goods movement account assignment, output determination, revenue account mapping) | Release Manager + Finance Controls lead + Business Process Owner required |
| `period-sensitive-financial-change` | Transport modifies any financial object that affects current or prior-period postings, or is being imported during a period-end close window | Release Manager + Finance Controls lead + CFO written approval required; import blocked during active closing cockpit run |
| `revenue-recognition-impact` | Transport modifies RAR configuration, SD billing plan, contract account assignment, or multi-element arrangement allocation | Release Manager + Finance Controls lead + CFO; external auditor notification if prior-period revenue is affected |
| `inventory-valuation-impact` | Transport modifies material ledger configuration, costing variants, valuation area assignment, or moving average price calculation | Release Manager + Finance Controls lead + CFO; balance sheet restatement risk must be assessed |

## Protocol workflow

### Phase 1 — Trigger classification (Release Manager)

1. Classify the trigger condition using the table above.
2. Identify which participating agents are activated.
3. Request the required evidence inventory from the responsible function.
4. Confirm whether the system is currently in a production freeze period.

### Phase 2 — Transport and collision assessment

1. `sap-release-change-collision-agent` performs collision analysis on all transports in the import queue.
2. Collision findings are classified by severity (blocking / warning / informational).
3. Transport manifest is confirmed complete and all transports have QA import evidence.

### Phase 3 — Financial controls impact assessment

1. `sap-finance-fico-controls-agent` classifies each transport by financial impact category.
2. Period-close status is confirmed: is the financial closing cockpit running, are posting periods open or locked, and what is the period-end close task status?
3. Revenue recognition and inventory valuation impact are assessed for any transports classified `period-sensitive-financial-change`, `revenue-recognition-impact`, or `inventory-valuation-impact`.
4. Blocking conditions are identified and communicated to the Release Manager.

### Phase 4 — Cutover readiness assessment (if applicable)

1. `sap-data-migration-cutover-readiness-agent` evaluates the readiness checklist.
2. Open items are classified as blocking or non-blocking.
3. Rollback plan completeness is confirmed.
4. Go/no-go posture is determined based on checklist status and Finance Controls sign-off.

### Phase 5 — Hypercare triage (if applicable)

1. `sap-hypercare-incident-commander-agent` classifies incident severity and business impact.
2. Workaround status and resolution timeline are assessed.
3. If a production correction transport is required, it is proposed to the approval chain and queued to `sap-guarded-transport-import-operator-agent` only after all approvals are confirmed.

### Phase 6 — Approval gate confirmation and audit package assembly

1. Each required approval is confirmed present or flagged as outstanding.
2. Irreversible-action gate is evaluated for each proposed action.
3. Audit package is assembled and residual risk is documented.

## Go/no-go decision criteria

A release, cutover, or freeze exception is GO only when all of the following are true:

- No unresolved transport collisions classified as blocking.
- All transports classified `financial-configuration-change` or higher have Finance Controls written approval.
- No transport classified `period-sensitive-financial-change` is being imported while the Financial Closing Cockpit is running an active close task for the affected company code.
- All required business process owner sign-offs are documented.
- A pre-approved rollback plan exists for all high-risk or period-sensitive transports.
- For cutover: all checklist items are green or exceptions are documented with named acceptance owners and Finance Controls concurrence.

Any single condition not met results in a NO-GO posture. The NO-GO condition, the owner responsible for clearing it, and the required action must be stated explicitly in the response.

## Output contract

Return, in order:

1. **Trigger classification**: Which trigger class(es) apply; which participating agents are activated.
2. **Transport or cutover scope**: Items assessed, financial impact classification per item, and collision status.
3. **Go/no-go posture**: GO or NO-GO, with every blocking condition listed explicitly and the responsible owner for each.
4. **Financial controls assessment**: Period-close status, financial impact classifications, and any blocking financial controls findings.
5. **Rollback plan status**: Pre-approval status for each high-risk or period-sensitive item, rollback trigger conditions, and rollback duration estimate.
6. **Approval gate status**: For each required approval, primary authority, documentation status (present / outstanding), and gate status (cleared / blocked).
7. **Irreversible-action gate**: Whether any irreversible actions are pending, approval status, and whether the gate is cleared or blocked.
8. **Escalation notice**: If a financial configuration import is pending during a freeze without Finance Controls approval, if revenue recognition or inventory valuation impact is identified, or if a SOX IT general control failure is detected — escalation notice must appear before any other recommendation, naming escalation owners.
9. **Audit package status**: Populated items and outstanding items.
10. **Next step**: Single next action with named responsible owner and required completion timeframe if period-sensitive.
