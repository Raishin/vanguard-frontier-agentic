# Safety checklist

Use this reference before any recommendation involving production pipeline runs, capacity changes, deployment-pipeline promotion, or OneLake access controls.

## Non-negotiables

- Never ask users to paste credentials, connection strings, tenant IDs, workspace URLs, or customer data into chat.
- Use sanitized notebook source, pipeline JSON, or Monitoring Hub exports for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent Spark session configurations, CU metrics, pipeline run histories, or eventstream routing rules.
- Require explicit human approval before recommending any production pipeline run, Spark job execution, capacity resize or pause, deployment-pipeline promotion, or OneLake access-control change.
- Use current official Microsoft Learn documentation for Fabric data engineering, Delta Lake, Spark, and CU behavior.
- Keep recommendations least-privilege and reversible; prefer additive non-breaking changes over schema or config replacement.

## Stress checks

- Which pipelines lack incremental-load watermarks or retry/error-handling activities (brittle or data-loss risk)?
- Which Spark notebooks run on oversized sessions relative to data volume, missing Delta V-Order or Liquid Clustering (CU waste)?
- Which medallion layers are collapsed or use the wrong storage format (correctness and query-performance risk)?
- Which bronze-layer tables copy data that could be a OneLake shortcut instead (unnecessary duplication)?
- Which eventstreams have no error-routing destination or missing windowing functions (silent data loss)?
- Which Direct Lake source tables have poor row-group sizes or missing V-Order (framing latency risk)?
- What rollback path exists if a deployment-pipeline promotion breaks a downstream semantic model or report?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual notebook logic, pipeline state, CU consumption, or eventstream configuration.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Triggering or scheduling production Fabric pipeline runs or Spark job definitions
- Promoting items between deployment-pipeline stages (dev → test → prod)
- Resizing, pausing, or reassigning Fabric capacity (CU changes)
- Modifying OneLake folder-level or item-level access controls in production
- Deleting or VACUUMing Delta tables in production beyond safe retention windows
- Changing production eventstream routing or destination configuration
