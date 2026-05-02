# 🔭 OpenTelemetry Skills

<p align="center">
  <!-- 🖼️ Add an OpenTelemetry logo to assets/logos/cnative/opentelemetry/ and update this path -->
  <span style="font-size:3.5em">🔭</span>
</p>

This folder contains OpenTelemetry-focused skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **1** local OpenTelemetry skill:

- `opentelemetry-collector-config-review`

## Portfolio posture

OpenTelemetry skills for evidence-backed observability pipeline review covering the four `OpenTelemetryCollector` deployment modes (`deployment`, `statefulset`, `daemonset`, `sidecar`), the `Instrumentation` CR for auto-instrumentation across Java/Node/Python/.NET/Go, the Target Allocator for distributed Prometheus scraping, and exporter/processor/receiver pipeline correctness.

These skills are intentionally conservative:

- prefer `kubectl get opentelemetrycollectors,instrumentations -A -o yaml` for live collector state grounding before any review
- treat **collector pipeline with no exporter** as a critical finding — telemetry is silently dropped at collector boundary
- treat **removal of `memory_limiter` processor** as a critical finding — collector OOMs and loses spans/metrics
- challenge tail sampling rule changes — past spans are not re-evaluated, sampling drift is permanent for already-collected windows
- challenge `Instrumentation` CR removal from a running namespace — auto-instrumented pods stop emitting telemetry on next restart
- challenge collector `service.pipelines` lacking the `k8sattributes` processor — telemetry loses Kubernetes context (namespace, pod, deployment)
- challenge TLS `insecure: true` on production exporters — telemetry data flows in plaintext, often containing PII
- use official OpenTelemetry documentation (opentelemetry.io, opentelemetry-operator) for Collector/Instrumentation CRD syntax, processor pipelines, and Target Allocator semantics

Run `npm run validate` after changing cataloged OpenTelemetry skills.
