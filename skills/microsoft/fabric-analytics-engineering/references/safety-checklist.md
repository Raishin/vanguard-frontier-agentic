# Safety checklist

Use this reference before any recommendation involving production warehouse schema changes, semantic-model deployment, or deployment-pipeline promotion.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, workspace URLs, connection strings, or customer data into chat.
- Use sanitized DDL scripts, semantic model metadata (PBIP/TMDL), or DAX measure definitions for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent warehouse schemas, relationship configurations, DAX measure outputs, or DirectQuery fallback states.
- Require explicit human approval before recommending any production warehouse DDL change (ALTER TABLE, DROP, schema rename), semantic-model publication or overwrite, deployment-pipeline promotion, or XMLA endpoint write operation.
- Use current official Microsoft Learn documentation for Fabric Data Warehouse T-SQL, Direct Lake, and DAX behavior.
- Keep recommendations least-breaking and reversible; prefer additive changes (new columns, new measures) over destructive schema replacements.

## Stress checks

- Which fact tables lack surrogate keys or have wrong granularity (measure accuracy risk)?
- Which DAX measures use implicit aggregation, calculated columns for aggregation, or incorrect filter context (result correctness risk)?
- Which Direct Lake models use SQL views that will silently fall back to DirectQuery and degrade performance?
- Which relationships have bidirectional cross-filter creating multiple ambiguous paths (incorrect filter propagation risk)?
- Which T-SQL queries carry implicit type conversions or missing WHERE clauses on large fact scans (query performance risk)?
- What rollback path exists if a warehouse DDL change breaks a downstream semantic model or Power BI report?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual warehouse schema, semantic model relationship state, DAX measure correctness, or DirectQuery fallback behavior.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Executing DDL (CREATE, ALTER TABLE, DROP, RENAME) against production Fabric Data Warehouse schemas
- Deploying or overwriting production semantic models via XMLA endpoint or Fabric REST API
- Promoting items between deployment-pipeline stages (dev → test → prod) for warehouse or semantic-model items
- Deleting or replacing certified shared semantic models in production workspaces
- Publishing Power BI reports that overwrite existing production content
