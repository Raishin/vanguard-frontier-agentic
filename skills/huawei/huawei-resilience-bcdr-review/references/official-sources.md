# Official sources

Use this reference only when you need source grounding for Huawei Cloud BCDR and resilience service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/gaussdb_mysql/index.html
- https://support.huaweicloud.com/intl/en-us/cbr/index.html
- https://support.huaweicloud.com/intl/en-us/cce/index.html
- https://support.huaweicloud.com/intl/en-us/drs/index.html
- https://support.huaweicloud.com/intl/en-us/elb/index.html
- https://support.huaweicloud.com/intl/en-us/ecs/index.html

## Service behavior reference

| Service | HA Scope | DR Mechanism | Notes |
|---------|----------|--------------|-------|
| GaussDB for MySQL | AZ pair failover (automatic) | Cross-region read replica (manual promote) | Cross-region failover is not automatic |
| CBR | Single region vault | Cross-region vault (explicit config) | Same-region vault has no regional DR value |
| CCE | Multi-AZ node distribution | Separate cluster + ELB + DNS | No native cross-region cluster failover |
| DRS | Cross-region replication | Replication task with lag monitoring | Verify consistency check results |
| ELB | Cross-AZ within region | DNS-based global ELB for cross-region | ELB does not span regions natively |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, CBR vault location, DRS replication lag, IAM boundary, or operational DR state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
