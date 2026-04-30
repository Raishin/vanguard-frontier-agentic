# AGENTS.md

## Purpose
- Store OCI marketplace agents with canonical identity and harness-specific variants.

## Patterns
- `agents/oci/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/oci/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/oci/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/oci/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/oci/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/oci/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/oci/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants; do not pretend IDE Markdown and CLI JSON are interchangeable.
- `agents/oci/<skill-id>-agent/metadata.json` mirrors `catalog/agents.json`.

## Live Guard Agents

Six live-guard agents enforce approval gates and rollback posture for high-risk OCI mutations.
OCI is a policy-based IAM system — all service principals and managed services require explicit
`Allow service <name>` grants in addition to human operator grants. Each live-guard agent
requires explicit tenancy, compartment, and active principal confirmation before any mutation.

| Agent | Purpose | Skill |
|-------|---------|-------|
| [oci-live-autonomous-db-lifecycle-guard-agent](oci-live-autonomous-db-lifecycle-guard-agent/) | Guard Autonomous Database lifecycle changes — scale, start, stop, clone, terminate — with protection-tag enforcement and backup verification | [oci-live-autonomous-db-lifecycle-guard](../../skills/oci/oci-live-autonomous-db-lifecycle-guard/) |
| [oci-live-cost-budget-runaway-guard-agent](oci-live-cost-budget-runaway-guard-agent/) | Guard cost budget runaway: 3-tier budget management (auditor/operator/admin), GPU shape gate via compartment quota, ONS topic alert routing | [oci-live-cost-budget-runaway-guard](../../skills/oci/oci-live-cost-budget-runaway-guard/) |
| [oci-live-iam-policy-compartment-guard-agent](oci-live-iam-policy-compartment-guard-agent/) | Guard IAM policy changes: 3-tier policy management with MFA-TOTP break-glass, name-pattern restrictions on dynamic groups, dual-approval for tenancy-root changes | [oci-live-iam-policy-compartment-guard](../../skills/oci/oci-live-iam-policy-compartment-guard/) |
| [oci-live-oke-rollout-guard-agent](oci-live-oke-rollout-guard-agent/) | Guard OKE rollout operations: DevOps pipeline approval stage enforcement, PDB audit, rollout pause/undo, node pool rollback, service-principal policy verification | [oci-live-oke-rollout-guard](../../skills/oci/oci-live-oke-rollout-guard/) |
| [oci-live-resource-manager-stack-guard-agent](oci-live-resource-manager-stack-guard-agent/) | Guard Resource Manager stack operations: plan-before-apply enforcement, drift detection, 3-tier operator model, service-principal policies for ResourceManager service | [oci-live-resource-manager-stack-guard](../../skills/oci/oci-live-resource-manager-stack-guard/) |
| [oci-live-vault-key-destruction-guard-agent](oci-live-vault-key-destruction-guard-agent/) | Guard Vault key destruction: rotation vs. destruction separation, deletion-window enforcement (7–30 day minimum), tag-condition gate, dependent-resource impact analysis | [oci-live-vault-key-destruction-guard](../../skills/oci/oci-live-vault-key-destruction-guard/) |

### Live guard permission model

OCI policy-based IAM requires explicit grants for every principal type. Key principles:

- **3-tier verb model**: auditor (inspect/read) / operator (use) / admin (manage) — never skip tiers.
- **Service principals**: `Allow service OKE`, `Allow service ResourceManager`, `Allow service devops` are required for managed services to act on tenancy resources — absence causes `NotAuthorized` even when human operators are correctly scoped.
- **Tag conditions**: production resources carry defined-tag namespace tags (`Operations.Lifecycle = protected`, `Lifecycle.Deletable = approved`) set in protected namespaces. Admins may only manage-verb when tag conditions are met.
- **IAM break-glass**: `<iam-tenancy-admins>` group is empty by default. Members are added only during approved change windows with MFA-TOTP verification enforced at policy-evaluation time.
- **ADB/Vault irreversibility**: termination and key deletion are permanent — tag-condition gates are necessary but not sufficient; both require dual-sign-off and a confirmed maintenance window.

See each agent's `PERMISSIONS.md` and `../../skills/oci/<skill-id>/references/permission-model.md` for full IAM policy statements.

## Rules
- Keep skill links pointed at `skills/oci/<skill-id>/SKILL.md`.
- Keep agent catalog IDs suffixed with `-agent` to avoid colliding with skill IDs.
- Do not create separate `agents/oci/codex/` or `agents/oci/copilot/` silos.
- Run `npm run validate` after changes.
