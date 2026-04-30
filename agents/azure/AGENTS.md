# AGENTS.md

## Purpose
- Store Azure marketplace agents with canonical identity and harness-specific variants.

## Patterns
- `agents/azure/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/azure/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/azure/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/azure/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/azure/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/azure/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/azure/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants; do not pretend IDE Markdown and CLI JSON are interchangeable.
- `agents/azure/<skill-id>-agent/metadata.json` mirrors agent metadata beside the asset and aligns with `catalog/agents.json`.

## Live Guard Agents

Six live-guard agents enforce approval gates and rollback posture for high-risk Azure mutations.
Each live-guard agent requires explicit confirmation of subscription, resource group, and principal
before any mutation, and treats missing rollback design as a stop condition.

| Agent | Purpose | Skill |
|-------|---------|-------|
| [azure-live-aks-rollout-guard-agent](azure-live-aks-rollout-guard-agent/) | Guard AKS deployment rollouts: PDB audit, maxUnavailable/surge validation, rollout pause/undo gates, post-rollout health verification | [azure-live-aks-rollout-guard](../../skills/azure/azure-live-aks-rollout-guard/) |
| [azure-live-app-service-slot-swap-guard-agent](azure-live-app-service-slot-swap-guard-agent/) | Guard App Service slot swaps: sticky-setting audit, traffic shifting, swap-back rollback path | [azure-live-app-service-slot-swap-guard](../../skills/azure/azure-live-app-service-slot-swap-guard/) |
| [azure-live-arm-deployment-stack-guard-agent](azure-live-arm-deployment-stack-guard-agent/) | Guard ARM/Bicep deployments and Deployment Stacks: what-if evidence, denySettings review, changeset diff, rollback posture, PIM-gated stack deletion | [azure-live-arm-deployment-stack-guard](../../skills/azure/azure-live-arm-deployment-stack-guard/) |
| [azure-live-cost-budget-action-guard-agent](azure-live-cost-budget-action-guard-agent/) | Guard cost budget actions and GPU quota gates: budget mutation with spend verification, GPU SKU policy enforcement, quota read (no write) | [azure-live-cost-budget-action-guard](../../skills/azure/azure-live-cost-budget-action-guard/) |
| [azure-live-keyvault-rotation-purge-guard-agent](azure-live-keyvault-rotation-purge-guard-agent/) | Guard Key Vault key/secret rotation and purge: rotation policy review, soft-delete and purge-protection verification, PIM-gated purge-enable | [azure-live-keyvault-rotation-purge-guard](../../skills/azure/azure-live-keyvault-rotation-purge-guard/) |
| [azure-live-pim-jit-activation-guard-agent](azure-live-pim-jit-activation-guard-agent/) | Guard PIM JIT activation: eligible assignment audit, MFA and justification gate, activation deactivation and emergency revocation | [azure-live-pim-jit-activation-guard](../../skills/azure/azure-live-pim-jit-activation-guard/) |

### Live guard permission model

All six live-guard agents use custom Azure RBAC roles scoped to the minimum required resource. Key principles:

- Operations that cannot be reversed (stack delete, slot delete, key purge) are placed in `NotActions` of the operational role and require a separate PIM-eligible role.
- GPU quota write is excluded from the cost guard role — increases go through an approval workflow.
- Key Vault purge-protection enablement is a one-way door and requires PIM activation with justification.
- PIM JIT roles are eligible-only (never standing active), MFA-gated, and time-bounded.

See each agent's `PERMISSIONS.md` and `../../skills/azure/<skill-id>/references/permission-model.md` for full RBAC definitions.

## Rules
- Keep skill links pointed at `skills/azure/<skill-id>/SKILL.md`.
- Keep agent catalog IDs suffixed with `-agent` to avoid colliding with skill IDs.
- Keep prompts role-first and token-lean; load skill references only on demand.
- Treat Azure MCP runtime exposure as truth; do not hard-code undocumented namespace assumptions into the agent contract.
- When discussing Azure MCP setup, prefer Microsoft-documented consolidated mode guidance for AI agents, but adapt to the tools actually exposed in the active client.
- Run `npm run validate` after changes.
