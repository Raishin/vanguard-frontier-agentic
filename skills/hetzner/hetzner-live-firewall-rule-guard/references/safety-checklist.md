# Safety checklist

Use this reference before every live Hetzner Cloud Firewall mutation, attachment change, or Firewall creation or deletion. This is a hard-stop guard — all non-negotiables must pass before any write operation proceeds.

## Non-negotiables

- Never ask users to paste API tokens, project secrets, customer data, or sensitive Hetzner account identifiers into chat.
- **Never execute a Firewall mutation without a pre-mutation snapshot** of the current rule set captured via `GET /v1/firewalls/{id}`.
- **Never execute a Firewall mutation without a confirmed rollback plan** — the exact rule revert procedure or Firewall detach path must be documented before proceeding.
- **Never execute a Firewall mutation without explicit human approval** naming the specific Firewall ID and rule change.
- **Never execute a Firewall mutation without a confirmed blast-radius review** — list every attached server and Label group by ID and name.
- Do not invent Firewall IDs, rule sets, IP addresses, server names, or live attachment state.
- Do not proceed if the API token scope is unknown or unconfirmed as project-scoped.
- Hetzner Firewall rule changes take effect immediately and affect all attached servers simultaneously — there is no staged rollout.
- An unattached Hetzner Firewall provides zero protection — confirm attachment state before and after every mutation.
- Challenge any rule that adds `0.0.0.0/0` inbound access to management ports (SSH 22, RDP 3389, database ports, admin UIs).
- Public IPs are opt-in since API v1.34 — verify which servers have public interfaces before assessing firewall exposure.

## Hard-stop conditions (BLOCK the mutation if any apply)

- Pre-mutation snapshot not captured
- Blast-radius review not completed (unknown attached servers or Label groups)
- Firewall ID not confirmed — only a name or description was provided
- Rollback plan not documented
- Named human approver has not confirmed this specific Firewall ID and change
- API token scope is unknown or confirmed as read-only
- Proposed rule adds broad inbound access (0.0.0.0/0) to a management port without explicit justification and risk acceptance

## Stress checks

- Does this rule change remove the only Firewall protecting a production server?
- Does this attachment change leave any server with no Firewall coverage on its public interface?
- What is the blast radius if the mutation is applied to the wrong Firewall?
- Is the rollback procedure tested and confirmed, or is it speculative?
- Are there Label-group-attached Firewalls that will affect more servers than the operator realizes?
- What post-change verification will confirm the mutation had the intended effect?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Hetzner Firewall rule set, attachment state, or affected server inventory.
