# AGENTS.md

## Purpose
Store Huawei Cloud marketplace agents with canonical identity and harness-specific variants.

## Patterns
- `agents/huawei/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/huawei/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/huawei/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/huawei/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/huawei/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/huawei/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/huawei/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants; do not pretend IDE Markdown and CLI JSON are interchangeable.
- `agents/huawei/<skill-id>-agent/metadata.json` mirrors `catalog/agents.json`.

## Huawei Cloud IAM and Compliance Notes

Huawei Cloud uses a two-level identity model:
- **IAM** (Identity and Access Management): manages users, user groups, agencies (cross-account), and fine-grained authorization within an account.
- **Organizations + SCP** (Service Control Policies): applied at the organizational unit level, these are guardrails that limit what IAM policies can permit within member accounts — equivalent to AWS SCPs.
- **Enterprise Projects**: a resource grouping mechanism (orthogonal to accounts) used for cost allocation, access control, and quota management. Not equivalent to AWS accounts; one Huawei Cloud account can contain many enterprise projects.

All agents targeting production workloads must identify the account, enterprise project, and region before recommending changes. SCP-level restrictions take precedence over IAM policies.

## MLPS 2.0 Compliance Context

Workloads operated in Huawei Cloud's China mainland regions are subject to MLPS 2.0 (GB/T 22239-2019). Level 3 requirements include:
- **Identity audit**: All privileged logins must be logged (LTS) with failed-login alerting (CES/SMN).
- **Boundary protection**: CFW (Cloud Firewall) with IPS enabled; network access control between security domains.
- **Intrusion detection**: HSS (Host Security Service) Agent installed on all ECS instances with automatic isolation enabled.
- **Data backup**: CBR policies with offsite backup copy and tested restore procedures.
- **Key management**: DEW/KMS for encryption at rest; CSMS for secrets; CBH for privileged access sessions.

Agents operating on MLPS-scoped workloads must flag gaps against these requirements and refuse to recommend configurations that would increase non-compliance exposure.

## Live Guard Agents

Six live-guard agents enforce approval gates and rollback posture for high-risk Huawei Cloud mutations.

| Agent | Purpose | Skill |
|-------|---------|-------|
| [huawei-live-cce-rollout-guard-agent](huawei-live-cce-rollout-guard-agent/) | Gate CCE deployment mutations, node pool upgrades, and cluster version changes against rollback posture | [huawei-live-cce-rollout-guard](../../skills/huawei/huawei-live-cce-rollout-guard/) |
| [huawei-live-iam-policy-change-guard-agent](huawei-live-iam-policy-change-guard-agent/) | Gate IAM policy and SCP mutations — account-wide blast radius, privilege escalation risk | [huawei-live-iam-policy-change-guard](../../skills/huawei/huawei-live-iam-policy-change-guard/) |
| [huawei-live-kms-key-destruction-guard-agent](huawei-live-kms-key-destruction-guard-agent/) | Gate DEW/KMS key deletion — CSMS secrets and DBSS-encrypted database data become permanently lost | [huawei-live-kms-key-destruction-guard](../../skills/huawei/huawei-live-kms-key-destruction-guard/) |
| [huawei-live-cost-budget-action-guard-agent](huawei-live-cost-budget-action-guard-agent/) | Gate budget threshold changes, RI purchases, and CUD commitments — financial authority gate | [huawei-live-cost-budget-action-guard](../../skills/huawei/huawei-live-cost-budget-action-guard/) |
| [huawei-live-obs-bucket-policy-guard-agent](huawei-live-obs-bucket-policy-guard-agent/) | Gate OBS bucket ACL and policy changes — public exposure or data residency violation | [huawei-live-obs-bucket-policy-guard](../../skills/huawei/huawei-live-obs-bucket-policy-guard/) |
| [huawei-live-gaussdb-mutation-guard-agent](huawei-live-gaussdb-mutation-guard-agent/) | Gate GaussDB/RDS instance deletion, spec downgrades, and backup policy changes — data loss and MLPS compliance risk | [huawei-live-gaussdb-mutation-guard](../../skills/huawei/huawei-live-gaussdb-mutation-guard/) |

### Live guard permission model

Huawei Cloud IAM uses a policy statement model with Effect/Action/Resource structure.

- **FullAccess system policies** (e.g., `ECS FullAccess`) grant complete service control — never auto-approved.
- **Agencies** are cross-account trust relationships. An agency granting `Security Administrator` is the most dangerous trust — requires explicit financial-authority approval.
- **SCP deny statements** cascade down to all member accounts and cannot be overridden by IAM policies — misconfigured SCPs can silently block all operations in an account.
- **GaussDB/RDS deletion** without automated backup is permanent. Confirm CBR policy status and last successful restore test before any deletion.
- **OBS bucket policy = public** triggers data residency violation in MLPS-scoped workloads — flag immediately and block.
- **KMS key deletion** schedules a 7-day pending period minimum. Once deleted, CSMS secrets and DBSS encryption keys are permanently unrecoverable — require dual sign-off.
