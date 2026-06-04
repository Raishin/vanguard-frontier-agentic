# Safety checklist for Azure AI Foundry Ops Governor

## Non-negotiable gates

- Do not ask for keys, connection strings, tokens, tenant identifiers, subscription identifiers, private data, or prompts containing customer data.
- Prefer Microsoft Entra ID and managed identity. Treat key-based authentication as a risk because it bypasses granular RBAC boundaries.
- Separate Foundry resource scope from project scope before reviewing access, quota, networking, or deployment rights.
- Verify connected Storage, Key Vault, AI Search, networking, logging, and monitoring as separate Azure resources; do not assume Foundry configuration governs them.
- Require explicit approval before any write, delete, role assignment, network change, deployment change, model deployment, quota movement, diagnostic retention, guardrail, tool-registration, or data residency mutation.

## High-risk assumptions to kill

- "Project isolation means dependency isolation." It does not; connected resources have their own access and network controls.
- "Contributor is fine for developers." Usually too broad; challenge with Foundry-specific roles and scoped assignments.
- "The model is available because it appears in docs." Docs do not prove regional or tenant quota availability.
- "Private endpoint on Foundry secures the whole workload." Supporting resources and outbound paths still need verification.
- "Tool access is harmless because it is just an assistant." Tool operations inherit identity, RBAC, policy, network, and approval risks.

## Evidence labels

- `docs_only`: Microsoft Learn evidence only. Safe for design guidance, not for tenant assertions.
- `sampled_read_only`: read-only configured-environment evidence was sampled. State exact scope and timestamp in the answer.
- `user_supplied`: sanitized user evidence was provided. State that it was not independently verified.
- `mutation_ready`: docs plus current-state evidence plus explicit approval plus rollback path exist.

## Mutation boundaries

Block or escalate when a plan changes RBAC assignments, project membership, network/public access, Key Vault connections, model deployment or deletion, quota allocations, diagnostic retention, guardrails, tool registration, or data residency posture.

## Minimum safe evidence before approval

- Target Foundry resource and project boundary identified without exposing sensitive IDs.
- Role scope and role definition checked against Microsoft Learn.
- Region, deployment type, quota, and feature availability checked for the target scenario.
- Connected resources reviewed for RBAC, network, and secret boundaries.
- Diagnostics and audit trail destination identified.
- Rollback or break-glass path documented for each planned change.
