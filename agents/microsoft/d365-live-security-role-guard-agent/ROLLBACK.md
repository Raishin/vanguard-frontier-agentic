# Rollback — D365 Live Security Role Guard

## Phase A (current): read-only-runtime

**No mutation was performed. There is nothing to roll back.**

This agent performed read-only Dataverse Web API GET/query calls only. No security roles, user assignments, team memberships, or environment configuration were modified. No rollback action is required or possible for Phase A runs.

## Phase B rollback contract (future — mutating-runtime, not yet implemented)

When Phase B (mutating-runtime) is implemented, the following rollback contract applies to each proposed change category. Phase B work is gated and requires explicit human approval, blast-radius review, and signed attestation before any mutation proceeds.

### Security role privilege changes

- **Before action**: export the full privilege matrix for the affected role via Dataverse Web API (`roleprivileges` collection).
- **Rollback**: restore the prior privilege set by re-adding removed privileges or removing added privileges. Use a solution import if the role was packaged in a managed solution.
- **Owner**: environment System Administrator.
- **Time-box**: rollback must be executable within 30 minutes of mutation.
- **Verification**: re-read the role's privilege matrix after rollback and confirm it matches the pre-change export.

### User or team role assignment changes

- **Before action**: record all current role assignments for the affected user or team (`systemuserroles` or `teamroles`).
- **Rollback**: re-add removed role assignments or remove added role assignments.
- **Owner**: environment System Administrator.
- **Time-box**: rollback must be executable within 15 minutes of mutation.
- **Irreversibility warning**: data accessed or exported by a user during an elevated-privilege window cannot be recalled. Document any data access that occurred before rollback.

### Application user role reassignment

- **Before action**: record the current security role bound to the application user.
- **Rollback**: reassign the application user to the prior security role.
- **Owner**: environment System Administrator.
- **Time-box**: rollback must be executable within 15 minutes of mutation.
- **Integration impact**: changing the role bound to an application user may break existing integrations. Verify all integration flows are healthy after rollback.

## Standing rule

If rollback is impossible or materially limited for a proposed Phase-B action, state that explicitly before approval is sought. Irreversible actions require additional sign-off beyond standard approval.
