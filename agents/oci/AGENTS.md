# AGENTS.md

## Purpose
- Store OCI marketplace agents with canonical identity and harness-specific variants.

## Patterns
- `agents/oci/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/oci/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/oci/<skill-id>-agent/harnesses/copilot.agent.md` is the Copilot variant.
- `agents/oci/<skill-id>-agent/metadata.json` mirrors `catalog/agents.json`.

## Rules
- Keep skill links pointed at `skills/oci/<skill-id>/SKILL.md`.
- Keep agent catalog IDs suffixed with `-agent` to avoid colliding with skill IDs.
- Add future `kiro` or `claude` variants under the same agent folder.
- Do not create separate `agents/oci/codex/` or `agents/oci/copilot/` silos.
- Run `npm run validate` after changes.
