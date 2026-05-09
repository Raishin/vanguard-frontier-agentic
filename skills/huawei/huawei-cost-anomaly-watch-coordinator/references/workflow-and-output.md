# Workflow and output contract

Use this reference only when performing a full cost anomaly investigation or cost optimization review for a Huawei Cloud environment.

## Review domains

Check these areas before giving a verdict:

- CBC Cost Center deltas: day-over-day spend changes by service, whether the change exceeds the 15% threshold, and whether it is explained by planned workload changes
- Budget alert coverage: whether budgets are configured for key services, whether alert thresholds are set at meaningful levels, and whether alert actions (SMS/email/function) are wired
- ECS billing mode: whether any ECS instances have run On-Demand for more than 30 days at stable load and are candidates for Yearly/Monthly conversion
- GaussDB billing mode: whether any production GaussDB instances are billed On-Demand and are candidates for Yearly/Monthly conversion
- OBS request costs: whether request cost spikes are explained by lifecycle policy gaps, application retry storms, or unexpected data access patterns
- EVS volume waste: whether any EVS volumes are unattached and billing at storage rate with no confirmed owner intent
- DWS idle clusters: whether any DWS clusters have had no query activity for 7 or more days and are candidates for pause or termination
- Reserved instance coverage: whether baseline On-Demand spend is covered by reserved instances, and what the estimated monthly savings from increased coverage would be

## Safe workflow

1. **Frame scope**
   - Services and Enterprise Projects in scope:
   - Region and account context:
   - Current-state evidence (CBC exports, billing console screenshots, or IaC):
   - Cost target or optimization goal:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live CBC Cost Center export data or Budget Management console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the findings**
   - What is the root cause of the top cost delta service if the >15% threshold is exceeded?
   - What On-Demand workloads would save the most by converting to Yearly/Monthly or reserved instances?
   - What EVS volumes are unattached and can be confirmed as waste without data loss risk?
   - What evidence is missing to confirm DWS cluster inactivity before recommending shutdown?
   - What budget alerts have no escalation action and therefore provide visibility without response capability?
4. **Recommend the smallest safe action**
   - Prioritize by monthly savings impact and implementation effort.
   - Flag destructive actions (EVS delete, DWS terminate) as requiring explicit owner confirmation.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Cost Anomaly Review: <scope>
## CBC Cost Center delta summary and anomaly threshold assessment
## Budget alert configuration coverage and gap analysis
## ECS billing mode anomaly findings
## GaussDB billing mode anomaly findings
## OBS request cost spike root cause assessment
## Unattached EVS volume waste identification
## DWS idle cluster cost findings
## Reserved instance coverage gap and savings estimate
## Prioritized cost remediation actions
```

Each section must include an evidence level label and, where applicable, an estimated monthly savings figure.
