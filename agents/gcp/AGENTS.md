# AGENTS.md

## Purpose
Store GCP marketplace agents with canonical identity and harness-specific variants.

## Patterns
- `agents/gcp/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/gcp/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/gcp/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/gcp/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/gcp/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/gcp/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/gcp/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants; do not pretend IDE Markdown and CLI JSON are interchangeable.
- `agents/gcp/<skill-id>-agent/metadata.json` mirrors `catalog/agents.json`.

## GCP IAM and Resource Hierarchy Notes

GCP uses a resource hierarchy: **Organization → Folder → Project**. IAM policy inheritance flows downward — a binding at org level applies to all folders and projects beneath it. Agents must always clarify which level of the hierarchy a change targets before recommending or approving mutations.

Key GCP security invariants all agents must respect:
- **Service Account keys** are long-lived credentials; key sprawl is the #1 GCP breach vector. Prefer Workload Identity Federation or impersonation over key files.
- **VPC is global** (not per-region); a single VPC spans all GCP regions. Shared VPC host/service project separation must be understood before any network change.
- **VPC Service Controls dry-run mode** must always precede enforcement — live enforcement can silently block legitimate API calls and is difficult to debug.
- **BigQuery on-demand pricing** charges per byte scanned — no cost cap by default. A single full-scan query on a large table can incur hundreds of dollars.

## Live Guard Agents

Six live-guard agents enforce approval gates and rollback posture for high-risk GCP mutations.

| Agent | Purpose | Skill |
|-------|---------|-------|
| [gcp-live-gke-rollout-guard-agent](gcp-live-gke-rollout-guard-agent/) | Gate GKE deployment mutations, node pool upgrades, and cluster control-plane version changes against rollback posture and PDB audit | [gcp-live-gke-rollout-guard](../../skills/gcp/gcp-live-gke-rollout-guard/) |
| [gcp-live-iam-policy-change-guard-agent](gcp-live-iam-policy-change-guard-agent/) | Gate IAM binding mutations, org policy changes, and Service Account key creation — org-wide blast radius, cannot be undone without audit trail | [gcp-live-iam-policy-change-guard](../../skills/gcp/gcp-live-iam-policy-change-guard/) |
| [gcp-live-kms-key-destruction-guard-agent](gcp-live-kms-key-destruction-guard-agent/) | Gate Cloud KMS key version destruction and key ring deletion — CMEK-encrypted data becomes permanently unrecoverable | [gcp-live-kms-key-destruction-guard](../../skills/gcp/gcp-live-kms-key-destruction-guard/) |
| [gcp-live-cost-budget-action-guard-agent](gcp-live-cost-budget-action-guard-agent/) | Gate budget threshold changes, committed-use discount purchases, and quota increase requests — financial authority confirmation required | [gcp-live-cost-budget-action-guard](../../skills/gcp/gcp-live-cost-budget-action-guard/) |
| [gcp-live-bigquery-dataset-deletion-guard-agent](gcp-live-bigquery-dataset-deletion-guard-agent/) | Gate BigQuery dataset deletion, table truncation, and authorized view changes — irreversible data loss risk | [gcp-live-bigquery-dataset-deletion-guard](../../skills/gcp/gcp-live-bigquery-dataset-deletion-guard/) |
| [gcp-live-cloud-run-traffic-migration-guard-agent](gcp-live-cloud-run-traffic-migration-guard-agent/) | Gate Cloud Run traffic percentage migrations, min-instances changes, and revision deletions — production traffic blast radius | [gcp-live-cloud-run-traffic-migration-guard](../../skills/gcp/gcp-live-cloud-run-traffic-migration-guard/) |

### Live guard permission model

GCP IAM requires explicit IAM bindings at the correct resource hierarchy level. Key principles:

- **Least-privilege roles**: Prefer predefined roles (e.g., `roles/container.developer`) over primitive roles (`roles/editor`). Never grant `roles/owner` to service accounts.
- **Separation of duties**: Live-guard agents require the human operator to hold a narrowly scoped role (e.g., `roles/iam.securityAdmin`) confirmed via `gcloud auth list` or equivalent before any mutation.
- **VPC Service Controls**: Changes to access policies or perimeters require `roles/accesscontextmanager.policyAdmin` at org level — one of the most powerful roles in GCP.
- **KMS irreversibility**: Key version destruction schedules a 24-hour minimum pending period. Once destroyed, CMEK-encrypted data is permanently inaccessible. Confirm dependent resources before scheduling.
- **BigQuery dataset deletion**: Datasets with expiration unset have no automatic cleanup — deletion is immediate and unrecoverable without a backup export.
