---
name: sap-release-cutover-finance-controls-protocol
description: Cross-functional coordination protocol governing handoffs between SAP Release Management, Business Process Owners, and Finance Controls for transport imports, production freeze enforcement, cutover readiness, financial-period close, O2C/P2P business process continuity, inventory valuation, revenue recognition impact, and hypercare. Defines go/no-go decision rights, irreversible-action gates, rollback requirements, and audit packaging. Never mutates live systems and never bypasses any guarded-mutating gate.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: compliance
  lifecycle: experimental
---

# SAP Release Cutover Finance Controls Protocol

## Purpose

This skill defines the cross-functional coordination and handoff contract between SAP Release Management, Business Process Owners, and Finance Controls when a transport import, production system cutover, or release-window event intersects with financial-period integrity, business process continuity, or hypercare obligations. It establishes go/no-go decision rights, evidence requirements for release readiness, rollback plan requirements, financial controls impact assessment gates, and the audit package structure required for SOX and internal audit compliance.

This is a governance coordination document. It does not import transports, execute cutover scripts, modify financial posting periods, close sales documents, or trigger any system action. All mutations remain gated behind the relevant guarded-mutating operator agents. No guarded gate is bypassed.

## When to use

Invoke this protocol when any of the following trigger conditions apply:

- **Transport import to production**: One or more transports are queued for import into an SAP production system (ECC, S/4HANA on-premise or Private Cloud Edition, SAP BTP ABAP environment) and the import window intersects with a financial-period boundary, a business-critical operational window (payroll run, customer invoice processing, period-end closing cockpit activity), or a known high-risk change category.
- **Production freeze enforcement**: A production freeze period has been declared (period-end close, quarter-end, year-end, regulatory reporting deadline, merger/acquisition close date, major SAP upgrade) and a change request has been submitted requesting an exception to the freeze.
- **Cutover readiness**: An SAP implementation or upgrade project is approaching the go-live cutover date and the cutover readiness checklist (technical, functional, data migration, and finance controls) has not been fully signed off.
- **Financial-period close impact**: A pending or recently completed transport modifies financial configuration (posting period variants, document types, account determination, tax codes, fiscal year variant, controlling area settings) or business logic affecting financial postings (pricing procedures, output determination, goods movement account assignment, revenue account mapping).
- **O2C or P2P business process disruption**: A transport, configuration change, or system migration has caused or risks causing disruption to order-to-cash (O2C) or procure-to-pay (P2P) processes during a period when business volume is high or process interruption has material revenue or liability impact.
- **Inventory valuation impact**: A transport or migration affects material ledger configuration, standard cost estimates, moving average price calculation, or valuation area assignment in a way that could alter inventory values on the balance sheet.
- **Revenue recognition impact**: A transport or configuration change modifies SAP Revenue Accounting and Reporting (RAR) setup, SD billing plan configuration, contract account assignment, or multi-element arrangement allocation in a way that could affect revenue recognized in the current or prior period.
- **Hypercare escalation**: The system is in a defined hypercare period post-go-live and an incident, defect, or configuration issue has been escalated that requires a production correction outside the standard change process.

## Participating agents

The following agents operate within this protocol. Each holds a defined role and must not act outside its lane without explicit cross-function approval per the decision rights table.

- `sap-release-change-collision-agent` — Release management specialist responsible for transport dependency analysis, change collision detection, release schedule governance, and production freeze enforcement. Assesses transport content, identifies conflicts, and confirms release window eligibility. Does not import transports.
- `sap-data-migration-cutover-readiness-agent` — Cutover readiness specialist assessing data migration completeness, cutover checklist status, go-live readiness scoring, and rollback trigger conditions. Does not execute cutover scripts or migration programs.
- `sap-finance-fico-controls-agent` — Finance Controls specialist assessing the financial impact of transports, configuration changes, and cutover events on FICO configuration integrity, posting period controls, financial statement accuracy, SOX IT general controls, and period-end close sequencing. Does not modify financial configuration.
- `sap-guarded-transport-import-operator-agent` — The sole agent authorized to execute transport import actions in SAP systems. All transport imports proposed by this protocol must be queued to this agent and require all approvals listed in the decision rights table before execution. This agent is the primary guarded-mutating gate for release actions in this protocol.
- `sap-hypercare-incident-commander-agent` — Hypercare coordination specialist responsible for incident triage, severity classification, resolution path governance, business impact assessment, and escalation to SAP Support or escalation management. Does not execute production changes directly; routes approved corrections through the guarded-transport-import-operator-agent.

## Required evidence

Before any cross-function handoff or go/no-go decision is initiated, the following evidence must be assembled by the requesting function:

1. **Transport manifest**: Complete list of transports in the import queue for the target production system, with transport number, description, development system origin, last test system import result, and responsible developer/configuration consultant.
2. **Change collision analysis**: Output from `sap-release-change-collision-agent` confirming whether any transports in the queue conflict with other queued or recently imported transports on shared configuration objects.
3. **Financial impact assessment**: Output from `sap-finance-fico-controls-agent` classifying each transport by financial impact category (no financial impact / financial configuration change / financial posting logic change / period-sensitive financial change) and identifying any transports that modify configuration affecting current-period financial statements.
4. **Test evidence**: Import results from the quality assurance system (or pre-production equivalent), regression test execution summary, and sign-off from the functional team lead for each affected business process.
5. **Business process owner sign-off**: Written confirmation from the process owner for each affected business process (O2C, P2P, Finance, HR) that the tested change is approved for production import.
6. **Rollback plan**: Documented rollback approach for each transport in the queue that is classified as high-risk or period-sensitive, including the rollback steps, estimated rollback duration, rollback trigger conditions, and the individual authorized to invoke rollback.
7. **Freeze exception justification** (if applicable): For changes submitted during a production freeze, a written justification from the requesting team, classification as critical/high/medium risk, and Finance Controls confirmation that the change is safe to import during the freeze window.
8. **Cutover readiness checklist status** (if applicable): Completed checklist covering technical prerequisites, data migration sign-off, financial opening balance validation, user training completion, support model readiness, and hypercare plan confirmation.
9. **Hypercare incident record** (if applicable): Incident ticket number, symptom description, affected business process, user and transaction volume impact, workaround status, and proposed correction transport details.

## Redaction policy

Sensitive financial data must be treated as follows before evidence is shared across functions or included in audit documentation:

- **Financial configuration values**: Posting period variants, tax code rates, account determination keys, and pricing condition values are included in evidence packages but are marked as restricted — access limited to Finance Controls and internal audit. Operational IT teams see the object names and change descriptions, not the configuration values.
- **Transport object lists**: Full ABAP object lists for transports are shared with the release manager and Finance Controls. Business process owners receive a functional impact summary, not the raw object list.
- **Period-end close status**: Current period-end close status (open items, unposted documents, blocked periods) is shared with Finance Controls and the business process owner. It is not included in general release documentation visible to the broader IT team.
- **Hypercare incident details**: Incident descriptions that include specific customer or vendor account numbers, financial amounts, or contract details are redacted in shared status reports — Finance Controls and the incident commander receive full details; stakeholder communications receive a business impact summary.

## Decision rights

| Decision | Primary authority | Secondary approval required | Notes |
|---|---|---|---|
| Approve transport import to production outside a freeze | Release Manager | Business Process Owner (affected process) + Finance Controls (if financial impact) | Executes via sap-guarded-transport-import-operator-agent |
| Approve freeze exception for critical production correction | CISO or Release Governance Board | Finance Controls (mandatory) + CIO or delegate | Written justification and risk acceptance required |
| Declare go/no-go for cutover | Project Steering Committee or designated Go-Live Authority | Finance Controls sign-off + Business Process Owner sign-off for each in-scope process | All checklist items must be confirmed green or exceptions documented with acceptance |
| Invoke production rollback | Release Manager (with CIO notification) | Finance Controls (if financial configuration is being rolled back) | Rollback trigger conditions must be met; rollback plan must be pre-approved |
| Extend production freeze | CIO or Release Governance Board | Finance Controls concurrence | Written communication to all stakeholders required |
| Classify a hypercare incident as critical | sap-hypercare-incident-commander-agent | Business Process Owner acknowledgment | Critical classification triggers SAP escalation path and executive notification |
| Approve a production correction transport during hypercare | sap-hypercare-incident-commander-agent + Release Manager | Finance Controls (if financial impact) | Executes via sap-guarded-transport-import-operator-agent after all approvals |
| Close hypercare and transition to steady state | Project Steering Committee | Release Manager + Finance Controls | Requires no open critical or high incidents and stable KPI baselines |

## Escalation owners

| Scenario | First escalation owner | Second escalation owner |
|---|---|---|
| Transport import causes financial statement impact in current period | Finance Controls lead | CFO and external auditor notification |
| Production freeze exception request for a non-critical change | Release Governance Board | CIO |
| Cutover go/no-go blocked due to open financial migration items | Project Steering Committee | CFO |
| O2C or P2P disruption causing revenue or liability impact during hypercare | sap-hypercare-incident-commander-agent | CFO + CIO |
| Inventory valuation error affecting balance sheet in closed period | Finance Controls lead | CFO + external auditor notification |
| Revenue recognition configuration error affecting recognized revenue | Finance Controls lead + General Counsel | CFO + external auditor notification |
| Rollback invoked for a financial configuration transport | Finance Controls lead | CFO + CIO |
| SOX IT general control failure identified during release or cutover | Internal Audit | CFO + Chief Compliance Officer |

## Irreversible-action gate

The following actions are classified as irreversible or high-consequence and must not proceed without the approvals listed:

- **Transport import to production modifying financial configuration (posting periods, account determination, tax codes, fiscal year variant)**: Requires Finance Controls written approval confirming the change is safe to import in the current period and does not affect closed-period financial statements. Change must be tested in QA system with Finance Controls sign-off on test results.
- **Period-end close sequence interruption (holding or re-opening a closed period)**: Requires CFO or Finance Director written approval. Re-opening a closed posting period to correct an error has accounting and audit implications that cannot be undone by re-closing — the audit trail is permanent.
- **Go-live cutover execution**: Requires Project Steering Committee written go/no-go decision with all checklist items confirmed or documented exceptions with named acceptance owners. Cutover execution cannot be reversed without invoking the rollback plan.
- **Production rollback of financial configuration**: Requires Finance Controls written confirmation that rollback will restore financial integrity rather than create a new inconsistency, plus CFO notification. Rollback of financial configuration changes may itself create journal entry discrepancies requiring manual correction.
- **Freeze exception import during year-end or quarter-end close**: Requires Finance Controls and CFO written approval. Changes during the final days of a financial period carry risk that cannot be fully mitigated by testing alone; executive acceptance of residual risk is required.

This protocol does not execute any of these actions. It surfaces the required approvals and routes the action to `sap-guarded-transport-import-operator-agent` once all approvals are documented.

## Approval requirements

All cross-function release and cutover actions under this protocol require written approval before execution. The audit package must include the approval record for every gate cleared.

Minimum approval quorum per action class:

- **Standard production transport import**: Release Manager + Business Process Owner (affected process).
- **Transport with financial configuration impact**: Release Manager + Business Process Owner + Finance Controls lead.
- **Freeze exception import**: Release Governance Board chair + Finance Controls lead + CIO or delegate.
- **Go-live cutover**: Project Steering Committee (or designated Go-Live Authority) + Finance Controls sign-off + all Business Process Owner sign-offs.
- **Rollback invocation**: Release Manager + Finance Controls lead (if financial config in scope) + CIO notification.
- **Hypercare production correction**: sap-hypercare-incident-commander-agent lead + Release Manager + Finance Controls lead (if financial impact).

## Audit package

The audit package for every protocol invocation must contain:

1. **Event summary**: Trigger condition, target production system, transport queue or cutover scope, and affected business processes.
2. **Transport manifest**: All transports in scope with transport numbers, descriptions, authors, and test system import results.
3. **Change collision analysis**: Output from `sap-release-change-collision-agent` with collision status per transport.
4. **Financial impact assessment**: Classification per transport from `sap-finance-fico-controls-agent`, with financial objects affected and period-sensitivity rating.
5. **Test evidence**: QA import results, regression test summary, and functional sign-off records per business process.
6. **Approval records**: Written approvals for each gate cleared, with approver identity, date, and scope of approval.
7. **Rollback plan reference**: Rollback plan version, pre-approval status, and trigger conditions.
8. **Freeze exception record** (if applicable): Justification, risk classification, Finance Controls assessment, and approval chain.
9. **Go/no-go decision record** (if applicable): Decision outcome, checklist sign-off status, and any documented exceptions with named acceptance owners.
10. **Irreversible-action gate log**: For each irreversible action, the gate status (cleared / blocked / pending) and the approval documentation reference.
11. **Escalation log**: Each escalation event, who was notified, when, and the response received.
12. **Hypercare incident record** (if applicable): Incident details, severity classification, resolution timeline, and production correction approval chain.
13. **Residual risk statement**: Any release or financial controls risks not fully mitigated, the rationale for acceptance, and the compensating controls in place.

## Refusal conditions

This protocol and all participating agents must refuse the following requests:

- Queuing a transport for import to production via `sap-guarded-transport-import-operator-agent` without all required written approvals.
- Approving or recommending approval of a freeze exception without Finance Controls written assessment.
- Declaring cutover go/no-go without a fully assessed readiness checklist and documented sign-offs from all required functions.
- Recommending rollback of financial configuration without Finance Controls confirmation that the rollback restores rather than further disrupts financial integrity.
- Accepting verbal approval for any cross-function release, cutover, or financial controls action.
- Bypassing any irreversible-action gate listed in this protocol.
- Importing or recommending import of transports that modify closed-period financial configuration without CFO and Finance Controls written approval.
- Closing a hypercare incident as resolved without Business Process Owner confirmation and KPI baseline validation.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP transport management, SAP financial accounting configuration, SAP Activate methodology, SOX IT general controls guidance, or ITIL change management documentation
- `user-provided evidence` — transport manifests, QA import results, regression test summaries, readiness checklist outputs, financial impact assessments, approval records, or incident records provided by the requesting function
- `inference` — derived reasoning not directly confirmed by official documentation or user-provided evidence; must always be labeled as such

## Response minimum

Every protocol invocation must return, at minimum:

- **Trigger classification**: Which trigger condition(s) apply and which participating agents are activated.
- **Transport and scope inventory**: Transports or cutover scope items assessed, financial impact classification per item, and collision analysis status.
- **Go/no-go posture**: Current go/no-go assessment based on available evidence, with blocking conditions listed explicitly.
- **Financial controls gate status**: Whether Finance Controls assessment is complete, pending, or flagged with a blocking finding.
- **Rollback plan status**: Whether a pre-approved rollback plan exists for each high-risk or period-sensitive item in scope.
- **Approval gate status**: For each required approval, primary authority, documentation status (present / outstanding), and gate status (cleared / blocked).
- **Irreversible-action gate status**: Whether any irreversible actions are pending, approval status, and whether the gate is cleared or blocked.
- **Escalation notice**: If a financial configuration import is pending during a freeze without required approvals, if a revenue recognition or inventory valuation impact is identified, or if a SOX IT general control failure is detected — the escalation notice must appear before any other recommendation, naming the escalation owners.
- **Audit package status**: Which audit package elements are populated and which are outstanding.
- **Next step**: The single next action the requesting function must take, with the responsible owner named.
