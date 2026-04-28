# MCP and Evidence Path

## Official Azure MCP Linkage

Use official Azure MCP servers as configured in the active runtime. Do not hard-code the server name; users may register the official Azure MCP server under any label. Detect by exposed tool capability and package identity hints, not by fixed server naming.

Preferred official Azure MCP capability for this role:

- `pricing` for retail pricing lookups, region/SKU comparisons, and template-backed cost estimation.

Secondary official context:

- broader Azure MCP tools inventory only to confirm documented capability existence, not to invent cost-analysis or billing behavior beyond the official docs.

If the expected Azure MCP pricing capability is missing or ambiguous, ask only which configured MCP server exposes the official Azure pricing tools. Do not ask for secrets, contract pricing, subscription dumps, credentials, or tokens.

Do not invent unsupported MCP tools. If a live need exceeds confirmed Azure MCP capability, switch to documentation mode and say so.

## Platform-Agnostic Execution

This skill must work on macOS, Windows, Linux, browser-first clients, and MCP-only clients. Prefer Azure MCP pricing evidence when available. When portal checks, pricing calculator workflows, CLI, PowerShell, Bicep, or ARM examples are useful, show neutral command or workflow shape with `<placeholders>` and adapt only after the user's active platform is known.

## Documentation Fallback When Live Data Is Unavailable

Live Azure MCP pricing data beats documentation. If live data is unavailable, incomplete, denied, or unsafe to query, switch to documentation/reference mode:

- Use Microsoft Learn pricing-calculator and cost-management documentation for estimate behavior, scope limits, and planning guidance.
- Use Microsoft Learn Azure MCP documentation only to describe confirmed official pricing-tool capability, not to imply tenant-specific cost visibility.
- Ask for sanitized estimate exports, screenshots, template snippets, region/SKU lists, uptime assumptions, transaction or throughput assumptions, storage-growth assumptions, and HA/DR assumptions when current-state evidence is required.
- Label every conclusion as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`.
- Do not pretend documentation proves the user's negotiated pricing, current discounts, exact invoice outcome, tax treatment, or real future utilization.
