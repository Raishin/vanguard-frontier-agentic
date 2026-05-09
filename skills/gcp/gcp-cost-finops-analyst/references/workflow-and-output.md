# Workflow and output contract

Use this reference only when performing the full cost review, implementation guidance, budget incident triage, or FinOps roadmap development.

## Review domains

Check these areas before giving a verdict:

- Billing export: configured to BigQuery, dataset name, standard vs. detailed export, last export date
- Spend breakdown: top services by cost, top projects by cost, month-over-month trend, label-attributed vs. unattributed
- CUD coverage: committed vCPUs and memory vs. actual usage, resource-based vs. spend-based, region breakdown
- SUD coverage: GCE and Cloud SQL instances running >25% of month, overlapping CUD coverage
- Label coverage: percentage of cost rows with team, environment, service, cost-center labels; top unlabeled services
- Rightsizing: Compute Engine (Recommender API findings), GKE (node pool sizing, Standard vs. Autopilot), Cloud SQL (machine type, storage)
- Budget alerts: configured thresholds, notification channels, alert history, actual vs. forecasted spend

## Safe workflow

1. **Frame scope**
   - Billing account and project scope:
   - Time range for analysis:
   - Compliance or chargeback driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Confirm billing export configuration and dataset name.
   - Prefer sanitized query results from `gcp_billing_export_v1`, Cloud Billing Console exports, or Recommender API findings.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What is the largest cost driver and is it trending up?
   - What percentage of spend is unattributable due to missing labels?
   - What is the CUD utilization rate and what is the coverage gap?
   - Which budget alerts have fired in the last 30 days and were they actioned?
   - What evidence is missing?
4. **Recommend in priority order**
   - First: enable billing export if missing.
   - Second: enforce labels for unattributed spend.
   - Third: rightsize overprovisioned resources.
   - Fourth: optimize commitments (CUDs) based on stable baseline.
   - If the safest action is to stop and gather billing evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# GCP Cost and FinOps Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest cost risk:
- Evidence level:
## Billing export status
- Configured: yes/no
- Dataset:
- Last export:
## Spend breakdown
| Service | Month spend | MoM trend | Label attributed % |
|---|---|---|---|
## CUD/SUD coverage
| Region | Committed vCPU/memory | Actual usage | SUD overlap | Gap |
|---|---|---|---|---|
## Label coverage audit
| Label key | Coverage % | Top unlabeled service |
|---|---|---|
## Rightsizing opportunities
| Resource | Current size | Recommended size | Monthly savings estimate |
|---|---|---|---|
## Budget alert review
| Budget | Threshold | Last fired | Notification channel | Gap |
|---|---|---|---|---|
## Action plan
1. <action> — owner: <owner>, savings estimate: <$>, validation: <check>
## Residual risk
- <risk or explicit none>
```
