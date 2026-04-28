# MCP and Evidence Path

## Official Azure MCP Linkage

Use only official Azure MCP capabilities actually exposed in the active runtime.

Relevant namespaces may include:

- `keyvault` for vault, key, secret, certificate, and Managed HSM evidence,
- `role` when RBAC assignment correlation is required,
- `monitor` if alerting or event visibility is part of the audit,
- `policy` when policy-enforced protections matter.

Do not confuse tool availability with audit sufficiency:

- Azure MCP can help inspect assets and settings.
- It does not automatically prove rotation logic, downstream dependency handling, or restore readiness.
- Secret-reading tools may require user confirmation because they can expose sensitive data. Avoid them unless absolutely necessary.

## Platform-Agnostic Execution

This skill must work in MCP-only, macOS, Linux, and Windows clients.

Prefer:

1. Azure MCP metadata and configuration evidence,
2. official Microsoft Learn and policy references,
3. sanitized inventories, screenshots, or exports from the user,
4. neutral placeholder commands only when needed.

Do not assume the user operates through Azure CLI, PowerShell, Terraform, Bicep, or portal-only workflows unless they say so.

## Documentation Fallback When Live Data Is Unavailable

Live vault posture beats theory. If live access is unavailable, incomplete, denied, or unsafe:

- fall back to official Microsoft documentation,
- ask for sanitized inventories such as vault settings, secret lists without values, expiration metadata, role assignments, and alert definitions,
- label each conclusion as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`,
- do not claim a vault is safe merely because the platform supports the right features.
