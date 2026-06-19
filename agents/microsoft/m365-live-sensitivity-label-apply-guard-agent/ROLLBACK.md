# Rollback — M365 Live Sensitivity Label Apply Guard

## Rollback contract

This agent operates at `mutating-runtime` (Phase B). Every label write is preceded by PREFLIGHT capture of the prior sensitivity label. Rollback is always available and is a named inverse operation: re-apply the prior label via `assignSensitivityLabel`.

### Prior-state capture

Before any `assignSensitivityLabel` action is issued, PREFLIGHT performs a GET on the target driveItem and captures the CURRENT sensitivity label (ID, display name, assignment method). These values are retained for the duration of the run and must be accessible to the rollback operator.

If the item currently has no sensitivity label, that fact is recorded as the prior state, and rollback would consist of removing the newly applied label (or applying the organization's default baseline label, as directed by the rollback owner).

### Rollback operation

Rollback = re-apply the prior label via `assignSensitivityLabel`:

```http
POST https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}/assignSensitivityLabel
Authorization: Bearer <token>
Content-Type: application/json

{
  "sensitivityLabelId": "<prior-label-id>",
  "assignmentMethod": "standard",
  "justificationText": "Rollback: restoring prior label as part of approved rollback operation. Original operation idempotency key: <key>."
}
```

Note: `assignSensitivityLabel` is an async action. Poll the operation status URL until completion before recording the rollback attestation.

### Rollback owner

- **Named owner**: the Microsoft 365 compliance administrator or the designated data steward for the in-scope content, as identified in the approval token's blast-radius assessment.
- The rollback owner must be named before the write proceeds. If no rollback owner is named, the write is blocked.

### Rollback time-box

- Rollback must be executable within **30 minutes** of the original label apply operation.
- If the rollback window has expired, the rollback owner must escalate — a manual correction may be required via the Microsoft Purview compliance portal.
- Note: if the applied label added encryption, users who gained or lost access during the window may have already acted on the content. Document any access that occurred before rollback.

### Rollback verification

After rollback `assignSensitivityLabel` completes:

1. Issue a GET on the target driveItem for the `sensitivityLabel` property and confirm the label matches the prior state captured in PREFLIGHT.
2. Record the verification result in the audit log.
3. If the label does not match, escalate to the compliance administrator immediately.

### Downgrade rollback note

If the original operation was a classification downgrade and rollback re-applies a higher-classification label, the rollback itself may trigger additional compliance controls (encryption re-application, DLP policy re-evaluation). The rollback owner must confirm that re-applying the higher-classification label is appropriate for the item's current state.

### Downstream impact on rollback

- DLP policies enforced by the new label may have already triggered during the window between the original write and rollback. Rolling back the label does not undo those DLP policy evaluations.
- If the new label added encryption, any access to the decrypted content during the window must be documented.
- The rollback owner must assess whether any triggered downstream effects must also be reversed.

## Write audit trail

Every completed write emits:

| Field | Value |
|---|---|
| Idempotency key | Generated before write; unique per operation |
| Tenant | Env-var reference (not value) |
| Drive ID | Exact drive ID from approval token |
| DriveItem ID | Exact driveItem ID from approval token |
| Item name | Display name from PREFLIGHT GET |
| Prior label ID | Captured in PREFLIGHT GET |
| Prior label name | Captured in PREFLIGHT GET |
| New label ID | From approval token |
| New label name | From approval token |
| Assignment method | From approval token |
| Justification text | From approval token (required for downgrades) |
| Approval token ref | Reference to the written approval token |
| Operation result | Completed / failed / error detail |
| Rollback owner | Named in approval token |
| Rollback ready | Yes — prior label retained for re-apply |

## Standing rule

If rollback is impossible or materially limited (e.g., a downgrade opened access to content that was subsequently exported or shared), state that explicitly in the PREFLIGHT diff before the final approval is sought. Irreversible side-effects require additional sign-off beyond the standard approval token.

If the write fails (operation did not complete), no rollback is needed — the label was not changed. Record the failure detail in the audit log.
