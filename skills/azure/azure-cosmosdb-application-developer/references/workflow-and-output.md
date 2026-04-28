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
   - Check the target API/workload, dominant read/write patterns, and whether the workload is greenfield or existing.
   - Stress-test data modeling: embed vs reference, unbounded arrays, precalculated aggregates, and document growth.
   - Prefer point reads and partition-aware queries; challenge lazy cross-partition scans and RU-blind joins-by-application.
   - Call out transactional batch scope, session-consistency behavior, and SDK request-charge observability.

4. **Check adjacent roles the user may be missing**
   - **Azure Cosmos DB Platform Operator** when the real issue is account/platform posture rather than code-facing design.
   - **Azure Observability Investigator** when the next problem is tracing query latency, alerts, or telemetry gaps.
   - **Azure AI Foundry Ops Governor** when Cosmos DB becomes part of a larger AI application governance path.

## Output contract

Use this structure:

1. **Verdict**
2. **Evidence level**
3. **Key findings**
4. **Safest next actions**
5. **Open questions**
