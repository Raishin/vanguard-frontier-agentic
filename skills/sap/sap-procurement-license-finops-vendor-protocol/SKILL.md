---
name: sap-procurement-license-finops-vendor-protocol
description: Cross-functional coordination protocol governing handoff contracts between SAP Procurement/License, FinOps, and Vendor Management. Activates on BTP consumption spikes, entitlement changes, license compliance events, RISE/SLA issues, vendor lock-in exposure, contractual risk, underused subscriptions, and over-provisioned services. Advisory and audit only — no live mutation.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: finops
  lifecycle: experimental
---

# SAP Procurement / License / FinOps / Vendor Protocol

## Purpose

This skill is a cross-functional coordination and handoff contract between four complementary advisory roles: SAP license and BTP consumption analysis, procurement and Ariba value-leakage review, RISE/SLA and vendor-risk assessment, and BTP entitlement governance oversight. It defines when each role activates, what evidence each role requires before it can produce output, how findings are handed off between roles, who holds decision rights over contractual and provisioning actions, and what approval is required before any irreversible change is recommended to a downstream guarded-mutating operator.

This protocol never mutates. It never invokes or bypasses any guarded-mutating operator gate. It produces advisory packages, handoff summaries, and escalation triggers only.

## When to use

Activate this protocol when one or more of the following conditions apply:

- BTP service consumption has spiked unexpectedly — credits consumed faster than forecast, or a subaccount is consuming entitlements beyond its approved budget.
- An entitlement change is planned or requested — new service enablement, quota increase, or entitlement transfer between subaccounts or directories.
- A license compliance review is required — SAP S/4HANA user counts, named user vs. concurrent user reclassification, indirect access exposure, or measurement audit preparation.
- A RISE with SAP contract event arises — SLA breach, support escalation, hyperscaler region change, contract renewal, or exit clause evaluation.
- Vendor lock-in risk has been identified — single-vendor dependency on BTP services with no exit path documented, or architectural decisions that increase switching costs without business justification.
- Contractual exposure has been flagged — penalty clauses, auto-renewal commitments, minimum consumption guarantees, or true-up obligations approaching a deadline.
- Underused or idle subscriptions exist — BTP service subscriptions that have not been consumed for 60 days or more, or CF/Kyma environments with zero workloads.
- Over-provisioned services are suspected — entitlement quota far exceeding actual measured consumption, or CPEA/BTPEA credits allocated to services with near-zero usage.

## Participating agents

The following agents are parties to this protocol. Each operates in its own advisory domain. No agent in this list holds authority to execute irreversible changes unilaterally.

- `sap-license-btp-consumption-finops-agent` — analyzes BTP credit consumption, CPEA/BTPEA burn rate, service-level cost attribution, and SAP license measurement data. Primary entry point for consumption spike triage.
- `sap-procurement-ariba-value-leakage-agent` — reviews SAP Ariba sourcing workflows, contract compliance, maverick spend, and procurement-side value leakage tied to SAP vendor agreements and BTP commercial terms.
- `sap-rise-sla-vendor-risk-agent` — assesses RISE with SAP SLA performance, hyperscaler dependency, vendor exit risk, contractual penalty exposure, and strategic vendor concentration risk.
- `sap-btp-entitlement-guarded-operator-agent` — holds the guarded mutation gate for any BTP entitlement change. No entitlement addition, removal, or quota modification proceeds without a signed handoff package from this protocol and explicit human approval. This agent does not activate from this protocol directly; the protocol produces the package that a human submits to the operator gate.

## Required evidence

Before this protocol can produce a handoff package, the activating party must supply:

1. **Consumption snapshot** — BTP cockpit usage report, CPEA/BTPEA credit statement, or equivalent export showing actual vs. allocated consumption for the relevant period.
2. **Entitlement inventory** — current entitlement assignments at global account, directory, and subaccount level, including service plan names and quota values.
3. **Contract reference** — relevant SAP contract ID, RISE agreement reference, or Ariba contract document (redacted of commercially sensitive terms if necessary) identifying the obligation in scope.
4. **Trigger description** — a written description of the event that triggered this protocol (spike alert, audit notice, renewal deadline, SLA breach report, etc.).
5. **Business owner confirmation** — identification of the named business owner or cost center responsible for the entitlements or subscriptions under review.

Optional but recommended:

- SAP license measurement report or user count extract for license compliance reviews.
- Vendor contract schedule or SLA annexe for RISE/SLA events.
- Ariba contract compliance report for procurement-side leakage reviews.

## Redaction policy

All evidence submitted to this protocol must be redacted before processing:

- Remove all SAP contract IDs, agreement numbers, and commercial unit prices.
- Remove all personally identifiable information (PII) from user count reports.
- Remove all OAuth tokens, service keys, API keys, and platform credentials.
- Remove customer names, tenant IDs, and internal cost center codes beyond the minimum needed to identify the responsible business unit.
- Label redacted fields with `[REDACTED]`.

This protocol does not store, persist, or transmit evidence artifacts. Evidence is used only within the advisory session in which it is provided.

## Decision rights

| Decision | Owner | Protocol role |
|----------|-------|---------------|
| Approve entitlement quota increase | Global Account Administrator + Finance sponsor | Protocol produces recommendation; human approves |
| Approve entitlement quota reduction | Global Account Administrator | Protocol produces recommendation; human approves |
| Initiate RISE contract escalation | SAP Account Executive + Procurement lead | Protocol produces escalation brief; human initiates |
| Approve subscription termination | Business owner + FinOps lead | Protocol produces risk summary; human approves |
| Authorize true-up payment or credit purchase | CFO or delegate | Protocol produces cost exposure summary; human authorizes |
| Flag vendor lock-in for architecture review | Enterprise Architect + CTO | Protocol raises flag; human schedules review |

## Escalation owners

- **FinOps lead** — primary escalation point for consumption anomalies and credit burn rate deviations.
- **SAP Commercial Manager / Account Executive** — escalation point for contract compliance, RISE SLA breaches, and true-up events.
- **Enterprise Architect** — escalation point for vendor lock-in risk and architectural dependency findings.
- **Chief Procurement Officer or delegate** — escalation point for contractual penalty exposure and sourcing governance failures.
- **Global Account Administrator** — escalation point for all BTP entitlement governance findings that require platform-level action.

## Irreversible-action gate

The following actions are classified as irreversible or high-consequence and require explicit human approval before any recommendation is executed:

- Reducing entitlement quota below current consumption (terminates running service instances).
- Removing a service subscription that has active bindings or deployed applications.
- Triggering a contract exit clause or termination notice.
- Reclassifying SAP named users in a way that reduces license count (creates compliance exposure if later reversed).
- Decommissioning a BTP subaccount with active workloads.

This protocol does not invoke `sap-btp-entitlement-guarded-operator-agent` or any other operator gate. It produces a signed handoff package. A human must present that package to the operator gate and confirm approval before any mutation proceeds.

## Approval requirements

| Action type | Minimum approvers | Approval form |
|-------------|------------------|---------------|
| Quota increase (any service) | Global Account Admin + FinOps lead | Written sign-off in change record |
| Quota reduction | Global Account Admin + Business owner | Written sign-off in change record with consumption evidence attached |
| Subscription termination | Business owner + FinOps lead + Legal review if contractual | Signed decommission record |
| RISE escalation submission | Procurement lead + Account Executive | Approved escalation brief |
| Credit purchase | Finance sponsor | Purchase order or approved budget line |

## Audit package

At session close, this protocol produces an audit package containing:

1. **Trigger summary** — what event activated the protocol and when.
2. **Evidence inventory** — list of evidence artifacts received (types only; no raw content), with redaction confirmation.
3. **Findings per participating role** — advisory output from each relevant agent role, labelled by evidence level.
4. **Decision rights mapping** — which decisions are pending and who holds approval authority.
5. **Irreversible-action gate status** — whether any irreversible action has been recommended, and whether human approval has been confirmed.
6. **Escalation log** — which escalation owners were notified and what action was requested.
7. **Handoff status** — whether the package is ready to submit to a guarded-mutating operator gate.

The audit package must be retained by the activating team for a minimum of 12 months or for the duration of the relevant contract, whichever is longer.

## Refusal conditions

This protocol refuses to proceed when:

- No consumption snapshot or entitlement inventory has been provided — advisory output without evidence is not reliable.
- A request is made to directly invoke `sap-btp-entitlement-guarded-operator-agent` or any other operator gate from within this protocol — that gate is human-mediated only.
- Raw contract pricing, commercial unit prices, or supplier margin data are submitted without redaction — request redaction before proceeding.
- A request is made to recommend an irreversible action without identifying the human approver — the approver must be named before the package is finalized.
- The trigger event is outside the scope defined in "When to use" — redirect to the appropriate protocol or skill.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP Help Portal, SAP RISE documentation, SAP Ariba product documentation, or official SAP FinOps guidance.
- `user-provided evidence` — consumption exports, entitlement summaries, contract references, or measurement reports supplied by the user.
- `inference` — derived reasoning not directly confirmed by official documentation or user evidence; always label as inference and note the assumption.

## Response minimum

Every protocol session must return, at minimum:

- **Trigger classification** — which "When to use" condition(s) activated the protocol.
- **Participating roles activated** — which of the four agents' advisory domains are relevant to this session.
- **Evidence received** — inventory of what was supplied, with confirmation of redaction compliance.
- **Findings per domain** — advisory output for each activated role, with evidence labels.
- **Decision rights table** — who must approve each pending action.
- **Irreversible-action gate status** — whether irreversible actions are in scope and whether human approval has been confirmed.
- **Audit package readiness** — whether the session is complete enough to produce a handoff package for the operator gate.
- **Next step** — the specific human action required to proceed (e.g., "Submit this package to Global Account Administrator for quota reduction approval").
