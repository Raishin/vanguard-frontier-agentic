# Official sources

Use this reference when grounding Scaleway pricing, billing behavior, or service cost model details.

## Scaleway pricing and billing documentation

Use these as starting points for cost analysis — not as proof of the user's live billing state or actual resource utilization:

- https://www.scaleway.com/en/pricing/ — Scaleway pricing page: instance families, Object Storage, SBS, RDB, Serverless, and reserved instance rates
- https://www.scaleway.com/en/docs/billing/ — billing docs: invoice structure, cost allocation, billing period, reserved instance commitment terms
- https://www.scaleway.com/en/docs/observability/cockpit/ — Cockpit docs: managed Grafana, Mimir, Loki, Tempo plan tiers and ingestion limits
- https://registry.terraform.io/providers/scaleway/scaleway/latest/docs/resources/instance_server — Terraform `scaleway_instance_server` resource: commercial type options, lifecycle flags

## Grounding rule

Official pricing documentation reflects Scaleway list prices and product tiers. It does not prove the user's actual billing totals, current resource utilization, reserved instance utilization rate, or egress volume. Savings estimates must be labeled as `estimated` when actual utilization data has not been provided by the user.
