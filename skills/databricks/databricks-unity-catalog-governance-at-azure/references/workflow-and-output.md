# Workflow and output contract

Use this reference only when performing the full governance review, incident triage, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- Namespace scope: metastore, catalog, schema, target securable, and intended operations
- Grant model: privilege type, parent USE grants present, group-based vs individual assignment
- Identity: account groups vs workspace-local groups, service principal vs interactive user
- Admin separation: account admin, workspace admin, metastore admin roles and their blast radii
- Workspace-catalog binding: read-only vs full, correct binding for target workload
- Least-privilege: schema-scoped grants preferred; ALL PRIVILEGES exclusions (EXTERNAL USE SCHEMA, MANAGE)
- Audit: system tables enabled, lineage and access logging configured
- Production posture: service principal identity, token lifecycle, Entra ID federation

## Safe workflow

1. **Frame scope**
   - Workspace/metastore/catalog/schema/environment:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only workspace MCP evidence, repository IaC (Terraform), or sanitized user-provided SQL/JSON for current-state claims.
   - Otherwise inspect official documentation.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What grants expose data beyond the intended consumer group?
   - What can escalate privilege in the metastore or account?
   - What interactive-user patterns break production automation?
   - What audit evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow grants, group-based assignment, staged rollout, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.
   - Production grant/policy changes are live-guard gated (escalate).

## Output contract

Return this structure:

```markdown
# Databricks Unity Catalog Governance Review: <scope>
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
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
