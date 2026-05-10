# Workflow and output contract

Use this reference only when performing a full IONOS Cloud security and GDPR compliance review.

## Review domains

Check these areas before giving a verdict:

- GDPR data residency: datacenter region matches declared data processing location; endpoint region is correct for all service calls
- Encryption posture: volumes encrypted at rest, TLS enforced in transit for all service endpoints
- Network isolation: private LAN segmentation, NIC firewall rules, no unintended public interfaces on sensitive workloads
- IAM hygiene: bearer token scope is least-privilege, token rotation cadence, no overly broad API token assignments
- Audit trail: logging enabled for control-plane and data-plane operations, retention meets compliance requirements
- Vulnerability posture: server OS update cadence, exposed port surface, known CVE exposure on managed services

## Safe workflow

1. **Frame scope**
   - Datacenter region(s) and declared GDPR processing location:
   - Workload type and data classification:
   - Compliance driver (GDPR, ISO 27001, internal policy):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live IONOS Cloud API or DCD export evidence if available.
   - Otherwise inspect IaC (Terraform), user-provided sanitized configuration, or official IONOS docs.
   - Label each finding as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What can expose data outside the declared GDPR region?
   - What can allow unauthorized access to production resources?
   - What can break audit trail continuity?
   - What can expose credentials or API tokens?
   - What compliance evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer additive hardening (add encryption, tighten scope) over disruptive remediation.
   - If the safest action is to gather more evidence before recommending, say that plainly.

## Output contract

Return this structure:

```markdown
# IONOS Security and Compliance Review: <scope>
## Executive verdict
- Status: COMPLIANT / COMPLIANT WITH RISKS / NON-COMPLIANT / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## GDPR data residency
- Datacenter region:
- Endpoint region validated:
- Status: CONFIRMED / MISMATCH / UNKNOWN
## Findings
| Severity | Domain | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Evidence gaps
- <gap or explicit none>
## Residual risk
- <risk or explicit none>
```
