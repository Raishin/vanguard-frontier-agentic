# MCP and Evidence Path

## Official Azure MCP Linkage

Use official Azure MCP capabilities only if they are actually exposed in the active runtime. This repo's evidence base supports Azure MCP as a generic official tool inventory, not a guaranteed private-endpoint-specific tool surface.

Safe rule:

- if the active client exposes Azure read-oriented discovery for subscription, resource group, or resource inspection, use that for current-state evidence;
- if private endpoint, DNS, or effective-route capabilities are not clearly exposed, switch to documentation mode instead of pretending the tools exist;
- never hard-code an invented MCP namespace like `privateendpoint`, `dnszone`, or `networkwatcher` unless the active client explicitly exposes it.

## Platform-Agnostic Execution

This skill must work in MCP-only, browser-only, macOS, Linux, and Windows environments. Prefer architecture reasoning plus official documentation. If examples are helpful, use neutral placeholders such as `<subscription>`, `<resource-group>`, `<vnet>`, and `<private-dns-zone>` rather than platform-specific scripts unless the user asks for a concrete execution path.

## Documentation Fallback When Live Data Is Unavailable

Live Azure evidence beats documentation, but documentation beats guessing.

If live Azure MCP evidence is unavailable, incomplete, or ambiguous:

- use the Microsoft Learn references above,
- ask for sanitized topology diagrams, private DNS zone names, VNet linkage descriptions, or redacted architecture notes,
- label conclusions as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`,
- do not claim the user's DNS links, route tables, resolver path, or effective access model are correct unless they are actually evidenced.
