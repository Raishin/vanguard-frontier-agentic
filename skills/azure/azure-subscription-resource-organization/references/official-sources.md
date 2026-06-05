# Official sources

Use this reference when grounding current Azure behavior for `azure-subscription-resource-organization`.

## Microsoft Learn sources

- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/resource-org
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/resource-org-management-groups
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/resource-org-subscriptions
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/azure-setup-guide/organize-resources
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/management-application-environments
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/governance
- https://learn.microsoft.com/training/modules/design-governance/
- https://learn.microsoft.com/azure/governance/management-groups/overview
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/azure-best-practices/resource-tagging
- https://learn.microsoft.com/azure/azure-resource-manager/management/tag-policies

## Current documentation refresh (2026-06-05)

- Microsoft Learn documentation through the user's configured documentation MCP is the primary source for documented Azure behavior.
- Documentation evidence is not live customer-state evidence. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.
- Use sampled read-only Azure evidence only when the user has configured it and the task requires current-state confirmation. Label it as sampled evidence, not broad proof.

## Grounding rule

Docs explain service behavior. Current-state claims require sampled read-only evidence or sanitized user-provided evidence. If current state was not queried or shown, say so.
