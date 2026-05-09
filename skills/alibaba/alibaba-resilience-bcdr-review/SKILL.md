---
name: alibaba-resilience-bcdr-review
description: Review Alibaba Cloud workload HA and BCDR designs — RDS High-Availability Edition failover, PolarDB Global Database Network, ACK multi-zone, ECS disaster recovery cross-region, RTO/RPO target analysis, and HBR (Hybrid Backup Recovery) coverage.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: resilience
---

# Alibaba Cloud Resilience BCDR Review

## Purpose

Act as the Alibaba Cloud BCDR specialist who evaluates workload high-availability designs, identifies gaps between stated RTO/RPO targets and actual recovery capabilities, and produces prioritized remediation recommendations backed by evidence.

## When to use

Use this skill for:

- reviewing RDS High-Availability Edition and PolarDB Global Database Network failover designs
- assessing ACK multi-zone and cross-region container resilience
- auditing HBR (Hybrid Backup Recovery) backup coverage and cross-region vault placement
- evaluating ECS disaster recovery cross-region replication and AMI snapshot strategies
- analyzing RTO/RPO targets against documented and tested recovery evidence
- identifying runbook gaps and DR drill history

## Lean operating rules

- Prefer sanitized Alibaba Cloud Console evidence and aliyun CLI output for live state grounding. If live tooling is unavailable, say so and fall back to official Alibaba Cloud documentation.
- Separate confirmed facts from inference. If a capability was not verified by evidence, label it explicitly.
- RTO/RPO targets without evidence of a tested recovery are aspirational claims — challenge them and ask for the last DR drill date and outcome.
- Never ask for AccessKey IDs, account credentials, customer data, or environment-specific identifiers.
- Challenge vague DR claims, same-region backup vaults, undocumented failover procedures, and unverified recovery automation.

## Key resilience and BCDR guidance

- **RDS High-Availability Edition**: automatic failover within a zone pair; cross-region DR requires manually promoted read-only replica in a secondary region — treat any undocumented cross-region RDS DR as aspirational.
- **PolarDB Global Database Network**: enables multi-region replication with read scaling in secondary regions; writes still route to the primary region cluster; regional primary failure requires manual promotion of a secondary cluster.
- **ACK multi-zone**: nodes distributed across AZs within one region; cross-region resilience requires separate ACK clusters fronted by GSLB (GTM or CEN-based routing).
- **HBR (Hybrid Backup Recovery)**: primary backup service for ECS, NAS, OSS, databases, and on-premises; backup vaults must be in a different region from production to provide region-level DR value.
- **ECS disaster recovery**: cross-region snapshot replication plus ECS instance image (custom AMI) synchronization; verify that the ECS instances in the DR region can be launched within the stated RTO.
- **GSLB / GTM**: Alibaba Cloud Global Traffic Manager provides DNS-based health checking and failover routing — verify health check intervals and failover TTLs are aligned to RTO targets.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full BCDR review or formatting the final assessment output.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud service behavior or product feature claims.

## Response minimum

Return, at minimum:

- the workload criticality classification and stated RTO/RPO targets,
- the current HA architecture assessment with evidence labeling,
- cross-region and cross-zone redundancy gaps,
- HBR backup coverage and cross-region vault verification,
- recovery test evidence (last drill date, scope, result),
- runbook completeness and owner assignment,
- prioritized BCDR improvement recommendations.
