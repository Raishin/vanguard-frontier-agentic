# Workflow and output contract

Use this reference only when performing full triage, routing an ambiguous multi-domain request, or generating a handoff context statement for a specialist.

## Review domains

Check these areas before classifying a request:

- Scope: is this a read-only advisory, a live mutation, or a billing-impacting change?
- Affected domain(s): cost/billing, capacity, security, instance lifecycle, Object Storage
- Live-guard requirement: does the request touch VPS/VDS create/reinstall/cancel or Object Storage deletion?
- Contract period exposure: does any part of the request bind a new 1/3/6/12-month obligation?
- Evidence level: is the user's current state known from live evidence, user-provided data, or inference?

## Safe workflow

1. **Frame scope**
   - Request type (advisory, planning, mutation):
   - Contabo account state available (live evidence vs. user description):
   - Domain(s) implicated:
   - Contract period or billing obligation involved:
   - Explicit non-goals:
2. **Classify the request**
   - Map against the routing domain table in the SKILL.md.
   - If multiple domains are implicated, list them and resolve priority (billing/lifecycle safety first).
   - If the request is ambiguous, ask one clarifying question before routing.
3. **Check live-guard threshold**
   - If the request may create, reinstall, or cancel a VPS/VDS → route to `contabo-live-instance-lifecycle-guard`.
   - If the request may delete buckets or objects → route to `contabo-live-storage-operations-guard`.
   - Do not route live-guard requests to advisory skills.
4. **Produce the handoff**
   - State the classified domain, evidence level, recommended specialist, and scope statement.
   - Include any blockers, assumptions, or open questions the specialist must resolve.

## Output contract

Return this structure:

```markdown
# Contabo Maestro: Routing Decision
## Classification
- Domain(s): <list>
- Request type: advisory | planning | live mutation
- Evidence level: live | user-provided | inference
## Specialist recommendation
- Skill: <skill-name>
- Scope statement: <one sentence describing the bounded task>
## Blockers or ambiguities
- <item or none>
## Open questions for specialist
- <question or none>
```
