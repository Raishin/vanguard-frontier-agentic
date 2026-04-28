## EVAL DEFINITION: azure-entra-id-specialist

### Capability evals

1. The repo includes a dedicated Microsoft Entra ID specialist skill beyond the narrower identity-governance review role.
2. The skill uses progressive disclosure with lean `SKILL.md` and on-demand references.
3. The repo includes a matching Azure agent that binds the Entra specialist skill and preserves Azure MCP runtime-truth guidance.

### Regression evals

1. `catalog/skills.json` remains valid after adding the Entra specialist skill.
2. `catalog/agents.json` remains valid after adding the Entra specialist agent.
3. `catalog/skill-manifest.json` is refreshed after the skill addition.
4. Offline validation still passes.

### Deterministic checks

- Ensure `skills/azure/azure-entra-id-specialist/` contains `SKILL.md`, `metadata.json`, and the three reference files.
- Ensure `agents/azure/azure-entra-id-specialist-agent/` contains `AGENT.md`, `metadata.json`, `harnesses/codex.toml`, and `harnesses/copilot.agent.md`.
- Ensure the agent binds `skills/azure/azure-entra-id-specialist/SKILL.md`.
- Run `npm run manifest:write`.
- Run `npm run validate`.
