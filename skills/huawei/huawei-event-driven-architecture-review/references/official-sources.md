# Official sources

Use this reference only when you need source grounding for Huawei Cloud event-driven service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/dms/index.html
- https://support.huaweicloud.com/intl/en-us/roma/index.html
- https://support.huaweicloud.com/intl/en-us/fg/index.html
- https://support.huaweicloud.com/intl/en-us/smn/index.html

## Key service behavior references

| Service | Behavior | Implication | Risk if Absent |
|---------|----------|-------------|----------------|
| DMS Kafka | Dead-letter topic (DLQ) must be configured per consumer group | Messages beyond max retries are silently dropped without DLQ | Data loss |
| DMS Kafka | Consumer group lag visible in CES metrics | CES alert on lag required for early warning | Undetected processing backlog |
| DMS Kafka | Cross-region topics not natively supported | MirrorMaker 2.0 or custom bridge required for cross-region replication | No DR for events |
| FunctionGraph | OBS/DMS/SMN event triggers have at-least-once delivery | Handler must be idempotent | Duplicate side effects on retry |
| SMN | HTTP subscribers receive no automatic retry by default | Retry policy must be explicitly configured | Notification loss |
| ROMA Connect | Per-instance invocation rate limit applies | Verify capacity against peak event rate | Throttling at high load |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
