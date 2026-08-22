# Rollback — Snowflake Live RBAC Grant Guard Agent

Rollback is a named, executable statement with a known window and known side effects. 'Undo the change' is not a rollback plan and is not accepted here.

## Rollback contract

| Property | Value |
|---|---|
| Trigger | The approver, the named security owner, or any workload owner reporting access loss attributable to the change |
| Owner | A named human administrator holding OWNERSHIP of the securable — never this agent and never an automation |
| Statement | The exact inverse: `REVOKE <privilege> ON <securable_type> <securable> FROM ROLE <role>` for a GRANT, or `GRANT <privilege> ON <securable_type> <securable> TO ROLE <role>` for a REVOKE |
| Required state snapshot | The verbatim `SHOW GRANTS ON <securable>` and `SHOW GRANTS TO ROLE <role>` output captured in preflight |
| Maximum rollback window | The inverse statement remains executable indefinitely while the securable and the role both exist; the practical window is bounded by the securable or role being dropped, and by how long the access consequence is tolerable |
| Reversibility | The privilege state is fully reversible. The data access that occurred during the window is not — a GRANT that was used cannot be un-read, and that asymmetry is stated in the proposal before approval, not after execution |

## Verification after rollback

- Re-run `SHOW GRANTS ON <securable>` and compare against the prior-state snapshot field by field.
- Re-run `SHOW GRANTS TO ROLE <role>` and confirm the privilege is absent (after rolling back a GRANT) or present (after rolling back a REVOKE).
- Re-run the effective-inheritance computation and confirm the inheriting principal set matches the pre-change set.
- Review `ACCESS_HISTORY` for the exposure window and record what was accessed under the privilege while it existed.

## Data-loss and side-effect implications

- Rolling back a GRANT does not recall data read during the window. Treat any sensitive-object exposure in that window as an incident to be assessed, not as a closed item.
- Rolling back a REVOKE restores access but does not restore the workloads that failed while it was absent; those need their own recovery.
- Every role inheriting the target role is affected by the rollback exactly as it was by the original change — the rollback has the same blast radius, not a smaller one.

## Where automatic rollback is unsafe

- The prior-state snapshot is missing or was not captured verbatim — there is nothing authoritative to restore to.
- The securable or the role has been dropped or recreated since the change; a recreated object is a different object and the inverse statement may bind to something unintended.
- Another privilege change has been made on the same securable since — resolve the sequence with the security owner before reverting, or the rollback overwrites someone else's approved change.
- The rollback would itself remove access that a running production workload currently depends on; that is a new change requiring its own approval.

## Standing rule

The rollback owner is a **named human operator**, never this agent and never an automation. The rollback statement goes through the same preflight and approval path as the original mutation. If the rollback itself would be materially destructive, it requires its own sign-off.
