# MCP and Evidence Path

## Official Azure MCP Linkage

Use only official Azure MCP capabilities that are actually exposed in the active client runtime. Do not invent namespaces or pretend undocumented tools exist.

Known official linkage relevant to this role, based on Microsoft documentation:

- `appservice` for web app details, app settings, deployment details, detectors, and App Service diagnostics actions.
- `monitor` for logs and metrics if the runtime exposes Azure Monitor tools.
- `resourcehealth` for current health events if the runtime exposes Azure Resource Health tools.
- `advisor` for recommendation discovery if the runtime exposes Azure Advisor tools.
- `pricing`, `quota`, `group`, and `subscription` can help with supporting evidence when exposed.

Rules:

- Treat `appservice` as useful but incomplete. Official documentation shows App Service MCP support for app details, settings, deployments, detectors, diagnostics, and database connection operations; it does not prove first-class slot, VNet, private endpoint, or plan-capacity mutation tooling in MCP.
- If a needed capability is generic or ambiguous, say so explicitly and switch to documentation-grounded review instead of hallucinating a tool path.
- Treat secret-bearing outputs, especially app settings, as sensitive even when a tool can retrieve them.
- Distinguish read-only evidence gathering from mutating operations such as updating app settings.

## Platform-Agnostic Execution

This skill must work on macOS, Windows, Linux, and MCP-only clients.

- Prefer evidence categories, architecture decisions, and operator checks over shell-specific commands.
- When examples are useful, use placeholders such as `<subscription>`, `<resource-group>`, `<web-app>`, `<plan>`, `<slot>`, and `<vnet-subnet>`.
- Adapt Azure CLI, PowerShell, IaC, or portal-specific steps only after the user’s actual toolchain is known.
- Keep recommendations portable across code apps and custom-container apps unless the runtime model is already fixed.

## Documentation Fallback When Live Data Is Unavailable

Live Azure evidence beats static guidance for current-state validation. If live inspection is unavailable, incomplete, unsafe, or denied:

- fall back to the Microsoft Learn and Well-Architected references listed above;
- ask for sanitized architecture diagrams, slot configuration summaries, dependency maps, redacted app settings keys, alert inventories, or runbook excerpts when current-state proof is required;
- label each conclusion as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`;
- do not claim the app is production-ready merely because the target SKU supports a feature;
- do not claim a feature is enabled in the user’s environment unless it was confirmed live.
