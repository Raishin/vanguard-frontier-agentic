# OpenCost to FOCUS Column Mapping

## Overview

OpenCost produces allocation data organized around Kubernetes primitives (namespace, pod, container, workload owner). FOCUS v1.2 is a vendor-neutral billing column specification. This reference maps OpenCost allocation output columns to their FOCUS v1.2 equivalents and documents where gaps exist.

## Column mapping table

| OpenCost allocation column | FOCUS v1.2 column | Notes |
|---|---|---|
| `namespace` | `ResourceName` (namespace scope) | Namespace is the primary Kubernetes resource boundary. No direct FOCUS equivalent; use `ResourceName` with `SubAccountName` for the cluster. |
| `workload` (deployment/statefulset/etc.) | `ResourceName` (workload scope) | Use a composite value: `<cluster>/<namespace>/<workload-kind>/<workload-name>`. |
| `container` | N/A | FOCUS has no container-level granularity; aggregate to pod or workload before mapping. |
| `cpuCost` | `BilledCost` (CPU share) | Split by resource dimension; no standard FOCUS sub-column for CPU vs memory. Use `ChargeDescription` to distinguish. |
| `memoryCost` | `BilledCost` (memory share) | Same as above. |
| `gpuCost` | `BilledCost` (GPU share) | If GPU costs are tracked, use a separate line item with `ChargeDescription = "GPU allocation"`. |
| `pvCost` | `BilledCost` (storage share) | Persistent volume cost. Map to a separate `ServiceCategory = Storage` line item. |
| `networkCost` | `BilledCost` (network share) | Map to `ServiceCategory = Networking`. |
| `totalCost` | `BilledCost` | Aggregate across all resource dimensions. |
| `efficiency` | N/A | OpenCost efficiency ratio (usage/request). Not a FOCUS column; carry as a tag (`Tags/efficiency`). |
| `cpuEfficiency` | N/A | Same as above; carry as `Tags/cpu_efficiency`. |
| `memoryEfficiency` | N/A | Carry as `Tags/memory_efficiency`. |
| `labels.<key>` | `Tags/<key>` | Kubernetes labels on the pod or namespace map to FOCUS tags. |
| `properties.cluster` | `SubAccountName` | The Kubernetes cluster name maps to the cloud sub-account or resource group that owns the nodes. |
| `properties.node` | `ResourceId` | Node name or cloud instance ID. Prefer the cloud instance ID for cross-referencing with cloud billing. |
| `window.start` | `BillingPeriodStart` | ISO 8601 start of the allocation window. |
| `window.end` | `BillingPeriodEnd` | ISO 8601 end of the allocation window. |

## Fixed FOCUS columns for Kubernetes allocation rows

These columns have fixed or near-fixed values for all Kubernetes allocation output:

| FOCUS column | Value |
|---|---|
| `ServiceCategory` | Containers |
| `ChargeCategory` | Usage |
| `ChargeFrequency` | Usage-based |
| `ProviderName` | Cloud provider of the underlying nodes (e.g., AWS, Azure, Google Cloud, Oracle Cloud) |
| `ServiceName` | Kubernetes (or the managed service name, e.g., Amazon EKS, Azure AKS, GKE) |
| `PublisherName` | Same as `ProviderName` |

## Gaps between OpenCost and FOCUS

| Gap | Description |
|---|---|
| No `EffectiveCost` in OpenCost | OpenCost computes allocation from public on-demand prices; it does not know about savings plans, reserved instances, committed use discounts, or EDP discounts. `EffectiveCost` (FOCUS) cannot be populated without cloud billing data. Mark as `excluded` if unavailable. |
| No `ContractedCost` in OpenCost | Same reason as above. Mark as `excluded`. |
| No `ListCost` in OpenCost | OpenCost uses list prices internally but does not expose them as a separate column. Where the allocation cost is computed from public on-demand prices with no discount applied, `ListCost` equals `BilledCost`. |
| No `SkuId` / `SkuPriceId` in OpenCost | OpenCost works at the node SKU level but does not surface the provider's SKU identifier. Populate `SkuId` from the node instance type if known (e.g., `m6i.4xlarge`). `SkuPriceId` is not mappable without cloud billing data. |
| Idle cost attribution | OpenCost surfaces idle cost as a cluster-level residual. FOCUS has no native idle cost construct; represent as a separate line item with `ResourceName = <cluster>/idle`, `ChargeDescription = "Unallocated cluster capacity"`. |
| Shared namespace cost | OpenCost supports shared namespace cost distribution; FOCUS has no native distribution mechanism. Document the distribution method in `ChargeDescription`. |

## Idle cost FOCUS representation

OpenCost idle cost = total cluster node cost minus sum of all allocated workload costs.

FOCUS line item for idle cost:

```
ProviderName:    <cloud provider>
ServiceCategory: Containers
ServiceName:     Kubernetes
ChargeCategory:  Usage
ChargeDescription: Unallocated cluster capacity
ResourceName:    <cluster-name>/idle
BilledCost:      <idle cost amount>
BillingPeriodStart: <window.start>
BillingPeriodEnd:   <window.end>
Tags/allocation_type: idle
```
