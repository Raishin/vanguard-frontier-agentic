# Vanguard Frontier Agentic

Use this repository as a **curated cloud and zero-trust AI workflow marketplace**, not as a generic prompt dump.

## Repository intent

The north star is practical, evidence-backed cloud engineering:

- secure by default
- least privilege by default
- compliance-aware
- understandable by engineers of any seniority

## What matters most

- Prefer **official documentation** and **observable evidence** over assumptions.
- Challenge vague architecture claims, broad access, and unsafe automation.
- Ask: who has access, why, where is the evidence, how is abuse/drift detected, and how is recovery handled?

## Repo map

- `skills/` — step-by-step workflows
- `agents/` — expert role definitions
- `rules/` — harness guidance
- `mcp/` — tool/server integration references
- `catalog/` — machine-readable indexes
- `schemas/` — metadata contracts
- `docs/` — governance, taxonomy, compatibility, and quality bar

## Change discipline

- Keep edits surgical.
- Preserve source grounding for cloud and compliance claims.
- Update catalog metadata when cataloged assets move or change.
- Run `npm run validate` before finishing.
- If intentional changes occur under cataloged `skills/**`, run `npm run manifest:write`.

## Cross-platform rule

Do not pretend Claude, Codex, Cursor, Copilot, Gemini, and Kiro use the same executable format.

- portable core: name, description, instructions
- non-portable details: file format, tool fields, model fields, MCP wiring, metadata support

See:

- `AGENTS.md`
- `README.md`
- `docs/compatibility.md`
- `docs/normalized-platform-matrix.md`

