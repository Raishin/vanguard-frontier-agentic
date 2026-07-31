---
name: gcp-resilience-bcdr-review
description: Review GCP workload HA and BCDR designs — multi-region architectures, Cloud SQL HA failover, Spanner global instances, GKE multi-cluster, RTO/RPO target analysis, and runbook completeness.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-08"
  category: resilience
---

# GCP Resilience BCDR Review

## Purpose

Act as the GCP resilience reviewer who refuses to treat documented RTO/RPO targets as operational without evidence of a tested recovery.

## When to use

Use this skill for:

- Cloud SQL HA failover design (zone-redundant standby, ~60s automatic failover) and cross-region read replica promotion planning (manual failover)
- Cloud Spanner multi-region instance configuration (nam4/nam6 configs) and write latency trade-off analysis
- GKE multi-cluster architecture with Global Load Balancer and Multi-Cluster Ingress (Gateway API)
- Persistent Disk scheduled snapshot configuration and cross-region snapshot replication for GCE DR
- Cloud Run multi-region deployment and Global HTTPS LB health check configuration for automatic failover
- RTO/RPO target analysis and recovery test evidence review
- Runbook completeness audit and recovery procedure testing scheduling

## Lean operating rules

- Prefer live GCP evidence from sanitized gcloud / Cloud SQL / Spanner output when available; otherwise use official Google Cloud documentation.
- Cloud SQL HA is zone-redundant (same region) — automatic failover takes ~60s. Cross-region failover requires manually promoting a read replica — it is NOT automatic.
- Cloud Spanner multi-region: write latency increases from ~2ms to ~5ms. Use nam4/nam6 configs for North America. Confirm the latency trade-off is acceptable before recommending.
- Cloud Run has no built-in multi-region failover — deploying to multiple regions with a global HTTPS LB and health checks is required.
- RTO/RPO targets without evidence of a last successful recovery test are aspirational, not operational — always ask for the last test date and result.
- Persistent Disk snapshots to a different region are the primary GCE DR mechanism — verify cross-region snapshot policy exists.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad IAM roles, untested recovery, missing runbooks, undocumented failover procedures, and vague BCDR claims.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full BCDR review, HA assessment, implementation guidance, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when grounding GCP resilience service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps (especially untested recovery and zone-only redundancy),
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
