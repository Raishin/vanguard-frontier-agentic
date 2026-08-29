# Billing System Tables And Join Predicates

System.billing.usage schema, system.billing.list_prices structure, and the critical time-predicate join to avoid double-counting.

- System.billing.usage is GA and carries account_id, workspace_id, usage_start_time, usage_end_time, usage_date, sku_name, cloud, usage_unit, usage_quantity, billing_origin_product, product_features, custom_tags, usage_metadata struct (cluster_id, job_id, warehouse_id, node_type, and more), and identity_metadata struct (run_as, owned_by, created_by).
- System.billing.list_prices is GA with price_start_time, price_end_time, account_id, sku_name, cloud, currency_code, usage_unit, and a pricing struct (default, promotional, effective_list).
- The join predicate for list_prices to usage is `list_prices.price_start_time <= usage.usage_date AND usage.usage_date < list_prices.price_end_time`; any other join predicate (without the time filter, or with > instead of <) double-counts charges when prices change.
- Serverless billing covers notebooks, jobs, data-quality monitoring, predictive optimization, materialized views, and Lakeflow Connect. For serverless, the DBU price includes VM cost. Classic bills DBU and infrastructure separately.
- There is no system.query.cost table; query cost is inferred by joining system.query.history to system.billing.usage on time and identity (run_as, owned_by, created_by), and this inference must be labelled as inference, not measured fact.

## Sources

- https://docs.databricks.com/aws/en/admin/system-tables/billing
- https://docs.databricks.com/aws/en/admin/system-tables/pricing
- https://docs.databricks.com/aws/en/admin/system-tables/serverless-billing
