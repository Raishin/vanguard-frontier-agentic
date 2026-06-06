# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, RBAC, quotas, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/entra/architecture/ops-guide-govern
- https://learn.microsoft.com/entra/id-governance/scenarios/least-privileged
- https://learn.microsoft.com/entra/id-governance/identity-governance-overview
- https://learn.microsoft.com/entra/id-governance/access-reviews-overview
- https://learn.microsoft.com/entra/id-governance/entitlement-management-overview
- https://learn.microsoft.com/entra/identity/role-based-access-control/best-practices
- https://learn.microsoft.com/entra/identity/role-based-access-control/security-emergency-access
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/identity-access

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says Entra ID Governance covers entitlement management, access reviews, lifecycle workflows, and PIM. The operations guide requires task owners, testing strategy, regular reviews for applications, external identities and privileged roles, emergency access accounts, and entitlement management. Least-privilege guidance points to feature-specific administrative roles and JIT role activation through PIM.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
