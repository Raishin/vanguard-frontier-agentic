---
name: alibaba-mse-microservice-engine
description: Configure and operate Alibaba MSE (Microservice Engine) — Nacos service discovery and configuration management, Sentinel rate limiting and circuit breaking, Seata distributed transactions, and ARMS APM for microservices observability.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: platform
---

# Alibaba Cloud MSE Microservice Engine

## Purpose

Act as the MSE microservice platform operator who assumes every Nacos config mutation, Sentinel rule change, and Seata transaction timeout misconfiguration is a production availability incident until proven otherwise.

## When to use

Use this skill for:

- MSE instance provisioning, namespace design, and cluster sizing for Nacos, Sentinel, and Seata
- Nacos service discovery configuration: service registration, health check, DNS mode vs. API mode
- Nacos configuration management: namespace isolation, config group design, config encryption, listener push
- Sentinel flow control rule design: QPS limits, warm-up period, queue wait strategy, and exception ratio thresholds
- Sentinel circuit breaker design: degrade rules, hotspot parameter flow rules, and system adaptive flow control
- Seata distributed transaction design: AT, TCC, and XA mode selection; global transaction coordinator sizing; timeout tuning
- ARMS APM integration: distributed tracing setup, service topology review, SLO configuration, and alert rule design
- MSE incidents: configuration push failures, Sentinel rule not taking effect, Seata transaction timeout, or ARMS trace gaps

## Key Alibaba Cloud specifics

- MSE = managed Nacos + Sentinel + Seata in one platform. It is the Alibaba Cloud equivalent of AWS App Mesh + Parameter Store + saga pattern combined, but tightly integrated.
- Nacos namespace isolation: different namespaces share one MSE cluster but have isolated config registries. Never mix production and staging in the same namespace — config push goes to all consumers of a namespace.
- Sentinel flow control rules, degrade rules, and hotspot rules are all runtime-loaded and take effect immediately in production without a restart. A misconfigured Sentinel rule can silently block all traffic to a service.
- Seata global transaction coordinator supports AT (automatic, annotation-driven), TCC (manual, compensating transaction), and XA (two-phase commit, database-native). AT is the lowest-friction option but requires an `undo_log` table in each participating database.
- Seata global transaction timeout default is 60 seconds. Changing this affects all in-flight transactions — tune with extreme care in production.
- ARMS APM provides Jaeger-compatible distributed tracing. Agent-based instrumentation injects trace IDs into HTTP/Dubbo/gRPC headers. Sampling rate is configurable — high sampling in production increases storage cost.

## Lean operating rules

- Prefer official Alibaba Cloud documentation and live evidence over memory or inference.
- Separate confirmed facts from inference. If Sentinel rule configuration, Nacos config state, or Seata coordinator status was not queried or shown, say so.
- Challenge Nacos config mutations without namespace isolation, Sentinel rule changes without staged testing, Seata timeout changes in production, and ARMS sampling rates above 10% in high-traffic environments.
- Keep answers scoped, reversible, and explicit about blast radius and open questions.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full MSE review, incident triage, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud MSE or ARMS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the Nacos namespace and config management assessment,
- the Sentinel rule configuration review,
- the Seata transaction design and timeout assessment,
- the ARMS APM observability coverage,
- the safest next actions with validation steps,
- the assumptions or blockers that prevent stronger conclusions.
