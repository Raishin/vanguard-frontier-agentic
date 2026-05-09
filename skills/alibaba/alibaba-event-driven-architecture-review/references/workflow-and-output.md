# Workflow and output contract

Use this reference only when performing a full event-driven architecture review or reliability assessment.

## Review domains

Check these areas before giving a recommendation:

- Event flow topology: service selection (EventBridge / MNS / RocketMQ / MSE Kafka) and justification
- Dead-letter queue configuration: presence, retention period, monitoring alerts
- Message ordering: orderly vs concurrent consumption, consumer thread model alignment
- Idempotency: consumer deduplication strategy and unique message ID usage
- Consumer lag: CloudMonitor alerts configured for lag thresholds on all production consumer groups
- Schema registry: EventBridge schema registry adoption and compatibility mode
- Retry storm risk: max retry depth, backoff strategy, circuit breaker presence
- Region context: CN-* mainland China vs international — feature and limit differences

## Safe workflow

1. **Frame the architecture**
   - Messaging services in use:
   - Event flow description (producers, topics/queues, consumers):
   - Region context (CN-* vs international):
   - Criticality of each message flow:
2. **Collect evidence**
   - Prefer live CloudMonitor metrics, queue configuration screenshots, or aliyun CLI output.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the design**
   - What happens if a consumer crashes mid-processing?
   - Are messages durably retained and replayable after consumer failure?
   - Does consumer lag breach the DLQ trigger before the consumer recovers?
   - What is the retry depth before a message is dropped or sent to DLQ?
   - Can schema changes break consumers silently?
4. **Recommend the smallest safe next step**
   - Prioritize by risk: missing DLQ > no lag monitoring > no idempotency strategy > missing schema registry.
   - If the safest action is to add a DLQ before anything else, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Event-Driven Architecture Review: <scope>
## Executive summary
- Reliability posture verdict:
- Evidence level:
- Critical gaps:
## Event flow topology
| Producer | Service | Topic/Queue | Consumer | Criticality |
|---|---|---|---|---|
## Dead-letter queue and retry assessment
| Queue/Topic | DLQ configured | Max retries | DLQ monitoring | Gap |
|---|---|---|---|---|
## Ordering and idempotency posture
- Ordering model:
- Consumer thread model alignment:
- Idempotency strategy:
## Consumer lag monitoring
- CloudMonitor alerts configured:
- Lag thresholds:
- Gaps:
## Schema evolution risks
- Schema registry adopted:
- Compatibility mode:
- Breaking change risks:
## Retry storm and cascading failure risks
- Max retry depth:
- Backoff strategy:
- Circuit breaker presence:
## Recommended hardening actions
1. <action> — priority: <critical/high/medium>, effort: <low/medium/high>
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
