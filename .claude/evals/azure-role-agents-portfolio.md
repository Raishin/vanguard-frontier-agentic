## EVAL DEFINITION: azure-role-agents-portfolio

### Capability evals

1. Every Azure role-based skill has a matching Azure marketplace agent under `agents/azure/`.
2. Every Azure agent exposes the OCI-style split:
   - canonical `AGENT.md`
   - `harnesses/codex.toml`
   - `harnesses/copilot.agent.md`
   - `metadata.json`
3. Every Azure Codex harness binds exactly one Azure skill and enforces:
   - read-only default posture,
   - progressive disclosure,
   - Azure MCP runtime-truth guidance,
   - concise evidence-labeled outputs.
4. Every Azure Copilot harness points at the same Azure skill contract and preserves the same safety rules.

### Regression evals

1. `catalog/agents.json` remains valid after adding the Azure agent portfolio.
2. Existing OCI and Terraform agent entries remain intact.
3. Repo validation still passes after the Azure agent expansion.

### Deterministic checks

- Count Azure role-based skills and ensure there is one matching `-agent` folder per skill.
- Ensure every Azure agent folder contains `AGENT.md`, `metadata.json`, `harnesses/codex.toml`, and `harnesses/copilot.agent.md`.
- Ensure every Azure Codex harness binds the matching `skills/azure/<skill-id>/SKILL.md`.
- Ensure every Azure agent metadata file matches a catalog entry with the same `id`.
- Ensure `catalog/agents.json` includes 20 Azure agents and preserves non-Azure entries.
- Run `npm run validate`.
