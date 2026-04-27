# AGENTS.md

## Purpose
- Store MCP references with trust, auth, and safety notes.

## Patterns
- `official/` → vendor-official MCP references.
- `community/` → non-official MCP references.
- `examples/` → sample config snippets only.
- `*.metadata.json` must align with `catalog/mcp-references.json`.

## Do Not Miss
- MCP tools can expose or mutate real cloud resources.
- State auth model, vendor, install example, and unofficial warning.
- Prefer official project URLs and vendor docs.
- Run `npm run validate` after metadata edits.

