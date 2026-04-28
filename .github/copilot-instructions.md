# Vanguard Frontier Agentic repository instructions

This repository is a curated marketplace for **cloud**, **zero-trust**, and **compliance-aware** AI workflows.

## What to optimize for

- Evidence-backed guidance over confident guessing
- Least privilege over convenience
- Zero-trust design over implicit trust
- Safe automation with approval, validation, and rollback paths
- Clear separation between facts, assumptions, inference, and unknowns

## Repo structure

- `skills/` = reusable workflows
- `agents/` = expert roles
- `rules/` = harness-specific instructions
- `mcp/` = MCP references and trust notes
- `catalog/` = machine-readable indexes
- `schemas/` = metadata contracts
- `docs/` = governance, taxonomy, quality, compatibility

## Rules for changes

- Keep changes tightly scoped to the request.
- Do not add secrets, credentials, tokens, tenant IDs, wallets, or customer data.
- Prefer official docs and live evidence for cloud/compliance claims.
- Treat destructive actions, broad permissions, and MCP mutation paths as high risk.
- Update catalog metadata when adding, moving, or removing cataloged assets.
- If intentional changes are made under cataloged `skills/**`, refresh the skill manifest with `npm run manifest:write`.
- Run `npm run validate` before finishing.

## Cross-platform asset rule

This repo supports multiple harnesses without pretending they are identical.

- Keep portable logic in canonical/shared assets.
- Keep harness-specific behavior in the correct adapter format.
- Do not invent unsupported executable metadata fields.

## Read these when relevant

- `README.md`
- `AGENTS.md`
- `docs/compatibility.md`
- `docs/normalized-platform-matrix.md`

