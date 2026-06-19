# Permissions — D365 Live Security Role Guard

## Execution tier

`read-only-runtime` (Phase A). No mutation permitted in this phase.

## Required Dataverse data-plane permissions

| Component | Requirement |
|---|---|
| Application user | SystemUser row in the target Dataverse environment with `ApplicationId` set to the registered app's client ID |
| Security role | **Custom read-only security role** — not any predefined role. Read privilege on: `systemuser`, `role`, `roleprivileges`, `team`, `businessunit`, `systemuserroles`, `teamroles`, `roletemplate` tables. Scope: Organization-level Read only. |
| Privilege scope | Read (prvRead) on in-scope tables. No Create, Write, Delete, Append, AppendTo on any table. |

The custom read-only security role must be created by an environment System Administrator before first run and associated with the application user in the Power Platform admin center.

## Denied permissions (must NOT be granted to the application user)

- `System Administrator` role
- `System Customizer` role
- Create privilege on any table
- Write privilege on any table
- Delete privilege on any table
- Append privilege on any table
- AppendTo privilege on any table
- `prvActOnBehalfOfAnotherUser` privilege
- **Power Platform management SPN path** (`pac admin create-service-principal`): this registers the SPN as a tenant-wide Power Platform Administrator, which cannot be scoped down. This path is explicitly forbidden for this agent.

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

This agent performs read-only Dataverse Web API GET/query calls. It cannot modify security roles, user assignments, or environment configuration. The risk surface is read access to security role privilege matrices and user assignment data, which is sensitive. Ensure the application user is monitored in the Power Platform admin center and the custom role is reviewed periodically.
