# Incident Evidence Correlation Guide

Use this reference for AWS incident investigation across CloudWatch metrics/logs/alarms, X-Ray traces, CloudTrail, EventBridge, AWS Health, deployment timelines, runbooks, and post-incident corrective actions.

## What people get wrong

The lazy story is:

> Find the alarm that fired first and call it root cause.

Wrong. First visible symptom is rarely root cause. Incidents need timeline discipline, blast-radius mapping, competing hypotheses, and evidence quality labels.

Common bad assumptions:

- Dashboard red equals customer impact.
- CloudWatch anomaly or generated insight proves root cause.
- No active alarm means the incident is over.
- Logs can be pasted raw into tickets or summaries.
- Deployment correlation proves deployment causation.
- Restart/retry remediation can happen before preserving evidence.

## Incident failure modes

- Metrics, logs, traces, and events use different clocks, dimensions, sampling, and retention.
- Alert storm hides the primary failing dependency or customer-facing symptom.
- CloudTrail control-plane changes are ignored during data-plane incidents.
- AWS Health/service events are not checked for the right account/Region/service scope.
- Runbook actions mutate state before evidence capture.
- Post-incident actions fix symptoms but not detection, rollback, ownership, or capacity gaps.

## Minimum safe workflow

1. Define incident window, affected users/services, severity, owner, and current mitigation state.
2. Build a timeline from alarms, metrics, logs, traces, CloudTrail, EventBridge, deployments, and AWS Health.
3. Separate symptoms, contributing factors, root-cause hypotheses, and confirmed causes.
4. Preserve sensitive data boundaries: summarize logs, redact payloads, and avoid secrets/customer identifiers.
5. Recommend safe read-only checks first; mutation or rollback requires explicit approval and owner alignment.
6. Produce next actions: containment, validation, communication, monitoring, and post-incident follow-up.
7. Mark unknowns honestly and list the exact evidence needed to close them.

## Verification targets

- CloudWatch alarm history, metric math, dimensions, dashboards, and anomaly bands
- CloudWatch Logs Insights queries, sanitized log excerpts, retention, and ingestion delay
- X-Ray traces, service maps, error/latency annotations, and sampling notes
- CloudTrail events for deployments, IAM, networking, scaling, KMS, and data-store changes
- EventBridge events, AWS Health events, Incident Manager engagement, OpsItems, and support cases
- deployment/change timeline, rollback status, customer-impact metrics, and postmortem action tracker

## When to push back

Push back if the user asks to:

- claim root cause from correlation alone
- suppress or close alarms before recovery validation
- paste raw secret-bearing logs into the response
- mutate production before evidence capture and approval
- ignore AWS Health or recent change history
- skip post-incident corrective actions because service recovered
