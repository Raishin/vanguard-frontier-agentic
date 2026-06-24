# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, incident triage, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- System role holders and MFA enrollment; ACCOUNTADMIN count and break-glass procedure
- Role hierarchy shape: custom roles rolling to SYSADMIN vs. ACCOUNTADMIN; SoD between SECURITYADMIN and SYSADMIN
- Privilege grants: USAGE chain (database + schema), object privileges, future grants precedence, PUBLIC exposure
- Managed-access schemas: centralized grant control, schema owner vs. SECURITYADMIN
- Network policies: account vs. user vs. integration level, AZURELINKID rules, activation and precedence
- MFA enforcement: policy scope, service user TYPE=SERVICE, key-pair vs. OAuth requirement
- Entra ID integration: External OAuth token issuer alignment, SAML SSO configuration, SCIM AAD_PROVISIONER role

## Safe workflow

1. **Frame scope**
   - Account name, Snowflake edition, and Azure region:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Snowflake MCP or read-only SQL query evidence for current-state claims when available.
   - Otherwise inspect repository IaC/config (Terraform, SnowSQL scripts), sanitized user evidence, or official Snowflake docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What can expose data through over-privileged roles or PUBLIC grants?
   - What can escalate privilege through the role hierarchy?
   - What can break production or block rollback?
   - What compliance or audit evidence is missing?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Snowflake RBAC Access Governance Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
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
- Queries or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
