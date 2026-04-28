# MCP and Evidence Path

## Official Azure MCP Linkage

Use official Azure MCP capabilities only when they are actually exposed in the active runtime.

Relevant namespaces may include:

- `group`, `subscription`, and `resourcehealth` for target-scope evidence,
- `azuremigrate` when the client exposes the official Azure Migrate tooling,
- `monitor` for validation or incident signal posture,
- `deploy` or `bicepschema` if landing-zone deployment evidence is relevant,
- generic Azure discovery namespaces when they materially help confirm the target environment.

Do not assume the Azure Migrate namespace exists in the current runtime just because Microsoft documents it. If migration-specific live tooling is missing, switch to documentation-grounded and sanitized-evidence review.

## Platform-Agnostic Execution

This skill must work in MCP-only, macOS, Linux, and Windows clients.

Prefer:

1. assessment evidence,
2. landing-zone readiness evidence,
3. sanitized dependency maps and migration wave plans,
4. neutral placeholder commands or artifacts only when needed.

Do not assume Azure CLI, PowerShell, Terraform, GitHub Actions, or Azure DevOps unless the user confirms the toolchain.

## Documentation Fallback When Live Data Is Unavailable

Live target-state evidence beats theory. If live access is unavailable, incomplete, denied, or too risky:

- fall back to official Azure Migrate and Cloud Adoption Framework guidance,
- ask for sanitized assessment exports, wave plans, dependency diagrams, landing-zone summaries, and rollback checklists,
- label each conclusion as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`,
- do not present a migration as safe merely because Azure Migrate produced a recommendation.
