# Permissions — D365 Live Record Field Update Guard

## Execution tier

`mutating-runtime` (Phase B). This agent performs a single, scoped PATCH on the Dataverse data plane. No bulk, wildcard, delete, or privilege-escalation operations are permitted.

## Required Dataverse data-plane permissions

| Component | Requirement |
|---|---|
| Application user | SystemUser row in the target Dataverse environment with `ApplicationId` set to the registered app's client ID |
| Security role | **Custom least-privilege write role** — not any predefined role. Write (prvWrite) on the ONE in-scope table only. Read (prvRead) on the same table to support PREFLIGHT capture of prior field values. Scope: record-level (owner-based) where the table supports it; organization-level Write only if owner-based scope is unavailable for the table type. |
| Privilege scope | Write (prvWrite) + Read (prvRead) on the single in-scope table only. No Create, Delete, Append, AppendTo on any table. No Write on any other table. |
| Field-level security | If the target fields are protected by a Field Security Profile, the application user's bound role must include the corresponding FieldPermission records for those specific columns only. No blanket Field Security Profile with write access to all secured columns. |

The custom least-privilege write role must be created by an environment System Administrator before first run, scoped to the in-scope table only, and associated with the application user in the Power Platform admin center.

## Denied permissions (must NOT be granted to the application user)

- `System Administrator` role
- `System Customizer` role
- Delete privilege on any table (`prvDelete`)
- Create privilege on any table (`prvCreate`)
- Bulk or multi-record write operations (any operation targeting more than one record ID at a time)
- Wildcard or all-records operations (no FetchXML or OData filter-based updates)
- Ownership change operations (`ownerid` field reassignment or `AssignRequest`)
- Security role or privilege edits (no write on `role`, `roleprivileges`, `systemuserroles`, `teamroles`, `roletemplate`)
- `prvActOnBehalfOfAnotherUser` privilege
- **Power Platform management SPN path** (`pac admin create-service-principal`): this registers the SPN as a tenant-wide Power Platform Administrator, which cannot be scoped down. This path is explicitly forbidden.
- Write on any table other than the single in-scope table

## Approval token requirement

Before any write, a **written human approval token** must be provided that explicitly names:

- The Dataverse environment (referenced by env-var name `DATAVERSE_ENV_URL`, not by value)
- The target table logical name
- The target record GUID
- The exact fields to update and their proposed new values
- The blast-radius assessment (what reads or depends on these field values)

An approval token that omits any of the above is rejected. The agent must not proceed with an incomplete token.

## Credential posture

- **Preferred**: certificate credential on the Entra app registration.
- **Acceptable**: client secret with short rotation (90 days maximum). Secret values must never appear in repo, chat, or logs.
- **Forbidden**: System Administrator-level credentials used as a shortcut, sharing credentials across agents, storing secret values in configuration files committed to source control.
- Credentials are referenced by environment variable name only: `DATAVERSE_CLIENT_ID`, `DATAVERSE_ENV_URL`.

## Egress allow-list

- `*.dynamics.com` — Dataverse Web API endpoint (environment-specific subdomain)
- `login.microsoftonline.com` — Microsoft Entra OAuth 2.0 token endpoint

No other egress destinations are required or permitted for this agent.

## Blast-radius boundary

This agent performs a single PATCH on one Dataverse record. The blast radius is:

- **Direct**: the record's field values change for the named fields only.
- **Indirect**: downstream Power Automate flows, plugins, or calculated fields that read the updated field values may be triggered or affected.
- **Integration impact**: any external system that reads this record's field values may observe the change.
- **Audit trail**: the change is recorded in the Dataverse audit log (if auditing is enabled for the table and fields).

Blast-radius must be assessed and documented in the approval token before any write proceeds.
