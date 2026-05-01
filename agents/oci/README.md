# 🟥 OCI Agents

<p align="center">
  <img src="../../assets/logos/cloud/oci/oracle-cloud-infrastructure.png" alt="Oracle Cloud Infrastructure logo" width="140" />
</p>

Oracle Cloud Infrastructure agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live OCI mutation |
|---|---|---|---|
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real OCI environments | workspace-write | approval-gated and target-confirmed only |

## 🚦 Guarded live-OCI operators

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `oci-live-autonomous-db-lifecycle-guard-agent` | ADB scale / stop / clone / terminate | tag enforcement + approval + rollback required | tag policy missing or lifecycle action irreversible without backup |
| `oci-live-oke-rollout-guard-agent` | live OKE rollout via DevOps pipelines | pipeline approval + PDB audit + rollout pause/undo | health signals are weak or circuit breaker is disabled |
| `oci-live-resource-manager-stack-guard-agent` | live Resource Manager stack apply | plan-before-apply + drift detection + job-lock | applying without a reviewed plan output |
| `oci-live-vault-key-destruction-guard-agent` | Vault key rotation vs. destruction | rotation-vs-destruction separation + 7–30 day window | key is in active use and destruction has no waiting period |
| `oci-live-iam-policy-compartment-guard-agent` | live IAM policy mutations on compartments | MFA break-glass + dual-approval for tenancy-root | tenancy-root changes without dual approval |
| `oci-live-cost-budget-runaway-guard-agent` | live budget threshold and ONS alert mutations | 3-tier budget baseline + GPU shape gate + ONS routing | budget action would remove cost alerts |
| `oci-live-network-security-rule-guard-agent` | Security List and NSG rule mutations | full current-rule capture + 0.0.0.0/0 detection + DB-subnet criticality | ingress rule opens database subnet to internet |

## 👀 Read-only advisory examples

| Agent | Focus |
|---|---|
| `oci-identity-access-governor-agent` | IAM policy, compartment scope, federation, least privilege |
| `oci-security-compliance-reviewer-agent` | Cloud Guard findings, CIS benchmarks, security posture |
| `oci-network-architect-agent` | VCN design, DRG, FastConnect, Private DNS topology |
| `oci-solution-architect-agent` | broad OCI architecture review and design guidance |
| `oci-observability-incident-responder-agent` | OCI Observability, Logging Analytics, incident triage |
| `oci-cost-finops-analyst-agent` | cost management, budgets, usage reports, reservation coverage |
| `oci-database-platform-dba-agent` | OCI Database, BaseDB, ExaDB operations and lifecycle |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- 🚦 guarded live operators must confirm tenancy OCID, compartment, target resource, approval, rollback, and verification before mutation
- ⚠️ `oci network security-list update` is a **full replace** — the guard always captures the complete current rule set before writing
- 🧾 all live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)
