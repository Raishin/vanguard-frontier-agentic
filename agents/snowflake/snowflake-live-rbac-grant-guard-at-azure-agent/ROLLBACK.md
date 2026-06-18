# Rollback — Snowflake Live RBAC Grant Guard at Azure

## Execution tier

`mutating-runtime` (Phase B). A mutation was performed. Rollback is required if the grant must be reversed.

## Rollback contract

This agent executes exactly one `GRANT ... TO ROLE` or `REVOKE ... FROM ROLE` statement per invocation. The rollback contract applies to that single statement.

### For a GRANT operation

- **Prior state**: the `SHOW GRANTS ON <securable_type> <securable>` output captured during preflight.
- **Inverse statement**: `REVOKE <privilege> ON <securable_type> <database>.<schema>[.<object>] FROM ROLE <role>;`
- **Owner**: Snowflake ACCOUNTADMIN or SECURITYADMIN (human operator — not automated).
- **Time-box**: rollback must be executable within 30 minutes of the original mutation.
- **Verification**: after rollback, run `SHOW GRANTS ON <securable_type> <securable>` and confirm the grant is absent and the prior state matches the captured snapshot.
- **Idempotency**: if the original GRANT was already absent (idempotency case), no rollback action is required — record and close.

### For a REVOKE operation

- **Prior state**: the `SHOW GRANTS ON <securable_type> <securable>` output captured during preflight (showing the grant that was present).
- **Inverse statement**: `GRANT <privilege> ON <securable_type> <database>.<schema>[.<object>] TO ROLE <role>;`
- **Owner**: Snowflake ACCOUNTADMIN or SECURITYADMIN (human operator).
- **Time-box**: rollback must be executable within 30 minutes of the original mutation.
- **Verification**: after rollback, run `SHOW GRANTS ON <securable_type> <securable>` and confirm the grant is present and matches the prior state.

## Role hierarchy note

Snowflake RBAC privilege changes take effect immediately. If the granted role was used to access data between the grant execution and rollback, that access cannot be recalled. Additionally:

- If the granted role is granted to other roles (role hierarchy), those downstream roles also received the effective privilege during the window. Document the full role hierarchy in the incident record.
- After rollback, verify downstream roles no longer have effective access via `SHOW GRANTS TO ROLE <role>` for each role in the hierarchy.

## Data access window

Document the access window (timestamp of grant to timestamp of rollback) in the incident record. Review Snowflake access history (`SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY`) for queries executed under the granted privilege during that window.

## Irreversibility warning

If rollback is impossible or materially limited (e.g., the prior state snapshot is missing, the securable was dropped, or the role was dropped), state that explicitly before the rollback owner is asked to proceed. Irreversible cases require additional sign-off beyond standard rollback.

## Standing rule

The rollback owner (Snowflake ACCOUNTADMIN or SECURITYADMIN) must be a named human operator — not automated. The rollback statement must be reviewed and approved by that owner before execution, following the same dry-run preflight process as the original mutation.
