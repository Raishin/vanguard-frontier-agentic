# Ticket Triage and Escalation Guide

Use this reference when classifying AWS tickets, alerts, OpsItems, incidents, support cases, health events, or operational requests into priority, owner, evidence gap, and escalation path.

## What people get wrong

The lazy story is:

> Triage means sort by severity label and assign a queue.

Wrong. Severity labels are often wrong, stale, duplicated, or missing impact context. Good triage protects responders from noise while escalating real risk fast.

Common bad assumptions:

- The ticket title states the root cause.
- CloudWatch alarm severity equals business severity.
- Duplicate tickets can be closed without checking shared impact.
- AWS Health events affect every workload in the same way.
- Trusted Advisor findings are urgent by default.
- Escalation is complete once a team is tagged.

## Triage-specific failure modes

- Misrouting IAM, KMS, networking, or data-store issues to generic platform queues.
- Treating symptoms as root cause and assigning to the wrong owner.
- Closing duplicates while losing the best evidence trail.
- Escalating without logs, timeframe, resource identifiers, Region, account scope, or customer impact.
- Ignoring incident-manager engagement, support case status, or change/deployment context.
- Allowing ticket backlog age to hide high-risk security or compliance issues.

## Minimum safe workflow

1. Identify ticket type: incident, alert, request, change, cost anomaly, security finding, or informational health event.
2. Extract evidence: affected service/resource class, account/Region, timeframe, severity, owner hints, and customer/business impact.
3. Classify priority using impact plus urgency, not label alone.
4. Map to accountable owner and backup escalation path.
5. Identify missing evidence required before remediation or closure.
6. Deduplicate carefully: link related tickets and preserve the strongest evidence record.
7. Return safe next actions; do not mutate resources, suppress alerts, or close tickets without approval.

## Verification targets

- alarm or event source, timestamp, affected account/Region/service, and current state
- AWS Health event scope and status
- OpsCenter OpsItem or Incident Manager incident status, engagement plan, and runbook reference
- support case severity, correspondence, and pending action owner
- deployment/change timeline around first occurrence
- previous related tickets, duplicate candidates, and known problem record
- owner mapping, escalation policy, and on-call coverage

## When to push back

Push back if the user asks to:

- close or suppress alerts because they look noisy
- assign a ticket without sufficient evidence or owner mapping
- paste secret-bearing logs into the ticket summary
- downgrade priority without impact evidence
- skip escalation for identity, encryption, data-loss, or customer-facing symptoms
- treat triage as root-cause analysis or remediation completion
