# Workflow and output contract

Use this reference only when performing a full data analytics platform review or pipeline implementation guidance.

## Data analytics areas to check

- DWS: cluster version, node count, storage utilization, vacuum/analyze schedule, query plan health, connection pool saturation
- DLI: Spark job configuration (driver/executor resources), Flink checkpoint interval, restart strategy, CU budget
- MRS: cluster composition (HDFS/HBase/Spark), scaling policy, job queue saturation, data locality
- DataArts: integration job inventory, quality rule coverage, lineage completeness, catalog tag governance
- Cross-service pipelines: data lineage from source through transformation to destination, dependency graph

## Safe workflow

1. **Frame scope** — confirm target services, data volume, SLA requirements, and non-goals
2. **Collect evidence** — prefer live cluster metrics and job logs; label all evidence types
3. **Stress-test** — DWS table bloat risk, Flink checkpoint failure scenarios, MRS scale impact, lineage break risk
4. **Recommend safest action** — phased optimization, maintenance-window scheduling, rollback path

## Output contract

Return this structure:

```markdown
# Huawei Cloud Data Analytics: <scope>
## Scope and evidence level
## DWS cluster health and performance posture
## DLI job checkpoint and restart configuration
## MRS cluster status
## DataArts pipeline and lineage posture
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
