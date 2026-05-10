# Safety checklist

Use this reference before emitting any routing decision that touches a live IONOS Cloud environment or a potentially destructive operation.

## Non-negotiables

- Never route to a live-guard agent without confirming the required hard-stop prerequisites (snapshot verification, named approving identity, rollback plan).
- Never attempt to answer domain questions directly at the routing layer — forward to the narrowest applicable specialist.
- Never request, echo, or transmit bearer tokens, API keys, credentials, database connection strings, or customer account identifiers.
- Do not invent datacenter IDs, cluster UUIDs, resource names, or live configuration state.
- Do not route a DBaaS operation to the live-guard agent unless backup existence is confirmed in writing.
- Label all claims: `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.

## Stress checks

- Is the blast-radius class correctly classified? Topology changes and DBaaS mutations carry datacenter-wide and data-loss risk respectively.
- Is this a multi-domain request that requires more than one specialist in sequence?
- Does this task touch a declared GDPR region — if so, has data residency been confirmed?
- Are the safe entry conditions for the target agent actually met, or am I routing prematurely?
- Is the evidence level sufficient to route, or should I ask for current-state evidence first?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. A verbal description of infrastructure without evidence is `inference` — flag it as such before routing to any agent that may act on it.
