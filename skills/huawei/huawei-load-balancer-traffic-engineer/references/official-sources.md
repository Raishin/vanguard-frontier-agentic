# Official sources

Use this reference only when you need source grounding for Huawei Cloud ELB and WAF service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/elb/index.html
- https://support.huaweicloud.com/intl/en-us/waf/index.html

## Key service behavior references

| Service | Behavior | Implication | Risk if Misconfigured |
|---------|----------|-------------|----------------------|
| Dedicated ELB | Supports HTTP/HTTPS/TCP/UDP listeners with custom TLS policy | Use for production workloads requiring WAF or custom cipher suites | Shared ELB lacks TLS policy control |
| Dedicated ELB | TLS policy selection controls cipher suites and protocol versions | TLS-1-2-Strict disables weak ciphers and TLSv1.0/1.1 | Older TLS versions expose traffic to downgrade attacks |
| ELB WAF integration | Routes listener traffic through WAF instance before backends | WAF must be provisioned in the same region; block mode may cause false positives | Untuned WAF policy blocks legitimate traffic |
| ELB health check | Supports TCP, HTTP, and HTTPS health check protocols | HTTP health checks validate application layer; TCP checks pass even when app layer is broken | Unhealthy backends receive traffic with TCP-only checks |
| Backend Server Group | Supports weighted routing across backend members | Weights control traffic share; stale weights persist after releases | Old backends receive unintended production traffic |
| Connection draining | Draining timeout allows in-flight requests to complete before backend removal | Timeout must exceed longest expected request duration | In-flight requests fail during rolling deployments |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
