# Official Oracle MCP Capability Mapping

Use this reference when selecting live Oracle MCP tools for `oci-autonomous-database-architect`.

## Selection rule

Detect by exposed tool capability, not by client-side MCP server label. Users can register official Oracle MCP servers under any name.

## Preferred capabilities

- oracle.oci-database-mcp-server for autonomous-database, autonomous-container-database, autonomous-vm-cluster, cloud-autonomous-vm-cluster, backups, peers, Data Guard, wallet, Operations Insights, and Data Safe posture; oracle.oci-recovery-mcp-server for recovery posture; oracle.oracle-db-doc-mcp-server for database documentation fallback.

## Useful OCI command families when generic OCI API MCP or CLI fallback is needed

- `db autonomous-database` for Autonomous Database and Autonomous AI Database list/get/update/backup/clone/DR operations where supported.
- `db autonomous-container-database` for dedicated autonomous container database lifecycle and Data Guard operations where supported.
- `db autonomous-vm-cluster` and `db cloud-autonomous-vm-cluster` for dedicated autonomous fleet capacity where supported.
- `db cloud-exa-infra`, `db exadata-infrastructure`, `db cloud-vm-cluster`, `db vm-cluster`, `db db-home`, `db database`, and `db pluggable-database` for Exadata and database stack evidence where supported.

## Missing or ambiguous MCP tools

If expected tools are not exposed or multiple Oracle-like servers are ambiguous, ask the user for the configured MCP server name that exposes the official Oracle tools. Ask for the server name only. Never ask for secrets, credentials, API tokens, fingerprints, tenancy identifiers, database passwords, wallets, or config contents.

## Runtime priority

1. Service-specific official Oracle MCP read/list/get/search tools.
2. Generic official OCI API MCP tools such as command help and OCI command execution, when service-specific tools are unavailable.
3. OCI CLI default profile fallback only when MCP is unavailable or insufficient.
