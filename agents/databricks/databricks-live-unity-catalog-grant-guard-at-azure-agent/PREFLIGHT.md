# Preflight — Databricks Live Unity Catalog Grant Guard at Azure

Before any live Databricks Live Unity Catalog Grant Guard run, confirm ALL of the following:

## 1. Approval token validation

- Confirm an explicit written human approval token has been received naming:
  - Exact securable: `<catalog>.<schema>[.<object>]` and securable type (schema, table, or volume)
  - Exact privilege name (e.g., `SELECT`, `READ VOLUME`, `MODIFY`)
  - Exact principal name (account group name or service principal application ID)
  - Blast radius statement
- If any element is missing or vague, stop — do not proceed until the approval token is complete and unambiguous.

## 2. Credential and identity confirmation

- Confirm `DATABRICKS_HOST` and `DATABRICKS_CLIENT_ID` environment variables are set. Do not print or echo their values.
- Confirm the run-as service principal exists as an account-level identity in the Databricks account console.
- Confirm the run-as service principal holds MANAGE on the single target securable (or IS OWNER) — not metastore admin, not account admin.
- Confirm Entra-managed SP (backed by an Entra app registration) — not a workspace-local identity.

## 3. Securable scope assertion

- Confirm the target securable is a schema, table, or volume — NOT a catalog or metastore.
- Confirm exactly one securable is named. If more than one securable appears in the request, stop.
- Confirm the privilege is a named Unity Catalog privilege — not `ALL PRIVILEGES` and not `MANAGE` at catalog or metastore level.

## 4. Denied-operation check

Confirm NONE of the following are present in the requested operation:
- `ALL PRIVILEGES`
- `MANAGE` on metastore, catalog, or any securable other than the single target
- Ownership transfer (`ALTER ... OWNER TO`)
- Grant to `metastore-admin`, `account-admin`, or `workspace-admin` groups
- Catalog-wide grants

If any denied operation is detected, stop immediately and report the violation.

## 5. Dry-run preflight execution

Before executing the grant:
- Run `SHOW GRANTS ON <securable_type> <catalog>.<schema>[.<object>]` and display the current grant state.
- Display the exact SQL statement to be executed:
  `GRANT <privilege> ON <securable_type> <catalog>.<schema>[.<object>] TO `<principal>`;`
  or
  `REVOKE <privilege> ON <securable_type> <catalog>.<schema>[.<object>] FROM `<principal>`;`
- Await explicit human confirmation of the dry-run output before proceeding.

## 6. Prior state capture

- Record the full output of `SHOW GRANTS ON <securable_type> <securable>` as the prior state snapshot.
- This snapshot is required for rollback — do not proceed without it.

## 7. Idempotency key generation

- Generate an idempotency key (UUID v4 or equivalent) before issuing the `GRANT`/`REVOKE` statement.
- Record the idempotency key in the pre-write audit log entry and carry it into the signed attestation (`signed_with: idempotency-key`).
- If the same idempotency key has already been used for a completed write against this securable, **stop** — do not replay.

## 8. Environment check

- Confirm outbound egress to `DATABRICKS_HOST` and `login.microsoftonline.com` is permitted from the execution environment.
- Confirm Unity Catalog audit logs are enabled for the target metastore.
- Confirm no prior invocation of this agent is pending rollback for the same securable.

## Block conditions

Stop and do not proceed if any of the following are true:

- No explicit written human approval token has been received.
- The approval token does not name the exact securable, privilege, principal, and blast radius.
- The target securable is a catalog or metastore (not schema, table, or volume).
- More than one securable appears in the request.
- The requested privilege is `ALL PRIVILEGES` or `MANAGE` at catalog/metastore scope.
- The run-as service principal holds metastore admin or account admin.
- A credential value has been exposed in any log, chat, or environment dump.
- A prior run for the same securable is pending rollback.
