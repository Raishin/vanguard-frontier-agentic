# AGENTS.md

## Purpose
- Store machine-readable marketplace indexes and integrity manifests.

## Files
- `skills.json` → cataloged skill entries.
- `agents.json` → cataloged agent entries.
- `rules.json` → cataloged rule entries.
- `mcp-references.json` → cataloged MCP reference entries.
- `skill-manifest.json` → SHA-256 integrity map for cataloged skills.
- `index.json` → catalog entrypoint.

## Rules
- Keep catalog paths relative to repo root.
- Keep IDs unique across all catalog files.
- Update catalog and adjacent metadata together.
- Regenerate `skill-manifest.json` after intentional skill file edits.
- Run `npm run validate` before finishing.

