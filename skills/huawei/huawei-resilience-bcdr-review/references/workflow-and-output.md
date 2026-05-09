# Workflow and output contract

Use this reference only when performing a full BCDR review, HA gap analysis, or recovery readiness assessment.

## Review domains

Check these areas before giving a verdict:

- Workload criticality: tier classification, business impact of outage, contractual SLA
- RTO/RPO targets: documented targets vs tested evidence; label as aspirational if untested
- GaussDB HA: instance type, failover mode, AZ pair configuration, cross-region read replica status
- CBR backup: vault region, retention policy, backup frequency, restore test evidence
- CCE multi-AZ: node group AZ distribution, pod anti-affinity rules, cross-region cluster existence
- DRS replication: task status, replication lag (seconds), last consistency check result and date
- ECS HA: Auto Scaling group span, ECS placement group, health check linkage to ELB
- Runbook completeness: documented steps, last tested date, who owns execution

## Safe workflow

1. **Frame scope**
   - Workload type and criticality tier:
   - Stated RTO/RPO targets:
   - Current-state evidence:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence (CBR vault region, DRS task status, CCE node AZ spread).
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test design**
   - What fails if a single AZ goes down?
   - What fails if the primary region is unavailable?
   - Is the CBR vault in a different region from the production workload?
   - Is the DRS task active and within acceptable replication lag?
   - When was the last DR drill performed and what was the result?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud BCDR Review: <scope>
## Workload criticality and RTO/RPO targets
## Current HA architecture assessment (GaussDB/CCE/ECS)
## Cross-region/AZ redundancy gaps
## CBR backup coverage and cross-region vault verification
## DRS replication lag and consistency status
## Recovery test evidence (last drill date, scope, result)
## Prioritized BCDR improvements
```

Each section must include an evidence level label.
