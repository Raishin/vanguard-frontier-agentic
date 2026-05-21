---
name: salesforce-routing-protocol
description: Use this skill when a Salesforce matter must be classified and routed to the right specialist agent, when a matter crosses multiple Salesforce domains and needs parallel review, or when specialist agents disagree and the conflict must be resolved. It defines routing rules per matter type, the cross-domain overlap matrix covering admin × dev × security × revops × marketing × compliance, and the conflict-resolution protocol. Does not give Salesforce or business advice; routing is a recommendation only and never makes a binding routing decision on behalf of a human owner.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-20"
  category: platform
  lifecycle: experimental
---

# Salesforce Routing Protocol

## Purpose
This skill defines how a Salesforce matter is classified, routed to the right
specialist agent, and coordinated when it crosses multiple Salesforce domains.
It exists so agents never work in silos when risk crosses a domain boundary,
and so every handoff is controlled and auditable rather than free-form. It does
not give Salesforce or business advice; routing is a recommendation that a
human owner confirms.

## When to use
- A maestro agent must classify an incoming Salesforce matter and dispatch it.
- A specialist agent finds a matter has crossed into another Salesforce domain.
- A matter triggers an escalation gate and must be paused.
- Specialist agents reach conflicting recommendations.
- A matter is ambiguous and cannot be confidently assigned to a single domain.

## When not to use
- The matter is already classified and a specialist agent is actively working it.
- The matter is outside Salesforce scope entirely — route to the appropriate
  non-Salesforce protocol.
- You need direct live-org mutation advice — that requires the
  salesforce-live-change-approval-protocol, not this routing skill.

## Communication principles
- One accountable human owner per matter. One primary agent per matter.
- Parallel review only when the matter genuinely crosses domains.
- No agent makes a final configuration, deployment, business-process,
  or compliance decision.
- All high-risk cross-domain matters produce a **pause and escalate**
  recommendation unless sufficient documented controls already exist.
- Every handoff uses a `salesforce-case-capsule`. No free-form agent chatter.
- Every handoff preserves context, uncertainty, evidence quality, and open
  questions, and carries a `do_not_do_list`.
- Every handoff labels privilege sensitivity and privacy sensitivity.
- Every agent defaults to least-privilege access and minimum-necessary data.

## Routing rules

### Admin / org-config
Declarative setup, org settings, sandbox management, permission sets,
profiles, sharing, OWD, role hierarchy, IP restrictions, session policy →
**salesforce-org-assessment-skill** or **salesforce-permission-model-review-skill**.

### Developer / code
Apex, LWC, triggers, async jobs, managed packages, ISV, API integration,
deployments → **salesforce-apex-lwc-code-review-skill**.

### Automation / process
Flow, validation rules, approval processes, Process Builder
,
record-triggered automation → **salesforce-flow-automation-review-skill**.

### Security / IAM
Permission topology, toxic combinations, guest-user exposure,
data classification, sharing rule widening, field-level security,
connected-app OAuth scopes → **salesforce-permission-model-review-skill**
AND **salesforce-data-exposure-escalation-protocol** (if triggered).

### RevOps / CPQ
Pricing rules, product catalog, quote configuration, CPQ
custom scripting, contract and order management → **salesforce-org-assessment-skill**
(risk register triage); escalate business-rule decisions to human RevOps owner.

### Marketing / consent
Marketing Cloud
data flows, consent
capture, preference centers, subscriber keys, SFMC
send classifications →
**salesforce-marketing-consent-review-skill**.

### Compliance / regulated verticals
Health Cloud
, Financial Services
Cloud
, Government Cloud
regulated-data requirements,
BAA/DPA obligations, audit requirements → **salesforce-org-assessment-skill**
plus external compliance counsel.

### AI / Agentforce
Agentforce
agent configuration,
Einstein
feature setup, autonomous
action scope, AI grounding and retrieval → **salesforce-agentforce-risk-review-skill**.

### Release / deploy
Change sets, unlocked packages, CI/CD pipelines, sandboxes,
destructive changes → **salesforce-release-readiness-skill**.

### Integration / middleware
REST/SOAP/Bulk/Streaming APIs, Platform Events
, CDC
, MuleSoft
, named credentials,
external services → **salesforce-integration-review-skill**.

### Metadata
Object model, custom fields, layouts, page layouts,
Lightning record pages
,
deprecated types → **salesforce-metadata-review-skill**.

## Cross-domain overlap matrix

| Overlap scenario | Primary agent | Secondary agents | Escalation gate |
|---|---|---|---|
| Permission widening + automation bypass | salesforce-permission-model-review-skill | salesforce-flow-automation-review-skill | data-exposure |
| Apex + integration secret handling | salesforce-apex-lwc-code-review-skill | salesforce-integration-review-skill | production data |
| Marketing + consent + regulated data | salesforce-marketing-consent-review-skill | external compliance counsel | regulated-vertical |
| Agentforce + permission scope | salesforce-agentforce-risk-review-skill | salesforce-permission-model-review-skill | autonomous-AI-action |
| Release + live mutation | salesforce-release-readiness-skill | salesforce-live-change-approval-protocol | irreversible-deploy |
| CPQ + finance logic | salesforce-org-assessment-skill | human RevOps + Finance owner | finance/revenue |
| Security + admin + code triangle | salesforce-permission-model-review-skill | salesforce-apex-lwc-code-review-skill | production data exposure |

## Conflict-resolution protocol
When specialist agents disagree, follow this order and stop at the first step
that is unmet:
1. Freeze any irreversible action.
2. Preserve evidence; preserve sandbox or scratch-org state if applicable.
3. State the disagreement precisely — what each agent concluded and why.
4. Separate technical risk from business/operational risk; do not collapse them.
5. Identify the accountable human owner.
6. Escalate to qualified Salesforce architect or technical lead.
7. Document unresolved assumptions and open questions.
8. Produce options, not a single conclusion.
9. Require human approval before any action.
10. Log the decision path in the audit log.

## Security notes
- Routing is a recommendation, never an authorization. The protocol never
  approves, denies, or directs a deployment or configuration action.
- A matter is routed on sanitized signals. Never request org credentials,
  session IDs, OAuth tokens, customer PII, or production org IDs to classify.
- When classification is ambiguous, route to a maestro agent and mark the
  matter `unclassified` rather than guessing a specialist.
- This protocol does not authorize live org mutations; that requires
  salesforce-live-change-approval-protocol.

## Audit log fields
- matter_id, skill_id, skill_version, invoked_by, input_hash, evidence_quality, output_verdict, escalation_fired, timestamp

## Stop conditions
- A live-mutation action is requested — stop and invoke salesforce-live-change-approval-protocol.
- Classification requires access to production org credentials — stop and refuse.
- Matter involves regulated personal data and jurisdiction is unknown — stop and escalate.
