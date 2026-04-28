# Workflow and output contract

Use this reference only when you are performing the full review or implementation-guidance pass.

## Workflow

1. **Scope the target**
   - Confirm API/workload if known: NoSQL, Mongo, Cassandra, Gremlin, Table, or unknown.
   - Confirm whether the user needs this role specifically, not an adjacent Azure role.
   - Confirm whether the question is about an existing workload or a new design.

2. **Establish evidence level**
   - Use live Azure MCP evidence when available.
   - Otherwise use official docs plus sanitized user evidence.
   - Explicitly label unknowns.

3. **Stress checks**
   - Check account, region, API surface, and whether the question is about an existing workload or a new design.
   - Stress-test partition-key choice, logical-partition growth, hot-partition risk, and query alignment.
   - Check throughput model, RU consumption, throttling behavior, indexing posture, and multi-region tradeoffs.
   - Call out adjacent roles when the dominant problem is RBAC, networking, observability, or cost governance.

4. **Check adjacent roles the user may be missing**
   - **Azure RBAC Review** when the real issue is overbroad account or data-plane access.
   - **Azure Network Topology Review** when private endpoints, DNS, routing, or egress control dominate the problem.
   - **Azure Observability Investigator** when the question becomes metrics, alerts, logs, or telemetry gaps.
   - **Azure Cost Optimization Governor** when the issue is sustained RU waste or poor autoscale governance.

## Output contract

Use this structure:

1. **Verdict**
2. **Evidence level**
3. **Key findings**
4. **Safest next actions**
5. **Open questions**
