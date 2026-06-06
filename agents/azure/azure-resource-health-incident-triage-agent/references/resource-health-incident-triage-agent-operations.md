# Azure Resource Health Incident Triage operations

Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state.

## What people get wrong

Provider health signals are evidence, not automatic root cause. A tenant-side deployment, config change, quota issue, or dependency can coincide with a service event.

## Officially grounded service shape

Microsoft guidance separates personalized Service Health notifications, activity-log service events, Resource Health, and alerting. Service Health events include incident type, status, timestamps, impacted services, and regions, and can be alerted through activity log alerts. That is the key insight: triage must correlate provider signals with tenant-side evidence.

## Non-negotiable design rules

1. Separate provider incident, resource health, tenant change, and inference.
2. Do not claim root cause from a single health event.
3. Check event type, status, impact window, affected services, regions, and activity log correlation.
4. Review recent tenant-side deployments, config changes, quota events, and alert routing.
5. Require safe read-only evidence before recommending remediation.

## Minimal safe implementation flow

1. Classify incident window, symptoms, affected resources, services, and regions.
2. Ground Service Health and Resource Health behavior in Microsoft Learn.
3. Collect sanitized sampled evidence from health events, activity logs, alerts, and recent changes.
4. Correlate timing and scope; identify provider-side, tenant-side, or unresolved evidence.
5. Return triage verdict, confidence, blockers, safe next probes, and support/escalation criteria.

## High-risk assumptions to kill

- Service Health event equals root cause.
- No Service Health event means Azure is uninvolved.
- Resource Health alone explains application symptoms.
- Broad remediation is safe before blast radius is known.

## Safe command/code verification targets

- Service Health incident type, status, tracking details, impacted services, regions, timestamps, and resolution state.
- Resource Health state and recent transitions for affected resources.
- Activity log events, deployments, alert rules, action groups, and change window evidence.

## When to push back

- The user wants root cause without telemetry correlation.
- The incident window or affected scope is unclear.
- Remediation would mutate production before evidence is gathered.
- Sensitive logs or identifiers are requested unnecessarily.
