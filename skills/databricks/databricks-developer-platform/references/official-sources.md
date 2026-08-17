# Official Sources

Primary Databricks bundle, authentication, and Git documentation underpinning the developer-platform review.

Primary sources, verified 2026-08-17 against current official Databricks documentation. Each was fetched and read; a source that could not be reached is not listed here.

- https://docs.databricks.com/aws/en/dev-tools/bundles
- https://docs.databricks.com/aws/en/dev-tools/bundles/reference
- https://docs.databricks.com/aws/en/dev-tools/bundles/deployment-modes
- https://docs.databricks.com/aws/en/dev-tools/bundles/run-as
- https://docs.databricks.com/aws/en/dev-tools/bundles/variables
- https://docs.databricks.com/aws/en/dev-tools/cli/bundle-commands
- https://docs.databricks.com/aws/en/dev-tools/cli/authentication
- https://docs.databricks.com/aws/en/dev-tools/terraform/

## Source notes

- The bundle CLI and Terraform provider surfaces were cross-checked against the Context7 MCP (`/databricks/cli`, `/databricks/terraform-provider-databricks`). Context7's CLI documentation uses 'Declarative Automation Bundles' and the 'DAB' acronym predominantly, with 'Databricks Asset Bundles' appearing as secondary repository-description language — which is why this skill leads with the former. No Terraform provider version is pinned here: the Context7 copy and the public registry reported different current versions, so the version is left to be resolved at recommendation time.

## Authority ranking

1. `FIRST_PARTY` — Databricks documentation, Databricks API/SDK reference, and the provider's own deprecation pages. Every claim in this skill that constrains a decision must trace to one of these.
2. `STANDARD_BODY` — Apache Spark, Delta Lake, MLflow, and OpenTelemetry project documentation for behaviour Databricks inherits rather than defines.
3. `SECONDARY` — blogs, conference talks, and press. Leads only. Never cited as evidence and never sufficient to encode a behaviour claim.

## Grounding rule

Documentation explains how the platform behaves in general. It does not prove the user's workspace configuration, Databricks Runtime version, compute type, region, cloud, edition, or actual grant state. Treat any claim that depends on those as `assumption` until an artifact or a sampled read-only query result confirms it, and name which artifact would settle it.
