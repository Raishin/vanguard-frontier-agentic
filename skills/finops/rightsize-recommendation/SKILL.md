---
name: rightsize-recommendation
description: Emit pod CPU and memory request/limit recommendations from user-pasted p50, p95, and p99 utilization metrics over a 7-14 day window. Outputs recommended requests at p95 plus 20% headroom, limits at p99 plus 30%, estimated monthly savings, and Karpenter consolidation eligibility flag. Read-only, no kubectl.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.2"
  updated: "2026-05-13"
  category: finops
  lifecycle: experimental
---

# Rightsize Recommendation

## Purpose

Produce CPU and memory rightsizing recommendations for Kubernetes pods based on user-supplied utilization percentile metrics. All math is performed on the inputs provided by the caller; no cluster connection or live metric fetch is performed.

No kubectl. No WebFetch. No cluster credentials accepted.

## When to use

Use this skill when:

- The user has pasted or attached p50, p95, and p99 CPU and memory utilization metrics for one or more pods or workloads
- The user wants actionable request and limit recommendations with explicit headroom rationale
- The user wants an estimated monthly savings figure (requires unit price from the caller)
- The user wants to know whether a pod is eligible for Karpenter node consolidation

## Operating rules

- **Inputs only.** All calculations are performed on the metrics and pod specs supplied by the caller. This skill does not fetch live metrics, connect to Prometheus, or query the Kubernetes API.
- **No credentials accepted.** No cloud credentials, billing account IDs, tenant data, kubeconfig, bearer tokens, or service account JWTs are accepted or required.
- **Metric window requirement.** The caller must supply metrics from a 7-14 day representative window. Metrics from shorter windows are accepted but the recommendation must be labeled with a reduced-confidence warning.
- **Provenance labels mandatory.** Every numeric output must carry one label:
  - `inference` — computed from caller-supplied inputs using the documented methodology
  - `assumed` — derived from a default assumption where caller data was missing (state the assumption)
  - `excluded` — cost savings figure excluded because unit price was not supplied by the caller
- **FOCUS column mapping.** Where a cost savings estimate is produced, note: `BilledCost` (current), `EffectiveCost` (projected after rightsizing), `ChargeCategory = Usage`, `ServiceCategory = Containers`.
- **Load references only when needed.**

## Required input

The caller must supply, per pod or workload:

- **Pod name and namespace** (or Deployment/StatefulSet name)
- **Current CPU request** (millicores) and **current memory request** (MiB)
- **Current CPU limit** (millicores) and **current memory limit** (MiB); state `no-limit` if unset
- **p50 CPU** (millicores), **p95 CPU** (millicores), **p99 CPU** (millicores)
- **p50 memory** (MiB), **p95 memory** (MiB), **p99 memory** (MiB)
- **Metric window** (days, must be 7-14; shorter accepted with warning)
- **Unit price per vCPU-hour and per GiB-hour** (optional; required only for $/mo saved estimate)

If any field is missing, ask one clarifying question per gap. Do not fabricate metric values.

## Recommendation methodology

### CPU

| Output | Formula | Rationale |
|---|---|---|
| Recommended CPU request | p95 CPU + 20% | p95 covers normal burst; 20% headroom absorbs measurement noise and brief spikes |
| Recommended CPU limit | p99 CPU + 30% | p99 covers rare spikes; 30% headroom prevents throttling during outlier bursts |

### Memory

| Output | Formula | Rationale |
|---|---|---|
| Recommended memory request | p95 memory + 20% | Same headroom logic as CPU; memory pressure leads to OOMKill rather than throttling |
| Recommended memory limit | p99 memory + 30% | Tighter limits risk OOMKill; 30% buffer is conservative for stateful workloads |

Round all recommendations up to the nearest 10 millicores (CPU) or 16 MiB (memory) for cleaner manifests.

### Monthly savings estimate

If the caller supplies unit price ($/vCPU-hour and $/GiB-hour):

```
cpu_savings_per_month = (current_cpu_request - recommended_cpu_request) / 1000
                       × unit_price_per_vcpu_hour × 730

memory_savings_per_month = (current_memory_request - recommended_memory_request) / 1024
                          × unit_price_per_gib_hour × 730
```

Label: `inference`. Note: savings are approximate; they assume the freed capacity is not replaced by new workloads and that the node pool scales down proportionally.

If unit price is not supplied, state the savings formula and mark the $/mo cell as `excluded — unit price not provided by caller`.

### Karpenter consolidation eligibility

A pod is flagged as consolidation-eligible when all of the following are true:

- No `PodDisruptionBudget` with `maxUnavailable: 0` or `minAvailable: 100%` applies to the pod.
- No `pod-anti-affinity` or `topologySpreadConstraints` rule that would prevent the pod from co-locating with other pods of the same owner.
- No `hostPath` or `local` PersistentVolume mount.
- No `nodeName` node selector pinning the pod to a specific node.
- The pod's request fits within a standard node SKU that Karpenter would provision (the caller must confirm the NodePool SKU list).

If the caller has not supplied enough information to confirm all five conditions, state which conditions could not be verified and flag as `not-verified — [missing conditions]`. Do not assume eligibility when data is incomplete; unknown blockers present a consolidation risk.

## Response shape

Return, per pod or workload:

```
Pod / Workload: <name> (<namespace>)

  CPU
    Current request:     <current> m
    Recommended request: <p95 × 1.20, rounded> m  [inference]
    Current limit:       <current> m  (or "no-limit")
    Recommended limit:   <p99 × 1.30, rounded> m  [inference]

  Memory
    Current request:     <current> MiB
    Recommended request: <p95 × 1.20, rounded> MiB  [inference]
    Current limit:       <current> MiB  (or "no-limit")
    Recommended limit:   <p99 × 1.30, rounded> MiB  [inference]

  Monthly savings estimate
    CPU:    $<amount>/mo  [inference]  or  [excluded — unit price not provided]
    Memory: $<amount>/mo  [inference]  or  [excluded — unit price not provided]
    Total:  $<amount>/mo

  Karpenter consolidation
    Eligible: <Yes | No | Not-verified>
    Blockers: <list any confirmed blockers, or list missing data preventing verification, or "None confirmed">

  Metric window: <N days>  [inference: window meets 7-14 day requirement]
  Confidence:   <Normal | Reduced — window < 7 days>
```

## References

Load these only when needed:

- [Metric sources](references/metric-sources.md) — how to gather p50/p95/p99 from Prometheus, Cloud Monitoring, and Azure Monitor for use as input to this skill.
- [Karpenter consolidation](references/karpenter-consolidation.md) — what makes a pod consolidation-eligible and known blockers.
