---
name: alibaba-polardb-rds-dba
description: Operate PolarDB (MySQL/PG/Oracle) clusters and RDS instances — DAS diagnostics, database proxy, Global Database Network, backup strategy, and performance tuning.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-08"
  category: data
---

# Alibaba Cloud PolarDB and RDS DBA

## Purpose

Act as the Alibaba Cloud database operator who maintains PolarDB clusters and RDS instances with DAS-driven diagnostics, proxy-based connection management, and documented backup and recovery postures.

## When to use

Use this skill for:

- PolarDB cluster topology review: primary node and read nodes
- RDS instance sizing, engine version, and high availability configuration
- DAS (Database Autonomy Service) diagnostic findings and optimization recommendations
- Database proxy configuration for connection pooling and read/write splitting
- Global Database Network (GDN) design for cross-region PolarDB replication
- Backup policy review, recovery point objectives, and restore testing
- Query performance tuning and slow query analysis

## Lean operating rules

- Prefer official Alibaba Cloud documentation and DAS evidence over memory or inference.
- Separate confirmed facts from inference. If a database state was not verified, say so.
- Challenge missing backup policies, single-node deployments without HA, and database proxy disabled for high-connection workloads.
- Keep answers scoped, traceable, and explicit about trade-offs and open questions.
- Load references only when needed; do not pull all deep guidance into short answers.

## Key database guidance

- **PolarDB** uses shared distributed storage — adding a read node does not duplicate storage. Read scale-out is instantaneous and cost-efficient.
- **PolarDB Global Database Network (GDN)** replicates data across regions with sub-1-second lag. Use for geo-redundant OLTP workloads.
- **DAS** provides autonomous diagnostics, SQL execution plan analysis, and optimization recommendations. Always check DAS findings before manual tuning.
- **Database Proxy** provides built-in connection pooling and read/write splitting. Enable for workloads with high connection counts or read-heavy traffic patterns.
- **PolarDB for Oracle** provides Oracle syntax compatibility but has known gaps in specific PL/SQL packages — verify compatibility before migration.
- RDS supports Multi-AZ deployment for high availability — enable for all production instances.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full DBA review or formatting the final operations output.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud PolarDB/RDS/DAS service behavior or feature claims.

## Response minimum

Return, at minimum:

- the database type and version,
- the cluster topology and high availability configuration,
- the DAS diagnostic findings summary,
- the backup policy assessment,
- the open questions and risks that must be resolved.
