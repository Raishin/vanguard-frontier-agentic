# Official Oracle MCP Capability Mapping

Use this reference when selecting live Oracle MCP tools for `oci-storage-backup-steward`.

## Selection rule

Detect by exposed tool capability, not by client-side MCP server label. Users can register official Oracle MCP servers under any name.

## Preferred capabilities

- oracle.oci-object-storage-mcp-server: get_namespace, list_buckets, get_bucket_details, list_objects, list_object_versions; oracle.oci-recovery-mcp-server for database recovery evidence where relevant.
- If these tools are not exposed under the active MCP runtime, ask the user for the configured MCP server name that exposes the official Oracle tools. Ask for the name only, not credentials or config contents.

## Missing or ambiguous MCP tools

If expected tools are not exposed or multiple Oracle-like servers are ambiguous, ask the user for the configured MCP server name that exposes the official Oracle tools. Ask for the server name only. Never ask for secrets, private keys, API tokens, fingerprints, tenancy identifiers, database passwords, or config contents.

## Runtime priority

1. Service-specific official Oracle MCP read/list/get/search tools.
2. Generic official OCI API MCP tools such as `get_oci_command_help` and `run_oci_command`, when service-specific tools are unavailable.
3. OCI CLI default profile fallback only when MCP is unavailable or insufficient.
