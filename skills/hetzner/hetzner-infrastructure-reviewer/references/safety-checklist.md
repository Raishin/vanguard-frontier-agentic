# Safety checklist

Use this reference before privileged, destructive, traffic-changing, or production-impacting recommendations.

## Non-negotiables

- Never ask users to paste API tokens, project secrets, customer data, or sensitive Hetzner account identifiers into chat.
- Prefer live Hetzner MCP read-only evidence when available. If unavailable, say: "I can't access live Hetzner MCP here, so I'm falling back to official docs." Then use https://docs.hetzner.com/cloud/ and Context7.
- Do not invent firewall IDs, server IDs, IP addresses, resource counts, or live configuration state.
- Do not recommend live Firewall mutations — route those to `hetzner-live-firewall-rule-guard` with full hard-stop pre-flight context.
- Require explicit human approval before any recommendation that involves removing Firewall coverage, changing IP exposure, or modifying Load Balancer routing.
- An unattached Hetzner Firewall provides zero protection — never treat a Firewall as active unless its attachment to specific servers or Label groups is confirmed.
- Keep remediation scoped, reversible, and least-privilege.

## Stress checks

- What inbound paths reach production services without a Firewall?
- What Firewalls exist but are unattached?
- What management ports (SSH 22, RDP 3389, etc.) are reachable from 0.0.0.0/0?
- What Load Balancer targets are unhealthy or have no redundancy?
- What is the blast radius if a recommended Firewall change is wrong?
- What compliance or audit evidence is missing?
- What rollback path is unproven?

## Evidence labels

Use `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Hetzner Firewall or network state.
