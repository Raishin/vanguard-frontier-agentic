# Official sources

Use this reference only when you need source grounding for Hetzner Cloud infrastructure behavior or the detailed source list.

## Hetzner Cloud documentation

Use these as starting points, not as proof of the user's live Hetzner project state:

- https://docs.hetzner.com/cloud/ — Hetzner Cloud product documentation (authoritative)
- https://docs.hetzner.com/cloud/firewalls/ — Firewall rules, attachment model, inbound/outbound behavior
- https://docs.hetzner.com/cloud/servers/ — server management, types, placement, and network interfaces
- https://docs.hetzner.com/cloud/servers/getting-started/creating-a-server/ — server creation flow including public IP opt-in (`public_net.ipv4.create`)
- https://docs.hetzner.com/cloud/api/getting-started/generating-api-token/ — API token generation and project scoping
- https://docs.hetzner.com/robot/ — Robot API for dedicated servers (separate from Cloud API)
- https://github.com/hetznercloud/hcloud-python — hcloud-python SDK (official)

## Key behavioral notes

- Hetzner Firewalls apply rules at the network interface level and affect all attached servers immediately when changed.
- A Firewall with no attached servers or Label groups provides **zero protection** — attachment must be confirmed, not assumed.
- Public IPs (IPv4 and IPv6) on Hetzner Cloud are **opt-in** since API v1.34 — servers without `public_net.ipv4.create: true` during creation have no public IPv4 by default.
- Hetzner has no official Terraform provider — recommend API-driven automation (REST API, hcloud-python, hcloud CLI) over community providers.

## Grounding rule

Official documentation explains Hetzner Cloud service behavior. It does not prove the user's current project state, Firewall attachment, IP assignments, Load Balancer health, or region distribution. Prefer live Hetzner MCP or user-provided sanitized evidence for current-state claims.
