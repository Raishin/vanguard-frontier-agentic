# Preflight — M365 Live Sensitivity Label Apply Guard

Before any live M365 Live Sensitivity Label Apply Guard write, confirm ALL of the following. The write MUST NOT proceed until every check passes.

## 1. Approval token verification

- Confirm a **written human approval token** has been provided.
- Confirm the token explicitly names:
  - The tenant (referenced by env-var name `GRAPH_TENANT_ID`, not by value)
  - The drive ID and driveItem ID (exact identifiers)
  - The proposed sensitivity label ID and display name
  - The assignment method (`standard` or `privileged`)
  - Justification text (required for downgrades or `privileged` assignments)
  - The blast-radius assessment
- If the token is absent or incomplete, **stop** and request a complete approval token. Do not proceed.
- If the proposed operation is a classification downgrade, confirm that additional sign-off (beyond the standard token) is documented. If not, **stop**.

## 2. Credential and permission confirmation

- Confirm `GRAPH_CLIENT_ID` and `GRAPH_TENANT_ID` environment variables are set. Do not print or echo their values.
- Confirm the app registration has been granted admin consent for `InformationProtectionPolicy.Read.All` and the narrowest available driveItem-labeling write scope (verify against the [Graph permissions reference](https://learn.microsoft.com/graph/permissions-reference)).
- Confirm the app does NOT hold `Directory.ReadWrite.All`, `Sites.FullControl.All`, `Files.ReadWrite.All` (broad), or `InformationProtectionPolicy.ReadWrite.All`.
- Confirm metered API setup is complete for the target tenant (required for `assignSensitivityLabel`).

## 3. Scope confirmation

- Confirm the drive ID and driveItem ID match exactly what is in the approval token. No fuzzy matching; no substitution.
- Confirm only ONE item is being targeted. Any multi-item or bulk operation is refused.
- Confirm no label policy write operation is implied.

## 4. Dry-run: GET current sensitivity label

- Issue a **GET** request to retrieve the current sensitivity label of the target driveItem:

  ```http
  GET https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}?$select=id,name,sensitivityLabel
  Authorization: Bearer <token>
  ```

- Confirm the item exists (200 OK). If 404, **stop** — the item does not exist.
- **Capture the current label**: label ID, display name, assignment method. Retain these values — they are required for ROLLBACK.
- **Emit the diff**: current label vs proposed label from the approval token.
- If the proposed label would lower the classification tier (downgrade), flag this prominently and confirm justification text and additional sign-off are present.
- Present the current vs proposed label to the approver for final confirmation before issuing the action.

## 5. Idempotency key generation

- Generate an idempotency key (UUID v4 or equivalent) before issuing the `assignSensitivityLabel` action.
- Record the idempotency key in the pre-write audit log entry.
- If the same idempotency key has already been used for a completed operation against this item, **stop** — do not replay.

## 6. Environment and egress check

- Confirm outbound egress to `graph.microsoft.com` and `login.microsoftonline.com` is permitted from the execution environment.
- Confirm metered API billing is configured for the tenant. If not confirmed, warn and await explicit confirmation before proceeding.

## 7. Concurrent operation check

- Confirm no other approval-pending or in-flight labeling operation targets the same driveItem ID.

## Block conditions

Stop and do not proceed if any of the following are true:

- No written approval token provided, or the token is missing the drive ID, driveItem ID, label ID, or blast-radius assessment.
- The approval token does not include justification text for a downgrade or `privileged` assignment.
- A classification downgrade is requested without additional sign-off documentation.
- The app holds `Directory.ReadWrite.All`, `Sites.FullControl.All`, `Files.ReadWrite.All` (broad), or `InformationProtectionPolicy.ReadWrite.All`.
- The credential value has been exposed in any log, chat, or environment dump.
- The target driveItem does not exist (GET returned 404).
- More than one item ID is specified.
- A wildcard, filter, or search query is used to target items instead of explicit drive ID + driveItem ID.
- A label policy write operation is requested.
- The label removal would leave the item with no sensitivity label and no additional approved label to apply.
- The idempotency key has already been used for a completed operation.
- Final confirmation from the approver (after reviewing the current vs proposed label diff) has not been received.
