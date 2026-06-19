# Permissions — Databricks Live Unity Catalog Grant Guard at Azure

## Execution tier

`mutating-runtime` (Phase B). Exactly one GRANT or REVOKE per invocation. Gated by explicit written human approval.

## Run-as principal

| Component | Requirement |
|---|---|
| Identity type | Entra-managed service principal (Entra app registration) — NOT a workspace-local user |
| Unity Catalog permission | MANAGE on the single target securable (schema, table, or volume), or IS OWNER of that securable |
| Scope | Scoped to the ONE target securable only — not metastore, not catalog, not all schemas |
| Entra role | No Azure RBAC role required beyond Contributor on the Azure Databricks workspace resource (for token auth); Unity Catalog permissions are separate |

The service principal must be added to the Unity Catalog metastore as an account-level identity and granted MANAGE or ownership on the specific target securable before first run. This setup must be performed by a metastore admin — not by this agent.

## Denied permissions (hard stops — must NOT be configured or executed)

- `metastore admin` role
- `account admin` role
- `workspace admin` role
- `ALL PRIVILEGES` grant on any securable
- `MANAGE` privilege grant at catalog level or above
- Ownership transfer (`ALTER <securable> OWNER TO`)
- Catalog-wide grants (target must be schema, table, or volume — not catalog or metastore)
- Any grant that touches more than one securable per invocation

## Credential posture

- **Required**: Entra-managed service principal (client ID + certificate or Entra federated credential via Azure Key Vault).
- **Acceptable**: OAuth M2M (machine-to-machine) via Entra — client credentials flow.
- **Forbidden**: interactive user tokens, personal access tokens (PATs) with broad workspace permissions, workspace-local service principals not backed by Entra.
- Credentials are referenced by environment variable name only: `DATABRICKS_HOST`, `DATABRICKS_CLIENT_ID`.
- Client secret or certificate must be stored in Azure Key Vault — never in repo, chat, or logs.

## Egress allow-list

- `DATABRICKS_HOST` — Azure Databricks workspace endpoint (e.g., `adb-<workspace-id>.<region>.azuredatabricks.net`)
- `login.microsoftonline.com` — Microsoft Entra OAuth 2.0 token endpoint for M2M client credentials flow

No other egress destinations are required or permitted for this agent.

## Blast-radius boundary

This agent executes exactly one Unity Catalog GRANT or REVOKE statement per invocation. The blast radius is bounded to the single securable named in the approval token. However:

- A schema-level grant propagates to all current tables and views within that schema — document this in the approval token blast radius.
- A volume-level grant affects all files within that volume.
- Granting to an account group propagates to all members of that group — document group membership size in the blast radius.
- Escalation via group membership or Unity Catalog inheritance must be assessed before approval.

Ensure the run-as service principal is monitored in the Databricks account console and Unity Catalog audit logs are enabled.
