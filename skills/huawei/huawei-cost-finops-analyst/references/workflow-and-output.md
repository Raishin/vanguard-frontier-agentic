# Workflow and output contract

Use this reference only when performing a full cost and FinOps review or optimization implementation guidance.

## FinOps areas to check

- CBC spend: total spend, top services, top regions, top enterprise projects, month-over-month trend
- RI coverage: RI inventory by flavor, utilization rate, expiry dates, coverage gaps
- CUD coverage: commitment amount, actual utilization, coverage efficiency
- Cost Center tags: tag coverage rate per resource type, untagged resource count
- Budget alerts: budget thresholds per enterprise project, SMN notification topic bindings, recent breach events
- Enterprise project cost isolation: cross-project cost leakage, cost attribution accuracy

## Safe workflow

1. **Frame scope** — confirm billing period, enterprise project scope, and non-goals
2. **Collect evidence** — prefer CBC export or user-provided billing data; label all evidence types
3. **Stress-test** — coverage gaps, commitment utilization risk, missing tags
4. **Recommend safest action** — phased RI/CUD purchase, tag remediation plan, budget threshold review

## Output contract

Return this structure:

```markdown
# Huawei Cloud Cost and FinOps: <scope>
## Scope and evidence level
## CBC spend summary
## RI and CUD coverage analysis
## Cost Center tag posture
## Budget alert governance
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
