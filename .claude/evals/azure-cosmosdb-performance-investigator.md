## EVAL DEFINITION: azure-cosmosdb-performance-investigator

### Capability evals

1. The repo includes a dedicated Azure Cosmos DB performance-investigation skill.
2. The skill includes a detailed step-by-step profiling reference beyond the normal three reference files.
3. The repo includes a matching Azure agent that binds the performance skill and preserves Azure MCP runtime-truth guidance.

### Regression evals

1. `catalog/skills.json` remains valid after adding the performance skill.
2. `catalog/agents.json` remains valid after adding the performance agent.
3. `catalog/skill-manifest.json` is refreshed after the skill addition.
4. Offline validation still passes.

### Deterministic checks

- Ensure `skills/azure/azure-cosmosdb-performance-investigator/` contains `SKILL.md`, `metadata.json`, and the required reference files.
- Ensure `references/data-profiling-playbook.md` exists and is step-by-step.
- Ensure the matching agent folder contains `AGENT.md`, `metadata.json`, `harnesses/codex.toml`, and `harnesses/copilot.agent.md`.
- Ensure the agent binds `skills/azure/azure-cosmosdb-performance-investigator/SKILL.md`.
- Run `npm run manifest:write`.
- Run `npm run validate`.
