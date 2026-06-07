# Official sources for Azure Cost Optimization Governor

Use Microsoft Learn documentation through the user's configured documentation MCP before recommending Azure cost optimizations. Documentation proves tool behavior and best practices; it does not prove the user's utilization, commitment coverage, budget ownership, or actual savings.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [How to optimize your cloud investment with Cost Management](https://learn.microsoft.com/en-us/azure/cost-management-billing/costs/cost-mgt-best-practices) | Use for planning, visibility, accountability, cost analysis, exports, budgets, APIs, savings plans, reservations, and iteration. |
| [What is Microsoft Cost Management](https://learn.microsoft.com/en-us/azure/cost-management-billing/costs/overview-cost-management) | Ground Cost Management capabilities and Advisor as first stop for optimization recommendations. |
| [Create and manage budgets](https://learn.microsoft.com/en-us/azure/cost-management-billing/costs/tutorial-acm-create-budgets) | Use for budget thresholds and alerting; budgets inform and automate actions, they are not hard spend caps. |
| [Create and manage Cost Management exports](https://learn.microsoft.com/en-us/azure/cost-management-billing/costs/tutorial-improved-exports) | Use for actual/amortized/FOCUS exports, price sheet, reservation, and recommendation datasets. |
| [Azure Advisor cost recommendations](https://learn.microsoft.com/en-us/azure/advisor/advisor-reference-cost-recommendations) | Use for recommendation classes and savings opportunities, but verify overlap and current utilization. |
| [Cost Optimization workbook](https://learn.microsoft.com/en-us/azure/advisor/advisor-workbook-cost-optimization) | Use for Advisor workbook-driven optimization across rate and usage recommendations. |
| [Savings plan recommendations](https://learn.microsoft.com/en-us/azure/cost-management-billing/savings-plan/purchase-recommendations) | Use for lookback, simulations, stale-data safeguards, and overcommitment risks. |
| [Decide between a savings plan and a reservation](https://learn.microsoft.com/en-us/azure/cost-management-billing/savings-plan/decide-between-savings-plan-reservation) | Use for right-size-first sequencing and commitment selection. |

## Source-grounding rules

- Do not promise savings; label them as potential until implemented and measured.
- Do not recommend commitments before waste removal and utilization stability checks.
- Do not treat budgets as enforcement unless action groups or automation are explicitly designed.
- Cost exports and cost data can contain sensitive business information; request sanitized summaries only.
