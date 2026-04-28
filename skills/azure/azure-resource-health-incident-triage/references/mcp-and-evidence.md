# MCP and Evidence Path

## Official Azure MCP Linkage

Use official Azure MCP capabilities as exposed in the active client. Do not invent a namespace, tool, or server label.

Based on the repo spec and Microsoft Azure MCP documentation, relevant capability families can include:

- `resourcehealth` for resource availability status and service-impacting health events,
- `monitor` for activity-log retrieval and related Azure Monitor evidence,
- `group` and `subscription` when scope discovery is required before health checks.

Rules:

- Prefer read-only health and activity evidence first.
- If the expected Azure health tooling is absent, switch to documentation mode instead of pretending live checks happened.
- Ask for the configured Azure MCP server name only if the client exposes multiple ambiguous Azure servers and the correct one is unclear.
- Never ask for secrets, credential exports, tenant dumps, or subscription-wide privileged changes just to triage health.

## Platform-Agnostic Execution

This skill must work on macOS, Windows, Linux, and MCP-only clients. Prefer Azure MCP evidence. When portal, CLI, PowerShell, REST, or ARM examples are useful, keep them neutral with `<placeholders>` until the user confirms the active platform and access path.

## Documentation Fallback When Live Data Is Unavailable

Live Azure evidence beats documentation, but documentation is safer than guessing.

If live Azure MCP access is unavailable, incomplete, denied, or clearly out of scope:

- use Microsoft Learn documentation to define what Resource Health, Service Health, Activity Log, and health alerts can and cannot prove,
- ask for sanitized screenshots, exported alert payloads, event timestamps, activity-log entries, or redacted incident notes,
- label each conclusion as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`,
- explicitly say when current tenant state is unverified,
- do not claim a real Azure incident exists unless current evidence shows it.
