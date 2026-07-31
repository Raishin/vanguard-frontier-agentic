---
name: sap-integration-platform-businessops-protocol
description: Cross-functional coordination protocol governing handoff contracts between SAP Integration, Platform Engineering, and Business Operations. Activates on failed integration flows, API throttling, event delivery failures, broken partner integrations, data replication failures, middleware instability, and business-process outage. Advisory and audit only — no live mutation.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-19"
  category: operational
  lifecycle: experimental
---

# SAP Integration / Platform / Business Operations Protocol

## Purpose

This skill is a cross-functional coordination and handoff contract between four complementary advisory roles: SAP Integration Suite flow review, Cloud ALM SRE incident analysis, integration flow governance oversight, and BTP account entitlement governance review. It defines when each role activates, what evidence each role requires before it can produce output, how findings are handed off between roles, who holds decision rights over integration and platform remediation actions, and what approval is required before any irreversible change is recommended to a downstream guarded-mutating operator.

This protocol never mutates. It never invokes or bypasses any guarded-mutating operator gate. It produces advisory packages, incident handoff summaries, and escalation triggers only.

## When to use

Activate this protocol when one or more of the following conditions apply:

- An integration flow has failed, is in an error state, or is producing incorrect output — including iFlow failures in SAP Integration Suite, failed message mappings, or broken adapter configurations.
- API throttling or rate-limit responses are being returned by SAP Integration Suite, SAP API Business Hub, or a connected system — impacting business processes that depend on real-time data exchange.
- Event delivery has failed — including SAP Event Mesh messages that are stuck, undelivered, or accumulating in dead-letter queues.
- A partner or third-party integration has broken — EDI, B2B, or external API integration with a supplier, logistics provider, or financial institution has stopped exchanging data correctly.
- Data replication has failed — CDS view replication, SAP Landscape Transformation (SLT), or SOAP/REST-based data synchronization between SAP and non-SAP systems is producing stale, incomplete, or corrupt data.
- Middleware instability is observed — the SAP Integration Suite tenant, a deployed Node instance, or an on-premise Integration Agent is behaving erratically, restarting unexpectedly, or failing health checks.
- A business-process outage has been declared — a critical business process (order-to-cash, procure-to-pay, hire-to-retire, or equivalent) is interrupted and the root cause is suspected to be in the integration or platform layer.

## Participating agents

The following agents are parties to this protocol. Each operates in its own advisory domain. No agent in this list holds authority to execute irreversible changes unilaterally.

- `sap-integration-suite-reviewer-agent` — reviews SAP Integration Suite iFlow design, adapter configuration, message mapping logic, error handling, retry policies, and monitoring artifacts. Primary entry point for integration flow failure triage.
- `sap-cloud-alm-sre-incident-agent` — analyzes SAP Cloud ALM health monitoring data, incident records, service desk tickets, and SRE runbook compliance for platform and integration incidents. Produces incident severity classification and escalation recommendations.
- `sap-integration-flow-guarded-operator-agent` — holds the guarded mutation gate for any integration flow deployment, redeployment, undeploy, or configuration change. No integration flow change proceeds without a signed handoff package from this protocol and explicit human approval. This agent does not activate from this protocol directly; the protocol produces the package that a human submits to the operator gate.
- `sap-btp-account-entitlement-governance-reviewer-agent` — reviews BTP account and entitlement governance context when platform-layer issues (missing service entitlements, quota exhaustion, environment provisioning gaps) are a contributing factor to integration or business-process outage.

## Required evidence

Before this protocol can produce a handoff package, the activating party must supply:

1. **Incident or error description** — a written description of the failure, including error messages, error codes, impacted integration flows or APIs, and the business process affected.
2. **Integration flow identifier** — the iFlow ID or name, tenant URL (sanitized — no credentials), and the SAP Integration Suite runtime node or cluster on which the failure occurred.
3. **Timeline** — when the failure first occurred, whether it is ongoing, and what changed immediately before the failure (deployment, configuration update, dependency change, or nothing identified).
4. **Monitoring artifacts** — message processing log excerpts, Cloud ALM alert details, or equivalent monitoring data (redacted of PII and credentials).
5. **Business impact statement** — which business process is affected, how many transactions or users are impacted, and whether a business-process outage has been declared.

Optional but recommended:

- SAP Cloud ALM health monitoring dashboard export or screenshot.
- Adapter trace log or message processing log for the failing message.
- Partner integration acknowledgment or error response (redacted of credentials).
- Current entitlement inventory if platform-layer quota exhaustion is suspected.

## Redaction policy

All evidence submitted to this protocol must be redacted before processing:

- Remove all OAuth tokens, Basic Authentication credentials, client secrets, API keys, and certificate private keys from adapter configurations and trace logs.
- Remove all personally identifiable information (PII) from message payloads — customer names, personal identification numbers, payment card data, health data.
- Remove all internal hostnames, IP addresses, and tenant-specific URLs beyond the minimum needed to identify the integration landscape topology.
- Remove all SAP system IDs (SIDs), client numbers, and basis user credentials.
- Label redacted fields with `[REDACTED]`.

This protocol does not store, persist, or transmit evidence artifacts. Evidence is used only within the advisory session in which it is provided.

## Decision rights

| Decision | Owner | Protocol role |
|----------|-------|---------------|
| Approve integration flow redeployment | Integration Platform Lead + Business Process Owner | Protocol produces recommendation; human approves |
| Approve iFlow undeploy (stopping an active flow) | Integration Platform Lead | Protocol produces risk assessment; human approves |
| Approve adapter configuration change | Integration Platform Lead + Security review if authentication scope changes | Protocol produces recommendation; human approves |
| Escalate to SAP support (incident ticket) | SAP Cloud ALM SRE lead | Protocol produces escalation brief; human opens ticket |
| Declare business-process outage resolved | Business Process Owner | Protocol confirms integration health signals; human declares resolution |
| Approve BTP entitlement change to restore service | Global Account Administrator + FinOps lead | Protocol produces recommendation; human approves via separate FinOps protocol if needed |
| Approve partner integration reconfiguration | Integration Platform Lead + Partner contact | Protocol produces change summary; human coordinates with partner |

## Escalation owners

- **Integration Platform Lead** — primary escalation point for iFlow failures, adapter configuration issues, and middleware instability.
- **SAP Cloud ALM SRE Lead** — escalation point for platform-level health degradation, incident management, and SRE runbook activation.
- **Business Process Owner** — escalation point for business-process outage declaration and resolution confirmation.
- **SAP Support** — escalation point for platform defects, known issues in SAP Integration Suite, and hyperscaler infrastructure events affecting the tenant.
- **Partner Integration Contact** — escalation point for B2B/EDI failures or third-party API integration breaks that require out-of-band coordination with the external party.
- **BTP Global Account Administrator** — escalation point when entitlement or environment provisioning is a contributing factor.

## Irreversible-action gate

The following actions are classified as irreversible or high-consequence and require explicit human approval before any recommendation is executed:

- Undeploying a running integration flow that is processing live business transactions (stops all in-flight message processing immediately).
- Changing the technical sender or receiver adapter credentials that affect all messages in the active channel.
- Restarting the SAP Integration Suite runtime node or worker (drops all in-memory message state).
- Disabling an Event Mesh topic subscription (causes immediate message loss for events published after the disable).
- Removing an iFlow from the integration package (not recoverable without redeployment from a versioned artifact).
- Revoking API credentials or OAuth client registrations used by partner integrations (immediately breaks all partner channels using those credentials).

This protocol does not invoke `sap-integration-flow-guarded-operator-agent` or any other operator gate. It produces a signed handoff package. A human must present that package to the operator gate and confirm approval before any mutation proceeds.

## Approval requirements

| Action type | Minimum approvers | Approval form |
|-------------|------------------|---------------|
| Integration flow redeployment | Integration Platform Lead | Written approval in change record with artifact version reference |
| iFlow undeploy | Integration Platform Lead + Business Process Owner | Written approval with business impact acknowledgment |
| Adapter credential rotation | Integration Platform Lead + Security lead | Written approval with credential rotation procedure attached |
| Event Mesh subscription change | Integration Platform Lead | Written approval with message-loss risk acknowledgment |
| BTP entitlement change (platform layer) | Global Account Admin + FinOps lead | Separate FinOps protocol handoff required |
| SAP support escalation (severity 1) | SAP Cloud ALM SRE Lead | Verbal or written authorization with incident reference |

## Audit package

At session close, this protocol produces an audit package containing:

1. **Incident or failure summary** — trigger classification, impacted flows and business processes, and timeline.
2. **Evidence inventory** — list of evidence artifacts received (types only; no raw content), with redaction confirmation.
3. **Per-role findings** — advisory output from each activated agent role, labelled by evidence level and severity.
4. **Root cause hypothesis** — most likely root cause with evidence basis and confidence level (confirmed / probable / possible / unknown).
5. **Decision rights mapping** — pending decisions and named approvers for each recommended action.
6. **Irreversible-action gate status** — whether any irreversible action has been recommended, and whether human approval has been confirmed.
7. **Escalation log** — which escalation owners were notified and what action was requested.
8. **Handoff status** — whether the package is ready to submit to a guarded-mutating operator gate.
9. **Business process resolution status** — confirmed resolved / monitoring / unresolved (with blocking items).

## Refusal conditions

This protocol refuses to proceed when:

- No incident description, error message, or monitoring artifact has been provided — advisory output without evidence is not reliable.
- A request is made to directly invoke `sap-integration-flow-guarded-operator-agent` from within this protocol — that gate is human-mediated only.
- Raw credentials, OAuth tokens, API keys, or certificate private keys are present in submitted artifacts without redaction — request redaction before proceeding.
- A request is made to recommend an irreversible action without identifying the human approver — the approver must be named before the package is finalized.
- The trigger event is outside the scope defined in "When to use" — redirect to the appropriate protocol or skill.
- A request is made to access live SAP Integration Suite APIs, execute adapter operations, or inspect live message processing logs directly — this protocol accepts only user-provided artifacts.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP Integration Suite documentation, SAP Cloud ALM documentation, SAP Event Mesh documentation, or official SAP Help Portal references.
- `user-provided evidence` — message processing log excerpts, Cloud ALM alert exports, adapter configuration descriptions, or monitoring screenshots supplied by the user.
- `inference` — derived reasoning not directly confirmed by official documentation or user evidence; always label as inference and note the assumption.

## Response minimum

Every protocol session must return, at minimum:

- **Trigger classification** — which "When to use" condition(s) activated the protocol.
- **Participating roles activated** — which of the four agents' advisory domains are relevant to this session.
- **Evidence received** — inventory of what was supplied, with confirmation of redaction compliance.
- **Root cause hypothesis** — most likely root cause with confidence level and evidence basis.
- **Findings per domain** — advisory output for each activated role, with evidence labels and severity.
- **Decision rights table** — who must approve each pending remediation action.
- **Irreversible-action gate status** — whether irreversible actions are in scope and whether human approval has been confirmed.
- **Business process impact** — current status of the affected business process (outage active / degraded / recovering / resolved).
- **Audit package readiness** — whether the session is complete enough to produce a handoff package for the operator gate.
- **Next step** — the specific human action required to proceed (e.g., "Obtain Integration Platform Lead approval for iFlow redeployment and submit handoff package to the integration flow operator gate").
