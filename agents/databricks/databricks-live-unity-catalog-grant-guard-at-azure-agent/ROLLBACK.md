# Rollback — Databricks Live Unity Catalog Grant Guard at Azure

## Execution tier

`mutating-runtime` (Phase B). A mutation was performed. Rollback is required if the grant must be reversed.

## Rollback contract

This agent executes exactly one `GRANT` or `REVOKE` statement per invocation. The rollback contract applies to that single statement.

### For a GRANT operation

- **Prior state**: the `SHOW GRANTS ON <securable_type> <securable>` output captured during preflight.
- **Inverse statement**: `REVOKE <privilege> ON <securable_type> <catalog>.<schema>[.<object>] FROM `<principal>`;`
- **Owner**: Databricks workspace admin or Unity Catalog metastore admin (human operator — not automated).
- **Time-box**: rollback must be executable within 30 minutes of the original mutation.
- **Verification**: after rollback, run `SHOW GRANTS ON <securable_type> <securable>` and confirm the grant is absent and the prior state matches the captured snapshot.
- **Idempotency**: if the original GRANT was already absent (idempotency case), no rollback action is required — record and close.

### For a REVOKE operation

- **Prior state**: the `SHOW GRANTS ON <securable_type> <securable>` output captured during preflight (showing the grant that was present).
- **Inverse statement**: `GRANT <privilege> ON <securable_type> <catalog>.<schema>[.<object>] TO `<principal>`;`
- **Owner**: Databricks workspace admin or Unity Catalog metastore admin (human operator).
- **Time-box**: rollback must be executable within 30 minutes of the original mutation.
- **Verification**: after rollback, run `SHOW GRANTS ON <securable_type> <securable>` and confirm the grant is present and matches the prior state.

## Schema-level grant note

If the GRANT was on a schema securable, the privilege propagated to all tables and views within that schema at the time of the grant. Rolling back the schema-level grant removes the privilege from current schema members. Tables or views added to the schema after the grant was revoked are not affected by the rollback. Document this window.

## Data access window

If a principal accessed data under the granted privilege between the grant execution and the rollback execution, that data access cannot be recalled. Document the access window (timestamp of grant to timestamp of rollback) in the incident record. Review Unity Catalog audit logs for data access during that window.

## Irreversibility warning

If rollback is impossible or materially limited (e.g., the prior state snapshot is missing or the securable was dropped), state that explicitly before the rollback owner is asked to proceed. Irreversible cases require additional sign-off beyond standard rollback.

## Standing rule

The rollback owner (Databricks workspace admin or Unity Catalog metastore admin) must be a named human operator — not automated. The rollback statement must be reviewed and approved by that owner before execution, following the same dry-run preflight process as the original mutation.
