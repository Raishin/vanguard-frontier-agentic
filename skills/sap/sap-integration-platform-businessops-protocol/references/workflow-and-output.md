# Workflow and output contract — SAP Integration / Platform / Business Operations Protocol

Use this reference for trigger classification, role activation logic, root cause hypothesis structure, finding severity, handoff sequencing, and output format.

## Trigger classification taxonomy

| Trigger class | Description | Primary role activated |
|---------------|-------------|----------------------|
| `iflow-failure` | iFlow in error state, message processing failures, or mapping exceptions | sap-integration-suite-reviewer-agent |
| `api-throttling` | Rate-limit or 429 responses from Integration Suite, API Business Hub, or connected systems | sap-integration-suite-reviewer-agent |
| `event-delivery-failure` | Event Mesh messages stuck, undelivered, or accumulating in dead-letter queue | sap-integration-suite-reviewer-agent + sap-cloud-alm-sre-incident-agent |
| `partner-integration-break` | EDI, B2B, or external API integration has stopped exchanging data | sap-integration-suite-reviewer-agent |
| `data-replication-failure` | CDS view replication, SLT, or SOAP/REST sync producing stale or corrupt data | sap-integration-suite-reviewer-agent + sap-cloud-alm-sre-incident-agent |
| `middleware-instability` | Integration Suite tenant, Node, or on-premise Agent exhibiting erratic behavior | sap-cloud-alm-sre-incident-agent |
| `business-process-outage` | Critical business process interrupted; integration or platform layer suspected | All four roles activated; sap-cloud-alm-sre-incident-agent leads triage |
| `platform-entitlement-gap` | Integration failure caused by missing BTP service entitlement or quota exhaustion | sap-btp-account-entitlement-governance-reviewer-agent |

## Root cause hypothesis structure

For each session, produce a root cause hypothesis using the following structure:

1. **Hypothesis** — the most likely root cause in plain language.
2. **Evidence basis** — what user-provided evidence or documentation supports this hypothesis.
3. **Confidence level** — one of: `confirmed` (directly evidenced), `probable` (strongly suggested by evidence), `possible` (plausible but not directly evidenced), `unknown` (insufficient evidence to hypothesize).
4. **Competing hypotheses** — alternative root causes not yet ruled out, with evidence that would confirm or exclude them.
5. **Evidence gap** — what additional evidence the user must collect to move from `possible` or `unknown` to `confirmed` or `probable`.

## Finding severity classification

| Severity | Criteria |
|----------|---------|
| `critical` | Live business-process outage; active data loss or message loss; security-impacting credential exposure in a deployed adapter; SLA breach with penalty exposure |
| `high` | Integration flow producing incorrect output without triggering an error (silent data corruption); Event Mesh dead-letter queue accumulating with no reprocessing plan; partner integration break affecting external SLA commitments |
| `medium` | Integration flow in error state with retry exhausted but no immediate data loss; API throttling degrading (not stopping) business processes; monitoring gap preventing timely incident detection |
| `low` | iFlow design deviation from best practices with no current impact; missing error notification configuration; suboptimal retry policy that increases blast radius if a downstream system becomes unavailable |

## Workflow

1. **Classify trigger** — identify which trigger class(es) apply from the incident description and evidence.
2. **Activate relevant roles** — determine which of the four participating agent roles are relevant to this session.
3. **Inventory evidence** — list all artifacts provided; confirm redaction compliance; request missing mandatory items.
4. **Produce root cause hypothesis** — structured hypothesis with confidence level and evidence gaps.
5. **Produce per-role findings** — for each activated role, produce advisory findings with evidence labels and severity classification.
6. **Map decision rights** — for each recommended action, identify the named human approver from the decision rights table in SKILL.md.
7. **Gate irreversible actions** — for any finding involving a listed irreversible action, confirm human approval is required before proceeding; do not pre-approve.
8. **Sequence remediation** — critical findings first (outage resolution), then high (silent corruption, SLA risk), then medium (monitoring gaps), then low (design improvements).
9. **Produce audit package** — assemble the audit package as defined in SKILL.md.

## Output contract

Return:

1. **Trigger classification** — which trigger class(es) activated, with evidence label.
2. **Roles activated** — which participating agent domains are in scope for this session.
3. **Evidence inventory** — types of artifacts received; redaction confirmation; missing mandatory evidence.
4. **Root cause hypothesis** — structured per the hypothesis template above.
5. **Per-domain findings** — severity, evidence label, and recommended advisory action for each finding.
6. **Decision rights table** — pending decisions mapped to named approver roles.
7. **Irreversible-action gate status** — list of any irreversible actions in scope; human approval status (confirmed / not yet / not applicable).
8. **Business process impact** — current status (outage active / degraded / recovering / resolved).
9. **Escalation log** — escalation owners to notify and the action requested from each.
10. **Audit package readiness** — complete / incomplete (with blocking items).
11. **Next human step** — specific action required to proceed.

## Handoff to guarded-mutating operator gate

This protocol does not invoke `sap-integration-flow-guarded-operator-agent` directly. When the audit package is complete and human approval has been confirmed for all irreversible actions, the protocol produces a handoff summary containing:

- Approved action type and scope (iFlow redeployment, undeploy, adapter credential rotation, Event Mesh subscription change).
- Integration artifact identifier (iFlow ID, package name, version reference).
- Evidence basis and root cause hypothesis supporting the action.
- Named approvers who confirmed the action.
- Timestamp of approval confirmation.
- Reference to the change record where approval is documented.

A human presents this handoff summary to the operator gate. The operator gate independently verifies approval before executing any mutation.
