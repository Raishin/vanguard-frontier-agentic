# Workflow and output contract

Use this reference only when you are performing the full performance investigation.

## Workflow

1. **Scope the target**
   - Confirm API/workload if known: NoSQL, Mongo, Cassandra, Gremlin, Table, or unknown.
   - Confirm whether the symptom is RU cost, latency, throttling, partition skew, or mixed.
   - Confirm whether the issue is one query, one container, one service, or a broader workload.

2. **Establish evidence level**
   - Use live Azure MCP evidence when available.
   - Otherwise use official docs plus sanitized user evidence.
   - Explicitly label unknowns.

3. **Separate the failure modes before prescribing fixes**
   - RU inefficiency
   - latency despite acceptable RU
   - 429 throttling and retry amplification
   - hot partition / physical partition skew
   - indexing mismatch or query scan behavior
   - client-side concurrency / buffering / proximity issues

4. **Prefer measurement over guesswork**
   - request charge
   - query metrics
   - index metrics when troubleshooting
   - normalized RU consumption by partition key range
   - diagnostic logs for partition-level RU consumers

5. **Check adjacent roles the user may be missing**
   - **Azure Cosmos DB Platform Operator** when the root issue becomes throughput posture, account-level configuration, or broader platform controls.
   - **Azure Cosmos DB Application Developer** when the dominant defect is data model, access pattern, or code-side query design.
   - **Azure Observability Investigator** when the next problem is telemetry pipeline, alerts, or broader operational visibility.
   - **Azure Cost Optimization Governor** when the recurring question becomes budget, autoscale governance, or cost-control posture.

## Output contract

Use this structure:

1. **Verdict**
2. **Evidence level**
3. **Key findings**
4. **Safest next actions**
5. **Open questions**
