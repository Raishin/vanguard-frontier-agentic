# Preflight — D365 Live Record Field Update Guard

Before any live D365 Live Record Field Update Guard write, confirm ALL of the following. The write MUST NOT proceed until every check passes.

## 1. Approval token verification

- Confirm a **written human approval token** has been provided.
- Confirm the token explicitly names:
  - The Dataverse environment (by env-var reference `DATAVERSE_ENV_URL`, not by value)
  - The target table logical name
  - The target record GUID (exact, not a filter or query)
  - The exact fields to update and their proposed new values
  - The blast-radius assessment
- If the token is absent or incomplete, **stop** and request a complete approval token. Do not proceed.

## 2. Credential and application user confirmation

- Confirm `DATAVERSE_CLIENT_ID` and `DATAVERSE_ENV_URL` environment variables are set. Do not print or echo their values.
- Confirm the application user exists in the target Dataverse environment (SystemUser row with the correct `ApplicationId`).
- Confirm the application user is bound to the custom least-privilege write role, NOT System Administrator or System Customizer.
- Confirm the custom write role grants Write (prvWrite) on ONLY the one in-scope table and no Delete, Create, or Append on any table.

## 3. SPN path assertion

- Confirm the application user was NOT registered via `pac admin create-service-principal`. If it was, **stop** — that path grants Power Platform Administrator-level access and is forbidden for this agent.

## 4. Scope confirmation

- Confirm the target table and record GUID match exactly what is in the approval token. No fuzzy matching; no substitution.
- Confirm the fields listed for update are in the approval token. Any field not named in the approval token is refused.
- Confirm no bulk, wildcard, or multi-record operation is implied.
- Confirm `ownerid` and all ownership-related fields are NOT in the update payload.
- Confirm no security role, privilege, or user assignment columns appear in the update payload.

## 5. Dry-run diff (GET current field values)

- Issue a **GET** request to retrieve the current values of all fields named in the approval token:

  ```http
  GET [DATAVERSE_ENV_URL]/api/data/v9.2/<tableset>(<record-guid>)?$select=<field1>,<field2>,...
  Authorization: Bearer <token>
  OData-MaxVersion: 4.0
  OData-Version: 4.0
  ```

- Confirm the record exists (200 OK). If 404, **stop** — the record does not exist.
- **Emit the diff**: display CURRENT value vs PROPOSED value for each named field.
- Retain the current field values — these are required for ROLLBACK.
- Present the diff to the approver and wait for explicit final confirmation before issuing the PATCH.

## 6. Idempotency key generation

- Generate an idempotency key (UUID v4 or equivalent) before issuing the PATCH.
- Record the idempotency key in the pre-write audit log entry.
- If the same idempotency key has already been used for a completed write against this record, **stop** — do not replay.

## 7. Environment and egress check

- Confirm outbound egress to `*.dynamics.com` and `login.microsoftonline.com` is permitted from the execution environment.
- Confirm no proxy or firewall will suppress or silently swallow a 204 response that would prevent detection of write completion.

## 8. Concurrent operation check

- Confirm no other approval-pending or in-flight operation targets the same table + record GUID.

## Block conditions

Stop and do not proceed if any of the following are true:

- No written approval token provided, or the token is missing the table, GUID, field names, or blast-radius assessment.
- The application user holds System Administrator or System Customizer role.
- The SPN was registered via the Power Platform management path.
- The credential value has been exposed in any log, chat, or environment dump.
- The target record does not exist (GET returned 404).
- The update payload contains `ownerid` or any ownership-related field.
- The update payload contains any security role, privilege, or user assignment column.
- More than one record GUID is specified.
- A wildcard, FetchXML filter, or OData filter is used to target records instead of an explicit GUID.
- A DELETE operation is requested.
- The idempotency key has already been used for a completed write.
- Final confirmation from the approver (after reviewing the diff) has not been received.
