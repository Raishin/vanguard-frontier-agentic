# Official sources

Use this reference only when you need source grounding for Hetzner Cloud server lifecycle behavior or the detailed source list.

## Hetzner Cloud documentation

Use these as starting points, not as proof of the user's live Hetzner server state:

- https://docs.hetzner.com/cloud/servers/ — server management, types, lifecycle, and power operations (authoritative)
- https://docs.hetzner.com/cloud/servers/getting-started/creating-a-server/ — server creation flow including public IP opt-in (`public_net.ipv4.create`, `public_net.ipv6.create`)
- https://docs.hetzner.com/cloud/ — Hetzner Cloud product documentation (authoritative)
- https://docs.hetzner.com/cloud/api/getting-started/generating-api-token/ — API token generation and project scoping
- https://docs.hetzner.com/cloud/firewalls/ — Firewall rules and attachment model (relevant when creating replacement servers)
- https://docs.hetzner.com/robot/ — Robot API for dedicated servers (separate from Cloud API)
- https://github.com/hetznercloud/hcloud-python — hcloud-python SDK (official)

## Key server lifecycle behavioral notes

- **Server deletion is irreversible** — once deleted, the server and its local disk data cannot be recovered without a prior snapshot. A snapshot must be confirmed before any deletion proceeds.
- **Type changes require the server to be stopped** — the `change_type` action will fail on a running server. Stopping causes downtime; confirm the window before issuing the stop.
- **Public IPs are opt-in since API v1.34** — new servers do not receive a public IPv4 unless `public_net.ipv4.create: true` is specified in the creation request. IPv6 is similarly opt-in.
- **Server creation is not instantaneous** — provisioning takes approximately 20–60 seconds; post-creation verification should poll `GET /v1/servers/{id}` until `status: running`.
- Available regions are fsn1 (Falkenstein DE), nbg1 (Nuremberg DE), and hel1 (Helsinki FI) — not all server types are available in all regions.
- Hetzner has **no native auto-scaling** — server provisioning is always manual.
- Hetzner has no official Terraform provider — recommend API-driven automation (REST API, hcloud-python, hcloud CLI) over community providers.

## Grounding rule

Official documentation explains Hetzner Cloud server lifecycle behavior, server types, and API semantics. It does not prove the user's current server state, snapshot inventory, power status, attached resource configuration, or available server types in a given region at this moment. Always confirm server state via `GET /v1/servers/{id}` before treating any state as current.
