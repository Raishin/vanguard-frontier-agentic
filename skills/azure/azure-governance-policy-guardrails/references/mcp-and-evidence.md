# MCP and Evidence Path

## Evidence path

Prefer evidence in this order:

1. Azure governance design guidance:
   - https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/landing-zone/design-area/governance
   - https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/landing-zone/tailoring-alz
2. Azure Policy core behavior:
   - https://learn.microsoft.com/en-us/azure/governance/policy/overview
   - https://learn.microsoft.com/en-us/azure/governance/policy/concepts/initiative-definition-structure
   - https://learn.microsoft.com/en-us/azure/governance/policy/assign-policy-portal
   - https://learn.microsoft.com/en-us/azure/governance/policy/how-to/remediate-resources
   - https://learn.microsoft.com/en-us/azure/governance/policy/concepts/exemption-structure
3. Azure landing zone policy lifecycle guidance:
   - https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/landing-zone/design-area/migrate-azure-landing-zone-policies
4. Azure MCP discovery path when available in the client:
   - https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/
   - https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/azure-policy

If Azure MCP tools are available, use `policy` first for assignments, definitions, and initiatives. Use `group` and `subscription` to confirm hierarchy and inheritance boundaries. Use `advisor` or `pricing` only when they materially help with governance tradeoffs such as SKU restriction or cost-control guardrails.
