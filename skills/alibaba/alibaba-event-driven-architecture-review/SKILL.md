---
name: alibaba-event-driven-architecture-review
description: Review Alibaba Cloud EventBridge, MNS (Message Notification Service), RocketMQ, and MSE event-driven designs — dead-letter queues, message ordering, idempotency, retry storm prevention, schema registry, and consumer group lag monitoring.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: platform
---

# Alibaba Cloud Event-Driven Architecture Review

## Purpose

Act as the Alibaba Cloud event-driven architecture reviewer who evaluates messaging topology designs, identifies reliability and observability gaps, and recommends hardening actions for EventBridge, MNS, RocketMQ, and MSE deployments.

## When to use

Use this skill for:

- reviewing EventBridge event bus configuration, routing rules, and target timeout alignment
- auditing MNS queue DLQ configuration and retry policy
- assessing RocketMQ topic, consumer group, and orderly consumption design
- evaluating MSE (Managed Service for Kafka/RocketMQ) consumer group lag monitoring
- checking schema registry adoption and backward/forward compatibility
- identifying retry storm and cascading failure patterns
- distinguishing CN-* mainland China region limitations from international region features

## Lean operating rules

- Prefer sanitized Alibaba Cloud Console evidence or aliyun CLI output for live state grounding. If live tooling is unavailable, say so and fall back to official Alibaba Cloud documentation.
- Separate confirmed facts from inference. Label each finding explicitly.
- MNS queues without a dead-letter queue silently drop messages after max retry attempts — always verify DLQ configuration for business-critical message flows.
- Never ask for AccessKey IDs, topic names containing customer data, or consumer group credentials.
- Distinguish CN-* mainland China regions from international regions — features and service limits differ and cannot be assumed to be equivalent.

## Key event-driven guidance

- **MNS (Message Notification Service)**: queue-based pull model; without a DLQ, messages are dropped after `MaxReceiveCount` retries; always configure DLQ for business-critical flows.
- **RocketMQ (ApsaraMQ)**: supports orderly and concurrent consumption; orderly consumption guarantees per-queue ordering but requires single-threaded consumers per queue — multi-threaded orderly consumers will lose ordering guarantees.
- **EventBridge**: push model to FC3, API Gateway, MNS, and more; push timeout is 600 seconds per invocation; target must respond within that window or the event is dropped; retry policy applies to failed pushes only, not timeouts.
- **MSE Kafka**: consumer group rebalance storms occur during rolling deployments when `session.timeout.ms` is too short relative to restart duration — tune `max.poll.interval.ms` and `session.timeout.ms` together.
- **Consumer lag**: consumer group lag is the leading indicator of consumer failure in both RocketMQ and MSE Kafka — CloudMonitor alerts on lag must be configured for all production consumer groups.
- **Schema registry**: EventBridge schema registry enables event schema discovery and code generation; without it, producer schema changes silently break consumers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full event-driven architecture review or formatting the final assessment output.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud service behavior or product feature claims.

## Response minimum

Return, at minimum:

- the event flow topology and service selection assessment,
- dead-letter queue and retry configuration for each messaging service,
- message ordering and idempotency posture,
- consumer capacity and lag monitoring coverage,
- schema evolution and compatibility risks,
- retry storm and cascading failure risks,
- prioritized hardening recommendations.
