# MCP and Evidence Path

## Official Azure / Foundry MCP Linkage

Use only official Microsoft MCP surfaces documented for this role.

Preferred order:
1. **Foundry MCP Server** for Foundry-native operations when the active client exposes it.
2. **Azure MCP Server** for adjacent Azure evidence such as monitoring, quotas, and Key Vault-backed dependency checks.
3. **Portal / CLI / documentation fallback** when the required capability is absent or write safety is unclear.

Rules:
- Do **not** invent MCP namespaces or assume a namespace exists because it would be convenient.
- Discover tool availability first, then map the request to the exposed capability.
- Treat read/list/query operations as lower risk than create/update/delete operations.
- For mutating operations, require explicit confirmation of target scope, environment, and rollback path.
- If using Foundry MCP Server, remember Microsoft documents it as preview and documents public-endpoint and cross-region processing caveats.

## Platform-Agnostic Execution

This skill must work in MCP-only, portal-guided, CLI-guided, macOS, Linux, and Windows environments.

When examples are needed:
- use neutral placeholders like `<subscription>`, `<resource-group>`, `<foundry-resource>`, and `<project>`,
- avoid shell-specific assumptions until the platform is known,
- prefer evidence collection and decision framing over long command dumps.

## Documentation Fallback When Live Data Is Unavailable

If live access is missing, denied, incomplete, or unsafe:
- ground recommendations in official Microsoft Learn documentation only,
- label conclusions as `live evidence`, `documentation-based`, `sanitized user evidence`, or `inference`,
- do not present documentation as proof of the user's current state,
- ask for sanitized scope details only when needed: subscription layout, resource count, project count, regions, environment split, or redacted screenshots.
