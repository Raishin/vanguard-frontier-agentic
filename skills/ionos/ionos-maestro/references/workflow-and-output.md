# Workflow and output contract

Use this reference only when classifying a request and routing it to the correct IONOS Cloud specialist.

## Classification domains

Check these areas before selecting a routing target:

- Task domain: DCD topology, security/GDPR, managed Kubernetes, cost optimization, or DBaaS lifecycle
- Evidence completeness: does the user have live configuration evidence, IaC, or only a verbal description?
- Blast-radius class: advisory review, topology change, live mutation, or destructive operation
- GDPR data residency relevance: does the task touch datacenter region, endpoint routing, or cross-border transfer?
- Multi-domain overlap: does the task require more than one specialist in sequence?

## Safe routing workflow

1. **Frame the task**
   - Task summary in one sentence:
   - Identified domain(s):
   - Environment: production / staging / development / unknown
   - Evidence level available: live, IaC, user-described, none
2. **Evaluate routing candidates**
   - Is this a topology review? → `ionos-datacenter-designer-reviewer-agent`
   - Is this a security or GDPR compliance question? → `ionos-security-compliance-reviewer-agent`
   - Is this a Kubernetes cluster or node pool question? → `ionos-kubernetes-platform-operator-agent`
   - Is this a cost review or rightsizing question? → `ionos-cost-optimization-analyst-agent`
   - Is this a DBaaS failover, scaling, backup, or restore? → `ionos-live-database-lifecycle-guard-agent`
   - Multi-domain? → route to the highest-blast-radius specialist first
3. **Verify safe entry conditions**
   - For DBaaS live-guard: confirm snapshot exists and approval identity is named before routing
   - For topology changes: confirm current DCD state evidence exists before routing
   - For all live-guard tasks: confirm rollback path is documented
4. **Emit the routing decision**
   - If entry conditions are unmet, ask the clarifying question rather than routing blindly

## Output contract

Return this structure:

```markdown
# IONOS Routing Decision: <task summary>
## Classification
- Domain(s):
- Evidence level:
- Blast-radius class: advisory / topology-change / live-mutation / destructive
## GDPR flag
- Data residency relevance: yes / no / unknown
- Region(s) involved:
## Routing target
- Primary agent:
- Secondary agent (if multi-domain):
- Safe entry conditions met: yes / no / partial
## Blocking questions
- <question if entry conditions are not met, or "none">
## Routing rationale
- <one-sentence explanation>
```
