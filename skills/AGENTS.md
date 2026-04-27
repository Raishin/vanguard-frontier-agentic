# AGENTS.md

## Purpose
- Store reusable task workflows grouped by provider or domain.

## Patterns
- `skills/<provider>/<skill-id>/SKILL.md` is the workflow entrypoint.
- `skills/<provider>/<skill-id>/metadata.json` mirrors `catalog/skills.json`.
- `references/` holds lazy-loaded deep context for hefty skills.
- OCI skills are consolidated under `skills/oci/`.

## Rules
- Use progressive disclosure; do not dump all references into `SKILL.md`.
- Keep cloud and compliance claims source-grounded.
- Never request secrets, wallets, credentials, tokens, or customer identifiers.
- Update `catalog/skills.json` when adding, moving, or removing a skill.
- Run `npm run manifest:write` after intentional skill edits.
- Run `npm run validate` before finishing.

## Do Not Miss
- Manifest drift means changed skill content; refresh only when intentional.
- New high-risk cloud skills need safety gates, rollback notes, and least-privilege posture.

