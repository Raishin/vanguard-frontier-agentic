# Official sources

Use this reference only when you need source grounding for Microsoft Fabric analytics engineering, dimensional modeling, Direct Lake, DAX, or Fabric Data Warehouse T-SQL behavior.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live warehouse schema, semantic model configuration, or DAX measure output:

- https://learn.microsoft.com/fabric/data-warehouse/dimensional-modeling-overview — Dimensional modeling in Fabric Data Warehouse: star schema design (fact tables, dimension tables, surrogate keys), snowflake vs star trade-offs, slowly changing dimension strategies, bridge tables for many-to-many, load patterns, and T-SQL best practices for Fabric Warehouse. Supports schema design and dimensional-modeling review steps.
- https://learn.microsoft.com/fabric/fundamentals/direct-lake-overview — Direct Lake overview: how framing works (metadata refresh vs full import), DirectQuery fallback triggers (SQL views, unsupported features), V-Order dependency for performance, Direct Lake on OneLake vs Direct Lake on SQL endpoints, composite model support, and framing latency considerations. Supports storage-mode selection and Direct Lake configuration review steps.
- https://learn.microsoft.com/fabric/data-warehouse/data-warehousing — Fabric Data Warehouse overview: enterprise T-SQL warehouse on Delta Lake foundation, star/snowflake schema use cases, multi-table ACID transactions, materialized views, stored procedures, COPY INTO, cross-database queries, autonomous workload management, and Power BI integration. Supports warehouse design and anti-pattern review steps.
- https://learn.microsoft.com/dax/dax-overview — DAX overview: measures vs calculated columns vs calculated tables, filter context and row context, CALCULATE, iterator functions (SUMX, AVERAGEX), evaluation context, time-intelligence functions, and common correctness pitfalls. Supports DAX measure quality and optimization review steps.
- https://learn.microsoft.com/credentials/certifications/resources/study-guides/dp-600 — DP-600 Fabric Analytics Engineer Associate study guide: skill areas covering semantic model design (storage mode, star schema, relationships, DAX, calculation groups, composite models, incremental refresh), data preparation (T-SQL views, Dataflows Gen2, star-schema loading), and analytics lifecycle (deployment pipelines, XMLA endpoint, shared semantic models). Supports scope-framing and gap-identification steps.

## Grounding rule

Official documentation explains Fabric Data Warehouse, Direct Lake, and DAX behavior. It does not prove the user's actual warehouse schema, semantic model relationships, measure correctness, or DirectQuery fallback state. Prefer user-provided sanitized DDL, semantic model metadata (PBIP/TMDL), or DAX measure definitions for current-state claims. Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.
