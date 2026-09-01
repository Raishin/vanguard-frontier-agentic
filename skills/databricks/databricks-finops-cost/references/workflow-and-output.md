# Workflow And Output

Cost analysis sequence and output contract for FinOps review, with attribution-confidence reporting.

## Workflow

1. Establish data scope: date range, workspace(s), and what exports are available (system.billing.usage, system.billing.list_prices, system.compute.clusters, system.lakeflow.jobs). Refuse-and-ask if core tables are missing.
2. Inspect system.billing.usage schema: confirm account_id, workspace_id, usage_date, sku_name, usage_quantity, custom_tags, usage_metadata, identity_metadata columns are present.
3. Verify the pricing join: check that `price_start_time <= usage_date AND usage_date < price_end_time` is used; any other join predicate double-counts charges.
4. Analyse custom-tag coverage: calculate the % of spend with custom_tags != NULL or empty. Report this as attribution confidence (e.g., '85% tagged, 15% untagged').
5. Identify expensive workloads: join usage to clusters/jobs to name the top spenders by custom tag. Flag the ranking as incomplete if coverage < 75%.
6. Review DBU uptime charging: confirm that warehouses and clusters are charged by uptime (not execution), serverless emits multiple records per hour when rates change, and auto-stop incurs full uptime charge.
7. Check serverless vs classic comparison: if comparing cost, confirm both VM and DBU costs are included (serverless VM is in the DBU price, classic VM is separate). Flag per-DBU comparisons as incomplete.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (pass / pass-with-conditions / block) and data scope (date range, workspaces, tag coverage %) assumed.
- Billing system-table schema, retention, and availability findings; data gaps that would affect analysis.
- Custom-tag coverage confidence (% tagged vs untagged) and any workload ranking (explicitly incomplete if <75% tagged).
- DBU uptime charging explanation and multi-record aggregation (serverless) confirmation.
- Severity-labelled findings (critical / high / medium / low), attribution-confidence labels, and inference limitations.
- Safe next actions and any data or scope gaps that would change the verdict.
