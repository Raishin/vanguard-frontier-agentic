# Evidence Path and Tooling

## Evidence path

1. Prefer live cluster evidence when a Kubernetes MCP server, `kubectl`, and access to the OpenTelemetry Operator namespace are available.
2. Fall back to official OpenTelemetry documentation (opentelemetry.io, opentelemetry-operator GitHub) when live inspection is unavailable.
3. Ask only for sanitized `OpenTelemetryCollector` / `Instrumentation` YAML, collector logs, and target backend reachability evidence when current-state proof matters.
4. Label conclusions as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`.

## Useful live-evidence commands

```shell
# All Collectors and Instrumentation CRs across the cluster
kubectl get opentelemetrycollectors,instrumentations -A -o yaml

# Detailed Collector status — replicas, mode, generated config map
kubectl -n <ns> get opentelemetrycollector <name> -o yaml
kubectl -n <ns> get configmap <collector-name>-collector -o yaml

# Operator state
kubectl -n opentelemetry-operator-system get deploy,svc,validatingwebhookconfiguration

# Collector pod logs — confirm pipeline is processing data
kubectl -n <ns> logs deploy/<collector-name>-collector --tail=200 -f

# Collector internal metrics (Prometheus on :8888 by default)
kubectl -n <ns> port-forward svc/<collector-name>-collector 8888:8888
curl http://localhost:8888/metrics | grep otelcol_

# Auto-instrumentation propagation — which pods received the init container?
kubectl get pods -A -o jsonpath='{range .items[?(@.metadata.annotations.instrumentation\.opentelemetry\.io/inject-java=="true")]}{.metadata.namespace}/{.metadata.name}{"\n"}{end}'

# Verify exporter reachability from within the collector pod
kubectl -n <ns> exec -it deploy/<collector-name>-collector -- nc -zv <exporter-host> <exporter-port>
```

## Operator and Collector state to confirm before review

- Operator version (`kubectl -n opentelemetry-operator-system get deploy opentelemetry-operator-controller-manager -o jsonpath='{.spec.template.spec.containers[*].image}'`) — `OpenTelemetryCollector` API has evolved; `v1beta1` is the current stable.
- Collector image and version — different versions support different receivers/processors/exporters. The contrib distribution has a much wider set than the core distribution.
- Whether Target Allocator is deployed — required for `mode: statefulset` Prometheus scraping at scale.
- Whether `Instrumentation` CRs exist and which language images are pinned (Java, Node, Python, .NET, Go) — version drift between auto-instrumentation images and application runtimes is a common silent failure mode.
- Backend reachability — the actual telemetry destination (vendor SaaS, Tempo, Jaeger, Prometheus remote write, Loki) must accept the collector's data; check from inside the pod.

## Sanitization rules

- Never request kubeconfig contents, vendor API keys, OTLP bearer tokens, or backend authentication secrets.
- Replace identifiable backend hostnames, vendor URLs, and tenant IDs with placeholders unless the user provides them.
- Do not print the collector's `Authorization` header values; reference them by configuration key only.
