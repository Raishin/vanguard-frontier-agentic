---
name: case-to-resolution-protocol
description: Use this skill when a customer service case must be triaged, routed to the right team, driven to resolution, and captured as reusable knowledge in Dynamics 365 Customer Service. Covers intake-to-routing, SLA tracking, escalation gates, knowledge capture, and post-resolution CSAT. Orchestrates the d365-customer-service-contact-center-agent as primary and invokes human escalation when SLA risk or knowledge gaps are detected. Does not make final resolution decisions or override customer service policies; all production-impacting steps escalate to the relevant service owner or quality team.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-16"
  category: operational
  lifecycle: experimental
---

# Case to Resolution Protocol

## Purpose
This skill defines how a customer service case moves from first contact through
routing, active resolution, and closure — and how every resolved case feeds
the knowledge base so future cases resolve faster. It exists to ensure SLA
accountability, consistent routing logic, and structured knowledge reuse across
Dynamics 365 Customer Service. It does not replace human judgment on complex
resolutions; it structures the handoffs so nothing falls through.

## When to use
- A new case must be triaged and routed to a queue, team, or specialist.
- An active case risks breaching its SLA and needs escalation.
- A case is resolved and a knowledge article should be created or updated.
- Post-resolution CSAT is due and results must feed continuous-improvement loops.
- Multiple channels (email, chat, voice, social) need a unified intake view.

## When NOT to use
- The case is already resolved and closed — routing decisions are moot.
- The matter is a product defect escalation requiring engineering triage — use
  your engineering escalation runbook.
- Regulated health or financial-services data handling requires compliance
  counsel that supersedes this protocol.
- You need live Dynamics 365 configuration changes — escalate to the service
  owner; this protocol is recommendation-only.

## Participating agents
- `d365-customer-service-contact-center-agent` (primary — case intake, routing, SLA monitoring, resolution, knowledge capture)

## Inputs required
- Case description and channel of origin (email, chat, voice, social, web form)
- Customer or employee identity (authenticated or pending authentication)
- Priority signal or SLA entitlement reference
- Existing case history for this customer (if available)
- Knowledge base search results (Dynamics 365 Customer Service or SharePoint)

## Evidence required
- Unified routing configuration (queues, skills, capacity rules) is in place
- SLA definitions and entitlements are configured in Dynamics 365 Customer Service
- Knowledge base is seeded and publishing workflow is active
- CSAT survey mechanism (Dynamics 365 Customer Voice or equivalent) is configured

## Workflow

1. **Intake** — Log the case: capture channel, customer identity, description,
   priority. Apply record creation and update rules to standardise fields.
2. **Authentication** — Verify the requester's identity via entitlement lookup
   or Dynamics 365 authentication flow before exposing account data.
3. **Route** — Apply unified routing rules (skills matching, queue capacity,
   SLA tier). Assign to the correct queue or representative automatically where
   rules allow; flag ambiguous cases for supervisor review.
4. **Acknowledge** — Send SLA-compliant acknowledgment to the customer. Start
   the SLA clock.
5. **Investigate** — Representative works the case using the knowledge base,
   AI suggestions (Copilot in Dynamics 365), and collaboration via Microsoft
   Teams swarming where needed.
6. **Gate 1 — SLA attainment check** — If the first-response or resolution SLA
   is at risk, escalate to supervisor and reassign if necessary. Do not allow
   silent SLA breach.
7. **Resolve** — Apply the resolution. Record resolution type, billable time,
   and resolution summary in the case record.
8. **Gate 2 — Knowledge capture** — Before closing, check whether the
   resolution should produce a new or updated knowledge article. If yes, draft
   the article and route it through the knowledge publishing workflow.
9. **Close** — Set case status to Resolved. Trigger post-resolution CSAT survey
   via Dynamics 365 Customer Voice.
10. **CSAT review** — Route CSAT results to the responsible team. Flag low-
    scoring cases for quality review and root-cause analysis.

## Decision gates

| Gate | Condition | Action |
|---|---|---|
| SLA attainment | First-response or resolution timer at 80% elapsed | Escalate to supervisor; consider reassignment |
| Knowledge capture | Resolution not covered by existing article | Draft knowledge article before close |
| Escalation trigger | Case complexity exceeds representative skill tier | Invoke Teams swarming or senior escalation queue |
| CSAT threshold | CSAT score below agreed threshold | Flag for quality review; root-cause required |

## Refusal triggers
- Stop if the case requires access to regulated personal data (health, financial)
  and applicable compliance requirements are not confirmed — escalate to
  compliance owner.
- Stop if resolving the case requires a live Dynamics 365 configuration change
  that has not been approved by the service owner.
- Stop if customer authentication has not been confirmed before exposing
  account-specific data.

## Handoff rules
- All handoffs carry: case_id, skill_id, skill_version, invoked_by,
  routing_basis, sla_status, open_questions, do_not_do_list.
- Human escalations always include the SLA time remaining and a clear statement
  of what has been tried.
- Knowledge articles drafted by this protocol are submitted for human review
  before publishing; this skill never auto-publishes.

## KPIs
- SLA attainment rate (first response, resolution)
- Case resolution time (mean, P90)
- Knowledge article creation rate per resolved case
- CSAT score (mean, trend, low-score rate)
- First-contact resolution rate

## References
- https://learn.microsoft.com/dynamics365/guidance/business-processes/case-to-resolution-introduction
- https://learn.microsoft.com/dynamics365/guidance/business-processes/case-to-resolution-areas
- https://learn.microsoft.com/dynamics365/customer-service/administer/overview-cases
- https://learn.microsoft.com/dynamics365/customer-service/use/customer-service-hub-user-guide-resolve-cancel-reassign-a-case
- https://learn.microsoft.com/dynamics365/customer-service/administer/set-up-case-resolution-agent
