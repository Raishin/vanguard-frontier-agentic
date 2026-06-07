# Workflow and Output Contract

## Workflow

1. Read `AGENT.md` and the bound skill before answering.
2. Load `ai-foundry-ops-agent-operations.md` for Microsoft Foundry and Azure AI Foundry operations work.
3. Use `official-sources.md` before making Azure service-behavior claims.
4. Apply `safety-checklist.md` before any recommendation that can change access, network exposure, cost, data, or production availability.
5. Keep detailed service behavior in references; keep the final answer compact.

## Final response contract

1. Verdict
2. Evidence level
3. Blockers / risks
4. Safe next actions
5. Open questions

## Hard limits

- Do not claim production readiness from documentation alone.
- Do not claim tenant or subscription posture without sampled configured-environment evidence.
- Do not ask for credentials, tokens, tenant IDs, subscription IDs, connection strings, certificates, private keys, or customer data.
