# Permissions — Snowflake Live RBAC Grant Guard at Azure

## Execution tier

`mutating-runtime` (Phase B). Exactly one GRANT or REVOKE per invocation. Gated by explicit written human approval.

## Run-as principal

| Component | Requirement |
|---|---|
| Identity type | Least-privilege custom Snowflake role — NOT ACCOUNTADMIN, SECURITYADMIN, SYSADMIN, or PUBLIC |
| Snowflake permission | OWNERSHIP (IS OWNER) of the ONE target securable — a role can GRANT/REVOKE privileges only on objects it owns. `MANAGE GRANTS` is an **account-level global** privilege that cannot be object-scoped, so it is **not** used here (it is in the denied list). |
| Scope | Scoped to the ONE target securable the run-as role owns — not account-wide, not database-wide |
| Authentication | Key-pair authentication (`SNOWFLAKE_PRIVATE_KEY_PATH`) or Entra OAuth (Azure AD external OAuth integration) — never password-based |

The custom run-as role must be granted OWNERSHIP of the target securable by an ACCOUNTADMIN or SECURITYADMIN before first run — not by this agent. (`MANAGE GRANTS` is never granted to it; that privilege is account-wide and would violate least privilege.)

## Denied permissions (hard stops — must NOT be configured or executed)

- Run-as role = `ACCOUNTADMIN`
- Run-as role = `SECURITYADMIN`
- Run-as role = `SYSADMIN`
- Grants TO role `ACCOUNTADMIN`, `SECURITYADMIN`, `SYSADMIN`, or `PUBLIC`
- `OWNERSHIP` privilege transfer (`GRANT OWNERSHIP ON ...`)
- `MANAGE GRANTS` privilege at account or database scope
- Future grants: `GRANT ... ON FUTURE <objects> IN DATABASE|ACCOUNT`
- Role creation: `CREATE ROLE`
- Any operation touching more than one securable per invocation

## Credential posture

- **Required**: key-pair authentication — RSA private key referenced by `SNOWFLAKE_PRIVATE_KEY_PATH`; public key registered on the Snowflake user object.
- **Acceptable**: Entra OAuth external OAuth integration (Azure AD token exchange) for Snowflake.
- **Forbidden**: password-based authentication for automation, sharing credentials across agents, storing private key content in repo, chat, or environment dumps.
- Credentials are referenced by environment variable name only: `SNOWFLAKE_ACCOUNT`, `SNOWFLAKE_USER`, `SNOWFLAKE_PRIVATE_KEY_PATH`.
- Private key must be stored in Azure Key Vault or equivalent secrets manager — never in repo or logs.

## Egress allow-list

- `SNOWFLAKE_ACCOUNT.snowflakecomputing.com` — Snowflake account endpoint
- Azure Private Link note: if the Snowflake account uses Azure Private Link, egress must route through the private endpoint and the public snowflakecomputing.com DNS must NOT be used. Configure the private endpoint hostname accordingly.

No other egress destinations are required or permitted for this agent.

## Blast-radius boundary

This agent executes exactly one Snowflake GRANT or REVOKE statement per invocation. The blast radius is bounded to the single securable and single custom role named in the approval token. However:

- Snowflake RBAC privilege changes take effect immediately — there is no staging or preview mode in production.
- Granting a privilege to a role propagates to all roles that have been granted that role (role hierarchy) — assess the full role hierarchy before approval.
- A schema-level grant (e.g., USAGE ON SCHEMA) is required before object-level grants are effective — document dependency in the blast radius.
- Data accessed by a role during the privilege window between grant and rollback cannot be recalled — document this window in the approval token.

Ensure Snowflake account-level access history and query history are enabled for the run-as user.
