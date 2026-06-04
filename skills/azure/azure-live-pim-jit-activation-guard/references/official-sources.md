# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, migration project, network, telemetry, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-configure
- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-resource-roles-activate-your-roles
- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-resource-roles-configure-role-settings
- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-resource-roles-approval-workflow
- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-deployment-plan
- https://learn.microsoft.com/entra/identity/role-based-access-control/best-practices

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says PIM provides just-in-time, time-bound, approval-based privileged access for Microsoft Entra and Azure resources. Azure resource role activation can require MFA, reduced scope, start time, duration, and reason; approval can leave a request pending. PIM temporarily adds active assignment and later removes it, but applications can cache role state so access changes may not appear immediately.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
