# Workflow and output contract

Use this reference only when performing the full Dynamics 365 Customer Service & Contact Center review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Case management: record creation/update rules, resolution experience, status reasons, timeline configuration, security roles
- Unified routing: rule-based vs. skills-based routing, work classification, capacity profiles, queue/agent assignment
- Omnichannel: configured channels (voice, chat, messaging), workstreams, agent experience, capacity
- Queues and entitlements: queue design, entitlement terms, service scheduling
- SLAs: KPIs, applicable-when conditions, success criteria, warning/failure actions, recalculation on terminal status
- Knowledge management: authoring/curation/publishing, internal and external (SharePoint) search, AI suggestions, article freshness/expiry
- Copilot in Service: case/conversation summaries, ask-a-question, draft-a-response, knowledge sources
- KPIs: first-response/resolution time, SLA attainment, routing accuracy, self-service deflection, CSAT

## Safe workflow

1. **Frame scope**
   - Area in scope (case management / routing / omnichannel / SLA / knowledge):
   - Licensing and channels in use (Customer Service, Contact Center, Omnichannel voice):
   - Required outcome (resolution time / routing accuracy / SLA attainment / knowledge reuse / CSAT):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported reports: case aging, SLA attainment, routing distribution, knowledge usage, CSAT survey results.
   - Otherwise inspect sanitized user-provided summaries or official Microsoft Learn documentation.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What cases are misrouted or manually triaged at scale?
   - Which SLAs lack warning actions or realistic business-hours calendars?
   - Is the knowledge base curated and current, or stale and duplicative?
   - Are channels configured with capacity profiles to prevent agent overload?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest safe action**
   - Prefer configuration and process fixes over channel/routing-engine changes.
   - Production routing-rule, SLA, and channel configuration changes require live-guard escalation with a rollback plan.

## Output contract

Return this structure:

```markdown
# D365 Customer Service & Contact Center Review: <scope>
## Executive verdict
- Status: HEALTHY / HEALTHY WITH RISKS / AT RISK / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Reports or checks to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
