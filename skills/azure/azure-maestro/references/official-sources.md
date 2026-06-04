# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, migration project, network, telemetry, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/architecture/
- https://learn.microsoft.com/azure/well-architected/
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-areas
- https://learn.microsoft.com/azure/role-based-access-control/best-practices
- https://learn.microsoft.com/azure/azure-monitor/fundamentals/overview

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence across Azure Architecture Center, Well-Architected Framework, Cloud Adoption Framework, Azure RBAC, and Azure Monitor supports domain-specific ownership, least privilege, operational evidence, and risk-based escalation. Maestro is a routing layer: official docs ground domains and safety principles, while repo catalog state proves which specialist IDs exist.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
