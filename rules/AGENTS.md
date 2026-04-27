# AGENTS.md

## Purpose
- Store harness-specific always-on behavior rules.

## Patterns
- `rules/<harness>/<rule-id>.md` is the rule body.
- `rules/<harness>/<rule-id>.metadata.json` mirrors catalog fields.
- `catalog/rules.json` path must match the rule file.

## Rules
- Keep rules short, enforceable, and harness-specific.
- Do not duplicate full skill procedures here.
- Run `npm run validate` after rule metadata edits.

