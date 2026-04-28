# MCP and Evidence Path

## Official Azure MCP Linkage

Use official Azure MCP capabilities as configured in the active runtime. Do not assume the server name.

Relevant namespaces may include:

- `aks` for cluster discovery and cluster configuration evidence,
- `monitor` for metrics/log signals when available,
- `applicationinsights` for Application Insights resource discovery only; use `monitor` for most telemetry analysis,
- `resourcehealth` when platform-health ambiguity exists,
- `advisor`, `role`, or `policy` only when the question crosses into posture or governance.

Do not invent unsupported AKS mutation flows. Based on current Azure MCP docs, AKS support is clearly cluster-oriented and should be treated primarily as evidence gathering unless the client exposes more.

## Platform-Agnostic Execution

This skill must work in MCP-only, macOS, Linux, and Windows clients.

Prefer:

1. Azure MCP evidence,
2. official Microsoft Learn and Architecture Center documentation,
3. sanitized user-provided exports or screenshots,
4. neutral command or query shapes with `<placeholders>` only when needed.

Do not assume the user runs `kubectl`, Azure CLI, Terraform, Helm, Flux, Argo CD, or GitHub Actions unless they say so.

## Documentation Fallback When Live Data Is Unavailable

Live Azure evidence beats theory. If live Azure MCP access is unavailable, incomplete, denied, or too risky:

- switch to Microsoft Learn and Azure Architecture Center guidance,
- ask for sanitized cluster configuration, architecture diagrams, upgrade runbooks, or redacted outputs,
- label each conclusion as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`,
- do not pretend documentation proves the user’s current cluster state.
