# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, deployed resources, current cost, vault state, app health, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/azure-resource-manager/templates/deploy-what-if
- https://learn.microsoft.com/azure/azure-resource-manager/bicep/deployment-stacks
- https://learn.microsoft.com/azure/templates/microsoft.resources/deploymentstacks
- https://learn.microsoft.com/azure/role-based-access-control/deny-assignments
- https://learn.microsoft.com/azure/azure-resource-manager/templates/best-practices

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says Deployment Stacks manage a group of resources as one unit, can detach or delete resources removed from the template through actionOnUnmanage, and can protect managed resources with deny settings. Documentation also calls out limitations: stacks do not manage implicitly created resources, deny settings apply to control plane only, some portal support is absent, and out-of-sync warnings require managed-resource review before bypass.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Live-operation claim: requires target, principal, approval, preflight evidence, rollback constraints, and post-action verification.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.

## Current Microsoft Learn deltas checked on 2026-06-05

- Deployment Stacks manage resources through Microsoft.Resources/deploymentStacks and action-on-unmanage choices; delete behavior must be explicit before execution.
- Microsoft Learn still distinguishes ARM/Bicep deployment what-if from Deployment Stack limitations; do not overclaim native stack what-if coverage.
- Deny settings protect control-plane operations, not data-plane content, and do not cover implicitly created resources.
- denyDelete, denyWriteAndDelete, exclusions, and deleteAll are governance decisions with materially different blast radius.

