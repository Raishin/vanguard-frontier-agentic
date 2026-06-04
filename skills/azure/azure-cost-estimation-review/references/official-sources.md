# Official sources for Azure Cost Estimation Review

Use Microsoft Learn documentation through the user's configured documentation MCP before critiquing Azure estimates. Documentation and calculators support planning; they do not prove invoice truth, negotiated rates, future utilization, or missing architecture components.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [Estimate costs with the Azure pricing calculator](https://learn.microsoft.com/en-us/azure/cost-management-billing/costs/pricing-calculator) | Use for calculator mechanics, product configuration, usage quantities, pricing plans, exports, and negotiated-price caveats. |
| [Plan to manage Azure costs](https://learn.microsoft.com/en-us/azure/cost-management-billing/understand/plan-manage-costs) | Use for estimation before adding services, price sheets, portal estimates, and planning limitations. |
| [How to optimize your cloud investment with Cost Management](https://learn.microsoft.com/en-us/azure/cost-management-billing/costs/cost-mgt-best-practices) | Use for planning, visibility, accountability, exports, budgets, APIs, and optimization lifecycle. |
| [Plan your Azure environment for cost estimation](https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/plan/estimate-total-cost-of-ownership) | Use for architecture-driven estimates, operational costs, and reevaluation when projections deviate. |
| [Azure Retail Prices API](https://learn.microsoft.com/en-us/rest/api/cost-management/retail-prices/azure-retail-prices) | Use for programmatic retail pricing source; distinguish from negotiated billing rates. |
| [Azure reservations](https://learn.microsoft.com/en-us/azure/cost-management-billing/reservations/save-compute-costs-reservations) | Use when estimates include one-year or three-year commitments. |
| [Azure savings plans for compute](https://learn.microsoft.com/en-us/azure/cost-management-billing/savings-plan/savings-plan-overview) | Use when estimates include hourly compute commitments. |

## Source-grounding rules

- Do not present calculator estimates as invoices.
- Do not assume negotiated pricing unless the user provides safe proof or a configured read-only evidence sample verifies it.
- Do not hide support, bandwidth, logging, backup, security, DR, NAT, private endpoint, or operational labor omissions.
- Keep usage assumptions explicit and challenge one-month flat 730-hour defaults when workload patterns differ.
