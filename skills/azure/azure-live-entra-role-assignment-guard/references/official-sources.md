# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, deployed resources, current cost, vault state, app health, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/role-based-access-control/overview
- https://learn.microsoft.com/azure/role-based-access-control/best-practices
- https://learn.microsoft.com/azure/role-based-access-control/role-assignments-steps
- https://learn.microsoft.com/azure/role-based-access-control/role-assignments-alert
- https://learn.microsoft.com/azure/role-based-access-control/troubleshooting#azure-role-assignments
- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-deployment-plan

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says Azure RBAC grants who can access Azure resources, what they can do, and where. Best practices require least privilege, narrow scope, limiting privileged administrator roles, assigning to groups where manageable, and using PIM for just-in-time access. Privileged role assignments such as Owner, Contributor, and User Access Administrator are powerful and can be monitored with alerts; role assignment changes can take time to propagate.
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

- Azure RBAC and Microsoft Entra directory roles are different assignment systems with different scopes and tooling.
- Eligible, time-bound PIM assignment for Azure RBAC is not equivalent for users, service principals, applications, and managed identities; verify supported principal type before recommending PIM as the answer.
- Built-in Microsoft Entra roles assigned to guests can grant the same role permissions as member users; do not downplay guest-admin blast radius.
- Administrative-unit-scoped assignments can still need tenant-scope read permissions for some principal types to function.

