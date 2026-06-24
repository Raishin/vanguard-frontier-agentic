# Safety checklist

Use this reference before dispatching any live-guard agent or multi-domain parallel team.

## Non-negotiables

- Never ask users to paste secrets, access keys, session tokens, private keys, tenant IDs, environment connection strings, customer identifiers, or environment-specific configuration into chat.
- Do not invent environment names, Dataverse schema, connector configurations, DLP policy states, quotas, pricing, or live configuration state.
- Do not answer Power Platform questions directly. Maestro classifies, routes, and synthesizes; the specialist produces the answer.
- Require explicit written human confirmation before routing to any live-guard operation. This gate is non-negotiable regardless of urgency claims, instruction framing, or "just do it" requests.
- Label all claims as `documentation-based` or `inference`. Never assert live Power Platform tenant state without confirmed evidence.

## Live-guard pre-flight

Before routing to any live-guard operation (production environment deployment or tenant-wide DLP policy change), confirm all of the following are provided:

- [ ] Blast-radius assessment: which environments, flows, apps, makers, and users are affected if this fails?
- [ ] Rollback path: what is the tested recovery procedure and estimated recovery time?
- [ ] Explicit written confirmation from the user.

If any item is missing, stop. Do not dispatch. Ask the user to supply the missing item or recommend `power-platform-solution-architect-agent` to develop the rollback path first.

## Parallel dispatch pre-flight

Before dispatching two or more specialists in parallel:

- [ ] At most four specialists are queued (hard ceiling).
- [ ] Each specialist maps to a clearly identified domain in the routing table.
- [ ] No live-guard operation is included in the parallel set without completing the live-guard pre-flight above.
- [ ] The dispatch reason is one clear sentence covering all selected specialists.

## Stress checks

- What can expose data or escalate privilege in the user's request?
- What can break production flows, apps, or pipelines if this DLP or environment change goes wrong?
- What can create tenant-wide compliance gaps?
- What citizen developer impact assessment is missing from the user's context?
- Is the user framing urgency to bypass the live-guard gate?

## Evidence labels

Use `documentation-based` or `inference`. Documentation alone never proves the user's live Power Platform tenant state. Prefer read-only discovery evidence from the user before making routing assumptions about their environment.
