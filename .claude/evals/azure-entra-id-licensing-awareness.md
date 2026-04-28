## EVAL DEFINITION: azure-entra-id-licensing-awareness

### Capability evals

1. The Azure Entra ID specialist skill includes an explicit licensing-awareness path for Microsoft Entra features.
2. The skill provides on-demand licensing references with concrete examples for Microsoft Azure, Microsoft 365, and Microsoft Fabric.
3. The matching agent contract warns against assuming feature availability without checking licensing prerequisites.
4. The skill includes an adjacent-service expansion path for Intune, Defender/Purview, and Agent ID style questions.
5. The Codex harness explicitly tells the agent to learn adjacent Microsoft services from official references before concluding.

### Regression evals

1. `catalog/skill-manifest.json` is refreshed after the Entra specialist skill changes.
2. Offline validation still passes after the licensing-aware updates.

### Deterministic checks

- Ensure `skills/azure/azure-entra-id-specialist/references/licensing-and-service-entitlements.md` exists.
- Ensure `skills/azure/azure-entra-id-specialist/references/adjacent-service-expansion.md` exists.
- Ensure `skills/azure/azure-entra-id-specialist/SKILL.md` links to the licensing reference.
- Ensure `skills/azure/azure-entra-id-specialist/SKILL.md` links to the adjacent-service expansion reference.
- Ensure `agents/azure/azure-entra-id-specialist-agent/AGENT.md` includes a licensing-awareness operating rule.
- Ensure `agents/azure/azure-entra-id-specialist-agent/harnesses/codex.toml` tells the agent to learn adjacent Microsoft services from official references before answering.
- Run `npm run manifest:write`.
- Run `npm run validate`.
