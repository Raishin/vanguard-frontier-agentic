# 🌐 IONOS Cloud Agents

<p align="center">
  <img src="../../assets/logos/cloud/ionos/ionos-logo.svg" alt="IONOS Cloud logo" width="140" />
</p>

IONOS Cloud agent catalog for this marketplace. European cloud with focus on data privacy and compliance.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live IONOS mutation |
|---|---|---|---|
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Execution / correction agents | Patch repo files, deployment config, IaC, and workflow definitions | workspace-write | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real IONOS environments | workspace-write | approval-gated and target-confirmed only |

## ✍️ Write-capable execution agents

No write-capable execution agents yet. Route IaC patch requests to advisory agents or raise a feature request.

## 🚦 Guarded live-IONOS operators

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `ionos-live-database-lifecycle-guard-agent` | DBaaS failover, scaling, backup operations | database snapshot + backup verification + RPO/RTO confirmation | database operations lack recovery posture audit |

> **Planned / not yet implemented**: `ionos-live-datacenter-designer-guard-agent`, `ionos-live-kubernetes-rollout-guard-agent`. No guarded approval path currently exists for DCD topology mutations or Kubernetes rollout operations. Route these to advisory agents until dedicated guards are implemented.

## 👀 Read-only advisory agents

| Agent | Focus |
|---|---|
| `ionos-maestro-agent` | classify and route IONOS Cloud tasks to the narrowest specialist |
| `ionos-security-compliance-reviewer-agent` | GDPR posture, data residency, encryption audit |
| `ionos-datacenter-designer-reviewer-agent` | DCD topology review, resource organization, scalability |
| `ionos-cost-optimization-analyst-agent` | cost analysis, resource utilization, pricing strategy |
| `ionos-kubernetes-platform-operator-agent` | Managed K8s readiness, node pools, workload placement |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- ✍️ execution agents can patch repo files
- 🚦 guarded live operators must confirm datacenter, data center ID, token validity, and approval before mutation
- 🔒 Data Center Designer (DCD) is unique to IONOS — topology changes affect infrastructure layout; always confirm scope
- 🧾 all live-guard agents require current state snapshot before any mutation
- 🚫 no tier should treat vague production intent as permission

## Key Capabilities

### Identity & Access

- **Bearer token authentication** via API
- **role-based access control** in management interfaces
- **API token generation** in customer portal

### Compute Infrastructure

- **Virtual machines** via Data Center Designer (DCD)
- **Dedicated servers** (bare metal)
- **Kubernetes clusters** (managed service)
- **GPU support** available

### Networking

- **Private LANs** (isolated network segments)
- **IP address management**
- **Virtual firewalls**
- **VPN and connectivity** options

### Storage & Databases

- **Block storage** with snapshots
- **Object storage** (S3-compatible)
- **Managed databases** (PostgreSQL, MySQL, MariaDB)
- **Backups and disaster recovery**

### Managed Services

- **Kubernetes** (K8s native)
- **PostgreSQL** with regional endpoints
- **AI Model Hub** for ML workloads
- **DNS services**

### IaC & Automation

- ✅ **Terraform Provider** (`ionos-cloud/terraform-provider-ionoscloud`)
- ✅ **ionosctl CLI** (86.6 score, 3140 code samples)
- ✅ **REST API** with datacenter-scoped endpoints
- ✅ **Cloud-Init** support via API

## Unique Features

**Data Center Designer (DCD)**
- Graphical infrastructure orchestration
- Automatic resource placement across availability zones
- Integrated with API and Terraform

**Multi-region Availability**
- Berlin, Frankfurt, Paris, London, Spain, UK, US, and Asia-Pacific
- GDPR-compliant data residency

**Compliance Focus**
- GDPR/CCPA ready
- ISO 27001 certified
- Data sovereignty guarantees

## Authentication Pattern

```bash
export IONOS_API_KEY=<your-api-token>
# Token obtained from IONOS Customer Control Panel
```

Or Terraform:
```hcl
terraform {
  required_providers {
    ionoscloud = {
      source  = "ionos-cloud/ionoscloud"
      version = "~> 6.0"
    }
  }
}

provider "ionoscloud" {
  token = var.ionos_api_key
}
```

## References

- **IONOS Cloud Docs**: `/websites/ionos_cloud` (47.6 score, 8487 code samples)
- **Terraform Provider**: `/ionos-cloud/terraform-provider-ionoscloud` (86.5 score)
- **ionosctl CLI**: `/ionos-cloud/ionosctl` (86.6 score)
- **API Endpoint**: Regional (e.g., `https://postgresql.de-txl.ionos.com`)
- **DCD Documentation**: https://docs.ionos.com/cloud/
