# Permissions — M365 Live Identity Posture Guard

## Execution tier

`read-only-runtime` (Phase A). No mutation permitted in this phase.

## Required Microsoft Graph application permissions (admin-consented)

| Permission | Purpose |
|---|---|
| `Directory.Read.All` | Read users, groups, service principals, and directory objects |
| `Policy.Read.All` | Read all tenant policies |
| `Policy.Read.ConditionalAccess` | Read Conditional Access policies (narrower than Policy.Read.All; both declared) |
| `RoleManagement.Read.Directory` | Read directory role assignments and PIM configuration |
| `AuditLog.Read.All` | Read sign-in logs, risky sign-ins, and risky users |

All permissions are **application (app-only)** permissions, not delegated. Admin consent by a Privileged Role Administrator is required before first run.

## Denied permissions (must NOT be granted)

- `Directory.ReadWrite.All`
- `Policy.ReadWrite.ConditionalAccess`
- `RoleManagement.ReadWrite.Directory`
- `User.ReadWrite.All`
- `Group.ReadWrite.All`
- `Application.ReadWrite.All`
- Any `*.Write` or `*.ReadWrite.*` Microsoft Graph permission

## Credential posture

- **Preferred**: certificate credential on the app registration, or a managed identity (Azure workload identity).
- **Acceptable**: client secret with short rotation (90 days maximum). Client secret values must never appear in repo, chat, or logs.
- **Forbidden**: long-lived secrets (>90 days), sharing credentials across agents/workloads, storing secret values in configuration files committed to source control.
- Credentials are referenced by environment variable name only: `GRAPH_CLIENT_ID`, `GRAPH_TENANT_ID`.

## Egress allow-list

- `graph.microsoft.com` — Microsoft Graph API
- `login.microsoftonline.com` — Microsoft Entra OAuth 2.0 token endpoint

No other egress destinations are required or permitted for this agent.

## Blast-radius boundary

This agent performs read-only Graph API calls. It cannot modify tenant configuration. The only risk surface is read access to sensitive directory data (user attributes, sign-in logs, role assignments). Ensure the app registration is governed by a Conditional Access policy for workload identities and is visible in Entra app governance.
