# Workflow and output contract

Use this reference only when performing the full engineering review, incident triage, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- Architecture: medallion layer design (bronze/silver/gold), Delta Lake table properties, schema enforcement
- Storage access: ADLS Gen2 HNS enabled, external location design, storage credential (Access Connector managed identity)
- Credential passthrough: detect deprecated pattern; require migration plan to Unity Catalog access controls
- Cluster access mode: Dedicated vs Standard vs No Isolation Shared; cluster policy enforcement via Premium plan
- Secret management: AKV-backed secret scope read-only semantics, Vault access policy model
- Network isolation: VNet injection configuration, Private Link workspace and data plane endpoints
- Production posture: service principal identity, Access Connector managed identity, no interactive-user storage access

## Safe workflow

1. **Frame scope**
   - Workspace/metastore/cluster/environment:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only workspace MCP evidence, repository IaC (Terraform), notebook/SQL source, or sanitized user-provided evidence for current-state claims.
   - Otherwise inspect official documentation.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What can expose storage data to unintended identities?
   - What deprecated patterns create a compliance or security gap?
   - What cluster mode violations break Unity Catalog enforcement?
   - What network paths bypass Private Link isolation?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer Unity Catalog managed access, managed identity, least-privilege cluster policies, and validated rollback.
   - If the safest action is to stop and gather evidence, say that plainly.
   - Production cluster/storage/network changes are live-guard gated (escalate).

## Output contract

Return this structure:

```markdown
# Databricks Lakehouse Engineering Review: <scope>
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
