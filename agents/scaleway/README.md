# 🇫🇷 Scaleway Agents

<p align="center">
  <img src="../../assets/logos/cloud/scaleway/scaleway-logo.svg" alt="Scaleway logo" width="140" />
</p>

Scaleway agent catalog for this marketplace. Developer-friendly European cloud with SDK support and global reach.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Scaleway mutation |
|---|---|---|---|
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Execution / correction agents | Patch repo files, deployment config, IaC, and workflow definitions | workspace-write | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real Scaleway environments | workspace-write | approval-gated and target-confirmed only |

## ✍️ Write-capable execution agents

No write-capable execution agents yet. Route IaC patch requests to advisory agents or raise a feature request.

## 🚦 Guarded live-Scaleway operators

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `scaleway-live-kapsule-rollout-guard-agent` | Kubernetes cluster and node pool mutations | cluster health + PDB audit + rollback plan | cluster control-plane change is unreviewed |

> **Planned / not yet implemented**: `scaleway-live-instance-lifecycle-guard-agent`, `scaleway-live-rdb-failover-guard-agent`. No guarded approval path currently exists for instance lifecycle or RDB failover operations. Route these to advisory agents until dedicated guards are implemented.

## 👀 Read-only advisory agents

| Agent | Focus |
|---|---|
| `scaleway-maestro-agent` | classify and route Scaleway tasks to the narrowest specialist |
| `scaleway-iam-policy-review-agent` | IAM bindings, service account access, API key governance |
| `scaleway-cost-optimizer-agent` | cost analysis, reserved instance utilization, rightsizing |
| `scaleway-kapsule-platform-operator-agent` | Kubernetes readiness, node pool strategy, workload placement |
| `scaleway-network-architect-agent` | VPC design, security groups, placement groups for HA |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- ✍️ execution agents can patch repo files
- 🚦 guarded live operators must confirm project, zone/region, API endpoint, approval, and rollback before mutation
- 📦 Scaleway SDKs (JS/Go) offer excellent developer experience — prefer SDK patterns for automation
- 🎯 Placement groups ensure HA — always review placement strategy for production workloads
- 🚫 no tier should treat vague production intent as permission

## Key Capabilities

### Identity & Access

- **Access key/secret key pairs** for authentication
- **API tokens** with granular scopes
- **IAM policies** for role-based access
- **service accounts** for automation

### Compute & Kubernetes

- **Scalable instances** (Intel/ARM/GPU)
- **Kapsule** (managed Kubernetes)
- **Serverless functions & containers**
- **Bare metal servers**

### Networking & Storage

- **VPC** with subnet isolation
- **Placement groups** for HA
- **Block Storage (SBS)** with snapshots
- **Object Storage (S3-compatible)**

### Databases & Cache

- **Managed RDB** (PostgreSQL, MySQL, MariaDB)
- **Redis** managed cache
- **Backups and PITR**

### Global Reach

- **EU regions** (Amsterdam, Paris)
- **Global expansion** (US, Asia-Pacific, Japan, India, Australia)

### IaC & Automation

- ✅ **Terraform Provider** (77.85 score, 1920+ samples)
- ✅ **Pulumi support** (IaC with Python/Go/JS)
- ✅ **Scaleway CLI** (official command-line)
- ✅ **SDKs** (JavaScript, Go with full type support)
- ✅ **REST API** with region-scoped endpoints

## Developer-Friendly Features

**Multiple SDKs**
- JavaScript/TypeScript (full support, pagination built-in)
- Go (native support)
- Python, Ruby, Java (community)

**Strong Kubernetes Integration**
- Kapsule (managed K8s)
- Marketplace for CNI plugins (Kilo, Flannel, Cilium)
- ExternalDNS support

**Placement Groups**
- Automatic HA orchestration
- Multi-zone resilience
- Optional enforcement

## Authentication Pattern

```bash
# CLI with API token
export SCW_ACCESS_KEY=<access-key>
export SCW_SECRET_KEY=<secret-key>
export SCW_DEFAULT_REGION=fr-par
```

Or Terraform:
```hcl
terraform {
  required_providers {
    scaleway = {
      source  = "scaleway/scaleway"
      version = "~> 2.0"
    }
  }
}

provider "scaleway" {
  access_key      = var.scaleway_access_key
  secret_key      = var.scaleway_secret_key
  region          = "fr-par"
  zone            = "fr-par-1"
}
```

## References

- **Scaleway Terraform**: `/scaleway/terraform-provider-scaleway` (77.85 score, 1920+ samples)
- **Scaleway JS SDK**: `/scaleway/scaleway-sdk-js` (79.3 score, 275 samples)
- **Scaleway Go SDK**: `/scaleway/scaleway-sdk-go` (76.3 score, 42 samples)
- **Scaleway CLI**: https://github.com/scaleway/scaleway-cli
- **Documentation**: https://www.scaleway.com/en/docs/
- **Pricing**: https://www.scaleway.com/en/pricing/
