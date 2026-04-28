## EVAL DEFINITION: azure-cosmosdb-split-skills-and-agents

### Capability evals

1. The repo includes separate Azure Cosmos DB platform and application-development skills.
2. Each skill has a matching Azure agent with OCI-style harness layout.
3. The split avoids the previous combined role and keeps both roles token-lean with on-demand references.

### Regression evals

1. `catalog/skills.json` remains valid after removing the combined role and adding the split roles.
2. `catalog/agents.json` remains valid after removing the combined role and adding the split roles.
3. `catalog/skill-manifest.json` is refreshed after the skill changes.
4. Offline validation still passes.

### Deterministic checks

- Ensure `skills/azure/azure-cosmosdb-platform-operator/` contains `SKILL.md`, `metadata.json`, and the three reference files.
- Ensure `skills/azure/azure-cosmosdb-application-developer/` contains `SKILL.md`, `metadata.json`, and the three reference files.
- Ensure matching agent folders exist for both roles with `AGENT.md`, `metadata.json`, `harnesses/codex.toml`, and `harnesses/copilot.agent.md`.
- Ensure both agents bind the correct skill path.
- Ensure the old combined role is absent from catalogs.
- Run `npm run manifest:write`.
- Run `npm run validate`.
