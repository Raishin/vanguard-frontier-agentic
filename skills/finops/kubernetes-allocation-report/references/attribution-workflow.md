# Attribution Workflow

## Cost categories in Kubernetes allocation

Kubernetes cluster cost is divided into three categories for FinOps reporting:

| Category | Definition |
|---|---|
| Allocated | Cost directly attributed to a pod's resource requests (CPU, memory, GPU, PV). |
| Idle | Cost of cluster capacity that is not claimed by any pod's requests. Idle = Total - Allocated. |
| Unallocated | Cost of cluster capacity for pods whose owner cannot be identified (e.g., pods with no workload label, completed Jobs, or orphaned pods). A subset of Idle in most implementations. |

## Request-based allocation (default)

Request-based allocation attributes cost by the resource requests declared in each pod's spec, regardless of actual utilization.

Steps:

1. Determine allocatable resources per node (total vCPU and RAM minus system overhead).
2. Sum allocatable resources across all nodes in the reporting window.
3. For each pod, compute its CPU share = (pod CPU request in millicores) / (total cluster allocatable CPU in millicores).
4. Compute memory share = (pod memory request in MiB) / (total cluster allocatable memory in MiB).
5. Multiply CPU share by total cluster cost to get pod CPU cost; same for memory.
6. Idle cost = total cluster cost - sum(all pod CPU cost) - sum(all pod memory cost).

Advantage: deterministic, derived from declared intent (the resource request), not variable usage data.
Limitation: over-provisioned pods that request more than they use show higher cost than their actual consumption.

## Usage-based allocation (optional)

Usage-based allocation attributes cost by actual measured CPU and memory consumption (p50, p95, or p99 of the utilization time series over the reporting window).

Steps 1-2 are the same as request-based. Replace step 3-4:

3. For each pod, compute CPU share = (pod CPU usage, e.g., p50) / (total cluster allocatable CPU).
4. Compute memory share = (pod memory working set, e.g., p50) / (total cluster allocatable memory).

The caller must provide the utilization metrics for usage-based allocation. This skill cannot fetch them from a live cluster.

Advantage: reflects actual consumption; reduces the apparent cost of over-provisioned but lightly loaded pods.
Limitation: requires metric data from Prometheus or equivalent; more variable; favors workloads that burst.

## Allocation mode comparison

| Mode | Input required | Deterministic | Reflects efficiency |
|---|---|---|---|
| Request-based | Pod specs only | Yes | No |
| Usage-based (p50) | Pod metrics (p50) | No (time-series) | Partially |
| Usage-based (p95) | Pod metrics (p95) | No (time-series) | Better |

## System overhead deduction

Before computing allocatable resources, deduct system overhead from each node's total capacity. Default overhead assumptions:

- CPU: 10% of node vCPU count reserved for kubelet, kube-proxy, container runtime, and OS.
- Memory: 10% of node RAM reserved for the same system components.

These defaults can be overridden by the caller if they have actual node allocatable data from `kubectl describe node`.

If the caller provides `kubectl describe node` output, use the explicit `Allocatable:` values rather than the default deduction.

## Shared namespace cost distribution

Some namespaces (kube-system, monitoring, ingress-nginx, etc.) provide shared platform services. Their cost can be attributed in three ways:

| Method | Description |
|---|---|
| Proportional | Distribute shared namespace cost across all other namespaces in proportion to their allocated cost. |
| Even split | Divide equally among all consuming namespaces. |
| First-party attribution | Attribute shared namespace cost to a platform team's cost center rather than distributing it. |

The allocation report defaults to **first-party attribution** (platform namespaces attributed to a `platform` team label) unless the caller specifies otherwise.

## Multi-window reporting

For multi-day or multi-week reporting windows, node prices may change mid-window (e.g., on-demand price updates, SKU changes, node pool scaling). Where the caller provides a single window with no intermediate pricing change events, apply the price fetched at the start of the window to the full window.

If the caller indicates a pricing change occurred mid-window, split the window at the change boundary and compute separate allocations for each sub-window, then sum.

## Output aggregation levels

The allocation report supports three aggregation levels:

| Level | Rows | Use case |
|---|---|---|
| Namespace | One row per namespace | Team-level chargeback |
| Workload | One row per Deployment/StatefulSet/DaemonSet/Job per namespace | Application-level cost attribution |
| Pod | One row per pod | Debugging over-provisioning at the pod level |

Default is workload-level. The caller can request a different level.
