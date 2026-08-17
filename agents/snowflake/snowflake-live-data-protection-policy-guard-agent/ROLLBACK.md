# Rollback — Snowflake Live Data Protection Policy Guard Agent

Rollback is a named, executable statement with a known window and known side effects. 'Undo the change' is not a rollback plan and is not accepted here.

## Rollback contract

| Property | Value |
|---|---|
| Trigger | A consumer reporting missing or masked data they are entitled to see, a downstream reconciliation break, a failed post-change verification, or — for a detachment — the re-attachment time being reached |
| Owner | A named human data owner or governance administrator holding OWNERSHIP of the object and APPLY on the policy |
| Statement | The exact inverse: `ALTER TABLE ... ALTER COLUMN ... UNSET MASKING POLICY` / `SET MASKING POLICY <prior>` for a masking change, or `ALTER TABLE ... DROP/ADD ROW ACCESS POLICY <prior>` for a row-access change — with the prior policy taken from the verbatim snapshot |
| Required state snapshot | The verbatim `POLICY_REFERENCES` rows for the target and the pre-change per-role-class visibility observation |
| Maximum rollback window | Immediate and indefinite while the object and the policy both exist. For a detachment the practical window is the agreed re-attachment time, which is a commitment rather than an option |
| Reversibility | The attachment state is fully reversible. The data exposure that occurred while a protection was absent, or the decisions made on data that was masked when it should not have been, are not — that asymmetry is stated in the proposal before approval |

## Verification after rollback

- Re-read `POLICY_REFERENCES` for the target and compare against the prior-state snapshot row by row.
- Re-run the per-role-class visibility check and confirm each class sees what it saw before the change — comparing shape and row counts, never sensitive values.
- Confirm downstream consumers that reported a break are working again, from their own signal rather than by assumption.
- For a detachment rollback, review `ACCESS_HISTORY` for the exposure window and record who read the object while it was unprotected.

## Data-loss and side-effect implications

- Rolling back a detachment does not recall the data read while the protection was absent. Treat the exposure window as an incident to assess, with the access-history record attached.
- Rolling back an attachment restores visibility, but any downstream aggregate computed while rows were excluded may be wrong and may already have been published — identify and correct those separately.
- A row-access policy change can alter result sets without error, so downstream systems may need reconciliation rather than merely reconnection.

## Where automatic rollback is unsafe

- The prior-state snapshot is missing — the previous attachment will be reconstructed from memory, and attaching the wrong policy is a new exposure rather than a restoration.
- The policy object has been altered since the change; re-attaching the same policy name now applies different logic.
- Rolling back an attachment would re-expose data that a named data owner has since determined must stay protected — that is a new decision requiring its own approval.
- A detachment rollback is being deferred past the agreed re-attachment time without a new written acceptance of the extended exposure.

## Standing rule

The rollback owner is a **named human operator**, never this agent and never an automation. The rollback statement goes through the same preflight and approval path as the original mutation. If the rollback itself would be materially destructive, it requires its own sign-off.
