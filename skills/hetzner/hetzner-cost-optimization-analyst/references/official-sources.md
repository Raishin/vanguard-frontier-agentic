# Official sources

Use this reference only when you need source grounding for Hetzner Cloud pricing or resource behavior, or the detailed source list.

## Hetzner Cloud documentation and pricing

Use these as starting points, not as proof of the user's current billing or resource state:

- https://www.hetzner.com/cloud#pricing — Hetzner Cloud pricing (server types, Volumes, Load Balancers, Floating IPs, Primary IPs)
- https://docs.hetzner.com/cloud/ — Hetzner Cloud product documentation (authoritative)
- https://docs.hetzner.com/cloud/servers/ — server types (CX shared, CCX dedicated, CAX ARM), billing model
- https://docs.hetzner.com/cloud/volumes/ — Volume pricing and attachment rules
- https://docs.hetzner.com/cloud/api/getting-started/generating-api-token/ — API token generation and project scoping
- https://docs.hetzner.com/robot/ — Robot API for dedicated servers (separate from Cloud API)
- https://github.com/hetznercloud/hcloud-python — hcloud-python SDK (official)

## Key cost behavioral notes

- Primary IPs and Floating IPs on Hetzner Cloud **incur cost when unattached** — they are billed regardless of whether they are assigned to a running server.
- Public IPs are **opt-in** since API v1.34 — servers without `public_net.ipv4.create: true` during creation have no public IPv4, which also removes that IP cost.
- Hetzner bills by the hour for most resources — short-lived servers and orphaned resources accumulate cost even if unused.
- Hetzner has no official Terraform provider — recommend API-driven automation (REST API, hcloud-python, hcloud CLI) over community providers.

## Grounding rule

Official documentation and pricing pages explain Hetzner Cloud service behavior and list prices. They do not prove the user's current billing total, resource inventory, utilization, or waste level. Prefer live Hetzner MCP or user-provided sanitized evidence for current-state and cost claims.
