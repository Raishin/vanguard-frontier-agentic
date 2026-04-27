# Official Oracle MCP Capability Mapping

Use this reference when selecting live Oracle MCP tools for `oci-devops-container-platform-engineer`.

## Selection rule

Detect by exposed tool capability, not by client-side MCP server label. Users can register official Oracle MCP servers under any name.

## Preferred capabilities

- oracle.oci-registry-mcp-server: list_container_repositories, get_container_repo_details, create_container_repository, delete_container_repository; oracle.oci-cloud-mcp-server for DevOps APIs not covered by a dedicated server.
- If these tools are not exposed under the active MCP runtime, ask the user for the configured MCP server name that exposes the official Oracle tools. Ask for the name only, not credentials or config contents.

## Missing or ambiguous MCP tools

If expected tools are not exposed or multiple Oracle-like servers are ambiguous, ask the user for the configured MCP server name that exposes the official Oracle tools. Ask for the server name only. Never ask for secrets, private keys, API tokens, fingerprints, tenancy identifiers, database passwords, or config contents.

## Runtime priority

1. Service-specific official Oracle MCP read/list/get/search tools.
2. Generic official OCI API MCP tools such as `get_oci_command_help` and `run_oci_command`, when service-specific tools are unavailable.
3. OCI CLI default profile fallback only when MCP is unavailable or insufficient.
