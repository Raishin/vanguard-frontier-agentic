# Workflow and output contract

Use this reference only when performing a full data replication review or migration planning implementation guidance.

## Data replication areas to check

- DRS tasks: source/target database types, sync mode (full/incremental), current lag, replication user privilege
- CDM jobs: source/target connector types, deduplication logic, retry configuration, schedule
- DMS Kafka: cluster configuration, partition count plan, topic inventory, consumer group offsets
- Cutover sequencing: lag threshold, source quiesce procedure, connection string switchover, DNS update
- Rollback plan: source-as-standby window, rollback trigger criteria, data reconciliation procedure

## Safe workflow

1. **Frame scope** — confirm source, target, sync mode, cutover date, and non-goals
2. **Collect evidence** — prefer live DRS lag metrics and CDM job status; label all evidence types
3. **Stress-test** — lag under load, CDM duplicate risk, Kafka partition saturation, rollback feasibility
4. **Recommend safest action** — staged cutover with lag-gated go/no-go criteria

## Output contract

Return this structure:

```markdown
# Huawei Cloud Data Replication: <scope>
## Scope and evidence level
## DRS task inventory and lag status
## CDM job deduplication posture
## DMS Kafka partition plan
## Cutover sequencing
## Rollback plan
## Open questions
```

Each section must include an evidence level label.
