---
name: sap-security-hr-legal-protocol
description: Cross-functional escalation protocol governing handoffs between SAP Security, HR, and Legal when identity misuse, privileged-access anomalies, SoD violations, insider-risk signals, HR-sensitive system access, joiner/mover/leaver lifecycle events, or fraud-sensitive access patterns require coordinated response. Defines decision rights, evidence standards, redaction policy, approval gates, and audit packaging. Never mutates live systems and never bypasses any guarded-mutating gate.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: compliance
  lifecycle: experimental
---

# SAP Security HR Legal Escalation Protocol

## Purpose

This skill defines the cross-functional coordination and handoff contract between SAP Security, Human Resources, and Legal when an identity or access event requires multi-domain response. It establishes which agents hold decision rights at each stage, what evidence must be collected and redacted before sharing across functions, which actions are irreversible and require explicit approval, and what the audit package must contain to satisfy internal audit, external regulators, and employment-law obligations.

This is a governance coordination document. It does not mutate any SAP system, does not trigger any provisioning or deprovisioning action, does not close or approve any GRC access violation, and does not initiate any HR or Legal action. All mutations remain gated behind the relevant guarded-mutating operator agents. No guarded gate is bypassed.

## When to use

Invoke this protocol when any of the following trigger conditions apply:

- **Identity misuse or suspected account compromise**: An active user account shows access patterns inconsistent with job function, time-of-access anomalies, bulk data extraction indicators, or repeated failed authentications from unexpected locations.
- **Privileged-access anomalies**: A superuser, firefighter (EAM), basis administrator, or SAP\_ALL holder exhibits access outside an approved change window, accesses HR-sensitive or payroll-relevant transactions not included in the original firefighter scope, or fails to complete mandatory post-access log review.
- **SoD violations with fraud-sensitive transaction pairs**: GRC Access Control raises a critical or high SoD conflict involving procure-to-pay, payroll disbursement, vendor master maintenance, financial period-close journals, or revenue recognition entries — and the conflict involves an active employee (not a system account).
- **Insider-risk signals**: Security monitoring, DLP, or SIEM surfaces exfiltration indicators against HR data stores (SuccessFactors, HCM), payroll systems, financial master data, or confidential business planning data.
- **HR-sensitive system access outside authorized scope**: Access to SuccessFactors Employee Central, compensation planning, performance ratings, succession data, or talent profiles by a user whose role does not include HR stewardship.
- **Joiner/mover/leaver lifecycle gaps**: A leavers process was not completed on the effective date (system access remains active for a departed employee), a mover's previous access was not removed when a new role was granted, or a joiner received access that exceeds the approved role matrix.
- **Fraud-sensitive access escalation**: Finance or audit identifies a pattern of journal entries, payment runs, or vendor changes executed by a single user without SoD separation, and the pattern warrants investigation that may touch HR employment records or legal hold obligations.

## Participating agents

The following agents operate within this protocol. Each holds a defined role and must not act outside its lane without explicit cross-function approval per the decision rights table.

- `sap-security-iam-grc-sod-reviewer-agent` — Lead for identity and access evidence collection, SoD conflict classification, GRC finding triage, and access risk narrative. Does not modify role assignments or provisioning state.
- `sap-successfactors-hr-process-risk-agent` — HR-domain specialist assessing SuccessFactors configuration, HR data access policy compliance, joiner/mover/leaver process integrity, and sensitivity classifications for HR data elements. Does not read or share unredacted HR data outside the HR function.
- `sap-role-assignment-guarded-operator-agent` — The sole agent authorized to execute role assignment or removal actions in SAP systems. All role changes proposed by the protocol must be queued to this agent and require dual approval before execution. This agent is the only guarded-mutating gate in this protocol.

## Required evidence

Before any cross-function handoff is initiated, the following evidence must be assembled by the requesting function:

1. **Identity evidence**: Username, employee ID (if known), system (ECC, S/4HANA, BTP subaccount, SuccessFactors tenant), and access log extract covering the relevant time window. Timestamps must be in UTC.
2. **Access pattern evidence**: Transaction codes accessed, authorization objects exercised, GRC Access Control conflict report (if available), and EAM firefighter log (if the user held a firefighter ID).
3. **Role assignment state**: Current role or role collection assignments, last provisioning event date, IPS provisioning job log (if applicable), and GRC user access review status.
4. **HR lifecycle state**: Joiner date, most recent position change date, effective termination date (if leaver), and HR system-of-record confirmation of current employment status. This evidence is assembled by HR and shared in redacted form per the redaction policy below.
5. **Business process context**: Which business process is affected (P2P, O2C, payroll, financial close, HR master data), whether the access event coincides with a period-end or approval window, and whether any compensating controls exist.

## Redaction policy

Personal HR data (compensation, performance ratings, disciplinary records, health-related information, succession planning data) must be redacted before evidence is shared outside the HR function. When HR evidence is required by Security or Legal:

- HR provides a **redacted summary** confirming employment status, role, effective dates, and whether the lifecycle event (join/move/leave) was processed on time.
- Full HR records are shared only with Legal under legal-hold or employment-law obligation, via a controlled document transfer that is logged in the audit package.
- System access logs that contain HR transaction data are treated as HR-sensitive and are handled by the `sap-successfactors-hr-process-risk-agent`; Security receives a classified summary, not the raw log.
- No unredacted HR data is included in GRC findings, change tickets, or incident reports visible to operational teams.

## Decision rights

| Decision | Primary authority | Secondary approval required | Notes |
|---|---|---|---|
| Classify access event as security incident | SAP Security (sap-security-iam-grc-sod-reviewer-agent) | None for classification | Classification does not trigger any system mutation |
| Suspend or disable a user account | SAP Security + HR jointly | Legal notification if employment action may follow | Must route through sap-role-assignment-guarded-operator-agent; requires dual approval |
| Remove role assignments | SAP Security proposes; HR confirms no operational dependency | sap-role-assignment-guarded-operator-agent executes only after dual approval | Irreversible-action gate applies; see below |
| Place legal hold on access logs | Legal | HR acknowledgment | Triggers evidence preservation; no system mutation by this protocol |
| Disclose access findings to employee | HR + Legal jointly | — | Security provides classified summary only; HR/Legal manage disclosure |
| Close GRC access violation | GRC / audit team | Outside scope of this protocol | This protocol does not close GRC violations |
| Re-grant access post-investigation | HR confirms role legitimacy; Security confirms SoD clearance | Management approval per access matrix | Routes through sap-role-assignment-guarded-operator-agent with all approvals documented |

## Escalation owners

| Scenario | First escalation owner | Second escalation owner |
|---|---|---|
| Unmitigated critical SoD conflict involving active employee | GRC / internal audit | Chief Risk Officer |
| Suspected identity misuse with HR data access | CISO | Chief People Officer |
| Insider-risk signal with potential fraud impact | Chief Risk Officer | General Counsel |
| Leavers process gap with post-termination access | HR Operations | CISO |
| Employment-law or disciplinary dimension | General Counsel | Chief People Officer |
| Regulatory reporting obligation triggered | Chief Risk Officer + General Counsel jointly | — |

## Irreversible-action gate

The following actions are classified as irreversible or high-consequence and must not proceed without the approvals listed:

- **Account suspension or disablement**: Requires written approval from CISO and VP HR. Approval must be recorded in the audit package before the action is queued to `sap-role-assignment-guarded-operator-agent`.
- **Role assignment removal in production**: Requires written approval from SAP Security lead and the business process owner of the affected function. If the removal affects payroll or financial-close roles, Finance Controls must also approve.
- **Permanent account deletion or anonymization**: Requires Legal sign-off confirming no legal-hold obligations attach to the account, plus HR confirmation that the employee lifecycle is fully closed.
- **Evidence deletion or log purge**: Prohibited during active investigation. Any retention policy execution that would delete relevant logs must be suspended via Legal hold.

This protocol does not execute any irreversible action. It surfaces the required approvals and queues the action to the appropriate guarded-mutating agent once all approvals are documented.

## Approval requirements

All cross-function actions under this protocol require approvals to be documented in writing (ticket, email chain, or signed document) before execution. Verbal approvals are not accepted. The audit package must include the approval record for every action taken.

Minimum approval quorum per action class:

- **Security-only actions** (classification, evidence collection): SAP Security lead sign-off.
- **HR-touching actions** (leaver processing, role removal for HR data access): SAP Security lead + HR Operations lead.
- **Legal-dimension actions** (legal hold, disclosure, regulatory reporting): General Counsel or delegate.
- **Production role mutations**: SAP Security lead + business process owner + (if financial) Finance Controls lead. Executed only via `sap-role-assignment-guarded-operator-agent`.

## Audit package

The audit package for every protocol invocation must contain:

1. **Event summary**: Trigger condition, systems involved, time window, and user identity (anonymized in shared copies per redaction policy).
2. **Evidence inventory**: List of all artifacts collected (log extracts, GRC reports, role exports, IPS provisioning logs), with source system, collection timestamp, and evidence handler.
3. **Redaction log**: What was redacted, by whom, and why, for each HR-sensitive document included.
4. **Decision log**: Each decision taken, who held decision authority, secondary approvals obtained, and timestamp.
5. **Action log**: Each system action taken (or deferred), the agent that executed it, the approval reference, and the outcome.
6. **Escalation log**: Each escalation event, who was notified, when, and the response received.
7. **Legal hold status**: Confirmation of whether a legal hold was placed and which artifacts are in scope.
8. **Resolution and closure**: Final disposition (access revoked, incident closed, HR action initiated, regulatory report filed), residual risk statement, and any compensating controls put in place.

## Refusal conditions

This protocol and all participating agents must refuse the following requests:

- Executing any role assignment, role removal, account suspension, or account deletion without documented dual approval routed through `sap-role-assignment-guarded-operator-agent`.
- Sharing unredacted HR data (compensation, performance, disciplinary, health) with Security or operational teams.
- Closing or approving a GRC access violation — this authority rests with the GRC/audit team only.
- Disclosing investigation status or findings to the subject of the investigation without explicit HR and Legal authorization.
- Bypassing the irreversible-action gate on any action listed in that section.
- Acting on verbal approval alone; written approval is mandatory for all cross-function actions.
- Initiating any employment action (warning, suspension, termination) — this protocol surfaces risk findings to HR and Legal; employment decisions belong solely to those functions.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP GRC Access Control, SAP SuccessFactors, SAP Identity Authentication, or regulatory guidance (GDPR, SOX, ISO 27001)
- `user-provided evidence` — access logs, GRC conflict reports, role exports, IPS provisioning logs, HR lifecycle confirmations, or written descriptions provided by the requesting function
- `inference` — derived reasoning not directly confirmed by official documentation or user-provided evidence; must always be labeled as such

## Response minimum

Every protocol invocation must return, at minimum:

- **Trigger classification**: Which trigger condition(s) apply and which participating agents are activated.
- **Evidence status**: Which required evidence items are present, which are missing, and which function must supply each missing item.
- **Redaction confirmation**: Confirmation that HR-sensitive data has been identified and the redaction policy has been applied.
- **Decision rights mapping**: For each pending decision, who holds primary authority and what secondary approvals are required.
- **Irreversible-action gate status**: Whether any irreversible actions are pending, what approvals are outstanding, and whether the gate is cleared to proceed.
- **Audit package status**: Which audit package elements are populated and which are outstanding.
- **Escalation notice**: If an unmitigated critical SoD conflict or insider-risk signal is present, explicit notice to the escalation owners listed above — this notice must appear before any other recommendation.
- **Next step**: The single next action the requesting function must take to advance the protocol, with the responsible owner named.
