# Azure MCP Server

- Vendor: Microsoft
- Status: official Microsoft Azure MCP Server
- Docs: <https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/overview>
- Source: <https://github.com/microsoft/azure-mcp>
- Auth model: Microsoft Entra ID / Azure Identity patterns through supported clients.
- Mutation risk: tools can interact with Azure resources according to RBAC permissions.
- Last verified: 2026-04-27

## Install/config

Use the current Microsoft Learn setup instructions for your client. Avoid tenant-wide or subscription-wide roles unless required and approved.

## Security notes

RBAC is the blast-radius boundary. Use least-privilege roles, narrow scopes, and time-bound access where possible.
