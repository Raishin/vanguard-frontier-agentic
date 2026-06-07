# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/decision-guides/latest/sns-or-sqs-or-eventbridge/sns-or-sqs-or-eventbridge.html
- https://docs.aws.amazon.com/lambda/latest/dg/concepts-event-driven-architectures.html
- https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-rules.html
- https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- AWS decision guidance differentiates SNS, SQS, and EventBridge by messaging pattern and integration need; choosing one is an architecture decision, not a default.
- Lambda event-driven architecture guidance calls out benefits, trade-offs, and anti-patterns; asynchronous flows need explicit retry, idempotency, and failure handling design.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon EventBridge and AWS Lambda as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `EventBridge+ListRules`, `SQS+GetQueueAttributes`, `SNS+GetTopicAttributes`, `Lambda+GetFunction`, and `SFN+DescribeStateMachine` were reported `isAvailableIn` in those regions.

Review implications:
- Require event contract, producer/consumer ownership, retry/DLQ policy, ordering/deduplication needs, idempotency, schema/versioning, observability, and replay strategy.
- Service/API availability does not prove that a specific event bus, queue, topic, rule, Lambda, or state machine is configured correctly.
