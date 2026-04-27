# Official Oracle MCP Capability Mapping

Use this reference when selecting live Oracle MCP tools for `oci-solution-architect`.

## Selection rule

Detect by exposed tool capability, not by client-side MCP server label. Users can register official Oracle MCP servers under any name.

## Preferred capabilities

- oracle.oci-resource-search-mcp-server for estate discovery; oracle.oci-cloud-mcp-server for generic service API exploration; domain-specific official Oracle MCP servers for IAM, network, compute, database, storage, monitoring, security, limits, and cost evidence.
- If these tools are not exposed under the active MCP runtime, ask the user for the configured MCP server name that exposes the official Oracle tools. Ask for the name only, not credentials or config contents.

## Missing or ambiguous MCP tools

If expected tools are not exposed or multiple Oracle-like servers are ambiguous, ask the user for the configured MCP server name that exposes the official Oracle tools. Ask for the server name only. Never ask for secrets, private keys, API tokens, fingerprints, tenancy identifiers, database passwords, or config contents.

## Runtime priority

1. Service-specific official Oracle MCP read/list/get/search tools.
2. Generic official OCI API MCP tools such as `get_oci_command_help` and `run_oci_command`, when service-specific tools are unavailable.
3. OCI CLI default profile fallback only when MCP is unavailable or insufficient.
