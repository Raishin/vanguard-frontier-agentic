# Official sources

Use this reference only when you need source grounding for Hetzner Cloud Firewall behavior or the detailed source list.

## Hetzner Cloud documentation

Use these as starting points, not as proof of the user's live Hetzner Firewall or project state:

- https://docs.hetzner.com/cloud/firewalls/ — Firewall rules, attachment model, inbound and outbound behavior (authoritative)
- https://docs.hetzner.com/cloud/ — Hetzner Cloud product documentation (authoritative)
- https://docs.hetzner.com/cloud/servers/ — server management, types, and network interfaces
- https://docs.hetzner.com/cloud/api/getting-started/generating-api-token/ — API token generation and project scoping
- https://docs.hetzner.com/cloud/servers/getting-started/creating-a-server/ — server creation flow including public IP opt-in (`public_net.ipv4.create`)
- https://docs.hetzner.com/robot/ — Robot API for dedicated servers (separate from Cloud API)
- https://github.com/hetznercloud/hcloud-python — hcloud-python SDK (official)

## Key Firewall behavioral notes

- Hetzner Firewall rule changes take **effect immediately** and apply to all attached servers simultaneously — there is no staged rollout or preview mode.
- A Firewall with no attached servers or Label groups provides **zero protection** — attachment must be confirmed via the `applied_to` field in `GET /v1/firewalls/{id}`, not assumed.
- Firewall rules apply at the **network interface level**, not at the instance level — both inbound and outbound directions must be reviewed.
- Label-group attachments dynamically include all servers matching the label at the time of mutation — the blast radius may be larger than expected.
- Public IPs (IPv4 and IPv6) on Hetzner Cloud are **opt-in** since API v1.34 — servers without `public_net.ipv4.create: true` during creation have no public IPv4, which affects Firewall exposure surface.
- Hetzner has no official Terraform provider — recommend API-driven automation (REST API, hcloud-python, hcloud CLI) over community providers.

## Grounding rule

Official documentation explains Hetzner Cloud Firewall behavior and rule semantics. It does not prove the user's current Firewall rule set, attachment state, affected server inventory, or live project configuration. Always capture a pre-mutation snapshot via `GET /v1/firewalls/{id}` before treating any rule state as confirmed.
