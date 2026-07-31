---
name: databricks-live-unity-catalog-grant-guard-at-azure
description: Mutating-runtime live guard for Unity Catalog privilege management on Azure Databricks. Executes exactly ONE GRANT or REVOKE of a single privilege on a single Unity Catalog securable (schema, table, or volume) to a single principal — with explicit written human approval, dry-run preflight, prior-state capture, and a named rollback owner. Phase B strictly-scoped controlled mutation; never bulk, never wildcard, never ALL PRIVILEGES, never metastore/account admin grants.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-17"
  category: security
  execution_tier: mutating-runtime
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required:
      - "Service principal (Entra-managed) that holds MANAGE or IS OWNER of the single target securable (schema, table, or volume) — scoped to that one object only"
      - "Unity Catalog privilege: MANAGE on the ONE target securable, or IS OWNER — not metastore admin, not account admin"
    denied:
      - "metastore admin"
      - "account admin"
      - "workspace admin"
      - "ALL PRIVILEGES grant"
      - "MANAGE grant at catalog level or above"
      - "Ownership transfer (ALTER ... OWNER TO)"
      - "catalog-wide grants"
      - "MANAGE grant on metastore or catalog"
  required_egress:
    - "DATABRICKS_HOST (workspace endpoint, e.g. adb-<id>.azuredatabricks.net)"
    - "login.microsoftonline.com"
  requires_credentials:
    - "DATABRICKS_HOST"
    - "DATABRICKS_CLIENT_ID"
  output_attestation:
    schema: "grant-guard-attestation-v1"
    signed_with: "idempotency-key"
    audit_log: "required"
  liveAgentFields:
    execution_tier: "mutating-runtime"
    gate: "explicit-written-human-approval"
    approval_token_requirements:
      - "exact securable name (catalog.schema.object or catalog.schema)"
      - "exact privilege name"
      - "exact principal name"
      - "blast radius statement"
    dry_run_required: true
    idempotent: true
    idempotency_key: true
    audit_log: true
    prior_state_capture: true
    rollback_owner: "Databricks workspace admin or Unity Catalog metastore admin"
    rollback_time_box: "30 minutes"
    never_auto_dispatched: true
  companion_agents:
    - "databricks-live-unity-catalog-grant-guard-at-azure-agent"
---

# Databricks Live Unity Catalog Grant Guard at Azure

## Purpose

Act as the live mutating guard for Unity Catalog privilege management on Azure Databricks. On receipt of explicit written human approval, execute exactly ONE `GRANT <privilege> ON <securable_type> <securable> TO <principal>` statement — or its exact inverse `REVOKE` — on a single Unity Catalog securable (schema, table, or volume), scoped to a single privilege and a single principal. Capture prior state before execution. Emit a signed attestation. Never mutate without approval. Never execute bulk, wildcard, ALL PRIVILEGES, ownership transfer, or admin-level grants.

## When to use

- A single, specific Unity Catalog privilege must be granted or revoked on one securable, with a full audit trail
- A human operator has provided a written approval token naming the exact securable, privilege, principal, and blast radius
- A prior-state capture and named rollback plan are required before any mutation proceeds
- The operation must be idempotent and produce a signed attestation

## Live-guard gate

This skill operates at `mutating-runtime` (Phase B). It is **never auto-dispatched** by a maestro. Before any mutation executes:

1. Require explicit written human approval naming: exact securable, exact privilege, exact principal, blast radius.
2. Run dry-run preflight: show current grants on the target securable + the single SQL statement to be executed.
3. Confirm scope and environment with the approver.
4. Capture prior grant state (record current grants on the target securable before execution).
5. Execute the single statement.
6. Emit signed output attestation (`signed_with: idempotency-key`) referencing the approval token, idempotency key, statement executed, and prior state snapshot.

## Strictly-scoped operation contract

- **EXACTLY ONE** `GRANT` or `REVOKE` per invocation — one privilege, one securable, one principal.
- Securable types: schema, table, or volume within Unity Catalog.
- Preferred principals: account groups. Service principals acceptable. Interactive users require additional justification.
- **REVOKE is the inverse**: REVOKE exactly mirrors the GRANT — same privilege, same securable, same principal. Prior state must be captured before REVOKE.
- Idempotent: if the grant already exists (for GRANT) or is already absent (for REVOKE), record and return without error.

## Denied operations (hard stops)

- `ALL PRIVILEGES` grant on any securable
- `MANAGE` privilege grant on metastore, catalog, or schema not owned by the run-as SP
- Ownership transfer (`ALTER <securable> OWNER TO`)
- `account-admin` or `metastore-admin` group grants
- Catalog-level grants (target must be schema, table, or volume — not catalog or metastore)
- Any bulk or wildcard operation touching more than one securable per invocation
- Grants to `account users` at broad scope

## Credential posture

- Run as: least-privilege Entra-managed service principal holding MANAGE or IS OWNER on the single target securable only.
- Credentials referenced by environment variable name only: `DATABRICKS_HOST`, `DATABRICKS_CLIENT_ID`.
- Client secret or certificate managed via Azure Key Vault — never stored in repo, chat, or logs.
- Entra app registration recommended; Entra-managed SP required (not workspace-local user).

## Dry-run preflight output

Before execution, emit:

```
DRY-RUN PREFLIGHT
Target securable : <catalog>.<schema>[.<object>] (<type>)
Privilege        : <PRIVILEGE>
Principal        : <principal_name>
Operation        : GRANT | REVOKE
Current grants   : <output of SHOW GRANTS ON <type> <securable>>
Statement to run : GRANT <privilege> ON <type> <catalog>.<schema>[.<object>] TO `<principal>`;
Approval token   : <token from approval>
Blast radius     : <description>
```

Await explicit confirmation before proceeding.

## Rollback

- Prior state: capture `SHOW GRANTS ON <securable_type> <securable>` output before execution.
- Inverse statement: `REVOKE <privilege> ON <securable_type> <securable> FROM `<principal>`` (for GRANT operations); `GRANT` inverse for REVOKE.
- Owner: Databricks workspace admin or Unity Catalog metastore admin.
- Time-box: rollback executable within 30 minutes of mutation.

## Lean operating rules

- Prefer docs.databricks.com and learn.microsoft.com documentation for platform-documented behavior.
- Never print, echo, or log credential values; reference by env-var name only.
- Label all observations as sampled configured-environment evidence.
- If the request implies more than one securable, push back — that exceeds this skill's scope.
- State what is unknown; documentation proves service behavior, not the workspace's deployed state.

## Official sources

- https://docs.databricks.com/en/data-governance/unity-catalog/manage-privileges/privileges.html
- https://docs.databricks.com/en/data-governance/unity-catalog/manage-privileges/index.html
- https://docs.databricks.com/en/sql/language-manual/sql-ref-syntax-ddl-grant.html
- https://docs.databricks.com/en/admin/users-groups/service-principals.html
- https://learn.microsoft.com/en-us/azure/databricks/data-governance/unity-catalog/
