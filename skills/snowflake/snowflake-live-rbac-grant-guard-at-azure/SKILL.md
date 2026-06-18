---
name: snowflake-live-rbac-grant-guard-at-azure
description: Mutating-runtime live guard for Snowflake RBAC privilege management on Azure. Executes exactly ONE GRANT or REVOKE of a single privilege on a single securable to a single custom role — with explicit written human approval, dry-run preflight (SHOW GRANTS prior state), and a named rollback owner. Phase B strictly-scoped controlled mutation; never ACCOUNTADMIN/SECURITYADMIN/SYSADMIN/PUBLIC, never OWNERSHIP, never MANAGE GRANTS, never future grants at database or account scope.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-17"
  category: security
  execution_tier: mutating-runtime
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required:
      - "Least-privilege custom role with MANAGE GRANTS scoped narrowly to the target object, or IS OWNER of the target securable — NOT ACCOUNTADMIN"
      - "Key-pair authentication or Entra OAuth (Azure AD) — never password-based auth for automation"
    denied:
      - "ACCOUNTADMIN"
      - "SECURITYADMIN"
      - "SYSADMIN"
      - "PUBLIC role"
      - "OWNERSHIP privilege transfer"
      - "MANAGE GRANTS at account or database scope"
      - "Future grants at database or account scope (GRANT ... ON FUTURE ...)"
      - "Role creation (CREATE ROLE)"
  required_egress:
    - "SNOWFLAKE_ACCOUNT.snowflakecomputing.com (account endpoint; Azure Private Link supported)"
  requires_credentials:
    - "SNOWFLAKE_ACCOUNT"
    - "SNOWFLAKE_USER"
    - "SNOWFLAKE_PRIVATE_KEY_PATH"
  output_attestation:
    schema: "grant-guard-attestation-v1"
    signed_with: "idempotency-key"
    audit_log: "required"
  liveAgentFields:
    execution_tier: "mutating-runtime"
    gate: "explicit-written-human-approval"
    approval_token_requirements:
      - "exact securable name (database.schema.object or database.schema)"
      - "exact privilege name"
      - "exact custom role name"
      - "blast radius statement"
    dry_run_required: true
    idempotent: true
    idempotency_key: true
    audit_log: true
    prior_state_capture: true
    rollback_owner: "Snowflake ACCOUNTADMIN or SECURITYADMIN (human operator)"
    rollback_time_box: "30 minutes"
    never_auto_dispatched: true
  companion_agents:
    - "snowflake-live-rbac-grant-guard-at-azure-agent"
---

# Snowflake Live RBAC Grant Guard at Azure

## Purpose

Act as the live mutating guard for Snowflake RBAC privilege management on Azure. On receipt of explicit written human approval, execute exactly ONE `GRANT <privilege> ON <securable_type> <securable> TO ROLE <role>` statement — or its exact inverse `REVOKE` — for a single privilege, single securable, and single custom role. Capture prior state via `SHOW GRANTS` before execution. Emit a signed attestation. Never mutate without approval. Never grant to system roles (ACCOUNTADMIN, SECURITYADMIN, SYSADMIN, PUBLIC), never transfer OWNERSHIP, never use MANAGE GRANTS at account or database scope, never create future grants at broad scope.

## When to use

- A single, specific Snowflake privilege must be granted or revoked on one securable to one custom role, with a full audit trail
- A human operator has provided a written approval token naming the exact securable, privilege, role, and blast radius
- Prior-state capture via SHOW GRANTS and a named rollback plan are required before any mutation proceeds
- The operation must be idempotent and produce a signed attestation

## Live-guard gate

This skill operates at `mutating-runtime` (Phase B). It is **never auto-dispatched** by a maestro. Before any mutation executes:

1. Require explicit written human approval naming: exact securable, exact privilege, exact custom role, blast radius.
2. Run dry-run preflight: show current grants on the target securable (`SHOW GRANTS ON <securable_type> <securable>`) + the single SQL statement to be executed.
3. Confirm scope and environment with the approver.
4. Capture prior grant state before execution.
5. Execute the single statement.
6. Emit signed output attestation (`signed_with: idempotency-key`) referencing the approval token, idempotency key, statement executed, and prior state snapshot.

## Strictly-scoped operation contract

- **EXACTLY ONE** `GRANT ... TO ROLE` or `REVOKE ... FROM ROLE` per invocation — one privilege, one securable, one custom role.
- Target roles must be custom roles (not system roles). Verify role is not ACCOUNTADMIN, SECURITYADMIN, SYSADMIN, or PUBLIC before execution.
- **REVOKE is the inverse**: `REVOKE <privilege> ON <securable_type> <securable> FROM ROLE <role>`. Prior state captured via `SHOW GRANTS ON <securable>` before both GRANT and REVOKE.
- Idempotent: if the grant already exists (for GRANT) or is already absent (for REVOKE), record and return without error.

## Denied operations (hard stops)

- Any grant to ACCOUNTADMIN, SECURITYADMIN, SYSADMIN, or PUBLIC
- OWNERSHIP privilege (`GRANT OWNERSHIP ON ...`)
- MANAGE GRANTS privilege at account or database scope
- Future grants: `GRANT ... ON FUTURE <objects> IN DATABASE|ACCOUNT`
- Role creation (`CREATE ROLE`)
- Any bulk or wildcard operation touching more than one securable per invocation
- Password-based authentication for the run-as principal

## Credential posture

- Run as: least-privilege custom Snowflake role with MANAGE GRANTS scoped narrowly, or IS OWNER of the target securable — never ACCOUNTADMIN.
- Authentication: key-pair auth (`SNOWFLAKE_PRIVATE_KEY_PATH`) or Entra OAuth (Azure AD external OAuth). Never password-based for automation.
- Azure Private Link supported for the Snowflake account endpoint — recommended for production.
- Credentials referenced by environment variable name only: `SNOWFLAKE_ACCOUNT`, `SNOWFLAKE_USER`, `SNOWFLAKE_PRIVATE_KEY_PATH`.
- Private key file must never appear in repo, chat, or logs.

## Dry-run preflight output

Before execution, emit:

```
DRY-RUN PREFLIGHT
Target securable : <database>.<schema>[.<object>] (<type>)
Privilege        : <PRIVILEGE>
Target role      : <custom_role_name>
Operation        : GRANT | REVOKE
Current grants   : <output of SHOW GRANTS ON <securable_type> <securable>>
Statement to run : GRANT <privilege> ON <type> <database>.<schema>[.<object>] TO ROLE <role>;
Approval token   : <token from approval>
Blast radius     : <description>
```

Await explicit confirmation before proceeding.

## Rollback

- Prior state: capture `SHOW GRANTS ON <securable_type> <securable>` output before execution.
- Inverse statement: `REVOKE <privilege> ON <securable_type> <securable> FROM ROLE <role>` (for GRANT operations); `GRANT` inverse for REVOKE.
- Owner: Snowflake ACCOUNTADMIN or SECURITYADMIN (human operator — not automated).
- Time-box: rollback executable within 30 minutes of mutation.
- Note: Snowflake RBAC changes take effect immediately. If a role was used to access data between grant and rollback, that access cannot be recalled — document window.

## Lean operating rules

- Prefer docs.snowflake.com documentation for platform-documented behavior.
- Never print, echo, or log credential values; reference by env-var name only.
- Label all observations as sampled configured-environment evidence.
- If the request implies more than one securable, privilege, or role, push back — that exceeds this skill's scope.
- State what is unknown; documentation proves service behavior, not the account's deployed state.
- Azure Private Link note: if the Snowflake account uses Azure Private Link, egress must route through the private endpoint — not the public snowflakecomputing.com DNS.

## Official sources

- https://docs.snowflake.com/en/sql-reference/sql/grant-privilege
- https://docs.snowflake.com/en/user-guide/security-access-control-overview
- https://docs.snowflake.com/en/user-guide/security-access-control-considerations
- https://docs.snowflake.com/en/user-guide/key-pair-auth
- https://docs.snowflake.com/en/sql-reference/sql/show-grants
