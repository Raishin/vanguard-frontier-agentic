# Rollback — D365 Live Record Field Update Guard

## Rollback contract

This agent operates at `mutating-runtime` (Phase B). Every write is preceded by PREFLIGHT capture of prior field values. Rollback is always available and is a named inverse PATCH operation.

### Prior-state capture

Before any PATCH is issued, PREFLIGHT performs a GET on the target record and captures the CURRENT values of all fields named in the approval token. These values are retained for the duration of the run and must be accessible to the rollback operator.

### Rollback operation

Rollback = PATCH the same record with the captured prior values:

```http
PATCH [DATAVERSE_ENV_URL]/api/data/v9.2/<tableset>(<record-guid>) HTTP/1.1
Authorization: Bearer <token>
OData-MaxVersion: 4.0
OData-Version: 4.0
If-Match: *
Content-Type: application/json

{
  "<field1>": "<prior-value-1>",
  "<field2>": "<prior-value-2>"
}
```

`If-Match: *` ensures the rollback PATCH applies to the existing record only (no accidental create).

### Rollback owner

- **Named owner**: the Dataverse environment System Administrator or the designated data steward for the in-scope table, as identified in the approval token's blast-radius assessment.
- The rollback owner must be named before the write proceeds. If no rollback owner is named, the write is blocked.

### Rollback time-box

- Rollback must be executable within **30 minutes** of the original write.
- If the rollback window has expired, the rollback owner must escalate — a manual correction may be required via the Power Platform admin center.

### Rollback verification

After rollback PATCH completes (HTTP 204):

1. Issue a GET on the target record for the affected fields and confirm the values match the prior-state captured in PREFLIGHT.
2. Record the verification result in the audit log.
3. If the values do not match, escalate to the environment System Administrator immediately.

### Downstream impact on rollback

- Power Automate flows or plugins triggered by the original write may have already executed. Rolling back the field values does not undo those downstream actions.
- The rollback owner must assess whether any triggered automation must also be reversed, and document that assessment.
- If a triggered workflow is irreversible, state that explicitly in the rollback attestation before rollback is signed off.

## Write audit trail

Every completed write emits:

| Field | Value |
|---|---|
| Idempotency key | Generated before write; unique per operation |
| Environment | Env-var reference (not value) |
| Table | Logical name |
| Record GUID | Exact GUID from approval token |
| Fields updated | Names and new values |
| Prior field values | Captured in PREFLIGHT GET |
| Approval token ref | Reference to the written approval token |
| Write result | HTTP 204 (success) or error detail |
| Rollback owner | Named in approval token |
| Rollback ready | Yes / No (prior values retained) |

## Standing rule

If rollback is impossible or materially limited (e.g., triggered downstream workflow is irreversible), state that explicitly in the PREFLIGHT diff before the final approval is sought. Irreversible side-effects require additional sign-off beyond the standard approval token.

If the write fails (non-204 response), no rollback is needed — the record was not changed. Record the failure detail in the audit log.
