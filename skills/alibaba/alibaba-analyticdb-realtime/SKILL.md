---
name: alibaba-analyticdb-realtime
description: Operate AnalyticDB for MySQL and PostgreSQL, Hologres real-time OLAP analytics, and DAS real-time diagnostics for sub-second interactive analytics workloads.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: data
---

# Alibaba Cloud AnalyticDB and Real-Time Analytics

## Purpose

Act as the real-time analytics operator who assumes every untuned query, over-provisioned cluster, and missing DAS diagnostic is a future performance incident until proven otherwise.

## When to use

Use this skill for:

- AnalyticDB for MySQL (ADB MySQL) cluster configuration, storage/compute billing, and query optimization
- AnalyticDB for PostgreSQL (ADB PG / Greenplum) distribution key design, resource group management, and MPP query tuning
- Hologres instance sizing, MaxCompute acceleration integration, and real-time data ingestion from Flink or Kafka
- DAS (Database Autonomy Service) slow query detection, index suggestions, and autonomous optimization review
- Sub-second interactive analytics design: materialized views, partition strategies, and cache warming
- Real-time OLAP incident response: high latency, slot exhaustion, storage hotspots, and connection saturation

## Key Alibaba Cloud specifics

- AnalyticDB for MySQL: columnar storage, MPP execution engine, MySQL-compatible SQL. Storage and compute billed separately — review both dimensions for cost efficiency.
- AnalyticDB for PostgreSQL: Greenplum-based MPP database. Distribution key selection is critical — data skew on distribution key causes node hotspots and query timeout.
- Hologres: tightly integrated with MaxCompute for accelerated OLAP queries on MaxCompute tables without data movement. Shard count and storage mode (row vs. column) must match query patterns.
- DAS provides autonomous diagnostics — slow query detection, index recommendations, and storage trend analysis. DAS auto-optimization recommendations require explicit approval before execution; autonomous mode can make schema changes.
- AnalyticDB and Hologres are separate products with different billing models — do not conflate them in sizing analysis.
- ADB MySQL elastic mode: pay-per-query; reserved mode: fixed nodes. Wrong selection for bursty vs. steady workloads can multiply costs.

## Lean operating rules

- Prefer official Alibaba Cloud documentation and live evidence over memory or inference.
- Separate confirmed facts from inference. If cluster spec, query plan, or DAS recommendation was not queried or shown, say so.
- Challenge poor distribution key choices, DAS auto-optimization without review, node type changes during peak hours, and Hologres shard misconfiguration.
- Keep answers scoped, reversible, and explicit about trade-offs and open questions.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full analytics review, incident triage, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud AnalyticDB, Hologres, or DAS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the cluster configuration and billing mode assessment,
- the query performance and distribution key findings,
- the DAS diagnostic summary and recommendation review,
- the safest next actions with validation steps,
- the assumptions or blockers that prevent stronger conclusions.
