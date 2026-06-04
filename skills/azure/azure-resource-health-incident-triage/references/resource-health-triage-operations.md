# Azure Resource Health Incident Triage Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Calling every user-visible incident an Azure outage before Resource Health, Service Health, and tenant changes are checked.
- Treating Resource Health status as root cause instead of first-pass evidence.
- Ignoring Unknown status or unsupported resources instead of stating evidence limits.
- Forgetting that Service Health events and Resource Health events are different alert categories.
- Remediating broadly before blast radius, start time, and recent changes are known.

## Officially grounded service shape

- Microsoft Learn evidence says Resource Health reports current and past health of specific Azure resources using service signals and statuses such as Available, Unavailable, Unknown, and Degraded.
- Service Health notifications are system-generated subscription activity-log events about incidents, maintenance, advisories, security, billing, and action-required items.
- Service Health events appear in the activity log when subscription scoped; some global emerging issues are not activity-log events.
- Resource Health events are recorded in the activity log for specific health annotations or transitions, but some Unknown transitions and short compute transitions are not recorded.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, incident state, or production readiness.

## Non-negotiable design rules

- Start with exact resource, region, subscription scope, start time, symptom, and suspected blast radius.
- Separate provider incident, planned maintenance, security advisory, tenant-side change, resource-specific degradation, and unknown.
- Use Resource Health for resource-specific state and Service Health for subscription-scoped service notifications.
- Check activity log and alert/action group routing before declaring notification coverage good.
- Do not recommend destructive remediation while platform-caused versus tenant-caused evidence is unresolved.

## Minimal safe implementation flow

- Scope affected resources, services, regions, symptoms, start time, and business impact.
- Collect Resource Health status/history, Service Health notifications, activity log events, recent deployment/change evidence, and monitor alerts.
- Classify evidence as provider incident, planned maintenance, resource-specific issue, tenant-side change, or unknown.
- Define immediate safe actions, communication posture, support escalation evidence, and deeper triage handoff.
- Return evidence level, confidence, open questions, and next checks.

## Safe verification targets

- Resource Health status and history are checked for each named resource where supported.
- Service Health notifications are checked for affected subscription, service, and region.
- Activity log includes or excludes relevant health events with timestamp and category caveats.
- Recent deployments, configuration changes, quota/limit signals, and access changes are not ignored.
- Action groups and alert rules exist for future health notifications where required.

## When to push back

- The user asks for root cause with only a screenshot or rumor.
- The request jumps to restart/delete/redeploy before health and change evidence are collected.
- The resource type is unsupported or Unknown and the user wants certainty anyway.
- Incident data contains customer or sensitive payloads that should be redacted.
