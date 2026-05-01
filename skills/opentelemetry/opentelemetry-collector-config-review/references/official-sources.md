# Official Sources

Load these only when needed:

- [OpenTelemetry documentation home](https://opentelemetry.io/docs/) — use as the entry point for any OTEL question.
- [Collector overview](https://opentelemetry.io/docs/collector/) — use for collector architecture, distributions (core vs contrib), and component model.
- [Collector configuration](https://opentelemetry.io/docs/collector/configuration/) — use for receivers, processors, exporters, extensions, and `service.pipelines` syntax.
- [Operator overview](https://opentelemetry.io/docs/kubernetes/operator/) — use for `OpenTelemetryCollector` CRD, deployment modes, and Operator behavior.
- [Operator automatic instrumentation](https://opentelemetry.io/docs/kubernetes/operator/automatic/) — use for `Instrumentation` CR, language-specific init containers, annotation-based pod injection.
- [Target Allocator](https://opentelemetry.io/docs/kubernetes/operator/target-allocator/) — use for sharding Prometheus scrape jobs across collector replicas.
- [opentelemetry-operator GitHub](https://github.com/open-telemetry/opentelemetry-operator) — use for CRD source, examples, and recent feature notes.
- [opentelemetry-collector-contrib processors](https://github.com/open-telemetry/opentelemetry-collector-contrib/tree/main/processor) — use for `k8sattributes`, `resourcedetection`, `tail_sampling`, `transform`, `filter`, `routing` processor configs.
- [opentelemetry-collector-contrib receivers](https://github.com/open-telemetry/opentelemetry-collector-contrib/tree/main/receiver) — use for `kubeletstats`, `k8s_cluster`, `prometheus`, `filelog` receiver configs.
- [opentelemetry-collector-contrib exporters](https://github.com/open-telemetry/opentelemetry-collector-contrib/tree/main/exporter) — use for vendor exporters and queue/retry semantics.
- [Sampling guide](https://opentelemetry.io/docs/concepts/sampling/) — use when designing tail sampling vs probabilistic sampling vs head sampling.
- [Semantic conventions for Kubernetes](https://opentelemetry.io/docs/specs/semconv/resource/k8s/) — use for the canonical `k8s.*` attribute names that `k8sattributes` populates.
- [Collector internal observability](https://opentelemetry.io/docs/collector/internal-telemetry/) — use for `otelcol_*` self-metrics that diagnose collector health.

## Grounded insights worth carrying into the skill

- The OpenTelemetry Operator manages `OpenTelemetryCollector` and `Instrumentation` CRs and supports four deployment modes: `deployment`, `statefulset`, `daemonset`, and `sidecar`. Each is appropriate for a different use case and the wrong mode silently produces incomplete or duplicate data.
- A pipeline with **no exporter** is valid YAML and silently drops every span/metric/log. The collector emits an internal warning at startup but otherwise behaves as if data is being processed.
- `memory_limiter` is the only protection against OOM under burst load. Without it, the collector consumes memory until the kernel kills the pod and loses everything in flight. It is recommended as the **first processor** in every pipeline.
- `batch` is recommended **last before exporters** because batching drops in-flight individual signals into batched export calls. Without it, every span is a separate export, which destroys throughput at any meaningful volume.
- `k8sattributes` enriches signals with Kubernetes object names. Without it, traces and metrics cannot be grouped by namespace/pod/deployment, breaking SLO dashboards and alerting. It requires RBAC: `pods/get,list,watch`, `namespaces/get,list,watch`, `replicasets/get,list,watch`.
- `tail_sampling` is the most common production sampling mode because it samples on complete trace properties (root span attributes, total duration). The critical caveat: **changes are not retroactive** — already-collected windows do not re-sample, so a sampling change creates a discontinuity in observed trace counts.
- `Instrumentation` CR removal is invisible to running pods; the next pod restart silently starts without auto-instrumentation. Many silent SLO regressions trace back to an `Instrumentation` CR being removed during a "cleanup".
- The Target Allocator is required for any `mode: statefulset` Prometheus collector serving more than a handful of scrape targets. Without it, every replica scrapes every target and the data is duplicated.
- Auto-instrumentation images are pinned per language (Java, Node.js, Python, .NET, Go). When the application's runtime version moves ahead of the instrumentation image, instrumentation can fail to load silently. Treat the auto-instrumentation image versions as a cataloged dependency.
- The collector exposes its own metrics on `:8888/metrics`. The most useful Prometheus series for diagnosing pipeline health: `otelcol_exporter_send_failed_spans`, `otelcol_processor_dropped_spans`, `otelcol_receiver_refused_spans`, `otelcol_processor_batch_send_size`. Any non-zero value on the failure counters is a finding.
- The `debug` exporter (formerly `logging` exporter) prints to the collector's stdout and is meant for development. It is a frequent silent failure mode in production when someone replaced a real exporter with `debug` for debugging and forgot to restore it.
