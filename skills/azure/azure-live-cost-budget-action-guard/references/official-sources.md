# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, deployed resources, current cost, vault state, app health, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/cost-management-billing/costs/tutorial-acm-create-budgets
- https://learn.microsoft.com/azure/cost-management-billing/costs/cost-mgt-alerts-monitor-usage-spending
- https://learn.microsoft.com/azure/cost-management-billing/costs/cost-mgt-best-practices
- https://learn.microsoft.com/cloud-computing/finops/framework/quantify/budgeting
- https://learn.microsoft.com/azure/quotas/quickstart-increase-quota-portal
- https://learn.microsoft.com/azure/azure-resource-manager/management/azure-subscription-service-limits

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says Cost Management budgets monitor spending and trigger notifications or action groups, but budget alerts do not stop resources or consumption. Cost and usage data is typically delayed, and budgets are evaluated on a schedule. Budgeting guidance recommends alerts, anomaly detection, stakeholder ownership, and automation only with clear accountable response paths.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Live-operation claim: requires target, principal, approval, preflight evidence, rollback constraints, and post-action verification.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
