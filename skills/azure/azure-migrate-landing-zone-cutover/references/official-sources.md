# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, migration project, network, telemetry, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/migrate/migrate-services-overview?view=migrate
- https://learn.microsoft.com/azure/migrate/concepts-migration-planning?view=migrate
- https://learn.microsoft.com/azure/migrate/common-questions-discovery-dependency-analysis?view=migrate
- https://learn.microsoft.com/azure/migrate/overview?view=migrate
- https://learn.microsoft.com/azure/migrate/platform-landing-zone?view=migrate
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says Azure Migrate supports decide, plan, and execute phases: discovery, business case, Azure readiness, right-sizing, cost estimation, dependency analysis, replication, migration, and modernization. Wave planning quality depends on complete discovery, accurate inventory, dependency analysis, application grouping, metadata enrichment, and assessments. Planning agents can interpret migration data but execution actions remain in Azure Migrate workflows.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
