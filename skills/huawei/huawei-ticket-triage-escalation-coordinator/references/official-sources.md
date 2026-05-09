# Official sources

Use this reference only when you need source grounding for Huawei Cloud incident triage service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/
- https://status.huaweicloud.com/
- https://support.huaweicloud.com/intl/en-us/aom/index.html
- https://support.huaweicloud.com/intl/en-us/ces/index.html
- https://support.huaweicloud.com/intl/en-us/lts/index.html

## Incident severity and SLA reference

| Severity | Label | SLA (Premium Support) | Escalation Path |
|---------|-------|----------------------|-----------------|
| P0 | Urgent (紧急) | 15-minute response | War room + Account Manager + TAM phone |
| P1 | High (高) | 2-hour response | TAM ticket escalation |
| P2 | Normal (中) | 8-hour response | Standard support ticket |
| P3 | Low (低) | Next business day | Standard support ticket |

## Evidence collection sources

| Source | What it provides | Typical latency |
|--------|-----------------|-----------------|
| CES (Cloud Eye) | Infrastructure metrics (CPU, memory, network, disk) | Near real-time |
| LTS (Log Tank) | Application and service logs | Near real-time |
| AOM | Application topology, alert history, service health | Near real-time |
| CCE Events | Pod scheduling, eviction, and restart events | Near real-time |
| Huawei Cloud status page | Platform-level Managed Incidents (MIs) | Updated by platform team |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
