# Safety checklist

Use this reference before every live Hetzner Cloud server creation, deletion, type change, or power operation. This is a hard-stop guard — all non-negotiables must pass before any write operation proceeds.

## Non-negotiables

- Never ask users to paste API tokens, project secrets, customer data, or sensitive Hetzner account identifiers into chat.
- **Never execute a server deletion without a confirmed snapshot ID** — server deletion on Hetzner is irreversible. A snapshot ID must be confirmed before the delete call is issued.
- **Never execute a type change without a confirmed stopped state and downtime window approval** — `change_type` on a running server will be rejected by the API; stopping a server causes downtime that must be pre-approved.
- **Never execute any lifecycle operation without explicit human approval** naming the specific server ID and operation.
- **Never target a server by name alone** — always confirm the numeric server ID via `GET /v1/servers/{id}` before any write operation.
- Do not invent server IDs, server types, snapshot IDs, regions, or live server state.
- Do not proceed if the API token scope is unknown or unconfirmed as project-scoped.
- Server creation: public IPs (IPv4/IPv6) are **no longer auto-assigned** since API v1.34 — confirm `public_net.ipv4.create` and `public_net.ipv6.create` intent explicitly before creation.
- Type changes are **destructive if the server is running** — confirm the stop sequence and downtime window are part of the approved plan.
- Verify the target server type is available in the target region before committing to a resize plan.

## Hard-stop conditions (BLOCK the operation if any apply)

- Server ID not confirmed — only a name or description was provided
- Snapshot does not exist and the operation is a deletion
- Downtime window not approved and the operation is a type change on a running server
- Rollback plan not documented
- Named human approver has not confirmed this specific server ID and operation
- API token scope is unknown or confirmed as read-only
- Target server type not available in the target region (for creation or resize)
- `public_net.ipv4.create` intent not confirmed (for server creation)

## Stress checks

- Is a confirmed snapshot the only recovery path for the server being deleted?
- What workloads or services depend on this server, and have their owners been notified of the downtime?
- Does a type change affect any attached Volumes (Volumes persist through type changes, but verify compatibility)?
- What Floating IPs or Primary IPs are attached — will they survive the lifecycle operation?
- Is the server attached to a Load Balancer target pool — will deletion or power-off cause production traffic impact?
- What Firewall attachments exist — will a new server created as a replacement inherit the correct Firewall coverage?
- What is the rollback time window if the type change leaves the server in a non-functional state?
- Is the evidence level sufficient to confirm this is the correct target, or is there naming ambiguity?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's current server state, snapshot inventory, type availability, or live project configuration. Always confirm server state via `GET /v1/servers/{id}` before treating any state as current.
