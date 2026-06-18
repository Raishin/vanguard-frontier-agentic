# Rollback — M365 Live Identity Posture Guard

## Phase A (current): read-only-runtime

**No mutation was performed. There is nothing to roll back.**

This agent performed read-only Microsoft Graph API calls only. No Conditional Access policies, role assignments, user states, or tenant configuration were modified. No rollback action is required or possible for Phase A runs.

## Phase B rollback contract (future — mutating-runtime, not yet implemented)

When Phase B (mutating-runtime) is implemented, the following rollback contract applies to each proposed change category. Phase B work is gated and requires explicit human approval, blast-radius review, and signed attestation before any mutation proceeds.

### Conditional Access policy changes

- **Before action**: export the full CA policy JSON via `GET /policies/conditionalAccessPolicies/{id}`.
- **Rollback**: restore the prior policy state via PATCH with the exported JSON, or disable the new policy if a net-new creation.
- **Owner**: Identity Administrator or Conditional Access Administrator.
- **Time-box**: rollback must be executable within 15 minutes of mutation.
- **Verification**: re-read the policy state after rollback and confirm it matches the pre-change export.

### Privileged role assignment changes

- **Before action**: record all current permanent and eligible role assignments for the affected principal.
- **Rollback**: re-add removed assignments or remove added assignments.
- **Owner**: Privileged Role Administrator.
- **Time-box**: rollback must be executable within 15 minutes of mutation.
- **Irreversibility warning**: PIM-eligible assignments that are activated during the change window may have produced audit trail entries that cannot be removed.

### Guest account lifecycle actions

- **Before action**: export the guest user object and all group memberships.
- **Rollback**: restore deleted or disabled guest accounts from Entra soft-delete (30-day recovery window). Re-add group memberships.
- **Owner**: User Administrator.
- **Time-box**: guest account soft-delete is recoverable within 30 days.

## Standing rule

If rollback is impossible or materially limited for a proposed Phase-B action, state that explicitly before approval is sought. Irreversible actions require additional sign-off beyond standard approval.
