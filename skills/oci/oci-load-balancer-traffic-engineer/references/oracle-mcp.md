# Official Oracle MCP Capability Mapping

Use this reference when selecting live Oracle MCP tools for `oci-load-balancer-traffic-engineer`.

## Selection rule

Detect by exposed tool capability, not by client-side MCP server label. Users can register official Oracle MCP servers under any name.

## Preferred capabilities

- oracle.oci-load-balancer-mcp-server; oracle.oci-network-load-balancer-mcp-server: list_network_load_balancers, get_network_load_balancer, list_network_load_balancer_listeners, list_network_load_balancer_backend_sets, list_network_load_balancer_backends

If the expected Oracle MCP tools are missing or ambiguous, ask the user for the configured MCP server name only that exposes the official Oracle tools. Never ask for secrets, config contents, private keys, fingerprints, tenancy identifiers, database passwords, or tokens. Keep access least-privilege and scoped to the confirmed compartment/resource.

## Missing or ambiguous MCP tools

If expected tools are not exposed or multiple Oracle-like servers are ambiguous, ask the user for the configured MCP server name that exposes the official Oracle tools. Ask for the server name only. Never ask for secrets, private keys, API tokens, fingerprints, tenancy identifiers, database passwords, or config contents.

## Runtime priority

1. Service-specific official Oracle MCP read/list/get/search tools.
2. Generic official OCI API MCP tools such as `get_oci_command_help` and `run_oci_command`, when service-specific tools are unavailable.
3. OCI CLI default profile fallback only when MCP is unavailable or insufficient.
