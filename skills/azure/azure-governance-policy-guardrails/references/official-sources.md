# Official Sources

Load these only when needed:

- [What is Azure Policy?](https://learn.microsoft.com/azure/governance/policy/overview) — use for policy object model, assignment scope behavior, evaluation timing, Azure RBAC interaction, and core rollout cautions.
- [Azure Policy definitions effect basics](https://learn.microsoft.com/azure/governance/policy/concepts/effect-basics) — use when comparing `audit`, `auditIfNotExists`, `deny`, `modify`, and `deployIfNotExists`.
- [Remediate non-compliant resources with Azure Policy](https://learn.microsoft.com/azure/governance/policy/how-to/remediate-resources) — use for managed identity, RBAC, and remediation-task implications.
- [Azure Policy built-in policy definitions](https://learn.microsoft.com/azure/governance/policy/samples/built-in-policies) — use when checking whether built-ins already cover tags, locations, SKUs, or baseline controls.
- [Adopt policy-driven guardrails](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/enterprise-scale/dine-guidance) — use for canary rollout, enforcement mode, and phased `audit` to `deny` or remediation sequencing.
- [Azure landing zone design principles](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-principles) — use when guardrails are part of the broader landing-zone operating model.
- [Azure MCP Server tools inventory](https://learn.microsoft.com/azure/developer/azure-mcp-server/tools/) — use to verify whether `policy`, `group`, `subscription`, `advisor`, or other namespaces are actually documented before naming them.

## Grounded insights worth carrying into the skill

- A policy can be assigned at management-group scope, but Azure Policy evaluates resources at subscription or resource-group level; do not imply it governs arbitrary tenant objects.
- `modify` and `deployIfNotExists` are not “free automation”; their assignment identities need the right Azure RBAC permissions to create or update target resources.
- Microsoft guidance explicitly recommends starting with `audit` or `auditIfNotExists` when rollout risk is unclear, rather than jumping straight to production `deny` or remediation.
- Broad exclusions are usually governance debt. Prefer narrow exclusions or time-bounded exemptions with named ownership.
