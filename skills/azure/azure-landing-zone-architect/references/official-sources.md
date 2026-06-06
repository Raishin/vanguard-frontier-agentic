# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, RBAC, quotas, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-areas
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/identity-access
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/governance
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/security
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/management
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/platform-automation-devops
- https://learn.microsoft.com/azure/architecture/networking/architecture/hub-spoke

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says Azure landing zones are platform foundations spanning platform and application landing zones, management groups, subscriptions, policy inheritance, connectivity, identity, governance, security, management, and platform automation. The design-areas guidance explicitly calls out billing/tenant, identity/access, resource organization, network topology/connectivity, security, management, governance, and platform automation/DevOps as decisions that affect the foundation.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
