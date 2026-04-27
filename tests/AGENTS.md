# AGENTS.md

## Purpose
- Store deterministic validators for catalog, links, and skill integrity.

## Commands
- `python3 tests/validate-catalog.py`
- `python3 tests/validate-skill-manifest.py`
- `python3 tests/validate-skill-manifest.py --write`
- `python3 tests/validate-links.py --offline`
- `python3 tests/validate-links.py`

## Rules
- Prefer deterministic code graders over model judgment.
- Keep validators dependency-free unless package metadata changes.
- Keep validator constants aligned with `schemas/*.schema.json`.
- Do not weaken secret scanning to make a noisy file pass.
