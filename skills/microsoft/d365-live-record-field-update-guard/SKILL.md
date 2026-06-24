---
name: d365-live-record-field-update-guard
description: Mutating-runtime live-guard for updating one or more named fields on a single Dataverse row identified by table and record GUID, via the Dataverse Web API PATCH (data plane). Strictly scoped — one record, named fields only. Requires explicit written human approval token referencing the exact target, proposed change, and blast-radius. PREFLIGHT performs a dry-run diff before any write. Fully reversible — prior field values are captured and the inverse PATCH is the rollback. Gate-only; never auto-dispatched. Phase B mutating-runtime.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-17"
  category: data
  execution_tier: mutating-runtime
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required:
      - "Custom Dataverse security role with Write (prvWrite) on ONLY the one in-scope table — record-level/owner-scoped where supported"
      - "Read (prvRead) on the same table to capture prior field values for ROLLBACK"
      - "Application user (SystemUser row) in the target Dataverse environment bound to the custom least-privilege write role — NOT System Administrator, NOT System Customizer"
      - "Dataverse data-plane access via S2S application user (ApplicationId/AzureActiveDirectoryObjectId on SystemUser)"
    denied:
      - "System Administrator"
      - "System Customizer"
      - "Delete privilege on any table (prvDelete)"
      - "Bulk/multi-record write operations (any query that targets more than one record ID)"
      - "Wildcard or all-records operations"
      - "Ownership change operations (ownerid field reassignment)"
      - "Security role or privilege edits (no write on role, roleprivileges, systemuserroles, teamroles)"
      - "prvActOnBehalfOfAnotherUser"
      - "Power Platform management SPN path (pac admin create-service-principal — cannot be least-privileged)"
      - "Write on any table other than the single in-scope table"
  required_egress:
    - "*.dynamics.com"
    - "login.microsoftonline.com"
  requires_credentials:
    - "DATAVERSE_CLIENT_ID"
    - "DATAVERSE_ENV_URL"
  output_attestation:
    schema: "field-update-attestation-v1"
    signed_with: "idempotency-key"
    audit_log: "required"
  liveAgentFields:
    execution_tier: "mutating-runtime"
    single_op: true
    reversible: true
    requires_approval_token: true
    dry_run_preflight: true
    idempotency_key: true
    blast_radius_required: true
  companion_agents:
    - "d365-live-record-field-update-guard-agent"
---

# D365 Live Record Field Update Guard

## Purpose

Act as the live mutating-runtime Dataverse field-update guard. On receipt of an explicit written human approval token, authenticate as an application user bound to a custom least-privilege write role, capture the prior field values of the target record (DRY-RUN / PREFLIGHT diff), and apply a PATCH to update ONLY the named fields on the ONE identified Dataverse row. Emit a signed, idempotency-keyed attestation and update the audit log. Reverse path is always available.

## When to use

- A named set of fields on a single identified Dataverse record must be updated in a controlled, auditable, reversible operation
- The update requires a human-approved, blast-radius-reviewed gate before any write proceeds
- A prior Phase A discovery (d365-live-security-role-guard or similar) identified a specific field value that must be corrected
- Compliance or data-quality remediation requires a traceable single-record mutation with capture of prior state

## Gate-only classification

This skill id contains `-live-` and ends in `-guard`. The maestro MUST NOT auto-dispatch this skill. Invocation requires:

1. An explicit **written human approval token** that references:
   - The exact Dataverse environment URL (by env-var name, not value)
   - The target table (logical name)
   - The target record GUID
   - The named fields to update and their proposed new values
   - The blast-radius assessment (who/what else reads or depends on these field values)
2. Completion of PREFLIGHT (dry-run diff confirming the target scope and environment)
3. Prior-state capture before any write

## Strict-control contract

- `execution_tier: mutating-runtime`
- **EXACTLY ONE** Dataverse record is updated per approved run. Target is identified by table logical name + record GUID.
- **Named fields only** — the PATCH body contains only the explicitly approved field names. No catch-all updates.
- **No bulk, no wildcard, no all-records** operations of any kind.
- **No irreversible delete** — this skill performs PATCH only; DELETE is explicitly refused.
- **No privilege escalation** — the application user holds Write on the in-scope table only.
- **No ownership changes** — `ownerid` and ownership-related fields are refused.
- **No security role or privilege edits** — `role`, `roleprivileges`, `systemuserroles`, `teamroles` are out of scope.

## Critical IAM constraint

The Power Platform management SPN path (`pac admin create-service-principal`) grants Power Platform Administrator privileges that cannot be scoped down. This skill explicitly forbids that path. The application user must be created manually in the target Dataverse environment and bound to a custom security role with Write (prvWrite) on ONLY the one in-scope table.

## Credential posture

- App registration: use a certificate credential or managed identity — never a long-lived client secret.
- Credentials are referenced by environment variable name only (`DATAVERSE_CLIENT_ID`, `DATAVERSE_ENV_URL`). Never print, echo, or log credential values.
- The application user must be created in the target Dataverse environment and associated with the custom least-privilege write role before this skill runs.

## PREFLIGHT (dry-run) requirements

Before issuing the PATCH:

1. Authenticate and confirm the application user identity and bound role.
2. Perform a GET on the target record (`/api/data/v9.2/<tableset>(<guid>)?$select=<field1>,<field2>,...`) to retrieve the CURRENT values of all fields to be updated.
3. Emit a diff: CURRENT values vs PROPOSED values for each named field.
4. Confirm the target record exists and is in the expected environment.
5. Confirm no other approval-pending operation targets the same record.
6. Present the diff to the approver for final confirmation before writing.

## Write operation

```http
PATCH [DATAVERSE_ENV_URL]/api/data/v9.2/<tableset>(<record-guid>) HTTP/1.1
Authorization: Bearer <token>
OData-MaxVersion: 4.0
OData-Version: 4.0
If-Match: *
Content-Type: application/json

{
  "<field1>": "<approved-value-1>",
  "<field2>": "<approved-value-2>"
}
```

`If-Match: *` ensures this is an update-only — it will 404 rather than create a new record.

## Rollback path

- Prior field values captured in PREFLIGHT GET must be retained.
- Rollback = PATCH the same record with the captured prior values (inverse operation).
- See ROLLBACK.md for owner, time-box, and verification steps.

## Output attestation

Every completed run must emit:

- Idempotency key (generated before the write; used to detect replay)
- Record of: environment (env-var reference), table, record GUID, fields updated, prior values, new values, approval token reference
- Audit log entry written before and after the write
- Result: success (HTTP 204) or failure with error detail

## Lean operating rules

- Prefer Microsoft Learn documentation through the configured documentation MCP for Dataverse and Power Platform service behavior.
- Use sampled live Dataverse Web API evidence; label it as live configured-environment evidence.
- Never request or accept credential values — env-var names only.
- If the request implies more than one record, bulk operations, delete, ownership change, or security role edit — refuse and explain why this skill cannot perform that operation.
- State what is unknown; documentation proves service behavior, not the environment's deployed state.

## Refuse conditions

Immediately refuse and do not proceed if:

- More than one record GUID is specified
- A wildcard, filter, or FetchXML targeting multiple records is used
- A DELETE operation is requested
- `ownerid` or ownership-related columns appear in the update payload
- Security role, privilege, or user assignment columns appear in the update payload
- No written approval token is provided
- The approval token does not reference the exact table + GUID + field names
- The application user holds System Administrator or System Customizer

## Official sources

- https://learn.microsoft.com/power-apps/developer/data-platform/webapi/update-delete-entities-using-web-api
- https://learn.microsoft.com/power-apps/developer/data-platform/column-level-security
- https://learn.microsoft.com/power-apps/developer/data-platform/use-multi-tenant-server-server-authentication
- https://learn.microsoft.com/power-platform/admin/database-security
- https://learn.microsoft.com/power-apps/developer/data-platform/reference/entities/fieldsecurityprofile
