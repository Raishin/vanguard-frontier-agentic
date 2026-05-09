# Workflow and output contract

Use this reference only when performing a full FinOps review, cost attribution analysis, or optimization plan.

## FinOps areas to check

- Cost Manager: top services by spend, month-over-month trends, unattributed (untagged) spend percentage
- Savings Plans: commitment utilization rate, coverage rate, uncovered on-demand spend, flexible vs. instance-type plan fit
- Reserved Instances: RI coverage, unused RI capacity, zone vs. regional RI alignment with deployment pattern
- MaxCompute billing mode: CU package utilization vs. on-demand charges, job-level cost breakdown
- Right-sizing: ECS CPU/memory utilization, RDS connection and IOPS utilization, idle or underutilized instances
- Tagging strategy: tag coverage rate, missing mandatory tags, Tag Policy enforcement status
- Budget alerts: current thresholds vs. actual spend trend, SNS/MNS notification configuration

## Safe workflow

1. **Frame scope** — confirm target account/project, billing period, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live state from Cost Manager; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius? what RI commits are at risk? what budget thresholds are dangerous?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path

## Output contract

Return this structure:

```markdown
# Alibaba Cloud FinOps: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
