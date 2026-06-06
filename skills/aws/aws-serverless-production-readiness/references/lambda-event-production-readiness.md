# Lambda Event Production Readiness Guide

Use this reference for production readiness reviews of Lambda-centered serverless workloads, event source mappings, retries, DLQs/destinations, concurrency, idempotency, observability, API Gateway, EventBridge, SQS/SNS, Step Functions, and DynamoDB stream integrations.

## What people get wrong

The lazy story is:

> Lambda scales and retries automatically, so production readiness is mostly IAM and monitoring.

Wrong. Serverless failures come from event semantics: duplicate delivery, poison messages, hot partitions, retry amplification, timeout mismatch, concurrency starvation, and silent data loss.

Common bad assumptions:

- At-least-once delivery can be ignored if the function is fast.
- DLQ presence proves recoverability.
- Reserved concurrency is only a cost control.
- API Gateway timeout and Lambda timeout can be tuned independently without user impact.
- EventBridge/SNS/SQS retries are harmless.
- CloudWatch logs are enough observability.

## Serverless failure modes

- Non-idempotent handlers double-charge, duplicate writes, or corrupt state during retry.
- SQS visibility timeout, batch size, partial batch response, and function timeout are inconsistent.
- Async Lambda destinations or DLQs are absent, unmonitored, or unreadable by responders.
- Reserved/provisioned concurrency starves critical functions or allows noisy neighbors to dominate.
- EventBridge rule pattern is too broad, causing fanout storms or unexpected consumers.
- Step Functions retry/catch hides failed business states or creates unbounded downstream calls.

## Minimum safe workflow

1. Map every trigger, event schema, retry policy, destination, DLQ, and downstream dependency.
2. Confirm idempotency key, deduplication behavior, ordering needs, and poison-message handling.
3. Check timeout and concurrency alignment across API Gateway, Lambda, SQS visibility, Step Functions, and downstream services.
4. Verify observability: RED/USE metrics, structured logs, traces, alarms, DLQ depth, throttles, iterator age, and business KPIs.
5. Review deployment safety: versions, aliases, CodeDeploy/SAM traffic shifting, rollback alarms, and previous version target.
6. Identify cost and quota risks: concurrency, payload size, retention, logs, retries, and downstream throttling.
7. Return readiness gaps with evidence level and approval-gated remediation.

## Verification targets

- Lambda timeout, memory, ephemeral storage, architecture, environment, layers, versions, aliases, and concurrency settings
- event source mapping: batch size, max batching window, filters, partial batch response, enabled state, and starting position
- SQS/SNS/EventBridge retry, DLQ, destination, archive/replay, and fanout configuration
- Step Functions state machine retries, catches, timeouts, task tokens, and compensation paths
- IAM execution role, resource policies, KMS, VPC config, and secrets access
- CloudWatch metrics/alarms for Errors, Throttles, Duration, IteratorAge, ConcurrentExecutions, DLQ depth, and custom business metrics

## When to push back

Push back if the user asks to:

- declare production-ready without idempotency and retry evidence
- remove DLQs or alarms to simplify deployment
- raise concurrency limits without downstream capacity proof
- ignore duplicate/out-of-order event handling
- log raw event payloads containing sensitive data
- deploy alias/version changes without rollback target and alarms
