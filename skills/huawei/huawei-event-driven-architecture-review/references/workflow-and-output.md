# Workflow and output contract

Use this reference only when performing a full event-driven architecture review for a Huawei Cloud workload.

## Review domains

Check these areas before giving a verdict:

- Event flow topology: which services produce and consume events, and whether service selection is appropriate for the use case
- DMS Kafka dead-letter topic: whether a DLQ is configured per consumer group and whether it is monitored
- Consumer group lag: whether CES lag metrics are collected and whether alerts are configured with an action runbook
- Message ordering: whether ordering guarantees are required and whether partition key strategy supports them
- Idempotency: whether FunctionGraph handlers and downstream consumers implement idempotent processing
- SMN retry policy: whether HTTP/HTTPS subscribers have explicit retry and whether failures are surfaced
- ROMA Connect capacity: whether the ROMA instance throughput matches the expected peak event rate
- Cross-region replication: whether a verified bridge exists and whether replication lag is monitored

## Safe workflow

1. **Frame scope**
   - Event sources and consumers in scope:
   - Region and account context:
   - Current-state evidence:
   - Required reliability and ordering guarantees:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the design**
   - What happens to messages if a consumer group falls behind and DLQ is absent?
   - What duplicate processing occurs if a FunctionGraph handler is not idempotent?
   - What notifications are silently lost if SMN HTTP retry is not configured?
   - What events are unreachable in the DR region if cross-region replication is not verified?
   - What evidence is missing to confirm the design handles the expected peak load?
4. **Recommend the smallest safe action**
   - Prefer targeted fixes, staged rollout, and verification steps.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Event-Driven Architecture Review: <scope>
## Event flow topology and service selection assessment
## DMS Kafka dead-letter and retry configuration
## Message ordering guarantees and idempotency posture
## Consumer group lag monitoring and alerting coverage
## SMN delivery reliability and HTTP subscriber retry policy
## ROMA Connect capacity and integration flow throughput
## Cross-region event replication architecture and gap analysis
## Prioritized hardening actions
```

Each section must include an evidence level label.
