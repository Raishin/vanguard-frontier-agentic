# Official sources

Use this reference only when you need source grounding for Huawei Cloud service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/organizations/index.html
- https://support.huaweicloud.com/intl/en-us/iam/index.html
- https://support.huaweicloud.com/intl/en-us/vpc/index.html
- https://support.huaweicloud.com/intl/en-us/gaussdb_mysql/index.html
- https://support.huaweicloud.com/intl/en-us/cce/index.html

## Key service behavior references

| Service | Change Type | Blast Radius | Rollback |
|---------|-------------|--------------|---------|
| Organizations SCP | Deny policy add/modify | All member accounts | Remove or update policy |
| IAM Agency | Modify/delete | All services assuming the agency | Recreate agency (no undo for deletion) |
| VPC Route Table | Route add/modify/delete | All subnets in VPC | Revert route entry |
| GaussDB | Instance class change | Brief maintenance window | Scale back to prior class |
| CCE Node Pool | Scale down | Pod eviction in drained nodes | Scale back up |
| Enterprise Project | Resource move | Billing attribution change | Move back |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
