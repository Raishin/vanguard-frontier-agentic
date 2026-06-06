# OCI Maestro Routing Skill Operations

> Version note: OCI service behavior, catalog entries, and tooling change over time. Verify exact command syntax, permissions, regional availability, feature maturity, and catalog targets before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Routing to a broad team when one specialist is enough.
- Auto-dispatching live-guard skills for production-changing actions.
- Answering as a generic OCI expert instead of routing or narrowing.
- Mixing Azure, AWS, and OCI specialists because the words sound similar.

## Officially grounded service shape

- Official OCI documentation provides service behavior and cloud concepts, but it does not prove which marketplace agent should be invoked or the user’s live posture.
- The repo catalog defines the available OCI specialists and is the source of truth for route targets in this workspace.
- OCI API evidence through the user’s configured read-only OCI MCP may support current-state scoping for a specialist, but the maestro must not turn sampled evidence into production approval.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Narrowest specialist wins for single-domain tasks.
- Use parallel routing only for genuinely multi-domain tasks and cap the team.
- Never auto-dispatch live-guard specialists without explicit human confirmation, blast-radius assessment, and rollback path.
- If the task is not OCI, say so and route to the correct provider only if that provider is available.

## Minimal safe implementation flow

- Classify provider, domain, risk level, and whether live mutation is requested.
- Select one specialist unless the task clearly spans multiple domains.
- For live-guard routing, require explicit confirmation and evidence of rollback/blast-radius review.
- Return route decision, why, evidence level, blockers, and next safe handoff.

## High-risk assumptions to kill

- “More agents means better answer.”
- “A live-guard can inspect safely and then decide to mutate.”
- “Provider-neutral means any cloud specialist is acceptable.”
- “Routing is complete even if the target agent is missing from the catalog.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check catalog target exists before naming it as routable.
- Check live-guard status before dispatch.
- Check provider and domain taxonomy before routing.
- Record evidence level and open questions when routing depends on missing context.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations or live-guard dispatch have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to auto-dispatch a live-guard for destructive work.
- The requested provider is not OCI.
- The task is too vague to choose a specialist without inventing scope.
