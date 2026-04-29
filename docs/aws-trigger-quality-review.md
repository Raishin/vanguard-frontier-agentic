# AWS Trigger-Quality Review

Date: 2026-04-29

## Executive verdict

The AWS portfolio was over-expanded to 26 skills. Static overlap analysis found one
clear duplicate that should not survive trigger testing:

- `aws-hybrid-multicloud-connectivity-review` overlapped strongly with
  `aws-network-architect` and did not have a distinct enough output contract.

It has been merged into `aws-network-architect` by expanding that skill's trigger
surface to cover Direct Connect, VPN, Cloud WAN, Route 53 Resolver, hybrid DNS,
CIDR/routing, segmentation, and failover.

## Pruned / merged

| Removed skill | Merged into | Reason |
|---|---|---|
| `aws-hybrid-multicloud-connectivity-review` | `aws-network-architect` | Same evidence path, same routing/DNS/connectivity output, high trigger collision risk. Separate skill would create noise. |

## Kept despite overlap

| Pair | Decision | Why |
|---|---|---|
| `aws-event-driven-architecture-review` / `aws-serverless-production-readiness` | Keep both | Event architecture owns contracts, buses, schemas, retries, DLQs, replay, cross-account routing. Serverless readiness owns Lambda runtime/deployment/concurrency. |
| `aws-compliance-evidence-mapper` / `aws-security-posture-hardening` | Keep both | Compliance mapper produces audit evidence packs; security posture produces remediation/hardening plan. |
| `aws-data-protection-backup-steward` / `aws-resilience-bcdr-review` | Keep both | Backup steward owns backup vault/restore mechanics; BCDR owns RTO/RPO/failover/game-day strategy. |
| `aws-rds-aurora-performance-investigator` / `aws-observability-incident-responder` | Keep both | RDS skill owns DB metrics/Performance Insights/query evidence; observability responder is broad incident triage. |
| `aws-iam-least-privilege-review` / `aws-kms-secrets-lifecycle-steward` | Keep both | IAM owns policy surgery; KMS/secrets owns cryptographic and secret lifecycle risk. |
| `aws-ecs-fargate-platform-operator` / `aws-eks-platform-operator` | Keep both | Different container control planes, evidence, IAM primitives, deployment behavior, and operational failure modes. |

## Description disambiguation applied

Descriptions were tightened for the highest-collision skills so trigger routing
has explicit "prefer X for Y" language. This affects frontmatter, which is the
most important trigger surface.

Updated routing boundaries include:

- `aws-solution-architect`: cross-domain architecture only; prefer narrower skills for single-domain asks.
- `aws-network-architect`: owns VPC/hybrid/multi-cloud connectivity; not API edge, S3, or security posture.
- `aws-serverless-production-readiness`: owns Lambda runtime/deployment readiness; not event-flow architecture.
- `aws-event-driven-architecture-review`: owns EventBridge/SNS/SQS/Step Functions design; not Lambda runtime readiness.
- `aws-security-posture-hardening`: broad CSPM/remediation; not audit evidence, IAM surgery, S3 perimeter, Bedrock, or KMS lifecycle.
- `aws-compliance-evidence-mapper`: audit evidence packs only; not general hardening.
- `aws-data-protection-backup-steward`: backup implementation; not BCDR strategy.
- `aws-resilience-bcdr-review`: RTO/RPO/failover strategy; not backup-vault mechanics.
- `aws-eks-platform-operator` vs `aws-ecs-fargate-platform-operator`: explicit EKS-only / ECS-only boundaries.
- `aws-rds-aurora-performance-investigator`: DB incidents; broad incidents stay with observability responder.

## Trigger eval artifact

Created:

- `.claude/evals/aws-trigger-quality-routing.json`

It contains 20 realistic routing prompts with expected skill and near-miss skills.

## Hard limitation

A real trigger benchmark needs the actual model/harness trigger decision. The
local `skill-comply` path depends on the `claude` CLI, which is unavailable in
this environment. So this pass is deterministic/static, not a real pass@k trigger
benchmark.

## Next benchmark command when CLI is available

```bash
cd <skill-comply-skill-directory>
uv run python -m scripts.run --dry-run \
  --output <repo>/.claude/evals/aws-trigger-quality-skill-comply.md \
  <repo>/skills/aws/aws-network-architect/SKILL.md
```

For full trigger benchmarking, use the JSON eval set with a harness that can
expose available skills and record which skill was selected.
