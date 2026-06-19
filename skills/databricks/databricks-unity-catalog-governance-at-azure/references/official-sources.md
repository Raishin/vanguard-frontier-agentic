# Official sources

Use this reference only when you need source grounding for Databricks Unity Catalog or Azure service behavior, or the detailed source list.

## Databricks documentation

Use these as starting points, not as proof of the user's live workspace state:
- https://docs.databricks.com/en/data-governance/unity-catalog/index.html
- https://docs.databricks.com/en/data-governance/unity-catalog/manage-privileges/privileges.html
- https://docs.databricks.com/en/admin/users-groups/service-principals.html
- https://learn.microsoft.com/en-us/azure/databricks/data-governance/unity-catalog/

## Grounding rule

Official documentation explains Databricks Unity Catalog service behavior. It does not prove the user's current workspace, metastore, catalog configuration, grant assignments, or operational state. Prefer read-only workspace MCP or CLI evidence, repository evidence (Terraform/IaC), or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Service facts from official docs:

**Three-level namespace:** Unity Catalog organizes data assets in a metastore → catalog → schema → table/view/volume/function hierarchy. A single metastore is attached per Azure region per account.

**GRANT model:** USE CATALOG and USE SCHEMA grant namespace traversal but no data access on their own. SELECT and MODIFY on tables/views require the parent USE CATALOG and USE SCHEMA grants. ALL PRIVILEGES on a securable does not include EXTERNAL USE SCHEMA or MANAGE; those must be granted explicitly.

**Least-privilege pattern:** Prefer schema-scoped grants (CREATE TABLE, CREATE VOLUME, CREATE FUNCTION at the schema level) over catalog-wide or ALL PRIVILEGES grants.

**Identity federation:** Use account groups (not workspace-local groups) for production. Assign grants to groups, not to individual users or service principals directly where avoidable. Microsoft Entra ID managed service principals are the preferred automation identity.

**Admin separation:** Account admin, workspace admin, and metastore admin are distinct roles with separate blast radii. Do not conflate them.

**Service principal posture:** Production automated workloads must run as SERVICE PRINCIPALs (Microsoft Entra ID), not interactive users. Interactive user tokens expire and carry broader implicit access.

**Workspace-catalog binding:** Workspaces can be bound to catalogs in read-only or read-write mode. Validate binding intent before assigning broad catalog-level grants.

**Audit:** Unity Catalog system tables (`system.access.audit`, `system.access.column_lineage`, `system.access.table_lineage`) provide audit trails. Confirm system schema is enabled on the metastore.

**Certification reference:** DP-750 (Azure Databricks Data Engineer Associate) covers Unity Catalog governance fundamentals.

Review implications:
- Do not approve broad catalog or ALL PRIVILEGES grants from intent alone. Require scope justification, group-based assignment, service principal identity, and metastore admin sign-off.
- Documentation cannot prove the user's actual metastore, workspace binding, or live grant assignments.
