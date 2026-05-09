# Safety Checklist

Use before recommending any Auto Scaling policy change, backup schedule modification, DR configuration, or production-impacting reliability control.

## Non-negotiables

- Never recommend deleting backups, reducing backup retention, or disabling Multi-AZ without explicit confirmation of business risk acceptance.
- Do not invent SLO values, SLA percentages, resource names, or ARNs.
- Require explicit user approval before modifying Auto Scaling policies, health check thresholds, or DR routing configurations in production.
- Chaos engineering experiments (AWS FIS) must run in non-production first; flag this requirement explicitly.
- SQS DLQ enablement and retry policies are non-destructive in most cases — but validate queue consumer idempotency before recommending increased retry counts.
- Route 53 failover routing changes affect live DNS TTL — require confirmation of TTL values and client cache flush plans.

## Stress checks

- What is the single point of failure if this change is applied?
- What is the blast radius of the recommended change?
- Does the recovery test account for data replication lag?
- Is the DLQ consumer tested and alerting configured for DLQ depth?
- What is the rollback plan if Auto Scaling acts unexpectedly?
