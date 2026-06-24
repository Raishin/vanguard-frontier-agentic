# Workflow and output contract

Use this reference only when performing the full Microsoft Fabric data engineering review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Medallion architecture: bronze/silver/gold layer separation, workspace-per-layer design, Delta as default storage for silver/gold, shortcut vs copy decision for bronze
- Delta Lake quality: V-Order, Liquid Clustering, partitioning strategy, file sizes (~1 GB target), VACUUM and time-travel retention settings
- Spark notebooks and job definitions: PySpark/SQL logic correctness, session sizing (executor count, memory), library management, error handling, parameterization, incremental vs full-load logic
- Data pipelines and Dataflows Gen2: activity chaining, parameterization, incremental-load watermarks, scheduling and event-based triggers, retry/error handling, Monitoring Hub visibility
- OneLake shortcuts: shortcut type selection, governance boundary awareness, bronze-layer shortcut-first pattern, access propagation to downstream engines
- Real-Time Intelligence: eventstream source and destination configuration, eventhouse/KQL database schema, routing rules, error-handling paths, Spark Structured Streaming integration
- Direct Lake source design: Delta table layout for framing performance, V-Order write-time optimization, row group sizing, partitioning aligned to semantic-model query patterns
- Capacity Unit efficiency: Spark cluster sizing relative to data volume, Dataflows Gen2 high-scale compute triggers, pipeline activity cost, eventhouse query optimization
- Lifecycle management: Git integration scope, deployment pipeline stage configuration (dev/test/prod), database projects for warehouse schema, impact analysis before promotion

## Safe workflow

1. **Frame scope**
   - Items in scope (lakehouse, notebooks, pipelines, Dataflows Gen2, eventstreams, KQL databases, deployment pipelines):
   - Required outcome (pipeline reliability / medallion correctness / CU reduction / Real-Time Intelligence design / Direct Lake readiness):
   - Available evidence (notebook source, pipeline JSON, Monitoring Hub export, capacity metrics):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer user-provided sanitized notebook code, pipeline JSON, Dataflows Gen2 export, or Monitoring Hub screenshots.
   - Otherwise inspect official Microsoft Learn documentation to assess likely behavior.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test reliability and efficiency**
   - Which pipelines lack incremental-load logic, retry handling, or parameterized paths (brittle)?
   - Which Spark notebooks use oversized sessions or missing Delta optimization (CU waste)?
   - Which medallion layers are collapsed, use wrong format, or copy data that could be a shortcut?
   - Which eventstreams have no error-routing destination or unbounded ingestion with no windowing?
   - Which Direct Lake sources have sub-optimal file sizes or missing V-Order that will hurt framing?

4. **Recommend the smallest safe action**
   - Prefer non-destructive changes: add optimization, add retry, add parameterization, add shortcut.
   - Production pipeline runs, capacity changes, and deployment-pipeline promotions require live-guard escalation with a rollback plan.

## Output contract

Return this structure:

```markdown
# Fabric Data Engineering Review: <scope>
## Engineering verdict
- Status: SOUND / SOUND WITH RISKS / AT RISK / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Checks or monitors to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
