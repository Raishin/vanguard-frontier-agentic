# Official sources

Use this reference only when you need source grounding for Huawei Cloud FunctionGraph, ServiceStage, or AOM service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/fg/index.html
- https://support.huaweicloud.com/intl/en-us/servicestage/index.html
- https://support.huaweicloud.com/intl/en-us/aom/index.html

## Key service behavior references

| Service | Behavior | Implication | Risk if Absent |
|---------|----------|-------------|----------------|
| FunctionGraph | VPC binding required to reach private resources | Functions without VPC cannot access RDS, GaussDB, or private ELB | Silent connection failures to private services |
| FunctionGraph | LTS log output not enabled by default | Log group and stream must be explicitly bound | Production errors are invisible |
| FunctionGraph | Concurrency soft-limited per function | Reserved instances prevent cold starts at burst | Unpredictable latency spikes under load |
| FunctionGraph | Cold start duration scales with package size | Trim dependencies and use layers to reduce initialization time | Poor p99 latency under burst |
| FunctionGraph | Function timeout must be less than trigger/caller timeout | Timeout mismatch causes retry loops and duplicate processing | Duplicate side effects and quota exhaustion |
| FunctionGraph | Custom runtimes require owner to manage security patches | Managed runtimes are patched by Huawei | Unpatched runtime CVEs in custom environments |
| AOM | Provides invocation count, error rate, and duration metrics | Configure alarms on error rate and p99 duration | Silent error accumulation without alerting |
| ServiceStage | Application lifecycle managed via health checks | Health check endpoint must reflect actual readiness | Traffic routed to unready function instances |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
