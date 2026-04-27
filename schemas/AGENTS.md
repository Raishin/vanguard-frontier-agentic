# AGENTS.md

## Purpose
- Store JSON Schema contracts for catalog metadata and manifests.

## Rules
- Schema-required-field changes are breaking for npm consumers.
- Keep schema enums aligned with validator constants in `tests/validate-catalog.py`.
- Update `docs/release-versioning.md` if schema compatibility policy changes.
- Run `npm run validate` after schema edits.

