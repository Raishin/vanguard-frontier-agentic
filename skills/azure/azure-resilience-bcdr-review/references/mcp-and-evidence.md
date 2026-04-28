# MCP and Evidence Path

## Official Azure MCP Linkage

Use only official Azure MCP capabilities that are actually exposed in the active client. Do not invent namespaces or claim live tooling exists when it is unavailable.

Useful namespaces for this role can include:

- `resourcehealth`
- `monitor`
- `advisor`
- `group`
- `subscription`

Use Azure MCP for evidence such as:

- current subscription or resource scope,
- resource-health and service-health signals,
- alerting and monitor configuration posture,
- Advisor recommendations that affect resiliency or operational readiness.

Do not treat MCP visibility as proof that recovery will succeed. Health signals and configuration inventory are supporting evidence, not a substitute for tested runbooks.

## Platform-Agnostic Execution

This skill must work on macOS, Windows, Linux, and MCP-only clients. Prefer Azure MCP evidence when available. When examples need CLI, PowerShell, ARM, Bicep, or Terraform context, keep them neutral with `<placeholders>` until the user confirms platform and toolchain.

## Documentation Fallback When Live Data Is Unavailable

Live Azure evidence beats static guidance for current-state posture. If live data is unavailable, incomplete, denied, or too risky to query:

- switch to documentation-grounded review mode,
- use only official Microsoft Learn guidance listed in this skill,
- ask for sanitized architecture diagrams, runbooks, dependency maps, recovery test notes, or redacted monitoring screenshots,
- label conclusions as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`,
- explicitly state that documentation cannot prove the tenant is actually recoverable.
