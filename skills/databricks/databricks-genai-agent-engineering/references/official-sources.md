# Official Sources

Primary Mosaic AI Agent Framework, AI Search, MCP, and Unity AI Gateway documentation.

Primary sources, verified 2026-08-17 against current official Databricks documentation. Each was fetched and read; a source that could not be reached is not listed here.

- https://docs.databricks.com/aws/en/agents/agent-framework/build-agents
- https://docs.databricks.com/aws/en/generative-ai/agent-framework/author-agent-db-app
- https://docs.databricks.com/aws/en/generative-ai/agent-framework/mcp
- https://docs.databricks.com/aws/en/generative-ai/mcp/managed-mcp
- https://docs.databricks.com/aws/en/vector-search/vector-search
- https://docs.databricks.com/aws/en/vector-search/query-vector-search
- https://docs.databricks.com/aws/en/machine-learning/foundation-models/external-models
- https://docs.databricks.com/aws/en/ai-gateway/

## Source notes

- The retrieval client surface was cross-checked against the Context7 MCP (`/websites/databricks`). Where Context7 surfaced a parameter only in the SQL form and not the Python client, this skill says so rather than presenting the Python signature as corroborated.

## Authority ranking

1. `FIRST_PARTY` — Databricks documentation, Databricks API/SDK reference, and the provider's own deprecation pages. Every claim in this skill that constrains a decision must trace to one of these.
2. `STANDARD_BODY` — Apache Spark, Delta Lake, MLflow, and OpenTelemetry project documentation for behaviour Databricks inherits rather than defines.
3. `SECONDARY` — blogs, conference talks, and press. Leads only. Never cited as evidence and never sufficient to encode a behaviour claim.

## Grounding rule

Documentation explains how the platform behaves in general. It does not prove the user's workspace configuration, Databricks Runtime version, compute type, region, cloud, edition, or actual grant state. Treat any claim that depends on those as `assumption` until an artifact or a sampled read-only query result confirms it, and name which artifact would settle it.
