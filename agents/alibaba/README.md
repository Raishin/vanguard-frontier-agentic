# 🟠 Alibaba Cloud Agents

Alibaba Cloud agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Alibaba Cloud mutation |
|---|---|---|---|
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real Alibaba Cloud environments | workspace-write | approval-gated and target-confirmed only |

## 🚦 Guarded live-Alibaba Cloud operators

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `alibaba-live-ack-rollout-guard-agent` | live ACK deployment mutations and node pool scaling | health signals + PDB audit + rollback required | cluster version upgrade or node pool deletion lacks preflight evidence |
| `alibaba-live-ram-policy-change-guard-agent` | live RAM policy and role mutations | current policy snapshot + privilege-escalation check + approval required | account-wide policy change lacks dual approval or Resource Directory scope is unclear |
| `alibaba-live-kms-key-mutation-guard-agent` | KMS key deletion and disable operations | key usage audit + rotation-vs-deletion separation + waiting period | key is actively used for envelope encryption and no backup key exists |
| `alibaba-live-cost-budget-action-guard-agent` | live budget threshold changes and Savings Plan purchases | budget baseline + alert thresholds + RI/SP commitment review | budget action removes cost controls or Savings Plan purchase lacks financial approval |
| `alibaba-live-oss-bucket-policy-guard-agent` | OSS bucket ACL and policy mutations | current ACL/policy capture + public-exposure check + MLPS 2.0 residency review | policy change would make bucket public or violate China data-residency requirements |
| `alibaba-live-rds-polardb-mutation-guard-agent` | RDS and PolarDB instance deletion and spec changes | instance snapshot + backup policy check + downgrade impact review | instance deletion has no recent backup or spec downgrade breaches SLA |

## 👀 Read-only advisory examples

| Agent | Focus |
|---|---|
| `alibaba-maestro-agent` | classify and route Alibaba Cloud tasks; China-region aware |
| `alibaba-ram-iam-review-agent` | RAM users/groups/roles/policies, STS token lifecycle, Resource Directory |
| `alibaba-security-center-hardening-agent` | Security Center, WAF, Anti-DDoS Pro, Cloud Firewall posture hardening |
| `alibaba-cost-finops-analyst-agent` | Cost Manager, Savings Plans, Reserved Instances, budget drift |
| `alibaba-china-compliance-agent` | MLPS 2.0 Level 3, DSL, PIPL, ICP filing, data residency controls |
| `alibaba-polardb-rds-dba-agent` | PolarDB MySQL/PG/Oracle, RDS, DAS autonomous, Global Database Network |
| `alibaba-ack-container-platform-operator-agent` | ACK managed/dedicated/serverless, ACR, ASM Service Mesh |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- 🚦 guarded live operators must confirm account ID, region (CN-* vs international), principal, approval, rollback, and verification before mutation
- ⚠️ China mainland regions (cn-hangzhou, cn-beijing, etc.) and international regions have **separate billing accounts and different regulatory scope** — always confirm which context applies before any cost or compliance action
- 🇨🇳 MLPS 2.0 Level 3 mandates specific Alibaba Cloud service configurations — `alibaba-china-compliance-agent` flags gaps; live guards check residency before OSS/KMS mutations
- 🧾 all live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)
