# AGENTS.md

## Purpose
- Store JSON Schema contracts for catalog metadata and manifests.

## Rules
- Schema-required-field changes are breaking for npm consumers.
- Keep schema enums aligned with validator constants in `tests/validate-catalog.py`.
- Update `docs/release-versioning.md` if schema compatibility policy changes.
- Run `npm run validate` after schema edits.

## Schemas

- `skill.frontmatter.schema.json` — required SKILL.md frontmatter contract;
  enforced by `tests/validate-skill-frontmatter-schema.py` (`validate:skill-schema`).
- `agent.schema.json` — agent `metadata.json` contract (id, provider, harnesses,
  source_type, official_docs, security_notes, last_verified, companion_skills).
- `agent.frontmatter.schema.json` — required AGENT.md frontmatter contract.
  Empirically derived from the 141-file corpus: only `metadata.author` and
  `metadata.version` are required. Optional fields (`name`, `description`,
  `model`, `allowed-tools`, `tools`, `color`) are typed when present, and
  `additionalProperties: true` keeps harness-specific extensions non-breaking.
  Enforced by `tests/validate-agent-frontmatter-schema.py`
  (`validate:agent-schema`).

