# Workflow and output contract

Use this reference only when performing the full certificate manager review, expiry monitoring audit, or TLS posture assessment.

## Review domains

Check these areas before giving a verdict:
- Certificate inventory: list of certificates, type (Certificate Manager / classic Google-managed / self-managed), domains covered
- Certificate map attachment: map existence, map entry configuration, attachment to target HTTPS proxy
- DNS authorization: authorization record existence, CNAME delegation correctness, authorization status (ACTIVE/PENDING/FAILED)
- CAA records: CAA record presence for each domain, Google Trust Services (pki.goog) allowed, issuance restriction risk
- Wildcard vs SAN coverage: wildcard scope verification (*.domain.com does not cover apex domain.com), SAN list for all required domains
- Rotation automation: auto-renewal enabled, renewal lead time (minimum 30 days before expiry), manual renewal dependency
- Expiry monitoring: Cloud Monitoring metric or alert policy for certificate expiry, Cloud Scheduler-based expiry check, notification channel configured
- SSL policy: SSL policy attached to target HTTPS proxy, TLS minimum version (1.2+ required), cipher suite restrictions

## Safe workflow

1. **Frame scope**
   - Domains and certificate types in scope:
   - Load balancer targets:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Are any certificates created but not mapped to an HTTPS proxy?
   - Do CAA records allow Google Trust Services for all domains?
   - Does the wildcard certificate cover the apex domain?
   - Is there an expiry alert configured with sufficient lead time?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged migration, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Certificate Manager Issuer Review: <scope>
## Executive verdict
- Status: HEALTHY / HEALTHY WITH RISKS / AT RISK / NEEDS EVIDENCE
- Biggest gap:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Certificate inventory
| Certificate | Type | Domains | Mapped to Proxy | Expiry Monitoring |
|---|---|---|---|---|
## DNS authorization and CAA posture
- DNS authorization status: <ACTIVE / PENDING / FAILED / unknown>
- CAA records allow pki.goog: <yes / no / unknown>
- Wildcard covers apex domain: <yes / no / N/A>
## Rotation and expiry posture
- Auto-renewal: <enabled / manual / unknown>
- Expiry alert configured: <yes / no>
- SSL policy TLS minimum: <TLS 1.2+ / TLS 1.0 / unknown>
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Residual risk
- <risk or explicit none>
```
