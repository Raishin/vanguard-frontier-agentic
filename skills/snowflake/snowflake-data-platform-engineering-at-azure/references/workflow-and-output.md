# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Virtual warehouse sizing, AUTO_SUSPEND/AUTO_RESUME settings, and workload isolation
- Azure Private Link activation, USE_PRIVATELINK_ENDPOINT settings, and edition requirements
- Storage integration: service principal trust, STORAGE_ALLOWED_LOCATIONS, RBAC assignments (Reader vs. Contributor)
- External stage URL format (azure:// blob endpoint for ADLS Gen2), private endpoint routing
- Snowpipe: notification integration type, blob + queue private endpoint configuration
- Object tagging: schema-level ownership, inheritance, 50-tag limit, APPLY TAG privilege
- Dynamic data masking: Enterprise requirement, APPLY privilege, tag-based masking automation
- Row access policies: Enterprise requirement, policy-owner evaluation, IS_ROLE_IN_SESSION(), one-policy-per-table limit
- ACCESS_HISTORY: Enterprise requirement, lineage columns, POLICIES_REFERENCED, retention period

## Safe workflow

1. **Frame scope**
   - Account name, Snowflake edition, and Azure region:
   - Business criticality and owner:
   - Data classification and compliance driver (GDPR, CCPA, etc.):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Snowflake MCP or read-only SQL query evidence for current-state claims when available.
   - Otherwise inspect repository IaC/config (Terraform, SnowSQL scripts), sanitized user evidence, or official Snowflake docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What can expose data through missing masking policies or ungoverned row access?
   - What can create unbounded cost through oversized or unsuspended warehouses?
   - What can break production or block rollback?
   - What compliance or audit lineage evidence is missing?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Snowflake Data Platform Engineering Review: <scope>
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
