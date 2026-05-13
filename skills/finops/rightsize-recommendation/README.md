# Rightsize Recommendation

A FinOps skill that emits Kubernetes pod CPU and memory request/limit recommendations from user-supplied utilization percentile metrics. Read-only, stateless math — no cluster connection, no kubectl.

## Purpose

Given p50, p95, and p99 CPU and memory utilization metrics from a 7-14 day window, compute recommended resource requests (p95 + 20% headroom) and limits (p99 + 30% headroom), estimate monthly savings if a unit price is provided, and assess Karpenter consolidation eligibility.

## Allowed tools

`Read` `Grep` `Glob`

## Usage

**Single pod:** Paste the pod name, namespace, current CPU/memory requests and limits, and p50/p95/p99 metrics. Optionally include $/vCPU-hour and $/GiB-hour for a savings estimate. The skill returns recommended requests and limits with headroom rationale and a consolidation eligibility flag.

**Batch:** Paste metrics for multiple pods or workloads in any consistent format (table, YAML snippet, or list). The skill returns a recommendation block per pod.

## Trust posture

Read-only. No cloud credentials, billing account IDs, or tenant data accepted. No cluster connection is made. All outputs are labeled `inference` (computed from caller inputs) or `assumed` (where defaults were applied). Monthly savings estimates are `excluded` unless unit price is provided by the caller.

See [SKILL.md](SKILL.md) for the full methodology, required input format, and response shape.
