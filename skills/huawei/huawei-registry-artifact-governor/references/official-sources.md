# Official sources

Use this reference only when you need source grounding for Huawei Cloud SWR service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/swr/index.html
- https://support.huaweicloud.com/intl/en-us/vss/index.html
- https://support.huaweicloud.com/intl/en-us/cce/index.html

## Key service behavior references

| Control | Default State | Risk if Unconfigured | Remediation |
|---------|--------------|---------------------|-------------|
| SWR namespace visibility | Private | — | Keep private for all production namespaces |
| VSS vulnerability scanning | Disabled | Undetected CVEs in images | Enable auto-scan on push; block on HIGH/CRITICAL |
| Image retention policy | Not configured | Unbounded storage growth | Configure retention by tag age or count |
| Cross-region synchronization | Not configured | Single-region DR gap | Configure sync to at least one additional region |
| IAM agency for CCE pull | Varies | Overpermissioned pull credentials | Restrict to swr:repository:pull only |
| Tag immutability | Mutable | Supply chain confusion attacks | Enforce immutability in production repos |
| Image signing | Not natively supported | No attestation | Use Notary v2 or cosign externally |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
