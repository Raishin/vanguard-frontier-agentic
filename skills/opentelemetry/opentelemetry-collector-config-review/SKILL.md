---
name: opentelemetry-collector-config-review
description: Use this skill for OpenTelemetry Operator review covering OpenTelemetryCollector deployment modes (Deployment, StatefulSet, DaemonSet, Sidecar), Instrumentation CR auto-instrumentation across Java/Node/Python/.NET/Go, Target Allocator for distributed Prometheus scraping, and pipeline correctness across receivers, processors, and exporters. Trigger when the user asks whether a collector configuration will lose telemetry, whether the right deployment mode is used, whether memory_limiter and batch are present, whether tail_sampling is safe to change, or whether auto-instrumentation will cover a workload after restart.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# OpenTelemetry Collector Config Review

## Purpose

Review OpenTelemetry Operator-managed `OpenTelemetryCollector` and `Instrumentation` resources against pipeline correctness, deployment-mode appropriateness, memory safety, sampling integrity, exporter security, and Kubernetes-attribute enrichment. Telemetry pipelines fail silently — a misconfigured exporter drops every span; a missing `memory_limiter` OOMs the collector; a deleted `Instrumentation` resource stops auto-instrumentation on next pod restart.

## Lean operating rules

- Prefer live cluster evidence (`kubectl get opentelemetrycollectors,instrumentations -A -o yaml` plus collector logs and metrics) when the active client exposes it; otherwise fall back to official OpenTelemetry documentation (opentelemetry.io, opentelemetry-operator) and sanitized YAML.
- Separate confirmed facts from inference. If collector pipeline state, exporter health, or `Instrumentation` propagation was not queried, say so.
- Treat **a pipeline with no exporter** (or with only `debug` exporter in production) as a critical finding — telemetry is dropped at the collector.
- Treat **removal of the `memory_limiter` processor** as a critical finding — collector OOMs and loses spans/metrics on burst traffic.
- Treat **removal of the `k8sattributes` processor** as a high finding — telemetry loses `k8s.namespace.name`, `k8s.pod.name`, `k8s.deployment.name`, and SLO dashboards lose context.
- Challenge tail sampling rule changes — past spans are not re-evaluated; sampling drift is permanent for already-collected windows.
- Challenge `Instrumentation` CR removal in a running namespace — auto-instrumented pods stop emitting telemetry after their next restart.
- Challenge collector exporters with `tls.insecure: true` in production — telemetry data flows in plaintext, often containing PII/PHI.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.

## References

Load these only when needed:

- [Evidence path and tooling](references/mcp-and-evidence.md) — use when choosing live evidence, confirming Operator version and Collector pipeline state, or switching to documentation mode.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, applying stress checks per deployment mode, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when you need the detailed OpenTelemetry documentation list, processor pipeline references, and grounded insights.

## Response minimum

Return, at minimum:

- the scoped target (`OpenTelemetryCollector` of which mode, `Instrumentation` CR, or pipeline element) and evidence level,
- the deployment-mode appropriateness (Deployment / StatefulSet / DaemonSet / Sidecar) for the use case,
- the pipeline correctness (receivers, processors, exporters all present and ordered safely),
- the failure mode if exporter is unreachable or downstream is full (queue, drop, retry semantics),
- the safest next actions and rollback plan,
- the assumptions or blockers that prevent stronger conclusions.
