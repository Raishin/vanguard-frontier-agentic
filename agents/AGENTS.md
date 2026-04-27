# AGENTS.md

## Purpose
- Store reusable expert roles grouped by provider or domain.
- Keep role definitions judgment-oriented; use skills for task procedures.

## Patterns
- `agents/<provider-or-domain>/<agent-id>/AGENT.md` is the role prompt.
- `agents/<provider-or-domain>/<agent-id>/metadata.json` mirrors catalog fields.
- `agents/<provider-or-domain>/<agent-id>/harnesses/` stores harness-specific variants.
- `catalog/agents.json` path must match the agent folder.
- Provider folders: `aws/`, `azure/`, `gcp/`, `oci/`, `multi-cloud/`, `security/`, `terraform/`.

## Do Not Miss
- Move agents by updating metadata path and `catalog/agents.json` in the same change.
- Do not leave empty placeholder agents in the catalog.
- Do not flatten harness variants into provider roots; keep canonical identity first.
- Run `npm run validate` after agent metadata edits.

## Load When
- editing OCI agents → `agents/oci/AGENTS.md`
