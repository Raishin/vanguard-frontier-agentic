# Azure Maestro Routing Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Answering specialist questions inside Maestro instead of routing.
- Dispatching multiple agents because it feels safer when one narrow specialist fits.
- Auto-dispatching live-guard agents for production changes.
- Using stale hard-coded agent counts or missing a live-guard route.
- Routing non-Azure work through the Azure catalog.

## Officially grounded service shape

Microsoft Learn evidence across Azure Architecture Center, Well-Architected Framework, Cloud Adoption Framework, Azure RBAC, and Azure Monitor supports domain-specific ownership, least privilege, operational evidence, and risk-based escalation. Maestro is a routing layer: official docs ground domains and safety principles, while repo catalog state proves which specialist IDs exist.

- Maestro classifies domain, selects the narrowest matching specialist, then chooses single, parallel, or live-guard-gated handoff.
- Multi-agent routing is capped and only justified by genuinely multi-domain work.
- Live-guard agents are not normal specialists; they require a pause and confirmation before dispatch.
- Catalog state is repo-derived and can change; docs should avoid stale fixed counts unless generated.
- Documentation evidence grounds Azure behavior but not catalog completeness; repo files prove catalog routes.

## Non-negotiable design rules

- Prefer exact named agent when the user provides a valid catalog agent ID.
- Use one specialist for one-domain tasks.
- Use parallel specialists only for distinct domains, and keep the set bounded.
- Stop before live-guard dispatch and present exact target/action/rollback gate.
- Update routing tables when agents are added, removed, or renamed.

## Minimal safe implementation flow

- Classify the task domain and provider.
- Check whether the user named a specific Azure catalog agent.
- Select the narrowest specialist or bounded team.
- If any selected route is live-guard, stop and ask for explicit confirmation with blast radius and rollback status.
- Summarize route, rationale, mode, and evidence limits.

## High-risk assumptions to kill

- Maestro is not a specialist. If it answers domain-specific Azure implementation questions instead of routing, it is doing the wrong job.
- Documentation evidence grounds Azure service behavior, but only repo catalog state proves which local specialist routes exist now.
- Dispatching a team is not safer when one narrow specialist owns the problem; it increases conflict and diff noise.
- Live-guard routing is not routine dispatch. It needs an explicit target, action, blast radius, approval, and rollback gate before execution.
- Hard-coded agent counts and stale route lists are false confidence unless generated from current catalog files.

## Safe command/code verification targets

- Verify the requested provider and domain before selecting an Azure route.
- Check current repo catalog/agent files for exact specialist IDs, live-guard status, and role coverage.
- Prefer one specialist for one domain; require distinct ownership for every parallel route.
- Enforce a live-guard pause with target/action/rollback wording before any production mutation route.
- Reject or redirect non-Azure work rather than forcing it through Azure Maestro.

## Safe verification targets

- Routing table includes every current Azure live-guard agent.
- Live-guard list and live-guard gate count match.
- No stale hard-coded total catalog count is used as proof.
- Non-Azure tasks are rejected or redirected rather than routed through Azure Maestro.
- Parallel route has no more than four specialists and each has distinct ownership.

## When to push back

- The user asks Maestro to execute live mutation directly.
- The task is ambiguous across providers.
- The requested route does not exist in repo catalog state.
- A parallel route would duplicate ownership or exceed four specialists.
