# AGENTS.md

## Purpose
- Cloud + zero-trust agentic marketplace for skills, agents, rules, MCP references, compliance-aware docs, and npm distribution.
- Optimize for evidence-backed security workflows: least privilege, source grounding, manifests, validation, and safe automation.

## Stack Map
- `.code-review-graph/` → generated local graph cache; do not edit.
- `.git/` → Git internals; do not edit.
- `agents/` → Markdown/JSON agent definitions; provider/domain layout.
- `assets/` → curated logos and visual assets.
- `catalog/` → JSON marketplace indexes and skill integrity manifest.
- `docs/` → Markdown governance, taxonomy, release, compatibility guidance.
- `mcp/` → Markdown/JSON MCP references.
- `rules/` → Markdown/JSON harness rules.
- `schemas/` → JSON Schema metadata contracts.
- `skills/` → Markdown/JSON skill packages with reference files.
- `templates/` → starter Markdown/JSON asset templates.
- `tests/` → Python validation scripts.
- `package.json` → npm package metadata and validation scripts.

## Workflows
- `npm run validate` → catalog + skill manifest + offline link validation.
- `npm run manifest:write` → refresh `catalog/skill-manifest.json` after intentional skill edits.
- `python3 tests/validate-links.py` → online link validation before release.
- `npm pack --dry-run` → inspect npm package contents before publish.

## Change Rules
- Update catalog JSON when adding, moving, or removing cataloged assets.
- Regenerate skill manifest after any intentional change under cataloged `skills/**`.
- Keep README human-friendly; keep this file agent-focused and compressed.
- Do not add secrets, credentials, tokens, wallets, tenant IDs, or customer data.
- Prefer official docs and live evidence over memory for cloud/compliance claims.
- Treat broad permissions, destructive automation, and MCP mutation paths as high-risk.

## Load When
- editing `agents/` → `agents/AGENTS.md`
- editing `catalog/` → `catalog/AGENTS.md`
- editing `docs/` → `docs/AGENTS.md`
- editing `mcp/` → `mcp/AGENTS.md`
- editing `rules/` → `rules/AGENTS.md`
- editing `schemas/` → `schemas/AGENTS.md`
- editing `skills/` → `skills/AGENTS.md`
- editing `templates/` → `templates/AGENTS.md`
- editing `tests/` or validation scripts → `tests/AGENTS.md`
