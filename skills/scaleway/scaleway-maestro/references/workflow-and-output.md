# Workflow and output contract

Use this reference only when performing full task classification, multi-domain decomposition, or specialist dispatch.

## Classification domains

Check these signals before issuing a routing verdict:

- IAM signals: API key scopes, permission sets, service account bindings, key expiry, organization vs project scope
- Cost signals: billing review, rightsizing, reserved instances, idle resources, Cockpit spend
- Kapsule advisory signals: cluster readiness, CNI selection, node pool sizing, PDB coverage, version planning
- Networking signals: VPC layout, Private Network attachment, security group rules, Load Balancer config, HA topology
- Live mutation signals: explicit intent to change a running Kapsule cluster or node pool

## Routing workflow

1. **Frame the request**
   - What service or resource is involved?
   - Is this a read/advisory or a live mutation request?
   - Does it span multiple domains?
2. **Identify classification signals**
   - List the domain signals found in the request.
   - Separate confirmed signals from inference.
   - If fewer than two domain signals are present and the domain is still ambiguous, ask one focused clarifying question.
3. **Select the narrowest specialist**
   - Map the strongest signal to the routing table.
   - If the request spans domains, identify the primary domain and note the secondary for handoff.
4. **Issue the routing verdict**
   - State domain, specialist, and signals used.
   - Note any pre-conditions or context the specialist will need.
   - Do not attempt to answer the specialist's question.

## Hard rule: live-guard dispatch

Never route to `scaleway-live-kapsule-rollout-guard-agent` without explicit user intent for a live cluster mutation. Advisory questions about Kapsule go to `scaleway-kapsule-platform-operator-agent`.

## Output contract

Return this structure:

```markdown
# Scaleway Maestro: Routing Verdict

## Domain verdict
- Primary domain: <IAM | Cost | Kapsule advisory | Networking | Live Kapsule mutation>
- Confidence: <confirmed | inferred | ambiguous>
- Signals used: <list signals from request>

## Recommended specialist
- Agent: <agent name>
- Why: <one sentence rationale>

## Handoff context
- What the specialist needs: <key facts from the request>
- Pre-conditions: <any missing information the specialist should ask for>

## Blockers
- <blocking ambiguity or clarifying question, or "none">
```
