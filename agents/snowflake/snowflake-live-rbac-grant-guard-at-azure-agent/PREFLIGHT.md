# Preflight — Snowflake Live RBAC Grant Guard at Azure

Before any live Snowflake Live RBAC Grant Guard run, confirm ALL of the following:

## 1. Approval token validation

- Confirm an explicit written human approval token has been received naming:
  - Exact securable: `<database>.<schema>[.<object>]` and securable type (e.g., TABLE, VIEW, SCHEMA)
  - Exact privilege name (e.g., `SELECT`, `INSERT`, `USAGE`)
  - Exact custom role name (must not be ACCOUNTADMIN, SECURITYADMIN, SYSADMIN, or PUBLIC)
  - Blast radius statement
- If any element is missing or vague, stop — do not proceed until the approval token is complete and unambiguous.

## 2. Credential and identity confirmation

- Confirm `SNOWFLAKE_ACCOUNT`, `SNOWFLAKE_USER`, and `SNOWFLAKE_PRIVATE_KEY_PATH` environment variables are set. Do not print or echo their values.
- Confirm the run-as custom role exists in the Snowflake account and is NOT ACCOUNTADMIN, SECURITYADMIN, SYSADMIN, or PUBLIC.
- Confirm the run-as role holds MANAGE GRANTS narrowly scoped to the target object, or IS OWNER of the target securable.
- Confirm authentication method is key-pair or Entra OAuth — not password-based.

## 3. Target role assertion

- Confirm the target role (the role receiving the grant/revoke) is a custom role — NOT ACCOUNTADMIN, SECURITYADMIN, SYSADMIN, or PUBLIC.
- If the target role is any system role or PUBLIC, stop immediately.

## 4. Securable scope assertion

- Confirm exactly one securable is named. If more than one securable appears in the request, stop.
- Confirm the privilege is a named Snowflake privilege — not `OWNERSHIP`, not `MANAGE GRANTS` at account/database scope.
- Confirm the operation is not a future grant (`ON FUTURE <objects> IN DATABASE|ACCOUNT`).

## 5. Denied-operation check

Confirm NONE of the following are present in the requested operation:
- Grant TO `ACCOUNTADMIN`, `SECURITYADMIN`, `SYSADMIN`, or `PUBLIC`
- `OWNERSHIP` privilege (`GRANT OWNERSHIP ON ...`)
- `MANAGE GRANTS` at account or database scope
- Future grants: `GRANT ... ON FUTURE <objects> IN DATABASE|ACCOUNT`
- Role creation: `CREATE ROLE`

If any denied operation is detected, stop immediately and report the violation.

## 6. Dry-run preflight execution

Before executing the grant:
- Run `SHOW GRANTS ON <securable_type> <database>.<schema>[.<object>]` and display the current grant state.
- Display the exact SQL statement to be executed:
  `GRANT <privilege> ON <securable_type> <database>.<schema>[.<object>] TO ROLE <role>;`
  or
  `REVOKE <privilege> ON <securable_type> <database>.<schema>[.<object>] FROM ROLE <role>;`
- Await explicit human confirmation of the dry-run output before proceeding.

## 7. Prior state capture

- Record the full output of `SHOW GRANTS ON <securable_type> <securable>` as the prior state snapshot.
- This snapshot is required for rollback — do not proceed without it.

## 8. Idempotency key generation

- Generate an idempotency key (UUID v4 or equivalent) before issuing the `GRANT`/`REVOKE` statement.
- Record the idempotency key in the pre-write audit log entry and carry it into the signed attestation (`signed_with: idempotency-key`).
- If the same idempotency key has already been used for a completed write against this securable, **stop** — do not replay.

## 9. Environment check

- Confirm outbound egress to `SNOWFLAKE_ACCOUNT.snowflakecomputing.com` (or Azure Private Link endpoint) is permitted from the execution environment.
- Confirm Snowflake access history logging is enabled for the run-as user.
- Confirm no prior invocation of this agent is pending rollback for the same securable.
- If Azure Private Link is configured for this account, confirm the private endpoint hostname is in use — not the public snowflakecomputing.com DNS.

## Block conditions

Stop and do not proceed if any of the following are true:

- No explicit written human approval token has been received.
- The approval token does not name the exact securable, privilege, custom role, and blast radius.
- The target role (receiving the grant) is ACCOUNTADMIN, SECURITYADMIN, SYSADMIN, or PUBLIC.
- More than one securable appears in the request.
- The requested operation is OWNERSHIP, MANAGE GRANTS at broad scope, or a future grant.
- The run-as role is ACCOUNTADMIN, SECURITYADMIN, or SYSADMIN.
- A credential value has been exposed in any log, chat, or environment dump.
- A prior run for the same securable is pending rollback.
