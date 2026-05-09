# Official sources

Use this reference only when you need source grounding for Huawei Cloud daily operations briefing service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/cbc/index.html
- https://support.huaweicloud.com/intl/en-us/aom/index.html
- https://support.huaweicloud.com/intl/en-us/cce/index.html
- https://support.huaweicloud.com/intl/en-us/ces/index.html
- https://support.huaweicloud.com/intl/en-us/secmaster/index.html
- https://support.huaweicloud.com/intl/en-us/lts/index.html

## Daily briefing signal reference

| Signal | Source | Threshold | Action Required |
|--------|--------|-----------|-----------------|
| Cost delta | CBC | >15% from prior day baseline | Assign investigation owner before briefing ends |
| Unacknowledged alert | AOM | Any HIGH or CRITICAL in last 24h | Assign owner; review SMN routing |
| Pod failure across AZs | CCE | Failures in >1 AZ | Escalate to platform team |
| Quota utilization | CES | >80% for ECS, EIP, GaussDB | Submit quota increase request immediately |
| Security finding age | SecMaster | HIGH/CRITICAL >24h without owner | Escalate to security team lead |
| Log error spike | LTS | >3x 7-day average error rate | Investigate service log stream |

## Common CBC cost spike causes

- Yearly/Monthly subscription changes (new or cancelled)
- DWS (Data Warehouse Service) or DLI (Data Lake Insight) query spikes
- ECS spot instance to on-demand instance transitions
- Unexpected data transfer or CDN egress
- New ECS or GaussDB instances added without budget approval

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
