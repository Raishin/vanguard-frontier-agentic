# Workflow and Output Contract

## Workflow

### Step 1 — Identify the deployment mode

`OpenTelemetryCollector` supports four deployment modes, each appropriate for different use cases:

1. **`mode: deployment`** — collector runs as a stateless `Deployment`, multiple replicas. Use for OTLP gateway / aggregation; NOT for hostmetrics.
2. **`mode: statefulset`** — ordered, stable identity. Required for Target Allocator (sharding Prometheus scrape jobs across collectors).
3. **`mode: daemonset`** — one collector per node. Use for hostmetrics, filelog (node-local logs), and per-node OTLP receiver.
4. **`mode: sidecar`** — injected into application pods via annotation `sidecar.opentelemetry.io/inject: <name>`. Use for short-lived workloads or when application cannot reach a cluster-wide collector.

Common mismatches that are findings:

- `mode: deployment` with `hostmetrics` receiver — only one replica gets host data; data is incomplete.
- `mode: daemonset` with HTTP receiver bound to `0.0.0.0:4318` — every node opens a port; verify network policy.
- `mode: statefulset` without Target Allocator — wastes the ordered identity.
- `mode: sidecar` for high-volume workloads — every pod runs a collector; CPU/memory cost multiplies.

Reference: [Operator Modes](https://opentelemetry.io/docs/kubernetes/operator/) and the operator README in [open-telemetry/opentelemetry-operator](https://github.com/open-telemetry/opentelemetry-operator).

### Step 2 — Audit the receivers

Receivers ingest telemetry. Common patterns:

- **`otlp`** — gRPC (`:4317`) and HTTP (`:4318`). Standard. Verify both protocols are needed; otherwise narrow.
- **`prometheus`** — scrapes Prometheus endpoints. Pair with Target Allocator at scale.
- **`hostmetrics`** — node CPU, memory, disk, network. Requires `hostNetwork` or volume mounts (`/hostfs`).
- **`filelog`** — reads pod/container logs. Requires `/var/log/pods` mount.
- **`k8s_cluster`** — cluster-level metrics (deployment status, node conditions). Requires RBAC.
- **`kubeletstats`** — kubelet per-node stats. Requires kubelet TLS configuration.

Findings to flag:

- `otlp` receiver with `tls.insecure: true` and inbound traffic from untrusted networks — telemetry can be tampered.
- `prometheus` receiver scraping endpoints with secrets in the response (rare; some vendor exporters do this) — sensitive data flows into the pipeline.
- `filelog` without a `multiline` config for stack traces — multi-line logs split into single-line entries.

### Step 3 — Audit the processors (the safety net)

Processors transform data between receiver and exporter. **Two are essentially mandatory in production**:

1. **`memory_limiter`** — drops data when collector memory exceeds a threshold. Without it, collector OOMs under load and loses everything in flight. Recommended position: **first** in the pipeline.
2. **`batch`** — batches data before export. Without it, every span/metric is a separate export call; backend rate limits or network overhead destroy throughput. Recommended position: **last** before export.

Other commonly required processors:

- **`k8sattributes`** — enriches data with `k8s.namespace.name`, `k8s.pod.name`, `k8s.deployment.name`, `k8s.node.name`. Without it, dashboards and SLOs cannot group by Kubernetes object.
- **`resource`** — sets static resource attributes (e.g., `cluster.name`, `deployment.environment`).
- **`resourcedetection`** — auto-detects from environment, system, docker, kubernetes, GCP, AWS, Azure metadata services.
- **`tail_sampling`** — keeps a sample of complete traces. **Critical caveat: changes are not retroactive — already-collected windows do not get re-sampled.**
- **`filter`** — drops spans/metrics by attribute. Risk: a typo can drop everything.
- **`transform`** — modifies attribute values via OTTL. Risk: a bad OTTL expression can corrupt every signal.
- **`probabilistic_sampler`** — randomly samples a percentage. Simpler than tail sampling but loses correlated traces.

Stress-tests:

- Pipeline with no `memory_limiter` and high-volume traces — collector OOMs on burst, loses everything.
- Pipeline with `memory_limiter` placed **after** other processors — those processors run on data that should have been dropped, wasting CPU.
- Pipeline with `batch` placed **before** `tail_sampling` — sampling decisions are made per-batch, breaking trace coherence.
- Pipeline with `k8sattributes` `auth_type: serviceAccount` but no RBAC granting `pods/get,list,watch` — enrichment fails silently.

Reference: [Collector configuration](https://opentelemetry.io/docs/collector/configuration/) and [Collector processors](https://github.com/open-telemetry/opentelemetry-collector-contrib/tree/main/processor).

### Step 4 — Audit the exporters

Exporters send data to backends. Findings:

- **No exporter on a pipeline** — the pipeline silently drops everything. Confirm at least one non-`debug` exporter per pipeline.
- **Only `debug` exporter** in production — data prints to collector logs and is not sent anywhere. Useful for testing only.
- **`tls.insecure: true`** on a production exporter — telemetry flows in plaintext. PII/PHI leak path.
- **Missing `sending_queue`** — exporter blocks the pipeline when backend is slow; backpressure cascades.
- **`sending_queue.enabled: false`** explicitly — telemetry is lost on any backend hiccup.
- **`retry_on_failure.enabled: false`** — temporary network failures lose data.
- **`prometheusremotewrite` exporter without `external_labels`** — multiple collectors write to the same Prometheus, time series collide.

Reference: [Exporter configuration patterns](https://opentelemetry.io/docs/collector/configuration/#exporters).

### Step 5 — Audit the `service.pipelines` ordering

Three signal pipelines (`traces`, `metrics`, `logs`) compose receivers → processors → exporters. Order in the `processors` list **matters** — it is the execution order.

Recommended order for a traces pipeline:

```yaml
service:
  pipelines:
    traces:
      receivers: [otlp]
      processors:
        - memory_limiter        # 1. drop early under pressure
        - resourcedetection     # 2. detect environment
        - k8sattributes         # 3. enrich with K8s context
        - resource              # 4. add static attributes
        - tail_sampling         # 5. sample after enrichment
        - batch                 # 6. batch last
      exporters: [otlp, debug]
```

Common findings: `batch` not last, `memory_limiter` not first, `k8sattributes` after `tail_sampling` (sampling on un-enriched data, then enriching what survived = wasted).

### Step 6 — Audit the `Instrumentation` CR

The `Instrumentation` CR (`opentelemetry.io/v1alpha1`) drives auto-instrumentation. Pods are instrumented when they have one of the annotations: `instrumentation.opentelemetry.io/inject-java`, `inject-nodejs`, `inject-python`, `inject-dotnet`, `inject-go`, or `inject-sdk`.

Critical concerns:

- **Removing an `Instrumentation` CR while pods reference it** — running pods continue working, but on next restart the init container injection fails, and the pod starts without instrumentation. Telemetry stops silently.
- **Image tag drift** — auto-instrumentation images are pinned per language. If the application moves to a newer runtime (e.g., Java 21) but the auto-instrumentation image hasn't been updated, instrumentation may not load.
- **`exporter.endpoint` pointing to a collector that no longer exists** — telemetry calls fail; application logs may show OTLP export errors.
- **`sampler.type: parentbased_traceidratio` with `argument: "0.0"`** — samples nothing.
- **Missing `propagators`** — distributed traces don't link across services.
- **`resource.resourceAttributes.deployment.environment` not set** — every environment looks the same in dashboards.

Reference: [Operator auto-instrumentation](https://opentelemetry.io/docs/kubernetes/operator/automatic/).

### Step 7 — Audit the Target Allocator (StatefulSet mode)

When `targetAllocator.enabled: true`, Prometheus scrape jobs are sharded across the StatefulSet replicas. Findings:

- `targetAllocator.allocationStrategy: least-weighted` (default) is good for even distribution; `consistent-hashing` is better for re-shard stability.
- `targetAllocator.prometheusCR.enabled: true` requires `ServiceMonitor`/`PodMonitor` selectors. An empty selector matches everything; a too-narrow selector matches nothing.
- Missing RBAC for the Target Allocator — it cannot list ServiceMonitors and silently scrapes nothing.

Reference: [Target Allocator](https://opentelemetry.io/docs/kubernetes/operator/target-allocator/).

### Step 8 — Stress-test operational hygiene

- Prefer `v1beta1` `OpenTelemetryCollector` over `v1alpha1` — current stable.
- Prefer named pipelines that match the source data shape (`traces/api`, `metrics/host`, `logs/app`) when one collector handles multiple streams.
- Prefer `debug` exporter only in non-production.
- Prefer `OTEL_RESOURCE_ATTRIBUTES` env propagation in `Instrumentation` over hardcoded values — makes the CR portable across environments.
- Test pipeline changes by sending synthetic OTLP and watching the collector's `otelcol_` self-metrics — `otelcol_exporter_send_failed_spans` should be zero.

## Output

Return:

- **target**: which `OpenTelemetryCollector` (and mode) or `Instrumentation` CR,
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **deployment-mode appropriateness** for the use case,
- **pipeline correctness**: receivers, processors (with explicit `memory_limiter` and `batch` audit), exporters,
- **failure mode**: what happens when backend is unreachable or backed up,
- **risk findings** (with severity: high / medium / low),
- **safest next actions** with sample manifest changes and self-metric expectations,
- **rollback plan**: how to revert without losing the in-flight buffer,
- **assumptions and missing facts**.

## Security notes

- Never recommend removing `memory_limiter` from a production pipeline.
- Never recommend `tls.insecure: true` on a production exporter shipping data outside the cluster.
- Never recommend deleting an `Instrumentation` CR without first confirming no running deployments reference it via annotation.
- Do not print collector authentication tokens or vendor API keys; reference them by configuration key only.
