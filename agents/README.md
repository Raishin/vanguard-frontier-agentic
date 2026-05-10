# Agents

Role definitions for repeatable review, architecture, operations, and bounded execution work.

## Provider catalog

| Provider | Current status | Agents | Notes |
| --- | --- | ---: | --- |
| 🟧 AWS | active | 47 | advisory, repo-write execution, and guarded live-AWS operator agents |
| 🟥 OCI | active | 39 | advisory and guarded live-OCI operator agents |
| 🟩 GCP | active | 39 | advisory, live-guard operators, maestro router |
| 🟦 Azure | active | 36 | advisory and guarded live-Azure operator agents |
| 🟠 Alibaba Cloud | active | 30 | advisory, live-guard operators, maestro router |
| 🔴 Huawei Cloud | active | 30 | advisory, live-guard operators, maestro router |
| ☁️ OVHcloud | active | 6 | maestro router, IAM, FinOps, MCK, network architect, KMS live-guard |
| 🌐 IONOS Cloud | active | 6 | maestro router, DCD review, GDPR/compliance, K8s, FinOps, DBaaS live-guard |
| 🇫🇷 Scaleway | active | 6 | maestro router, IAM, Kapsule, FinOps, network, Kapsule rollout live-guard |
| 🇩🇪 Hetzner Cloud | active | 6 | maestro router, FinOps, infra review, capacity, firewall + server live-guards |
| 💰 Contabo | active | 6 | maestro router, FinOps, capacity, security hardening, instance + storage live-guards |
| ☸️ Kubernetes | active | 15 | RBAC, workload identity, PSA, live-guards, maestro |
| 🟩 Terraform | active | 2 | generic IaC review |
| 💰 Multi-cloud | limited | 1 | FinOps cross-cloud price advisor |
| CNCF ecosystem | active | 14 | Kyverno, Argo CD, Istio, Cilium, OTEL, Prometheus, Falco, Sigstore, cert-manager, FluxCD, Backstage, Velero |

## Agent tiers

All provider portfolios follow the same three-tier model:

### Advisory agents (read-only by default)

Use for review, diagnosis, planning, briefing, triage, and non-destructive coordination. These never write to live cloud environments.

### Execution agents (workspace-write)

Workspace-write in Codex but still non-destructive toward live cloud by default. Used for patching repo files — manifests, IaC, CI/CD configs, rollout definitions.

| Agent | Provider | Primary use |
| --- | --- | --- |
| `aws-deployment-hotfix-operator-agent` | AWS | rapid repo-side deployment corrections |
| `aws-iac-patch-executor-agent` | AWS | bounded IaC patching |
| `aws-pipeline-fix-operator-agent` | AWS | CI/CD config fixes |
| `aws-serverless-rollout-corrector-agent` | AWS | serverless rollout file corrections |
| `aws-ecs-service-remediation-operator-agent` | AWS | ECS/Fargate config remediation |

### Guarded live operators

Workspace-write in Codex, but designed for repos or shells connected to real cloud credentials or deployment authority. They must confirm target identity, require explicit approval, prefer preview or dry-run evidence, and define rollback plus post-change verification before mutation.

See each provider's README for the full live-guard catalog:

- [`agents/aws/README.md`](aws/README.md) — AWS live operators (5)
- [`agents/gcp/README.md`](gcp/README.md) — GCP live operators (6)
- [`agents/alibaba/README.md`](alibaba/README.md) — Alibaba Cloud live operators (6)
- [`agents/huawei/README.md`](huawei/README.md) — Huawei Cloud live operators (6)
- [`agents/azure/README.md`](azure/README.md) — Azure live operators (7)
- [`agents/oci/README.md`](oci/README.md) — OCI live operators (7)
- [`agents/ovhcloud/README.md`](ovhcloud/README.md) — OVHcloud live operators (1: KMS key destruction)
- [`agents/ionos/README.md`](ionos/README.md) — IONOS Cloud live operators (1: DBaaS lifecycle)
- [`agents/scaleway/README.md`](scaleway/README.md) — Scaleway live operators (1: Kapsule rollout)
- [`agents/hetzner/README.md`](hetzner/README.md) — Hetzner Cloud live operators (2: firewall, server lifecycle)
- [`agents/contabo/README.md`](contabo/README.md) — Contabo live operators (2: instance, storage)
