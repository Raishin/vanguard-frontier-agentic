# Packet A: Platform Automation, Private Endpoint, RBAC

## Targets

- `skills/azure/azure-platform-automation-devops`
- `skills/azure/azure-private-endpoint-adoption-planner`
- `skills/azure/azure-rbac-review`

## Evidence used

Microsoft Learn documentation through the user's configured documentation MCP. No live Azure environment state was sampled.

## Findings addressed

- Platform automation guidance was too thin versus the AgentCore reference standard; it now covers what-if limitations, lint/preflight/approval gates, bootstrap versus run separation, secret boundaries, and rollback evidence.
- Private endpoint guidance now treats DNS as the critical design boundary, including single private DNS zone expectations, VNet links, zone groups, resolver/forwarder paths, and at-scale governance.
- RBAC guidance now covers privileged administrator roles, PIM, group-based assignment, unique role IDs, wildcard custom-role risk, narrow scopes, and conditions.
