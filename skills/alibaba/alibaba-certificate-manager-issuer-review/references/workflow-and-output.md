# Workflow and output contract

Use this reference only when performing a full certificate lifecycle review or expiry risk assessment.

## Review domains

Check these areas before giving a recommendation:

- Certificate inventory: all certificates, their domains, types (DV/OV/EV), expiry dates, and issuer CA
- Certificate type compliance: DV sufficient vs OV/EV required for the compliance framework in scope
- Auto-renewal: enabled status, DNS validation record present and resolvable
- Deployment bindings: SLB listeners, ALB HTTPS listeners, CDN domains, OSS buckets — all updated after last renewal?
- CAA records: DNS CAA record allows the CA issuing the certificate (DigiCert or GlobalSign)
- CloudMonitor alerts: expiry alert configured with at least 30-day advance notice
- TLS policy: TLS 1.2+ enforced via ALB/SLB security policy
- Private key posture: platform-generated (stored by Alibaba) vs. CSR-uploaded (customer-controlled)

## Safe workflow

1. **Frame the certificate scope**
   - Certificate domains (sanitized — no real domains required if not available):
   - Certificate types in use (DV/OV/EV):
   - Compliance requirements (PCI-DSS / MLPS 2.0 / none):
   - Resources bound to certificates (SLB/ALB/CDN/OSS):
2. **Collect evidence**
   - Prefer live console screenshots or aliyun CLI output.
   - Otherwise inspect DNS records, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the configuration**
   - Is auto-renewal enabled AND is the DNS CNAME validation record resolvable? (both must be true)
   - Has the certificate been redeployed to all bound resources after the last renewal?
   - Is the CAA record correct for the issuing CA?
   - Is CloudMonitor configured with 30+ day advance expiry alerts?
   - Is TLS 1.0/1.1 blocked via security policy?
4. **Recommend the smallest safe next step**
   - Prioritize by risk: certificates expiring within 30 days > missing deployment binding > DNS validation record failure > no CloudMonitor alerts > TLS downgrade.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Certificate Review: <scope>
## Executive summary
- Certificate posture verdict:
- Evidence level:
- Critical findings:
## Certificate inventory
| Domain (sanitized) | Type | Issuer CA | Expiry date | Days remaining |
|---|---|---|---|---|
## Certificate type and compliance assessment
- Compliance requirement:
- DV sufficient: yes/no
- OV/EV required: yes/no
- Current type coverage: adequate/gap
## Auto-renewal and DNS validation
| Certificate | Auto-renewal | DNS validation record | Resolvable | Risk |
|---|---|---|---|---|
## Deployment coverage
| Certificate | SLB | ALB | CDN | OSS | Last redeployed |
|---|---|---|---|---|---|
## CAA record compliance
- CAA record present:
- CA allowed:
- Gap:
## Expiry monitoring
- CloudMonitor alert configured:
- Alert lead time:
- Notification channels:
## TLS version posture
- TLS 1.2+ enforced:
- Security policy applied:
## Certificate hygiene recommendations
1. <recommendation> — priority: <critical/high/medium>, effort: <low/medium/high>
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
