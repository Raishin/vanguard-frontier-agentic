# 🇩🇪 Hetzner Cloud Agents

<p align="center">
  <img src="../../assets/logos/cloud/hetzner/hetzner-logo.svg" alt="Hetzner Cloud logo" width="140" />
</p>

Hetzner Cloud agent catalog for this marketplace. Cost-effective, performance-focused European cloud infrastructure.

## ⚠️ Implementation Note

Hetzner Cloud **lacks an official Terraform provider**. Agents focus on:
- **API-driven automation** (REST API)
- **Community Terraform** (if available and approved)
- **Infrastructure review and cost optimization**
- **Server lifecycle management via API**

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Hetzner mutation |
|---|---|---|---|
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real Hetzner environments | workspace-write | approval-gated and target-confirmed only |

> **Note:** Execution agents (IaC patch) not implemented — no Terraform provider available.

## 🚦 Guarded live-Hetzner operators

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `hetzner-live-server-lifecycle-guard-agent` | server creation, destruction, type changes | server ID + region + rollback plan required | operations are ambiguous about target server or region |
| `hetzner-live-firewall-rule-guard-agent` | firewall rule mutations and attachment | current rules + blast-radius review | changes lack server attachment audit |

> **Planned / not yet implemented**: `hetzner-live-load-balancer-guard-agent`. No guarded approval path currently exists for load balancer operations. Route to advisory agents until a load balancer guard is implemented.

## 👀 Read-only advisory agents

| Agent | Focus |
|---|---|
| `hetzner-maestro-agent` | classify and route Hetzner Cloud tasks to the narrowest specialist |
| `hetzner-infrastructure-reviewer-agent` | firewall rules, load balancer config, placement strategy |
| `hetzner-cost-optimization-analyst-agent` | instance type review, resource utilization, cost savings |
| `hetzner-capacity-planner-agent` | resource limits, quota tracking, growth planning |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- 🚦 guarded live operators must confirm project, server ID, region, approval, and rollback before mutation
- 🔑 API tokens are project-scoped — always verify token scope before operations
- 📊 Firewall and Load Balancer APIs are separate from Servers API — coordinate changes across endpoints
- 🚫 no tier should treat vague production intent as permission

## Key Capabilities

### Compute Infrastructure

- **Cloud Servers** (Intel/ARM instances)
- **Dedicated servers** (bare metal)
- **Server snapshots and images**
- **Network drives** (persistent block storage)

### Networking

- **Firewalls** with inbound/outbound rules
- **Load Balancers** (stateful)
- **Public IP management**
- **Floating IP** support
- **Private networking** (VPC-like)

### Storage

- **Network Drives** (block storage)
- **Storage Boxes** (S3-compatible object storage)
- **Snapshots** for backup and recovery

### Regions

| Region | Datacenters | Availability |
|---|---|---|
| 🇩🇪 Germany | Falkenstein, Nuremberg | 3+ AZs |
| 🇫🇮 Finland | Helsinki | 2 AZs |

### API & Automation

- ✅ **REST API** (comprehensive, well-documented)
- ✅ **Official Python library** (`hcloud-python`)
- ❌ **No official Terraform provider** (community alternatives exist)
- ✅ **API tokens** for authentication
- ⚠️ **No official CLI** (community tools available)

## Constraints

**No Terraform Provider**
- Automation must be API-driven (Python, Go, Bash)
- Community providers exist (evaluate for approval)
- Infrastructure-as-code requires custom tooling

**API Limitations**
- Stateless REST API (no state backends)
- Server creation is synchronous but slow (2-5 minutes)
- No policy enforcement via API (firewall rules must be managed separately)

## Authentication Pattern

```bash
# API Token (project-scoped)
export HCLOUD_TOKEN=<your-api-token>
export HCLOUD_REGION=fsn1  # falkenstein | nbg1 | hel1
```

Or Python SDK:
```python
import os
from hcloud import Client

client = Client(token=os.environ["HCLOUD_TOKEN"])
servers = client.servers.get_list
```

## API Reference

```bash
# Create server
curl -X POST https://api.hetzner.cloud/v1/servers \
  -H "Authorization: Bearer $HCLOUD_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my-server",
    "server_type": "cx22",
    "image": "ubuntu-22.04"
  }'

# List servers
curl https://api.hetzner.cloud/v1/servers \
  -H "Authorization: Bearer $HCLOUD_TOKEN"
```

## References

- **Hetzner Cloud API**: `/websites/hetzner_cloud` (51.7 score, 536 code samples)
- **Hetzner Python Library**: `/hetznercloud/hcloud-python` (80.3 score, 153 samples)
- **API Documentation**: https://docs.hetzner.cloud/
- **Console**: https://console.hetzner.cloud/
- **Pricing**: https://www.hetzner.com/cloud/pricing/

## Tools & Integration

**Community Options**
- Terraform (community provider, use with caution)
- Pulumi (community support)
- Ansible (playbooks available)
- Packer (image building)

**Recommended Approach**
- Use Python SDK for complex automation
- Bash + curl for simple operations
- Document any custom tooling in repos
