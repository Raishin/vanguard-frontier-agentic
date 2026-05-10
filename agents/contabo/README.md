# 💰 Contabo Agents

<p align="center">
  <img src="../../assets/logos/cloud/contabo/contabo-logo.svg" alt="Contabo logo" width="140" />
</p>

Contabo agent catalog for this marketplace. Budget-friendly European VPS/VDS provider with global datacenter coverage.

## ⚠️ Implementation Note

Contabo **does not provide an official Terraform provider**. Agents focus on:
- **API-driven automation** via REST API (OAuth2 password grant)
- **CLI-based operations** via `cntb` (official CLI)
- **Infrastructure review and cost optimization**
- **VPS/VDS lifecycle management**

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Contabo mutation |
|---|---|---|---|
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real Contabo environments | workspace-write | approval-gated and target-confirmed only |

> **Note:** Execution agents (IaC patch) not implemented — no Terraform provider available.

## 🚦 Guarded live-Contabo operators

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `contabo-live-instance-lifecycle-guard-agent` | VPS/VDS creation, reinstallation, cancellation | instance ID + region + contract period confirmation | operations affect billing without explicit period acknowledgment |
| `contabo-live-storage-operations-guard-agent` | Object Storage and S3 bucket operations | bucket inventory + access policy + retention audit | deletion requested without backup verification |
| `contabo-live-private-network-guard-agent` | Private network attachment and routing | current attachments + isolation scope | network changes affect multiple instances without review |

## 👀 Read-only advisory agents

| Agent | Focus |
|---|---|
| `contabo-maestro-agent` | classify and route Contabo tasks to the narrowest specialist |
| `contabo-cost-optimization-analyst-agent` | contract period analysis, VPS sizing, addon utilization |
| `contabo-capacity-planner-agent` | resource planning, region coverage, instance sizing |
| `contabo-security-hardening-agent` | SSH key management, default user policy, firewall posture |
| `contabo-infrastructure-reviewer-agent` | VPS configuration, image strategy, addon review |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- 🚦 guarded live operators must confirm tenant ID, customer ID, instance ID, region, and contract period before mutation
- ⏰ **Contractual periods** (1, 3, 6, 12 months) — instance termination/changes have billing impact; always confirm period
- 🔐 **OAuth2 password grant flow** — credentials must be in environment variables, never hardcoded
- 🌍 Regions: EU, US-central, US-east, US-west, SIN, UK, AUS, JPN, IND
- 🚫 no tier should treat vague production intent as permission

## Key Capabilities

### Authentication

- **OAuth2 password grant flow** via Keycloak (`auth.contabo.com`)
- Combined credentials: `client_id`, `client_secret`, `api_user`, `api_password`
- Bearer token-based API access
- `x-request-id` (UUIDv4) for request tracing

### Compute Products

- **VPS** (Virtual Private Server) - shared CPU/RAM
- **VDS** (Virtual Dedicated Server) - dedicated resources
- **Storage VPS** - high-storage instances
- **GPU instances** (limited regions)

### Storage & Networking

- **Object Storage** (S3-compatible)
- **Private Networking** (add-on)
- **Additional IPs** (add-on)
- **Extra Storage** (add-on)
- **Custom Images** (add-on)

### Image & Init Support

- **Pre-built images** (Linux distributions, Windows)
- **Custom images** (via add-on)
- **Cloud-Init** user data support
- **Application IDs** for pre-configured stacks
- **SSH key management** (secret IDs)

### Regional Coverage

| Region | Code | Notes |
|---|---|---|
| 🇪🇺 European Union | EU | primary, default |
| 🇬🇧 United Kingdom | UK | London |
| 🇺🇸 United States | US-central, US-east, US-west | multi-region |
| 🌏 Asia-Pacific | SIN, JPN, IND | Singapore, Japan, India |
| 🇦🇺 Australia | AUS | Sydney |

### CLI & Automation

- ✅ **Contabo CLI (`cntb`)** - official CLI tool (85.7 score, 2324 samples)
- ✅ **REST API** - comprehensive API with OAuth2 (73.1 score, 501 samples)
- ❌ **No official Terraform provider** (community options not endorsed)
- ❌ **No SDKs** in major languages
- ✅ **Cloud-Init** for instance configuration

## Pricing & Contracts

**Contract Periods** (key constraint)
- 1 month
- 3 months
- 6 months
- 12 months

**Important:** Period selection is required at creation; cancellation has billing implications. Live-guard agents must verify contractual period before any lifecycle change.

## Authentication Pattern

```bash
# OAuth2 password grant flow — load from environment, never hardcode
: "${CONTABO_CLIENT_ID:?set in env}"
: "${CONTABO_CLIENT_SECRET:?set in env}"
: "${CONTABO_API_USER:?set in env}"
: "${CONTABO_API_PASSWORD:?set in env}"

# Get access token
ACCESS_TOKEN=$(curl -s -d "client_id=$CONTABO_CLIENT_ID" \
  -d "client_secret=$CONTABO_CLIENT_SECRET" \
  --data-urlencode "username=$CONTABO_API_USER" \
  --data-urlencode "password=$CONTABO_API_PASSWORD" \
  -d 'grant_type=password' \
  'https://auth.contabo.com/auth/realms/contabo/protocol/openid-connect/token' \
  | jq -r '.access_token')

# Use token (with required x-request-id header)
curl -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "x-request-id: $(uuidgen)" \
  https://api.contabo.com/v1/compute/instances
```

## API Reference

```bash
# Create VPS
curl -X POST https://api.contabo.com/v1/compute/instances \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "x-request-id: $(uuidgen)" \
  -H "Content-Type: application/json" \
  -d '{
    "imageId": "afecbb85-e2fc-46f0-9684-b46b1faf00bb",
    "productId": "V92",
    "region": "EU",
    "period": 1,
    "displayName": "my-vps"
  }'
```

## References

- **Contabo API**: `/websites/api_contabo` (73.1 score, 501 code samples)
- **Contabo CLI**: `/contabo/cntb` (85.7 score, 2324 samples)
- **API Documentation**: https://api.contabo.com/
- **Customer Control Panel**: https://my.contabo.com/

## Tools & Integration

**Recommended Approach**
- Use `cntb` CLI for interactive operations
- Use REST API + bash/jq for automation
- Use Cloud-Init for instance configuration
- Document any custom tooling in repos
- Always tag operations with `x-request-id` for support traceability

**Caution Areas**
- No native IaC support — automation requires custom tooling
- Contractual periods affect billing — never auto-extend or auto-cancel
- API is rate-limited — bake retry/backoff into automation
- OAuth2 tokens are short-lived (typically 5 min) — refresh handling required
