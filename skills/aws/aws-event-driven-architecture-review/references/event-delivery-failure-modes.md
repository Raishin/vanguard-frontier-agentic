# Event Delivery Failure Modes Guide

Use this reference for AWS event-driven architecture reviews involving EventBridge, SQS, SNS, Step Functions, Pipes, event schemas, filtering, cross-account routing, retries, DLQs, archive/replay, idempotency, and event-loop risk.

## What people get wrong

The lazy story is:

> Async decoupling makes the system more reliable.

Wrong. Async systems move failure into queues, retries, duplicates, ordering gaps, poison messages, and invisible backlog. Decoupling without contracts and observability is just delayed failure.

Common bad assumptions:

- EventBridge patterns are precise by default.
- DLQs prove no event is lost.
- SNS, SQS, and EventBridge are interchangeable.
- Replay is safe without idempotent consumers.
- Step Functions state machines automatically document business correctness.
- Cross-account event buses are safe if the policy allows the account.

## Event-driven failure modes

- Broad event patterns create fanout storms, loops, or unintended consumers.
- Consumer retries amplify downstream throttling or duplicate side effects.
- SQS visibility timeout, max receive count, FIFO ordering, or deduplication is misaligned with handlers.
- SNS filtering drops events silently from business perspective.
- EventBridge archive/replay reprocesses stale events into non-idempotent workflows.
- Pipes/enrichment target roles allow unintended read/write access.

## Minimum safe workflow

1. Map producers, buses/topics/queues/state machines, consumers, schemas, ownership, and account boundaries.
2. Define delivery guarantees: ordering, duplication, latency, retention, replay, and poison-message handling.
3. Review filtering and routing precision for EventBridge rules, SNS filters, Pipes, and cross-account bus policies.
4. Verify retry, DLQ, redrive, archive, replay, timeout, and idempotency behavior for every consumer.
5. Check observability: backlog, age, failures, throttles, retries, DLQ depth, business counters, and trace correlation.
6. Compare service choice: SNS, SQS, EventBridge, Step Functions, or Pipes based on coupling and failure semantics.
7. Require approval before live replay, redrive, rule broadening, or policy mutation.

## Verification targets

- EventBridge event buses, rules, patterns, targets, DLQs, archives, replays, global endpoints, and bus policies
- SQS queue type, visibility timeout, retention, redrive policy, DLQ, FIFO/dedup, and ApproximateAgeOfOldestMessage
- SNS topic policy, subscription filters, delivery status, DLQ, encryption, and cross-account subscribers
- Step Functions retries, catches, timeouts, compensation paths, execution history, and idempotency keys
- EventBridge Pipes source/filter/enrichment/target roles and failure destinations
- schema registry/contracts, producer ownership, consumer ownership, and version compatibility

## When to push back

Push back if the user asks to:

- replay or redrive events without idempotency and blast-radius proof
- broaden EventBridge rules to catch unknown events
- ignore DLQ growth because primary flow is healthy
- use async messaging to hide slow or unreliable dependencies
- grant cross-account bus access without source and detail-type constraints
- choose SNS/SQS/EventBridge without stating delivery semantics
