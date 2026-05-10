# Safety checklist

Use this reference before routing to live-guard specialists or before making any recommendation that involves privileged, destructive, or production-impacting operations.

## Non-negotiables

- Never ask users to paste API tokens, project secrets, customer data, or sensitive Hetzner account identifiers into chat.
- Do not invent server IDs, firewall IDs, project IDs, resource counts, quotas, IP addresses, or live configuration state.
- Prefer live Hetzner MCP evidence when available. If unavailable, say: "I can't access live Hetzner MCP here, so I'm falling back to official docs." Then use https://docs.hetzner.com/cloud/ and Context7.
- Never route to a live-guard specialist (`hetzner-live-firewall-rule-guard`, `hetzner-live-server-lifecycle-guard`) without confirming the hard-stop pre-flight context is present or will be gathered.
- Require explicit human approval before any recommendation that involves privileged writes, production-impacting changes, or destructive operations.
- Keep routing outputs scoped to classification and handoff — do not attempt to answer the specialist question directly.

## Stress checks

- Is the request clearly scoped to a single domain or does it span multiple?
- Is a live mutation involved that requires a hard-stop guard?
- Is the API token known to be project-scoped and least-privilege?
- Is the target resource (server, firewall, project) confirmed or still ambiguous?
- What evidence is missing that would change the routing decision?
- What rollback or recovery path must the specialist confirm before acting?

## Evidence labels

Use `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Routing based on inference alone must be flagged as provisional.
