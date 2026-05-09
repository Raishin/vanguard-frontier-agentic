# 🔴 Huawei Cloud Agents

<p align="center">
  <img src="../../assets/logos/cloud/huawei/huawei-cloud.svg" alt="Huawei Cloud logo" width="140" />
</p>

Huawei Cloud agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Huawei Cloud mutation |
|---|---|---|---|
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real Huawei Cloud environments | workspace-write | approval-gated and target-confirmed only |

## 🚦 Guarded live-Huawei Cloud operators

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `huawei-live-cce-rollout-guard-agent` | live CCE deployment mutations and node pool upgrades | health signals + PDB audit + rollback required | cluster version upgrade or control-plane change lacks preflight evidence |
| `huawei-live-iam-policy-change-guard-agent` | live IAM policy and SCP mutations | current policy snapshot + SCP scope review + approval required | org-level SCP change lacks dual approval or Enterprise Project boundary is unclear |
| `huawei-live-kms-key-destruction-guard-agent` | DEW/KMS key deletion and CSMS secret destruction | key usage audit + DBSS dependency check + scheduled-deletion window | key is used for CMEK-encrypted GaussDB or OBS bucket with no backup key |
| `huawei-live-cost-budget-action-guard-agent` | live budget threshold changes and Yearly/Monthly subscription commits | budget baseline + alert thresholds + RI commitment review | budget action removes cost controls or subscription commit lacks financial approval |
| `huawei-live-obs-bucket-policy-guard-agent` | OBS bucket ACL and policy mutations | current ACL/policy capture + public-exposure check + MLPS 2.0 data-residency review | policy change would make bucket public or violate China data-localization requirements |
| `huawei-live-gaussdb-mutation-guard-agent` | GaussDB and RDS instance deletion and spec downgrade | instance snapshot + CBR backup check + HA topology review | instance deletion has no recent CBR backup or spec downgrade breaches SLA |

## 👀 Read-only advisory examples

| Agent | Focus |
|---|---|
| `huawei-maestro-agent` | classify and route Huawei Cloud tasks; MLPS/sovereignty-aware |
| `huawei-iam-least-privilege-review-agent` | IAM fine-grained policies, SCP at org level, agencies (cross-account) |
| `huawei-secmaster-security-operations-agent` | SecMaster SIEM/SOAR, HSS, CFW, WAF, Anti-DDoS security posture |
| `huawei-dew-kms-lifecycle-steward-agent` | DEW (KMS + CSMS + CBH bastion host), key rotation, DBSS, secret lifecycle |
| `huawei-compliance-sovereignty-agent` | MLPS 2.0 Level 3, China data localization, Trusted Cloud cert, government cloud |
| `huawei-gaussdb-rds-dba-agent` | GaussDB for MySQL/PG/Oracle, RDS, DDS MongoDB-compatible, proxy, HA |
| `huawei-modelarts-mlops-engineer-agent` | ModelArts GPU/NPU cost governance, Pangu model deployment, AI Gallery |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- 🚦 guarded live operators must confirm account ID, region, project, principal, approval, rollback, and verification before mutation
- ⚠️ **Enterprise Projects** are billing/attribution constructs, not security boundaries — do not confuse them with IAM permission boundaries; always verify the IAM policy and SCP scope independently
- 🇨🇳 MLPS 2.0 Level 3 (GB/T 22239-2019) requires specific Huawei Cloud service configurations — `huawei-compliance-sovereignty-agent` flags gaps; live guards check residency before OBS/DEW mutations
- 🔮 ModelArts uses **Ascend NPU** flavors in addition to Nvidia GPUs — job configuration and cost model differ; confirm flavor before any training job submission
- 🧾 all live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)
