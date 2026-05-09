# Workflow and output contract

Use this reference only when performing a full migration planning review or cutover sequencing guidance.

## Migration areas to check

- MgC: workload discovery completeness, dependency mapping, wave plan, migration tracking dashboard
- SMS: agent installation status, network path verification (source → Huawei Cloud), supported OS versions, replication status
- DRS: task inventory, source database privilege level, replication lag, cutover readiness
- OMS: source connector configuration, checksum verification enabled, bandwidth throttle setting, progress
- Cutover sequencing: database → app → DNS/LB ordering, lag gate criteria, maintenance window, rollback trigger
- Post-cutover: source-as-standby retention window, rollback procedure, decommission checklist

## Safe workflow

1. **Frame scope** — confirm source environment, target Huawei Cloud region, wave plan, and non-goals
2. **Collect evidence** — prefer live MgC dashboard and DRS lag metrics; label all evidence types
3. **Stress-test** — lag under load, SMS network path failures, OMS checksum miss rate, rollback feasibility
4. **Recommend safest action** — lag-gated cutover, staged wave execution, 24h source-as-standby

## Output contract

Return this structure:

```markdown
# Huawei Cloud Migration Plan: <scope>
## Scope and evidence level
## Workload inventory and dependency map
## DRS replication status and lag
## SMS agent and network path verification
## OMS checksum posture
## Cutover sequencing
## Rollback plan
## Open questions
```

Each section must include an evidence level label.
