# Oracle MCP Servers

- Vendor: Oracle
- Status: official Oracle MCP resources
- Landing page: <https://www.oracle.com/mcp>
- Source: <https://github.com/oracle/mcp>
- Auth model: varies by server, including Oracle Database, SQLcl, OCI, MySQL, or documentation-specific authentication.
- Mutation risk: varies by server; database and OCI access can expose or mutate sensitive systems.
- Last verified: 2026-04-27

## Install/config

Start from Oracle's MCP landing page and the official `oracle/mcp` repository. Do not assume all Oracle MCP servers share the same runtime, authentication, or permission model.

## Security notes

Use scoped database users, least-privilege OCI policies, network restrictions, and audit logging. Treat database-connected MCP servers as sensitive production access unless proven otherwise.
