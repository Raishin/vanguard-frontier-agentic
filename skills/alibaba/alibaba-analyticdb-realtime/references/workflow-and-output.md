# Workflow and output contract

Use this reference only when performing a full AnalyticDB or Hologres review, incident triage, or implementation guidance.

## Real-time analytics areas to check

- Cluster configuration: ADB MySQL (elastic vs. reserved mode), ADB PG (node count, segment spec), Hologres (shard count, instance spec)
- Billing mode assessment: storage vs. compute billing split; elastic vs. reserved mode fit for workload pattern
- Query performance: slow query log, query plan analysis, distribution key skew (ADB PG), shard hotspots (Hologres)
- DAS diagnostics: autonomous recommendations pending approval, index suggestions, storage trends, connection saturation
- MaxCompute acceleration (Hologres): foreign table configuration, external table access latency, data synchronization lag
- Real-time ingestion: Flink/Kafka connector configuration, write throughput, and ingestion lag monitoring

## Safe workflow

1. **Frame scope** — confirm target cluster, workload type, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live state; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius? what queries or jobs fail? what cost spike is possible?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path

## Output contract

Return this structure:

```markdown
# Alibaba Cloud Real-Time Analytics: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
