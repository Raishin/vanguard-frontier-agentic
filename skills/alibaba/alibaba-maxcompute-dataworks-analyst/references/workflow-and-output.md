# Workflow and output contract

Use this reference only when performing a full MaxCompute or DataWorks review, incident triage, or big data optimization plan.

## Big data areas to check

- MaxCompute billing mode: CU package utilization vs. on-demand charges; job-level cost breakdown; hybrid mode assessment
- Query cost drivers: top queries by CU consumption, full table scans on large tables, missing partition pruning
- Partitioning and clustering: partition column selection, pruning effectiveness, lifecycle policies for partition expiration
- DataWorks scheduling: job dependency graph health, failed/stuck jobs, retry configuration, alert rules
- DataWorks Data Integration: sync task throughput, error rates, incremental vs. full sync mode
- Quick BI data sources: dataset refresh schedule, direct query vs. cached mode, dashboard performance
- PAI integration: MaxCompute table access by PAI training jobs, data lineage, and data quality gates

## Safe workflow

1. **Frame scope** — confirm target workspace, billing mode, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live MaxCompute cost reports and DataWorks job logs; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the cost impact of switching billing mode? what jobs fail without dependency? what tables are unpartitioned?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path

## Output contract

Return this structure:

```markdown
# Alibaba Cloud MaxCompute and DataWorks: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
