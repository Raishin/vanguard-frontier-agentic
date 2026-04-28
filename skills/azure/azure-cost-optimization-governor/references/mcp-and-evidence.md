# MCP and Evidence Path

## Official Azure MCP Linkage

Use official Azure MCP servers as configured in the active runtime. Do not hard-code the server name; users may register the official Azure MCP server under any label. Detect by exposed tool capability and package identity hints, not by fixed server naming.

Preferred official Azure MCP capability for this role:

- `pricing` for retail pricing, deployment-cost estimation, and pricing comparisons.
- `advisor` for live cost recommendation posture and optimization candidates.
- `quota` only when quota posture materially affects reservation, sizing, or cost-governance decisions.

If the expected Azure MCP tools are missing or ambiguous, ask only which configured MCP server exposes the official Azure tools. Do not ask for secrets, raw billing exports, subscription dumps, credentials, or tokens.

Do not invent unsupported MCP tools. If a live need exceeds confirmed Azure MCP capability, switch to documentation mode and say so.

## Platform-Agnostic Execution

This skill must work on macOS, Windows, Linux, browser-first clients, and MCP-only clients. Prefer Azure MCP evidence. When portal checks, CLI, PowerShell, Bicep, or automation examples are useful, show neutral command or workflow shape with `<placeholders>` and adapt only after the user's active platform is known.

## Documentation Fallback When Live Data Is Unavailable

Live Azure MCP data beats documentation. If live data is unavailable, incomplete, denied, or unsafe to query, switch to documentation/reference mode:

- Use Microsoft Learn Cost Management and Billing documentation for cost-governance behavior, budget mechanics, cost-analysis visibility, exports, and optimization framing.
- Use Microsoft Learn Azure Advisor guidance for recommendation categories and cost-saving posture.
- Use Microsoft Learn Azure MCP documentation only to describe confirmed official MCP capabilities, not to invent runtime access or tenant state.
- Ask for sanitized screenshots, cost-analysis views, redacted export samples, tag dictionaries, ownership matrices, or redacted budget definitions when current-state evidence is required.
- Label every conclusion as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`.
- Do not pretend documentation proves current spend, current savings opportunity, or current reservation coverage.
