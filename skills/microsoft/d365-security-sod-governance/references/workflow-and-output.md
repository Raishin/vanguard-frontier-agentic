# Workflow and output contract

Use this reference only when performing the full SoD or security review, implementation guidance, audit evidence gathering, or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Role structure: role hierarchy, duty composition, privilege assignments, entry point permissions
- SoD rule set: duty pairs covered, severity levels, missing business-critical rules (e.g., vendor maintenance + payment processing, goods receipt + vendor payment)
- User-role assignments: users holding conflicting roles, SoD override history with justifications
- Privileged access: system administrator role usage, super-user accounts, break-glass procedures
- Legal entity scoping: whether role assignments are appropriately restricted by legal entity
- Security reports: evidence that reports have been run and reviewed by compliance or audit teams
- Compensating controls: detective controls in place where SoD preventive controls are overridden

## Safe workflow

1. **Frame scope**
   - Environment / legal entities in scope:
   - Business process domain (e.g., procure-to-pay, order-to-cash, record-to-report):
   - Compliance driver (SOX, internal audit, IFRS, FDA, other):
   - Required outcome (new role design / conflict remediation / audit evidence):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported security reports (duty assignment report, roles violating SoD view, user role assignments report) for current-state claims.
   - Otherwise inspect sanitized user-provided evidence, role definition exports, or official D365 documentation.
   - Label each finding as `live evidence`, `report evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What duty pairs could enable fraud (e.g., creating a vendor and approving their payment)?
   - What broad privileges or system administrator assignments exist without justification?
   - What SoD conflicts are overridden without documented compensating controls?
   - What evidence is missing that would change the verdict?
   - What role changes have been made without SoD validation?

4. **Recommend the smallest safe action**
   - Prefer duty segregation over role merging, staged role rollout, and SoD rule validation before production deployment.
   - If the safest action is to stop and gather evidence (run security reports first), say that plainly.
   - Production role changes require live-guard escalation. Do not recommend live role changes without explicit human approval.

## Output contract

Return this structure:

```markdown
# D365 SoD & Security Review: <scope>
## Executive verdict
- Status: COMPLIANT / COMPLIANT WITH RISKS / NON-COMPLIANT / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Reports or checks to run:
- Expected result:
## Residual risk
- <risk or explicit none>
```
