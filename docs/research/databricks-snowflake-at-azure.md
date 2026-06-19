# Azure Databricks & Snowflake-on-Azure — Field Research

> Grounding for new providers `databricks` and `snowflake` (agents named `…-at-azure` to
> denote Azure-ecosystem deployment, per decision). Static-review reviewers; least-privilege
> and governance focused. **Date:** 2026-06-17 · Evidence: E3 vendor docs · Conf H/M/L.

---

## Part 1 — Azure Databricks (Unity Catalog governance + identity least privilege)

| Claim | Evidence | Conf |
|---|---|---|
| Unity Catalog top object is the **metastore** (one per region); data is a **three-level namespace** `catalog.schema.table`; securables also include views, volumes, functions, models, **storage credentials, external locations, connections, shares**. | [UC securable objects](https://docs.databricks.com/aws/en/data-governance/unity-catalog/securable-objects), [What is UC](https://docs.databricks.com/aws/en/data-governance/unity-catalog/) | H |
| Privilege model is `GRANT <priv> ON <securable> TO <principal>`; **`USE CATALOG`/`USE SCHEMA` are required but grant no data access**; `SELECT`/`MODIFY` need the parent USE grants; **privileges inherit** to child objects; **`ALL PRIVILEGES` excludes EXTERNAL USE / MANAGE**. | [manage privileges](https://docs.databricks.com/aws/en/data-governance/unity-catalog/manage-privileges/), [privileges ref](https://docs.databricks.com/aws/en/data-governance/unity-catalog/manage-privileges/privileges) | H |
| **Least privilege is schema-scoped**: Databricks recommends granting `CREATE TABLE`/`CREATE VOLUME`/`CREATE FUNCTION` at the **schema** level and `EXECUTE` on **individual** functions — never broad metastore/catalog grants. | [UC manage privileges](https://learn.microsoft.com/azure/databricks/data-governance/unity-catalog/manage-privileges/) | H |
| **Identity federation** is default and non-disablable on new workspaces; use **account groups**, not workspace-local groups; assign access to **groups, not individuals**. | [identity best practices](https://learn.microsoft.com/azure/databricks/admin/users-groups/best-practices) | H |
| **Admin layers are deliberately separated** — account admin (infra), workspace admin (workspace + catalog bootstrap), **metastore admin (optional, highly privileged, nominate a group)**. Assigning all three to one principal is an explicit anti-pattern. | [admin privileges](https://learn.microsoft.com/azure/databricks/data-governance/unity-catalog/manage-privileges/admin-privileges) | H |
| **Run production with service principals**, not interactive users — "interactive users do not need any write, delete, or modify privileges in production," eliminating accidental overwrite. SP can be a catalog/schema **owner**. | [identity best practices](https://learn.microsoft.com/azure/databricks/admin/users-groups/best-practices) | H |
| Entra integration: **Entra ID managed service principals** (OAuth/Entra tokens) for cross-Azure-resource auth; **automatic identity management** (default post-2025-08-01) syncs Entra users/SPs/nested groups (SCIM cannot sync SPs or nested groups). ADLS Gen2 access via UC **external locations + storage credentials** (managed identity), Key Vault-backed secret scopes. | [service principals](https://learn.microsoft.com/azure/databricks/admin/users-groups/service-principals), [SCIM/Entra](https://learn.microsoft.com/azure/databricks/admin/users-groups/scim/aad) | H |
| **Workspace-catalog binding** can restrict a catalog to specific workspaces and optionally **read-only**, overriding individual grants. | [workspace-catalog binding](https://docs.databricks.com/aws/en/data-governance/unity-catalog/access-control/workspace-catalog-binding) | H |
| Cert anchors: Databricks Certified **Data Engineer Associate/Professional**, **Data Analyst Associate** (E4 verify currency). | docs.databricks.com/certifications | M |

**Agent scope (provider `databricks`, `-at-azure`):**
- `databricks-unity-catalog-governance-at-azure` — UC securables, least-privilege grants (schema-scoped), workspace-catalog binding, identity federation/account groups, admin-layer separation, prod-via-SP. Category `security`.
- `databricks-lakehouse-engineering-at-azure` — Lakehouse/medallion, Spark notebooks/jobs, Delta, external locations to ADLS Gen2, cluster policies/access modes, capacity/cost. Category `data`.

---

## Part 2 — Snowflake on Azure (RBAC least privilege + governance)

| Claim | Evidence | Conf |
|---|---|---|
| RBAC: privileges → **roles** → users; system roles **ACCOUNTADMIN** (most powerful; billing; not a superuser), **SECURITYADMIN** (MANAGE GRANTS; users/roles), **USERADMIN**, **SYSADMIN** (create warehouses/databases), **PUBLIC**. | [access control overview](https://docs.snowflake.com/en/user-guide/security-access-control-overview) | H |
| **Least privilege**: create **custom roles** aligned to business functions on a narrow set of securables; **Snowflake recommends a role other than ACCOUNTADMIN for automated scripts**; build a role hierarchy under SYSADMIN. | [access control best practices](https://docs.snowflake.com/en/user-guide/security-access-control-considerations) | H |
| Separation of duties: **SECURITYADMIN** (grants/identity) vs **SYSADMIN** (objects); avoid granting to **PUBLIC**; use **future grants** and **managed-access schemas**; enforce **network policies** and **MFA**; periodic access review. | [best practices](https://docs.snowflake.com/en/user-guide/security-access-control-considerations) | H |
| Auth on Azure: prefer **key-pair** or **OAuth/SSO with Microsoft Entra ID as IdP** (SAML/SCIM), avoid passwords for service accounts. | [access control configure](https://docs.snowflake.com/en/user-guide/security-access-control-configure) | M |
| Azure integration: Snowflake on Azure, **Azure Private Link**, external stages via **storage integration** to Azure Blob/ADLS Gen2 (Entra tenant/managed identity), Entra OAuth. | docs.snowflake.com (Azure storage integration) | M |
| Governance: object **tagging**, **masking policies**, **row access policies**, **ACCESS_HISTORY** for least-privilege auditing. | [row access policies](https://docs.snowflake.com/en/user-guide/security-row-intro), [privileges](https://docs.snowflake.com/en/user-guide/security-access-control-privileges) | H |
| Cert anchor: **SnowPro Core** / **SnowPro Advanced: Architect/Data Engineer** (E4 verify). | snowflake.com/certifications | M |

**Agent scope (provider `snowflake`, `-at-azure`):**
- `snowflake-rbac-access-governance-at-azure` — role hierarchy, least privilege, ACCOUNTADMIN restriction, SECURITYADMIN/SYSADMIN SoD, future grants, network policies, Entra SSO/SCIM. Category `security`.
- `snowflake-data-platform-engineering-at-azure` — warehouses/databases, performance & cost, Azure Private Link/storage integration, masking/row-access/tagging governance. Category `data`.

---

## Cross-cutting design rules (alignment to the repo)

- **New providers** `databricks` + `snowflake` registered across the four schemas, `tests/validate-catalog.py` `ALLOWED_PROVIDERS`, `generate-kiro-powers.mjs`, `generate-docs-data.mjs`. Flat specialist providers (no maestro), like `backstage`/`falco`.
- **`-at-azure` naming** on every agent/skill id signals Azure-ecosystem deployment.
- **Static-review** tier (review notebooks/SQL/grants/policies as source + sanitized evidence; never execute against a live workspace/account). Production grant/role/policy/warehouse changes are **live-guard gated** (escalate) — a future mutating wave, same contract as the M365/D365 live agents.
- RBAC install roles: `azure-databricks-platform-engineer`, `azure-snowflake-platform-engineer` (or a shared `azure-data-platform-engineer`).

## Verification debt
- Re-confirm Databricks & Snowflake certification names/levels before stamping cert maps.
- Confirm exact Azure storage-integration / Private Link steps against current vendor docs at build time; mark SDK/CLI syntax for Context7 re-verification.
