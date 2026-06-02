# Non-Destructive Automation Patterns Guide

Use this reference when designing AWS automation for reporting, evidence gathering, notification, approvals, and coordination without direct remediation or destructive mutation.

## What people get wrong

The lazy story is:

> It only automates an operator task, so it is safe.

Wrong. Automation changes risk shape. A read-only inventory flow can become destructive if it adds broad IAM, invokes remediation runbooks, changes approvals, or leaks sensitive evidence into messages.

Common bad assumptions:

- Systems Manager Automation is non-destructive by default.
- Step Functions orchestration is safe because each step is small.
- EventBridge rules cannot cause incidents if the target is serverless.
- Notifications may include raw logs, ARNs, account IDs, or secret-looking values.
- Manual approval steps are enough without least-privilege execution roles.
- Retry policies are harmless for ticket creation or notifications.

## Automation-specific failure modes

- Runbook step calls mutating APIs such as update, delete, stop, detach, revoke, or put-policy.
- Lambda or Step Functions role has wildcard permissions beyond read/report/notify.
- EventBridge rule fans out noisy events into duplicate tickets or alert storms.
- SNS/SQS messages expose sensitive log excerpts or customer identifiers.
- Approval path is bypassable because the automation role can execute the final action directly.
- Retry/catch logic hides partial failure or repeats side effects.

## Minimum safe workflow

1. Classify every step as read, calculate, notify, approve, ticket, or mutate.
2. Reject or isolate mutate steps; this skill should design non-destructive flows only.
3. Choose the simplest orchestration: EventBridge schedule/rule, Lambda report, Step Functions approval flow, SNS/SQS fanout, or Systems Manager Automation read-only runbook.
4. Define least-privilege role boundaries and data-redaction rules before workflow shape.
5. Add idempotency, deduplication, retry limits, and failure visibility.
6. Make human approvals explicit and external to any role that could perform mutation.
7. Provide implementation guidance as design or repo patch only; live deployment remains approval-gated.

## Verification targets

- IAM policy actions: prove no destructive verbs are needed for the workflow
- EventBridge pattern/schedule and target list
- Step Functions states, retries, catches, approval waits, and terminal failure paths
- Lambda/reporting code paths and sanitized output fields
- Systems Manager Automation document steps and `assumeRole` permissions
- SNS/SQS topic/queue policy, retention, DLQ, and message content boundaries
- evidence contract: what is collected, redacted, stored, and sent to whom

## When to push back

Push back if the user asks to:

- include remediation in a non-destructive automation
- grant wildcard IAM because the workflow is “just reporting”
- send raw logs, credentials, account IDs, or customer data into notifications
- suppress alarms/tickets automatically instead of routing them
- make approval a checkbox while the automation can still mutate directly
- deploy the automation without testing event volume and retry behavior
