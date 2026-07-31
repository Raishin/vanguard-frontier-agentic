---
name: kubernetes-allocation-report
description: Produce OpenCost-compatible namespace, pod, and workload cost allocation tables from user-supplied cluster shape data and public cloud pricing. Input is cluster topology provided by the caller; no cluster credentials or live API access are required or accepted. Output maps to FOCUS v1.2 columns.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.2"
  updated: "2026-05-13"
  category: finops
  lifecycle: experimental
---

# Kubernetes Allocation Report

## Purpose

Generate an OpenCost-compatible cost allocation breakdown for Kubernetes workloads using cluster shape data supplied by the caller and live public cloud pricing. The output attributes cost by namespace, workload, and labeled team, following OpenCost allocation semantics and mapping to FOCUS v1.2 billing columns.

No cluster credentials are accepted. All pricing is fetched from public pricing APIs.

## When to use

Use this skill when:

- The user wants to understand how cluster spend is distributed across namespaces, workloads, or teams
- The user has pasted or attached a node SKU list, namespace inventory, pod resource requests, and runtime hours and wants a cost attribution table
- The user wants to compare allocation against actual billing to identify idle or unallocated cost
- The user wants output that maps to OpenCost allocation columns or FOCUS v1.2 columns for FinOps reporting

## Operating rules

- **No cluster credentials accepted.** No kubeconfig, bearer token, service account JWT, or cloud IAM credential is accepted or required. All cluster data is user-supplied in the prompt.
- **User-supplied input only.** The cluster shape (node SKU mix, namespace list, pod resource requests, runtime hours) is provided by the caller. This skill does not connect to the Kubernetes API server or any cloud billing API.
- **Fetch live node pricing.** Use WebFetch to retrieve current on-demand prices for the user-supplied node SKUs from the relevant cloud provider's public pricing API. Label fetched prices as `live-price` with source URL and ISO 8601 timestamp.
- **Provenance labels mandatory.** Every cost figure must carry one label:
  - `live-price` — price fetched from a public API within this session (include URL and timestamp)
  - `documentation-based` — price sourced from official documentation when live fetch failed (include URL)
  - `assumed` — price derived from analogous SKU or default ratio (state the assumption)
  - `excluded` — costs intentionally omitted from the allocation (e.g., data transfer, licensing); explain why
- **Request-based allocation by default.** Use pod resource requests (not live usage metrics) as the allocation numerator unless the caller explicitly provides usage data.
- **No credentials accepted.** No cloud credentials, billing account IDs, or tenant data accepted at any point.
- **FOCUS column mapping.** Map every allocation output to the corresponding FOCUS v1.2 columns as specified in the response shape below.
- **Load references only when needed.**

## Required input

The caller must supply:

- **Node SKU mix**: cloud provider, instance type (e.g., `m6i.4xlarge`), region, and count per node pool.
- **Namespace list**: names of all namespaces to include in the allocation.
- **Pod resource requests**: for each pod or workload, CPU request (millicores) and memory request (MiB), plus the namespace and owner (Deployment, StatefulSet, DaemonSet, Job, or standalone Pod).
- **Runtime hours**: the reporting window in hours (e.g., 720 for a calendar month).
- **Team labels** (optional): namespace or workload label values for the `team` or `cost-center` label key, used to group allocation by team.

If any required input is missing, ask one clarifying question per gap. Do not fabricate cluster data.

## Allocation methodology

1. **Node cost per hour**: fetch on-demand price for each node SKU. Multiply by node count and runtime hours to get total cluster node cost.
2. **Allocatable resources per node**: CPU = (node vCPU count - system overhead, default 10%) allocatable; memory = (node RAM - system overhead, default 10%) allocatable.
3. **Split node cost by dimension**: divide total cluster cost between CPU and memory (default: 50% CPU, 50% memory; adjust if provider publishes component pricing).
4. **Request-based share per dimension**: for each pod, compute CPU share = pod CPU request / total cluster allocatable CPU; and memory share = pod memory request / total cluster allocatable memory.
5. **Allocated cost per pod**: CPU cost = CPU share × CPU portion of node cost; Memory cost = Memory share × memory portion of node cost.
6. **Idle cost**: cluster cost minus sum of all pod-allocated costs.
7. **Aggregate by namespace / workload / team**: sum pod-level costs to produce namespace, workload, and team totals.

## Response shape

Return a structured allocation table with these columns:

| Column | Description | FOCUS mapping |
|---|---|---|
| Namespace | Kubernetes namespace | `ResourceName` (namespace scope) |
| Workload | Deployment / StatefulSet / DaemonSet / Job name | `ResourceName` (workload scope) |
| Team | Value of `team` or `cost-center` label | `Tags/team` |
| CPU request (m) | Millicores requested | N/A |
| Memory request (MiB) | MiB requested | N/A |
| CPU cost ($/period) | Cost attributed to CPU requests | `BilledCost` (CPU share) |
| Memory cost ($/period) | Cost attributed to memory requests | `BilledCost` (memory share) |
| Total allocated cost ($/period) | Sum of CPU and memory allocated cost | `BilledCost` |
| Provenance | Label for the price source | N/A |

Followed by:

- **Idle cost row**: total cluster cost minus sum of all allocated costs; label as `unallocated`.
- **Total cluster cost row**: on-demand price × node count × runtime hours; labeled with provenance.
- **Key assumptions**: system overhead percentage, request-based vs usage-based allocation, pricing source timestamp.
- **FOCUS metadata row**: `ServiceCategory = Containers`, `ChargeCategory = Usage`, `ServiceName = Kubernetes` (user-supplied cluster name or "unnamed"), `ProviderName` = cloud provider of the node SKUs.

## References

Load these only when needed:

- [OpenCost to FOCUS column mapping](references/opencost-mapping.md) — how OpenCost allocation columns map to FOCUS v1.2 columns and where gaps exist.
- [Attribution workflow](references/attribution-workflow.md) — idle vs allocated vs unallocated cost, request-vs-usage allocation modes.
