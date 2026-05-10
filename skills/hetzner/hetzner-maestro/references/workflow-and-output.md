# Workflow and output contract

Use this reference only when performing a full routing triage, multi-domain classification, or producing a structured routing response.

## Routing domains

Classify the request against these domains before routing:

- **Cost / FinOps** — bill review, rightsizing, idle resource detection, spend spike
- **Infrastructure / Security** — firewall rules, load balancer config, network topology, IP exposure, region placement
- **Capacity / Growth** — resource limits, quota exhaustion, growth trajectory, project splits
- **Live Firewall mutation** — add/update/delete firewall rules, attachment changes (hard-stop guard required)
- **Live Server lifecycle** — server creation, deletion, type change, power operation (hard-stop guard required)

## Safe routing workflow

1. **Frame the request**
   - Incoming request summary:
   - Domains detected (list all that apply):
   - Evidence or signals used for classification:
   - Is live mutation involved? (yes → route only to a live-guard specialist)

2. **Collect and label evidence**
   - Prefer live Hetzner API evidence if available via MCP.
   - Otherwise use user-provided sanitized evidence, repository config, or official Hetzner docs.
   - Label each input as `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Assess routing blockers**
   - Is the target resource (server ID, firewall ID, project) confirmed?
   - Is API token scope known and appropriate?
   - Are there ambiguities that must be resolved before the specialist can act safely?

4. **Route to the narrowest qualified specialist**
   - Prefer a single specialist when the request clearly falls in one domain.
   - Prefer `hetzner-maestro` decomposition when the request spans multiple domains.
   - Never route to a live-guard specialist without hard-stop pre-flight context.

## Output contract

Return this structure:

```markdown
# Hetzner Cloud Routing: <request summary>
## Domain classification
- Domains detected:
- Evidence and signals:
## Routing decision
- Recommended specialist:
- Reason:
## Blockers and open questions
- Blocker (if any):
- Open questions for the specialist:
## Decomposed sub-tasks (if multi-domain)
1. <sub-task> → <specialist>
2. <sub-task> → <specialist>
```
