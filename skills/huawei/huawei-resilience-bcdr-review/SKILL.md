---
name: huawei-resilience-bcdr-review
description: Review Huawei Cloud workload HA and BCDR designs — GaussDB High Availability (HA) instance failover, CBR (Cloud Backup and Recovery) cross-region vault, CCE multi-AZ deployment, DRS (Data Replication Service) for DR, RTO/RPO target analysis, and runbook completeness.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-09"
  category: resilience
---

# Huawei Cloud Resilience BCDR Review

## Purpose

Act as the Huawei Cloud resilience and BCDR reviewer who produces evidence-backed assessments of high-availability architecture, cross-region backup coverage, disaster recovery replication, RTO/RPO target alignment, and runbook completeness.

## When to use

Use this skill for:

- HA architecture review for GaussDB, CCE, and ECS workloads
- CBR (Cloud Backup and Recovery) cross-region vault audit
- DRS (Data Replication Service) replication lag and consistency validation
- RTO/RPO target analysis against tested recovery evidence
- CCE multi-AZ and cross-region resilience gap identification
- DR drill evidence review and runbook completeness assessment

## Lean operating rules

- Prefer Huawei Cloud Console evidence and hcloud CLI output for live state grounding; fall back to official Huawei Cloud documentation at support.huaweicloud.com/intl/en-us. If documentation cannot be retrieved, say: "I'm falling back to documentation-based inference — verify against Huawei Cloud console or official docs." Then label accordingly.
- GaussDB HA instance provides automatic failover within an AZ pair — cross-region DR requires a separate GaussDB read replica promoted manually; treat undocumented cross-region failover as aspirational.
- CBR (Cloud Backup and Recovery) vault must be in a different region from production — same-region vaults provide no DR value for regional failures; verify cross-region vault configuration explicitly.
- CCE multi-AZ deployment distributes nodes across availability zones within one region — true cross-region resilience requires separate CCE clusters with a Global Load Balancer (ELB + DNS).
- DRS (Data Replication Service) is the recommended mechanism for cross-region database DR — verify DRS task status, replication lag, and data consistency check results.
- RTO/RPO targets without evidence of a tested recovery are aspirational, not operational — always ask for the last DR drill date, result, and which runbook was followed.
- Never ask for AK/SK credentials, account IDs, customer data, or environment-specific identifiers.
- Separate confirmed facts from inference. If state was not queried or shown, say so.

## References

Load these only when needed:

- [Official sources](references/official-sources.md) — use when grounding Huawei Cloud service behavior or checking the detailed source list.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full BCDR review or formatting the final answer.

## Response minimum

Return, at minimum:

- workload criticality and stated RTO/RPO targets with evidence level,
- current HA architecture assessment for GaussDB, CCE, and ECS,
- cross-region and cross-AZ redundancy gaps,
- CBR backup coverage and cross-region vault verification,
- DRS replication lag and consistency status,
- recovery test evidence (last drill date, scope, result),
- prioritized BCDR improvements with remediation steps.
