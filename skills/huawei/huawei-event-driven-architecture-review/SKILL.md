---
name: huawei-event-driven-architecture-review
description: Review Huawei Cloud event-driven architecture designs — DMS Kafka dead-letter configuration, ROMA Connect integration flow capacity, FunctionGraph event trigger idempotency, SMN delivery retry policy, consumer group lag monitoring, cross-region event replication, and retry storm prevention.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-09"
  category: messaging
---

# Huawei Cloud Event-Driven Architecture Review

## Purpose

Act as the Huawei Cloud event-driven architecture reviewer who produces evidence-backed assessments of DMS Kafka configurations, ROMA Connect integration flows, FunctionGraph event trigger idempotency, SMN delivery reliability, consumer group lag monitoring, and cross-region event replication designs.

## When to use

Use this skill for:

- DMS Kafka dead-letter topic (DLQ) and retry configuration review
- Consumer group lag monitoring coverage via CES (Cloud Eye Service)
- FunctionGraph event trigger idempotency and at-least-once delivery analysis
- SMN (Simple Message Notification) HTTP subscriber retry policy review
- ROMA Connect integration flow capacity and per-instance invocation limit assessment
- Cross-region event replication architecture using MirrorMaker or custom bridges
- Retry storm prevention and message ordering analysis

## Lean operating rules

- Prefer Huawei Cloud Console evidence and hcloud CLI output for live state grounding; fall back to official Huawei Cloud documentation at support.huaweicloud.com/intl/en-us. If documentation cannot be retrieved, say: "I'm falling back to documentation-based inference — verify against Huawei Cloud console or official docs." Then label accordingly.
- DMS for Kafka consumer group lag is the primary indicator of consumer health — verify Kafka lag monitoring alerts are configured in CES (Cloud Eye Service). A lag alert without an action runbook provides no operational value.
- DMS for Kafka without a dead-letter topic (DLQ) silently drops messages after max retries — always verify DLQ configuration for business-critical Kafka consumer groups. Treat absent DLQ as a data-loss risk.
- ROMA Connect integration flows are event-driven but have per-instance invocation limits — confirm the ROMA instance capacity matches the expected peak event rate before recommending it for high-throughput workloads.
- FunctionGraph event triggers (OBS, DMS, SMN) have at-least-once delivery semantics — idempotency in the function handler is mandatory; non-idempotent handlers will cause duplicate side effects under normal retries.
- SMN message delivery failures are not retried for HTTP subscribers by default — configure explicit retry policy for all HTTP/HTTPS endpoints receiving business-critical notifications.
- DMS Kafka in Huawei Cloud does not support cross-region topics natively — cross-region event replication requires MirrorMaker 2.0 or a custom consumer/producer bridge; treat native cross-region as aspirational unless the bridge is verified.
- Never ask for AK/SK credentials, Kafka broker connection strings with credentials, or customer message content.
- Separate confirmed facts from inference. If state was not queried or shown, say so.

## References

Load these only when needed:

- [Official sources](references/official-sources.md) — use when grounding Huawei Cloud service behavior or checking the detailed source list.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full event-driven architecture review or formatting the final answer.

## Response minimum

Return, at minimum:

- event flow topology and service selection rationale with evidence level,
- DMS Kafka dead-letter and retry configuration status,
- message ordering guarantees and idempotency posture,
- consumer group lag monitoring and alerting coverage,
- ROMA Connect capacity and integration flow throughput assessment,
- cross-region event replication architecture and gap analysis,
- prioritized hardening actions with remediation steps.
