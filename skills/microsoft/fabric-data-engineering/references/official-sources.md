# Official sources

Use this reference only when you need source grounding for Microsoft Fabric data engineering, Spark, Delta Lake, Real-Time Intelligence, or Capacity Unit behavior.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live workspace, pipeline state, or capacity consumption:

- https://learn.microsoft.com/fabric/onelake/onelake-medallion-lakehouse-architecture — Medallion architecture in OneLake: bronze/silver/gold layer design, Delta Lake as default storage format, deployment patterns (lakehouse-only vs lakehouse+warehouse), materialized lake views, partitioning, Liquid Clustering, file-size guidance, and historical retention via VACUUM. Supports medallion-layer and Delta optimization review steps.
- https://learn.microsoft.com/fabric/data-engineering/tutorial-lakehouse-introduction — Lakehouse end-to-end scenario: workspace setup, OneLake structure (Tables/Files areas), ingestion via pipelines and Dataflows Gen2, Spark notebook transformation, Delta table optimization, SQL analytics endpoint, and semantic model creation. Supports ingestion-pattern and Spark notebook review steps.
- https://learn.microsoft.com/fabric/data-factory/dataflows-gen2-overview — Dataflows Gen2: Power Query-based low-code transformation, multiple output destinations (Lakehouse, Warehouse), pipeline integration, Mashup Engine vs Spark/high-scale compute, monitoring via Monitoring Hub, and Refresh History. Supports Dataflows Gen2 and pipeline orchestration review steps.
- https://learn.microsoft.com/fabric/real-time-intelligence/event-streams/overview — Eventstreams overview: supported sources (IoT, Event Hubs, Fabric workspace/OneLake events), routing to destinations (KQL database, Lakehouse, Activator, custom app), event processing before ingestion, Spark Structured Streaming integration. Supports Real-Time Intelligence review steps.
- https://learn.microsoft.com/credentials/certifications/resources/study-guides/dp-700 — DP-700 Fabric Data Engineer Associate study guide: skill areas covering workspace configuration, Spark settings, OneLake security, Git/deployment pipelines, full/incremental load patterns, streaming data ingestion, pipeline and Dataflows Gen2 error resolution, Spark and eventhouse optimization. Supports scope-framing and gap-identification steps.

## Grounding rule

Official documentation explains Fabric data engineering behavior, Delta Lake semantics, and CU consumption patterns. It does not prove the user's actual notebook logic, pipeline state, CU metrics, eventstream routing, or deployment-pipeline configuration. Prefer user-provided sanitized code, pipeline JSON, or Monitoring Hub exports for current-state claims. Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.
