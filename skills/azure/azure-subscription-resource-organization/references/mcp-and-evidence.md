# MCP and Evidence Path

## Official Azure MCP Linkage

Use official Azure MCP tools only if they are actually available in the active client.

Preferred evidence path:

- `subscription` for current subscription inventory and placement context.
- `group` for resource-group inventory and workload grouping inside a subscription.
- `group` only if the active client exposes an official group-related Azure capability; do not assume this means management-group mutation support.

Important constraint:

- The official Azure MCP documentation identified for this repo clearly documents subscription and resource-group tools. It does **not** prove that every client exposes management-group tools. If management-group inspection is unavailable, switch to documentation mode plus sanitized user-provided hierarchy evidence.

## Platform-Agnostic Execution

This skill must work on macOS, Windows, Linux, and MCP-only clients.

- Prefer Azure MCP evidence first when available.
- When examples need commands or checks, keep them abstract with `<placeholders>` unless the user’s active platform and toolchain are known.
- Do not anchor the workflow to Azure CLI, PowerShell, Terraform, or portal-only steps unless the user asks for that execution path next.

## Documentation Fallback When Live Data Is Unavailable

Live Azure evidence beats documentation for current-state questions. If live MCP data is unavailable, denied, incomplete, or unsafe to query:

- Fall back to the Microsoft Learn references listed above.
- Ask for sanitized hierarchy diagrams, subscription inventories, policy-assignment views, naming standards, or ownership maps if current-state proof matters.
- Label conclusions as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`.
- Do not pretend Microsoft Learn proves the user’s actual hierarchy, subscription sprawl, policy inheritance, or team ownership reality.
