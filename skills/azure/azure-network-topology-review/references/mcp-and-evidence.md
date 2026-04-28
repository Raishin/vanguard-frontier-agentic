# MCP and Evidence Path

## Evidence path

Prefer evidence in this order:

1. Azure landing-zone design areas for the platform context and design-area coupling:
   - https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/landing-zone/design-areas
2. Azure Architecture Center hub-spoke guidance for topology behavior and ownership implications:
   - https://learn.microsoft.com/en-us/azure/architecture/networking/architecture/hub-spoke
3. Azure Architecture Center Private Link hub-and-spoke guidance when private networking, DNS, or `/32` route behavior is materially relevant:
   - https://learn.microsoft.com/en-us/azure/architecture/networking/guide/private-link-hub-spoke-network
4. Azure MCP discovery guidance, only when the current client actually exposes useful Azure tools:
   - https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/

If Azure MCP is available, use it for read-focused evidence gathering, not to skip architecture reasoning. Repo-backed docs only support generic Azure MCP tooling plus `group` and `subscription` as clearly named namespaces for scope confirmation, so do not invent unsupported network namespace claims.
