# Official sources for Azure AI Foundry Ops Governor

Use this file to ground reviews in Microsoft Learn documentation through the user's configured documentation MCP. Documentation evidence proves Microsoft-published behavior; it does not prove the user's tenant, RBAC, quotas, network routes, deployed resources, or production readiness.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [Microsoft Foundry architecture](https://learn.microsoft.com/en-us/azure/foundry/concepts/architecture) | Treat the Foundry resource as the governance/security/network/deployment boundary and projects as team development boundaries. Verify connected resources separately. |
| [Role-based access control for Microsoft Foundry](https://learn.microsoft.com/en-us/azure/foundry/concepts/rbac-foundry) | Validate scope and role fit. Do not recommend broad Owner/Contributor when Foundry-specific roles or split data/control-plane roles are enough. |
| [Azure AI security best practices](https://learn.microsoft.com/en-us/azure/security/fundamentals/ai-security-best-practices) | Ground network isolation, least privilege, model governance, secure compute, and monitoring recommendations. |
| [Configure secure networking for Azure AI platform services](https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ai/platform/networking) | Use for private endpoint and supporting-service network-boundary checks. |
| [Foundry Model Context Protocol security best practices](https://learn.microsoft.com/en-us/azure/foundry/mcp/security-best-practices) | Use only for Foundry product MCP behavior, identity, RBAC categories, and Conditional Access review. Do not confuse product documentation with evidence about this user's runtime. |
| [Feature availability across cloud regions](https://learn.microsoft.com/en-us/azure/foundry/reference/region-support) | Confirm model, agent, evaluation, and feature regional support before endorsing a rollout. |
| [Azure OpenAI quotas and limits](https://learn.microsoft.com/en-us/azure/foundry/openai/quotas-limits) | Treat quota as regional and deployment-dependent; require configured-environment evidence before claiming capacity. |
| [Agent Service limits, quotas, and regions](https://learn.microsoft.com/en-us/azure/foundry/agents/concepts/limits-quotas-regions) | Use for agent-specific limit checks; do not extrapolate from model quota alone. |

## Source-grounding rules

- Documentation claim: cite the relevant Microsoft Learn source and state it as documented behavior.
- Current environment claim: require sampled read-only Azure evidence or user-provided sanitized output.
- Inference: label it as inference and identify the missing proof.
- Unsupported claim: do not include it. Push back and ask for safe evidence instead.

## Release-note deltas to keep current

- Foundry role names can appear under old or new names during rollout. Prefer role definition IDs when writing automation, and explain the rename instead of treating one label as wrong.
- Some APIs and UI paths are project-scoped while others remain resource-scoped. Verify scope before recommending automation.
- Regional availability varies by model, deployment type, agents, evaluations, and quota. A successful design in one region does not prove availability in another.
