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

## 👀 Read-only advisory agents

| Agent | Focus |
|---|---|
| `huawei-maestro-agent` | classify and route Huawei Cloud tasks; MLPS/sovereignty-aware |
| `huawei-iam-least-privilege-review-agent` | IAM fine-grained policies, SCP at org level, agencies (cross-account) |
| `huawei-secmaster-security-operations-agent` | SecMaster SIEM/SOAR, HSS, CFW, WAF, Anti-DDoS security posture |
| `huawei-dew-kms-lifecycle-steward-agent` | DEW (KMS + CSMS + CBH bastion host), key rotation, DBSS, secret lifecycle |
| `huawei-compliance-sovereignty-agent` | MLPS 2.0 Level 3, China data localization, Trusted Cloud cert, government cloud |
| `huawei-gaussdb-rds-dba-agent` | GaussDB for MySQL/PG/Oracle, RDS, DDS MongoDB-compatible, proxy, HA |
| `huawei-modelarts-mlops-engineer-agent` | ModelArts GPU/NPU cost governance, Pangu model deployment, AI Gallery |
| `huawei-resilience-bcdr-review-agent` | GaussDB HA failover, CBR cross-region vault, CCE multi-AZ, DRS DR, RTO/RPO |
| `huawei-iac-change-safety-review-agent` | Terraform/IaC blast radius review, deletion protection, plan-before-apply |
| `huawei-event-driven-architecture-review-agent` | DMS Kafka dead-letter, ROMA Connect, FunctionGraph idempotency, SMN retry |
| `huawei-load-balancer-traffic-engineer-agent` | Dedicated/Shared ELB type selection, health checks, WAF integration, TLS policy |
| `huawei-serverless-production-readiness-agent` | FunctionGraph VPC access, concurrency limits, cold-start, LTS observability |
| `huawei-certificate-manager-issuer-review-agent` | SCM certificate lifecycle, ELB SSL binding, expiry alerting, HTTPS enforcement |
| `huawei-cost-anomaly-watch-coordinator-agent` | CBC cost delta, budget alerts, ECS/GaussDB mode anomalies, EVS waste detection |
| `huawei-change-impact-advisor-agent` | pre-change blast radius: SCP cascade, VPC routing, GaussDB maintenance windows |
| `huawei-registry-artifact-governor-agent` | SWR namespace public exposure, VSS image scanning, tag immutability, supply chain |
| `huawei-ticket-triage-escalation-coordinator-agent` | P0/P1/P2/P3 triage, Premium Support SLA enforcement, AOM alert routing |
| `huawei-obs-data-perimeter-governor-agent` | OBS public ACL/policy exposure, Block Public Access, VPCEP binding, WORM review |
| `huawei-support-incident-coordinator-agent` | severity mapping 紧急/高/中/低, TAM escalation, evidence scrubbing, PIR rights |
| `huawei-daily-operations-briefing-coordinator-agent` | CBC cost delta, AOM alert owners, CCE multi-AZ health, SecMaster finding SLA |
| `huawei-cce-container-platform-operator-agent` | CCE cluster lifecycle, SWR image management, ASM traffic policies, IEF edge node integration |
| `huawei-codearts-devops-operator-agent` | CodeArts (CodeHub, Build, Deploy, TestPlan, Pipeline), SWR image lifecycle, release automation |
| `huawei-cost-finops-analyst-agent` | CBC spend analysis, RI/resource package optimization, Cost Center budgets, budget alert drift |
| `huawei-drs-data-replication-operator-agent` | DRS migration and real-time sync, CDM batch ETL, DMS Kafka cluster operations, safe cutover sequencing |
| `huawei-dws-dli-data-analyst-agent` | DWS (GaussDB DWS), DLI Spark/Flink, MRS, DataArts Studio governance and pipeline orchestration |
| `huawei-ecs-compute-operator-agent` | ECS lifecycle, AS group configuration, IMS custom images, DeH dedicated host tenancy, CSBS snapshots |
| `huawei-functiongraph-serverless-operator-agent` | FunctionGraph event triggers, reserved concurrency, cold-start tuning, ServiceStage, CSE microservice governance |
| `huawei-ief-edge-computing-operator-agent` | IEF edge node lifecycle, edge app deployment, IoT device twin management, cloud-edge-device unified control plane |
| `huawei-landing-zone-architect-agent` | Organizations SCP baseline, IAM fine-grained structure, Enterprise Projects governance, multi-account governance |
| `huawei-migration-architect-agent` | MgC, SMS server migration, DRS database replication, OMS object migration, cutover sequencing |
| `huawei-network-architect-agent` | VPC, ELB type selection (dedicated/shared), VPN/DC Gateway, Cloud Connect, CFW, Anti-DDoS, DNS |
| `huawei-obs-storage-steward-agent` | OBS lifecycle policies, bucket ACL/policy governance, SFS, EVS, CBR backup strategies |
| `huawei-observability-incident-responder-agent` | CES (Cloud Eye), LTS log analytics, AOM, APM, SMN incident response and observability setup |
| `huawei-solution-architect-agent` | product selection, Enterprise Project model, region/MLPS sovereignty requirements, multi-AZ/multi-region HA patterns |
| `huawei-waf-cost-optimization-review-agent` | ECS flavor selection (including Kunpeng Arm), billing mode optimization, Spot Instances, Enterprise Project cost attribution |
| `huawei-waf-reliability-review-agent` | AZ distribution, ELB load balancing, Auto Scaling, GaussDB/RDS multi-AZ HA, CBR data protection |
| `huawei-waf-security-review-agent` | IAM SCP governance, VPC isolation, DEW key management, SecMaster SIEM/SOAR, MLPS 2.0 technical controls |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- 🚦 guarded live operators must confirm account ID, region, project, principal, approval, rollback, and verification before mutation
- ⚠️ **Enterprise Projects** are billing/attribution constructs, not security boundaries — do not confuse them with IAM permission boundaries; always verify the IAM policy and SCP scope independently
- 🇨🇳 MLPS 2.0 Level 3 (GB/T 22239-2019) requires specific Huawei Cloud service configurations — `huawei-compliance-sovereignty-agent` flags gaps; live guards check residency before OBS/DEW mutations
- 🔮 ModelArts uses **Ascend NPU** flavors in addition to Nvidia GPUs — job configuration and cost model differ; confirm flavor before any training job submission
- 🧾 all live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)
