# AWS Official MCP Servers

- Vendor: AWS Labs
- Status: official AWS Labs open-source MCP server suite
- Source: <https://github.com/awslabs/mcp>
- Docs: <https://awslabs.github.io/mcp/>
- Auth model: AWS credentials supplied through the selected MCP server/client environment.
- Mutation risk: varies by server; some servers are documentation-only, others can interact with AWS APIs.
- Last verified: 2026-04-27

## Install/config

Use the official repository and server-specific documentation. Do not copy credentials into MCP config files; use named profiles, environment variables, or managed identity patterns where supported.

## Security notes

Start with documentation/read-only servers before API mutation servers. Scope AWS credentials by account, region, role, and permission boundary where possible.
