# Official sources

Use this reference only when you need source grounding for Huawei Cloud SCM, ELB, or DEW certificate-related service behavior or the detailed source list.

## Huawei Cloud documentation

Use these as starting points, not as proof of the user's live Huawei Cloud state:

- https://support.huaweicloud.com/intl/en-us/scm/index.html
- https://support.huaweicloud.com/intl/en-us/elb/index.html
- https://support.huaweicloud.com/intl/en-us/dew/index.html

## Key service behavior references

| Service | Behavior | Implication | Risk if Misconfigured |
|---------|----------|-------------|----------------------|
| SCM | Certificates are region-scoped | Certificate must be uploaded or imported in each target region | ELB listeners in other regions cannot bind the certificate |
| SCM | No automatic renewal by default | Manual renewal required unless automation is wired externally | Production outage on certificate expiry |
| ELB | HTTPS listener requires a valid bound SSL certificate | Missing or expired binding causes immediate TLS handshake failure | All client connections fail at TLS handshake |
| ELB | HTTP listener can be configured with HTTPS redirect rule | Redirect enforces HTTPS without requiring clients to use HTTPS natively | HTTP traffic sent in plaintext if redirect is absent |
| DEW | Stores HSM-backed private keys for SCM certificates | IAM access policy controls which identities can use the key | Overly permissive policy exposes private key material |
| CES | Supports custom metric alarms | Configure certificate expiry alarms at 30-day and 7-day thresholds | Insufficient lead time for manual renewal without 30-day alarm |
| Wildcard cert | Covers *.example.com — not apex or second-level subdomains | Verify domain inventory covers all required subdomains | Uncovered domains fail TLS handshake |
| SAN cert | Covers multiple specific domains listed in Subject Alternative Names | Preferred when domain set is bounded and well-defined | Additional domains require certificate reissuance if not listed |

## Grounding rule

Official documentation explains Huawei Cloud service behavior. It does not prove the user's current account, region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer live console evidence or sanitized user-provided evidence for current-state claims.
