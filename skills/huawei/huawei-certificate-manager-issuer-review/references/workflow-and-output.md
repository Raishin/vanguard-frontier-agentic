# Workflow and output contract

Use this reference only when performing a full SSL certificate management review for a Huawei Cloud environment.

## Review domains

Check these areas before giving a verdict:

- SCM certificate inventory: all certificates in scope, their expiry dates, issuer, and whether they are bound to ELB listeners
- ELB listener binding coverage: all public-facing HTTPS listeners and whether each has a valid, non-expired certificate bound
- DEW key storage: whether private keys are stored in DEW and whether key access policies are appropriately restricted
- Renewal automation: whether an automated renewal workflow exists and whether any certificate within 30 days of expiry lacks automation
- Certificate type selection: whether wildcard or SAN certificates are appropriate for the domain inventory in use
- CES expiry alerting: whether alarms are configured at 30-day and 7-day thresholds and whether alert actions are wired
- HTTPS enforcement: whether all public-facing HTTP listeners have redirect rules to HTTPS equivalents
- Region coverage: whether certificates are present in all regions where they are consumed

## Safe workflow

1. **Frame scope**
   - ELB instances and HTTPS listeners in scope:
   - Region and account context:
   - Current-state evidence:
   - Required certificate coverage and renewal lead time:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the configuration**
   - What happens to HTTPS traffic if a certificate expires and no renewal automation exists?
   - What clients are affected if the ELB listener certificate binding is missing in a specific region?
   - What is the blast radius of an overly permissive DEW key access policy?
   - What domains are uncovered if a wildcard certificate is used without verifying second-level subdomain requirements?
   - What alerts are missed if CES expiry alarms are not configured at the 30-day threshold?
   - What evidence is missing to confirm certificate coverage across all regions?
4. **Recommend the smallest safe action**
   - Prefer targeted fixes, staged changes, and verification steps.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Certificate Management Review: <scope>
## SCM certificate inventory and expiry timeline
## ELB listener SSL certificate binding coverage
## DEW key storage and access policy assessment
## Renewal automation coverage and manual renewal risk
## Wildcard vs SAN certificate selection rationale
## CES expiry alerting configuration review
## HTTPS enforcement on public-facing ELB listeners
## Prioritized certificate management improvements
```

Each section must include an evidence level label.
