# Official sources for Azure Governance Policy Guardrails

Use Microsoft Learn documentation through the user's configured documentation MCP before designing Azure Policy guardrails. Documentation proves policy behavior; it does not prove the user's assignment scope, current compliance, remediation identity permissions, or workload impact.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [What is Azure Policy?](https://learn.microsoft.com/en-us/azure/governance/policy/overview) | Ground policy definitions, initiatives, assignments, evaluation triggers, remediation, RBAC, and start-with-audit recommendations. |
| [Azure Policy effect basics](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/effect-basics) | Use for effect behavior, evaluation order, and why effects are not interchangeable. |
| [DeployIfNotExists effect](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/effect-deploy-if-not-exists) | Use for DINE timing, managed identity, and remediation caveats. |
| [Policy compliance states](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/compliance-states) | Use for compliance interpretation and limitations. |
| [Policy initiative definition structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/initiative-definition-structure) | Use for grouping definitions and initiative parameter strategy. |
| [Policy exemption structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/exemption-structure) | Use for exception governance and expiry. |
| [Remediate non-compliant resources](https://learn.microsoft.com/en-us/azure/governance/policy/how-to/remediate-resources) | Use for remediation tasks and required managed identity permissions. |
| [Adopt policy-driven guardrails](https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/enterprise-scale/dine-guidance) | Use for phased DINE/Modify rollout and DoNotEnforce/canary patterns. |

## Source-grounding rules

- Do not use Azure Policy as a workload deployment engine.
- Do not deploy broad deny/modify/remediation first; start with audit or staged scope unless risk justifies enforcement.
- Do not treat compliance percentage as safety proof; inspect applicability, exclusions, exemptions, and stale evaluations.
- Require identity permission review for DINE/Modify remediation.
