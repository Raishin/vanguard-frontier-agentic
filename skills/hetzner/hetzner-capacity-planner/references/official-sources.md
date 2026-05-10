# Official sources

Use this reference only when you need source grounding for Hetzner Cloud resource limits, server types, or region behavior, or the detailed source list.

## Hetzner Cloud documentation

Use these as starting points, not as proof of the user's live Hetzner project state:

- https://docs.hetzner.com/cloud/ — Hetzner Cloud product documentation (authoritative)
- https://docs.hetzner.com/cloud/servers/ — server types (CX shared, CCX dedicated, CAX ARM), location availability, and lifecycle
- https://docs.hetzner.com/cloud/servers/getting-started/creating-a-server/ — server creation flow including public IP opt-in (`public_net.ipv4.create`)
- https://docs.hetzner.com/cloud/api/getting-started/generating-api-token/ — API token generation and project scoping
- https://docs.hetzner.com/robot/ — Robot API for dedicated servers (separate from Cloud API)
- https://github.com/hetznercloud/hcloud-python — hcloud-python SDK (official)

## Key capacity behavioral notes

- Hetzner Cloud enforces per-project resource quotas on servers, Volumes, Load Balancers, Floating IPs, Primary IPs, Networks, SSH keys, and certificates — quota increases must be requested through the Cloud Console or support.
- Hetzner has **no native auto-scaling** — server provisioning is manual with approximately 2–5 minutes lead time per server; capacity plans must pre-provision or include manual runbooks.
- Available regions are fsn1 (Falkenstein DE), nbg1 (Nuremberg DE), and hel1 (Helsinki FI) — not all server types are available in all regions; verify availability before committing to a region strategy.
- Public IPs are **opt-in** since API v1.34 — Primary IPs and Floating IPs count separately against per-project quotas.
- Storage Box Snapshot Plans require both `hour` and `minute` parameters — missing either parameter causes the snapshot plan to silently fail.
- Hetzner has no official Terraform provider — recommend API-driven automation (REST API, hcloud-python, hcloud CLI) over community providers.

## Grounding rule

Official documentation explains Hetzner Cloud service behavior, resource types, and quota structure. It does not prove the user's current project resource counts, quota headroom, growth trajectory, or regional availability for specific server types at a given moment. Prefer live Hetzner MCP or user-provided sanitized evidence for current-state and capacity claims.
