# Safety checklist — SAP Maestro

Use before any dispatch decision, especially when routing leads to a live-tier skill.

## Non-negotiables

- Do not accept, store, echo, or propagate SAP system credentials, client secrets, OAuth tokens, BTP subaccount IDs, S/4HANA system IDs, tenant identifiers, or user passwords.
- Do not invent skill IDs or agent names. Only route to skills and agents declared in the catalog.
- Do not bypass the live-guard gate. A routing classification alone is not authorization to access a live system.
- Do not propose dispatch to a `mutating-runtime` skill without explicit step-by-step approval from the user.
- Do not conflate domain guidance across routing buckets (e.g., do not apply S/4HANA transport logic to BTP platform requests).
- Do not mark a routing decision as `catalog-evidence` unless the skill or agent is actually declared in this repository's catalog.
- Do not skip the split-routing step for `cross-domain` requests. Routing a cross-domain request to a single-domain skill will produce wrong guidance.

## What people get wrong

- **Skipping classification**: Dispatching directly to a skill without classifying the domain first. This causes wrong-skill errors and conflated guidance.
- **Treating routing as authorization**: Classification and routing tell you *which* skill to load. They do not authorize live-system access.
- **Conflating BTP platform with S/4HANA**: BTP subaccount operations and S/4HANA system configuration are different domains with different live-tier rules.
- **Routing `unrouted` domains to the closest-sounding skill**: If no routing table entry matches, return `unrouted`. Do not approximate.
- **Forgetting the live-guard gate**: Every `gated` dispatch mode requires explicit user confirmation. This is not optional.

## When to push back

- Push back when the user asks to skip classification and go straight to a live-tier skill.
- Push back when the user provides a system URL or credentials and asks to "just connect."
- Push back when the routing table returns `unrouted` and the user asks you to pick the "closest" skill anyway.
- Push back when the dispatch would be to `sap-guarded-transport-import` without the 17-step sequence acknowledged.

## Evidence labels

Use exactly one of:

- `documentation-based` — grounded in SAP official docs or BTP taxonomy
- `catalog-evidence` — grounded in a declared skill/agent entry in this repo
- `user-provided evidence` — stated by the user in this session
- `inference` — derived reasoning not directly confirmed by docs or catalog

Inference must always be labeled. Never present inferred routing as documentation-based.
