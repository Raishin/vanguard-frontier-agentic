# KPI Measurability And Attribution Limits

Which candidate KPIs are measurable from platform evidence, which depend entirely on customer systems, and how to bound an attribution claim.

- Platform-measurable metrics can be evidenced from Databricks itself: cost per successful workload, cost per inference, pipeline failure rate, job success rate, incident mean-time-to-restore, data freshness against an SLA, and time-to-data-product. These carry the strongest evidence and the weakest business framing.
- Customer-system metrics cannot be evidenced from Databricks at all: revenue lift, gross-margin improvement, fraud loss avoided, inventory reduction, forecast error, time-to-decision, customer-resolution time, model-assisted conversion, and analyst-hours saved. Each requires the organisation's own source of truth to be named before it is used.
- Analyst-hours saved is the most frequently inflated benefit because it is computed from a self-reported before-and-after estimate. Require a measured task time or a ticket-volume proxy, and state plainly that a survey-derived figure is an assumption, not a measurement.
- Cost per successful workload is more honest than raw spend because it moves in the right direction when reliability improves; raw spend can fall simply because work stopped running, which is a failure that looks like a saving.
- A cost reduction is only cash when the organisation removes the cost. Released compute headroom becomes money when the reservation or cluster is actually decommissioned; freed analyst hours become money when the work is redeployed to something with a stated value or the headcount changes. Name who must act for the conversion to occur.
- Attribution requires a control or a plausible counterfactual. Where neither exists, the defensible claim is that the KPI moved during the period in which the intervention shipped, and that no other identified change accounts for it — which is materially weaker than causation and must be worded as such.
- Seasonality defeats naive before-and-after comparison on most commercial KPIs. Compare like periods year over year, or state that the comparison is uncontrolled for seasonality.
- When several initiatives ship into the same KPI in the same window, the movement cannot be split between them by assumption. Either instrument them separately in advance or report the aggregate and decline to apportion.
- A risk-reduction or compliance outcome usually has no revenue KPI, and forcing one produces a fiction. Measure the leading operational metric — audit findings closed, controls evidenced, exposure window shortened — and frame the case as risk reduction rather than benefit.
- Databricks-side spend is derived by joining `system.billing.usage` to `system.billing.list_prices` on the documented time predicate; omitting that predicate double-counts and inflates the cost baseline, which then flatters any saving computed against it.
- Tag attribution coverage must be reported before any per-team or per-initiative cost claim. Untagged usage is not zero-cost usage, and silently excluding it understates the denominator of every efficiency ratio built on top of it.
- State the confidence of each figure separately. A value case commonly pairs a high-confidence cost number with a low-confidence benefit number, and presenting them at equal confidence is the misleading step, not either number on its own.

## Sources

- https://docs.databricks.com/aws/en/admin/system-tables/billing
- https://docs.databricks.com/aws/en/admin/system-tables/
- https://docs.databricks.com/aws/en/data-governance/unity-catalog/data-quality-monitoring/
