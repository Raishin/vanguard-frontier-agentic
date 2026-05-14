# Metric Sources

This reference describes how to gather p50, p95, and p99 CPU and memory utilization metrics from common observability platforms for use as input to the rightsize-recommendation skill. These are recipes and query patterns, not commands to be executed by the skill.

## Why percentile metrics matter

Using only the average (mean) of CPU or memory utilization understates peaks and leads to under-resourced pods. Using the p99 alone for requests leads to over-provisioning for workloads with rare spikes. The combination of p50, p95, and p99 over a 7-14 day window captures normal load, burst behavior, and outlier events.

- p50: median utilization; represents steady-state load.
- p95: captures the top 5% of utilization events; used for request sizing.
- p99: captures the top 1% of utilization events; used for limit sizing.

## Prometheus (self-hosted or managed)

### CPU utilization per pod (millicores)

```
# p50 CPU (millicores) over 14 days
quantile_over_time(0.50,
  rate(container_cpu_usage_seconds_total{
    namespace="<namespace>",
    pod=~"<pod-name-prefix>.*",
    container!="POD",
    container!=""
  }[5m])[14d:5m]
) * 1000

# p95 CPU (millicores) over 14 days
quantile_over_time(0.95, ...) * 1000

# p99 CPU (millicores) over 14 days
quantile_over_time(0.99, ...) * 1000
```

Replace `<namespace>` and `<pod-name-prefix>` with your values. Adjust the window (`14d`) and step (`5m`) as needed. For a Deployment or StatefulSet, aggregate across all replica pods by including `pod=~"<deployment-name>-.*"`.

### Memory working set per pod (MiB)

```
# p95 memory working set (MiB) over 14 days
quantile_over_time(0.95,
  container_memory_working_set_bytes{
    namespace="<namespace>",
    pod=~"<pod-name-prefix>.*",
    container!="POD",
    container!=""
  }[14d:5m]
) / 1048576
```

Use `container_memory_working_set_bytes` (not `container_memory_usage_bytes`) because the working set excludes file-backed pages that can be evicted by the kernel without causing an OOMKill. This gives a more accurate picture of the memory the container actually needs to retain.

## Google Cloud Monitoring (GKE)

Use the Cloud Monitoring Metrics Explorer or the Metrics Query Language (MQL) console.

Relevant metric types:

- CPU: `kubernetes.io/container/cpu/request_utilization` (fraction of requested CPU used) or `kubernetes.io/container/cpu/core_usage_time` (cumulative core-seconds)
- Memory: `kubernetes.io/container/memory/request_utilization` (fraction of requested memory used) or `kubernetes.io/container/memory/used_bytes`

To compute p95 over 14 days using MQL:

```
fetch k8s_container
| metric 'kubernetes.io/container/memory/used_bytes'
| filter (resource.namespace_name == '<namespace>'
      && resource.container_name != 'POD')
| group_by [resource.pod_name, resource.container_name],
    percentile(value.used_bytes, 95)
| within 14d
```

Divide the result by 1,048,576 to convert bytes to MiB before passing to the skill.

## Azure Monitor (AKS)

Use Container Insights or the Azure Monitor Metrics blade.

Relevant metrics:

- CPU: `cpuUsageNanoCores` (container CPU usage in nanocores; divide by 1,000,000 to convert to millicores)
- Memory: `memoryWorkingSetBytes` (working set in bytes; divide by 1,048,576 for MiB)

To query p95 over 14 days using Azure Monitor Logs (KQL):

```kql
InsightsMetrics
| where Namespace == "container.azm.ms/cpuUsageNanoCores"
| where Tags contains '"namespace":"<namespace>"'
| summarize p95 = percentile(Val, 95) by bin(TimeGenerated, 5m)
| where TimeGenerated > ago(14d)
| summarize p95_cpu_mc = percentile(p95, 95) / 1000000
```

## OpenCost (in-cluster)

If OpenCost is deployed, it exposes a REST API that returns pre-computed allocation data including max usage over a window. The skill cannot call this API directly, but the caller can retrieve the data using:

```
curl -G http://<opencost-service>:9003/allocation \
  --data-urlencode 'window=14d' \
  --data-urlencode 'aggregate=pod' \
  --data-urlencode 'namespace=<namespace>'
```

From the response, extract `cpuCoreRequestAverage`, `cpuCoreUsageAverage`, `ramByteRequestAverage`, and `ramByteUsageAverage`. Note: OpenCost exposes averages, not percentiles. If percentile data is required, use Prometheus directly.

## Vertical Pod Autoscaler (VPA) recommendations as a starting point

If VPA is deployed in recommendation mode (not auto), it exposes `LowerBound`, `Target`, `UpperBound`, and `UncappedTarget` for CPU and memory. These can serve as an approximate p50-to-p99 range:

- `LowerBound` ≈ p50 equivalent (conservative minimum)
- `Target` ≈ p95 equivalent (VPA recommendation)
- `UpperBound` ≈ p99 equivalent (headroom ceiling)

These are VPA's internal heuristics, not directly equivalent to Prometheus percentiles. Label them as `assumed` when using VPA output as a substitute for percentile metrics.

To retrieve VPA recommendations without modifying the cluster:

```
# Output from: kubectl describe vpa <vpa-name> -n <namespace>
# Paste the "Container Recommendations:" block into the skill input
```
