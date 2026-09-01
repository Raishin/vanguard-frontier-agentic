# Workflow And Output

Privilege-design review sequence and output contract for UC governance assessment.

## Workflow

1. Establish the UC structure: catalog names, schema names, table names, ownership chain.
2. Map privilege assignments: identify every GRANT and its scope (catalog, schema, table, column level if masks are in scope).
3. Check for privilege cascade: confirm that GRANT hierarchies exploit inheritance (one GRANT at the top level, not one GRANT per object).
4. Verify ownership: confirm that each securable has exactly one owner, no co-ownership, and an identified handoff path if the owner leaves.
5. Assess workspace-catalog binding: identify which catalogs are in ISOLATED mode, which workspaces are bound, and confirm that enforcement is active.
6. Validate governed tags: enumerate account-level tags, check character sets, verify inheritance rules, identify columns without explicit tags if needed.
7. Check storage credentials: verify that only the credential owner may delete, that binding is consistent, and that external locations are properly scoped.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (compliant-as-designed / compliant-with-conditions / governance-risk) with explicit confidence.
- Privilege hierarchy findings: cascade points, overly-broad grants, ANY instance of co-ownership or multi-owner design.
- Ownership design audit: single-principal enforcement, identified gaps, transfer mechanisms.
- Workspace-catalog binding status: ISOLATED mode enforcement, binding inventory, cross-workspace access impact.
- Governed-tag inventory and character-set validation; inheritance patterns and column-tag coverage.
- Storage-credential ownership and binding consistency; audit-log and lineage-coverage findings.
