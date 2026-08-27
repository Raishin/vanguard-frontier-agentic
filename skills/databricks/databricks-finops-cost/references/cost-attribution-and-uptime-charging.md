# Cost Attribution, Uptime Charging, And Cost Controls

Custom-tag-based attribution with coverage reporting, DBU uptime semantics, and compute policies and idle settings as cost controls.

- DBUs are charged by UPTIME, not execution time — a 12 DBU/hour warehouse up for 30 minutes costs 6 DBU, regardless of query execution time.
- One serverless workload can emit MULTIPLE usage records at different DBU rates within the same hour; they must be summed, not picked (max, min, or any other aggregation).
- Cost attribution runs on custom_tags propagated from compute resources; tag propagation for non-compute resources is not documented, creating an attribution gap for data-quality monitoring, materialized views, and Lakeflow Connect.
- Attribution coverage is reported as % tagged vs untagged spend. A ranking of expensive workloads is only reliable if coverage > 75%; below that, the ranking is incomplete and the true top spender may be in the untagged portion.
- Budgets support up to 4 alert thresholds, are ESTIMATE-BASED, are NOT a hard cap, and email notification can lag up to 24 hours. Usage blocking (hard enforcement) exists only for Unity AI Gateway.
- Interactive serverless notebooks have a default 2.5-hour execution timeout (admin-configurable) as runaway-spend protection.
- Cluster policy constraint types are fixed, forbidden, allowlist, blocklist, regex, range, unlimited. Policies can enforce minimum cluster count (cost floor) and maximum count (cost cap).
- Instance-pool minimum-idle instances NEVER terminate regardless of autotermination setting, so they are a standing cost floor that continues to accrue when the workload is idle.

## Sources

- https://docs.databricks.com/aws/en/admin/system-tables/billing
- https://docs.databricks.com/aws/en/admin/system-tables/serverless-billing
- https://docs.databricks.com/aws/en/admin/account-settings/budgets
- https://docs.databricks.com/aws/en/admin/clusters/policy-definition
- https://docs.databricks.com/aws/en/compute/pools
