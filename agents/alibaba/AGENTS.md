# AGENTS.md

## Purpose
Store Alibaba Cloud marketplace agents with canonical identity and harness-specific variants.

## Patterns
- `agents/alibaba/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/alibaba/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/alibaba/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/alibaba/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/alibaba/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/alibaba/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/alibaba/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants; do not pretend IDE Markdown and CLI JSON are interchangeable.
- `agents/alibaba/<skill-id>-agent/metadata.json` mirrors `catalog/agents.json`.

## Alibaba Cloud Region and Compliance Notes

Alibaba Cloud operates **China mainland regions** (CN-*) and **international regions** (AP-*, EU-*, US-*) as distinct environments with different regulatory obligations:

- **China mainland regions** are subject to the PRC Cybersecurity Law, Data Security Law (DSL), Personal Information Protection Law (PIPL), and MLPS 2.0. Workloads here require ICP filing for internet-facing services and must meet MLPS 2.0 grading requirements.
- **International regions** follow the laws of the jurisdiction in which the region operates (e.g., Singapore MAS TRM, Germany GDPR, US SOC 2). Cross-border data transfers between China and international regions are restricted by DSL Article 31.

All agents serving China-based workloads must explicitly identify the target region and applicable regulatory framework before making recommendations.

## RAM and Resource Directory Notes

Alibaba Cloud identity has two levels that are frequently confused:
- **Resource Directory (RD)**: The org-level account hierarchy (master account → member accounts). Control Policy (SCP equivalent) is applied at the RD level.
- **RAM (Resource Access Management)**: Per-account IAM (users, groups, roles, policies, STS). RAM operates within a single account.

Cross-account access uses RAM role assumption + STS tokens. RAM AdministratorAccess is the most dangerous RAM policy — widespread assignment is the #1 Alibaba Cloud security debt pattern.

## Live Guard Agents

Six live-guard agents enforce approval gates and rollback posture for high-risk Alibaba Cloud mutations.

| Agent | Purpose | Skill |
|-------|---------|-------|
| [alibaba-live-ack-rollout-guard-agent](alibaba-live-ack-rollout-guard-agent/) | Gate ACK deployment mutations, node pool scaling, and cluster version upgrades against rollback posture | [alibaba-live-ack-rollout-guard](../../skills/alibaba/alibaba-live-ack-rollout-guard/) |
| [alibaba-live-ram-policy-change-guard-agent](alibaba-live-ram-policy-change-guard-agent/) | Gate RAM policy and role mutations — account-wide blast radius, privilege escalation risk | [alibaba-live-ram-policy-change-guard](../../skills/alibaba/alibaba-live-ram-policy-change-guard/) |
| [alibaba-live-kms-key-mutation-guard-agent](alibaba-live-kms-key-mutation-guard-agent/) | Gate KMS key deletion and disable operations — encrypted data becomes permanently inaccessible | [alibaba-live-kms-key-mutation-guard](../../skills/alibaba/alibaba-live-kms-key-mutation-guard/) |
| [alibaba-live-cost-budget-action-guard-agent](alibaba-live-cost-budget-action-guard-agent/) | Gate budget threshold changes, Savings Plan purchases, and RI commitments — financial authority gate | [alibaba-live-cost-budget-action-guard](../../skills/alibaba/alibaba-live-cost-budget-action-guard/) |
| [alibaba-live-oss-bucket-policy-guard-agent](alibaba-live-oss-bucket-policy-guard-agent/) | Gate OSS bucket ACL and policy changes — public exposure or access denial blast radius | [alibaba-live-oss-bucket-policy-guard](../../skills/alibaba/alibaba-live-oss-bucket-policy-guard/) |
| [alibaba-live-rds-polardb-mutation-guard-agent](alibaba-live-rds-polardb-mutation-guard-agent/) | Gate RDS/PolarDB instance deletion, spec downgrades, and backup policy removal — data loss risk | [alibaba-live-rds-polardb-mutation-guard](../../skills/alibaba/alibaba-live-rds-polardb-mutation-guard/) |

### Live guard permission model

Alibaba Cloud RAM uses policy statements with Effect/Action/Resource/Condition structure similar to AWS IAM.

- **RAM AdministratorAccess** is a system policy granting full account control — never auto-approved for any mutation.
- **STS AssumeRole** tokens have maximum 12-hour TTL; live-guard agents must confirm the active STS token expiry before long-running operations.
- **OSS bucket ACL = public-read or public-read-write** is irreversible in effect (data may be indexed by crawlers within seconds of exposure). Require explicit confirmation of public vs. private intent.
- **PolarDB/RDS deletion** with backup retention disabled is permanent. Always confirm backup policy status before any instance deletion.
- **China mainland OSS** cross-border data transfer requires data export compliance review under DSL Article 31 — flag before any replication configuration change.
