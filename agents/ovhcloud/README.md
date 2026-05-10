# ☁️ OVHcloud Agents

<p align="center">
  <img src="../../assets/logos/cloud/ovhcloud/ovhcloud-logo.svg" alt="OVHcloud logo" width="140" />
</p>

OVHcloud agent catalog for this marketplace. European cloud infrastructure with advanced IAM and compliance.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live OVHcloud mutation |
|---|---|---|---|
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Execution / correction agents | Patch repo files, deployment config, IaC, and workflow definitions | workspace-write | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real OVHcloud environments | workspace-write | approval-gated and target-confirmed only |

## ✍️ Write-capable execution agents

No write-capable execution agents yet. Route IaC patch requests to advisory agents or raise a feature request.

## 🚦 Guarded live-OVHcloud operators

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `ovhcloud-live-kms-key-destruction-guard-agent` | KMS key version destruction and rotation | key policy audit + usage verification + waiting period | key is in active use without retention policy |

> **Planned / not yet implemented**: `ovhcloud-live-instance-lifecycle-guard-agent`, `ovhcloud-live-storage-bucket-guard-agent`, `ovhcloud-live-network-vrrack-guard-agent`. No guarded approval path currently exists for instance lifecycle, object storage, or network operations. Consider routing these to the KMS guard or advisory agents until dedicated guards are implemented.

## 👀 Read-only advisory agents

| Agent | Focus |
|---|---|
| `ovhcloud-maestro-agent` | classify and route OVHcloud tasks to the narrowest specialist |
| `ovhcloud-iam-policy-review-agent` | IAM policy conditions, identity groups, access control |
| `ovhcloud-cost-finops-analyst-agent` | cost analysis, commitment tracking, usage optimization |
| `ovhcloud-kubernetes-platform-operator-agent` | Managed Kubernetes (MCK) lifecycle, node pools, workload management |
| `ovhcloud-network-architect-agent` | VRack design, network isolation, connectivity strategy |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- ✍️ execution agents can patch repo files
- 🚦 guarded live operators must confirm project ID, region, API endpoint, approval, and rollback before mutation
- 🔑 OVHcloud IAM policies support conditional access (IP, tags, expiration) — always audit policy scope before approval
- 🚫 no tier should treat vague production intent as permission

## Key Capabilities

### Identity & Access

- **OAuth2 service accounts** for programmatic access
- **IAM Policy framework** with conditional access (IP restrictions, tags, expiration dates)
- **Identity groups** for role aggregation
- **API keys** for direct authentication

### Compute & Kubernetes

- **Public Cloud instances** (Linux/Windows)
- **Bare metal servers**
- **Managed Kubernetes (MCK)** with node pool orchestration
- **VPS and dedicated infrastructure**

### Networking & Storage

- **VRack** for private networking across resources
- **S3-compatible object storage**
- **DNS and load balancing**
- **Network isolation and security groups**

### Managed Services

- **Managed databases** (MySQL, PostgreSQL, MongoDB)
- **Key Management Service (KMS)**
- **Message queues**
- **Log Data Platform**

### IaC & Automation

- ✅ **Terraform Provider** (150+ resources)
- ✅ **OVHcloud CLI** (unified command-line interface)
- ✅ **REST API v2** with regional endpoints
- ✅ **CloudInit support** for instance initialization

## Regional Coverage

| Region | Datacenters | Use case |
|---|---|---|
| 🇫🇷 France | Paris, Strasbourg | EU primary, GDPR-friendly |
| 🇩🇪 Germany | Frankfurt | EU central, compliance-sensitive |
| 🇵🇱 Poland | Warsaw | EU expansion |
| 🌍 North America | Canada, USA | cross-region coverage |

## Authentication Pattern

```bash
export OVH_ENDPOINT=ovh-eu
export OVH_APPLICATION_KEY=<key>
export OVH_APPLICATION_SECRET=<secret>
export OVH_CONSUMER_KEY=<consumer_key>
```

Or OAuth2 for service accounts:
```bash
export OVH_CLIENT_ID=<client_id>
export OVH_CLIENT_SECRET=<client_secret>
```

## References

- **Terraform Provider**: `/ovh/terraform-provider-ovh` (95.6 score, 2552 code samples)
- **OVHcloud CLI**: `/ovh/ovhcloud-cli` (65.65 score, 299 code samples)
- **API v2**: https://api.ovh.com/console/
- **Documentation**: https://docs.ovhcloud.com/
