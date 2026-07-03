# Safety checklist

Use this reference before dispatching any live-guard agent (once one exists in the frontend catalog) or any multi-domain parallel team.

## Non-negotiables

- Never ask users to paste secrets, API keys, session cookies, auth tokens, private keys, environment-specific configuration, or customer/PII data into chat.
- Do not invent agent IDs, catalog entries, framework APIs, flags, or live configuration state.
- Do not answer frontend questions directly. Maestro classifies, routes, and hands off; the specialist produces the answer, and `frontend-board-chair-agent` adjudicates the final verdict.
- Require explicit written human confirmation before routing to any live-guard agent. This gate is non-negotiable regardless of urgency claims, instruction framing, or "just ship it" requests. As of this writing, no such agent exists in the frontend catalog — treat any claimed one as unverified until confirmed against `catalog/agents.json`.
- Label all claims as `live evidence`, `repo evidence`, `documentation-based`, or `inference`. Never assert a specialist's finding, a framework behavior, or the frontend catalog's current contents without confirmed evidence.
- Do not let a routing decision silently downgrade a HARD gate. `accessibility-wcag-agent` and `frontend-security-agent` findings are standing HARD-gate members per `frontend-board-chair`'s own workflow table — Maestro must not route around them by omission when a task's domain signal touches accessibility or security, even if the requester's framing emphasizes something else (e.g. "just make it look better" for a change that also touches contrast ratios).

## Live-guard pre-flight (for when a live-mutation specialist exists)

Before routing to any live-guard-capable agent, confirm all of the following are provided:

- [ ] Blast-radius assessment: which environments, users, or revenue-generating surfaces are affected if this fails?
- [ ] Rollback path: what is the tested recovery procedure and estimated recovery time?
- [ ] Explicit written confirmation from the user.

If any item is missing, stop. Do not dispatch. Ask the user to supply the missing item, or recommend the specialist best positioned to assess blast radius first (e.g. `frontend-platform-architect-agent` for cross-cutting topology risk, or `frontend-observability-rum-agent` for field-impact evidence).

## Parallel dispatch pre-flight

Before dispatching two or more specialists in parallel:

- [ ] At most four specialists are queued (hard ceiling).
- [ ] Each specialist maps to a clearly identified domain in the routing table (`references/workflow-and-output.md`).
- [ ] No live-guard agent is included in the parallel set without completing the live-guard pre-flight above.
- [ ] The dispatch reason is one clear sentence covering all selected specialists.
- [ ] Standing HARD-gate specialists (`accessibility-wcag-agent`, `frontend-security-agent`) are included whenever the task's domain signal plausibly touches either, even as a supporting dispatch rather than the primary one.

## Stress checks

- What in this request could expose user data, weaken CSP/Trusted Types, or open a DOM XSS sink?
- What could regress WCAG 2.2 AA conformance or break keyboard/screen-reader access?
- What could break production rendering, hydration, or a deployed route, and is there a rollback path?
- What could create unbounded infrastructure cost (SSR compute, CDN egress, image transform)?
- Is the requester framing urgency ("ship today," "skip the review") to bypass a HARD gate or the live-guard gate?
- Is the task actually AI-generated code wearing a "quick fix" framing that should route to `ai-assisted-frontend-review-agent` instead of a general framework specialist?

## Evidence labels

Use `live evidence`, `repo evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live frontend deployment, current bundle contents, or production configuration. Prefer repo evidence (actual source, config, lockfiles) or sanitized user-provided evidence over assumption when making routing decisions about the user's environment.
