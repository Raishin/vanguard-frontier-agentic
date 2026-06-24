# Safety checklist

Use this reference before any recommendation involving production unified-routing rules, SLA configuration, channel/workstream setup, or knowledge publishing in Dynamics 365 Customer Service.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, environment URLs, connection strings, certificates, or customer data into chat.
- Use exported reports or sanitized user-provided evidence for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent SLA attainment percentages, case volumes, routing distributions, or CSAT scores.
- Require explicit human approval before recommending any production routing-rule, SLA, channel, or knowledge-publishing change.
- Use current official Microsoft Learn documentation for Customer Service, Omnichannel, and unified routing behavior.
- Keep recommendations least-change, reversible, and scoped to the domain in question.

## Stress checks

- What cases are misrouted, reassigned repeatedly, or manually triaged at scale?
- Which SLAs lack warning actions, realistic business calendars, or recalculation-on-reopen settings?
- Is the knowledge base curated, current, and de-duplicated, or stale?
- Are omnichannel capacity profiles set to prevent agent overload?
- What rollback exists if a routing-rule or SLA change misbehaves in production?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual SLA attainment, routing accuracy, knowledge freshness, or CSAT.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Modifying production unified-routing rules, work classification, or capacity profiles
- Changing production SLA KPIs, success criteria, or warning/failure actions
- Configuring or reconfiguring production channels/workstreams (voice, chat, messaging)
- Publishing, unpublishing, or bulk-editing production knowledge articles
- Executing bulk case reassignment, status changes, or queue moves
