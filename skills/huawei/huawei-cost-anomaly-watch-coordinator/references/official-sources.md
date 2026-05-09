# Official sources

Use this reference only when you need source grounding for Huawei Cloud CBC, Budget Management, or CES cost-related service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/billing/index.html
- https://support.huaweicloud.com/intl/en-us/costcenter/index.html
- https://support.huaweicloud.com/intl/en-us/ces/index.html

## Key service behavior references

| Service / Resource | Behavior | Anomaly Signal | Action |
|--------------------|----------|----------------|--------|
| CBC Cost Center | Provides day-over-day and month-over-month spend breakdowns by service and Enterprise Project | >15% day-over-day increase without planned cause | Investigate top contributing service |
| Budget Management | Configures spend and usage budgets with alert thresholds | Alert without escalation action provides visibility only | Configure SMS/email/function alert actions |
| ECS billing mode | On-Demand vs Yearly/Monthly; On-Demand costs 2-3x for stable baseline workloads | Long-running On-Demand instance (>30 days) at stable load | Convert to Yearly/Monthly or reserved instance |
| GaussDB billing mode | Same On-Demand vs Yearly/Monthly distinction applies | GaussDB On-Demand for production DB running 24/7 | Convert to Yearly/Monthly for significant savings |
| OBS request costs | Charged per API call type (GET, PUT, LIST, DELETE) and storage tier | Unexpected request volume spike | Check lifecycle policies and application retry patterns |
| EVS volumes | Unattached volumes (not mounted to any ECS) continue to bill at storage rate | Volumes with no attachment and no snapshot activity | Confirm with owner and delete if truly unused |
| DWS clusters | Charged per node-hour when running; pause reduces cost to storage only | No query activity for 7+ days | Pause or terminate after owner confirmation |
| Reserved instances | Pre-purchased capacity at discount; unused reserved capacity is wasted | Low utilization coverage of On-Demand baseline | Identify baseline and size reserved instance purchase |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
