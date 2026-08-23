# Rollback — Snowflake Live Pipeline and Streaming Change Guard Agent

Rollback is a named, executable statement with a known window and known side effects. 'Undo the change' is not a rollback plan and is not accepted here.

## Rollback contract

| Property | Value |
|---|---|
| Trigger | A failed post-change reconciliation, a downstream consumer reporting wrong or missing data, or the object re-entering the failure state that caused the change |
| Owner | A named human data engineer or platform owner holding OWNERSHIP of the target object |
| Statement | For a suspend or resume: the inverse `ALTER ... RESUME|SUSPEND`. For a setting change: `ALTER ... SET <property> = <prior value>` from the snapshot. For a backfill or replay: there is no statement-level inverse — the compensating action is a scoped delete or merge over the affected window, written and approved as its own change |
| Required state snapshot | The verbatim object definition, the last-successful-state record, the offset or checkpoint position, and the target row counts by window |
| Maximum rollback window | Immediate for a state or setting change. For any operation that moved data, there is no time-bounded rollback — the compensating action must be authored, approved, and reconciled like any other data change |
| Reversibility | State and setting changes are fully reversible. Data movement is NOT: re-delivered rows do not un-arrive, and a reset offset does not restore the changes it skipped. This asymmetry is stated in the proposal before approval, and it is the reason the duplication-or-loss analysis is mandatory |

## Verification after rollback

- Re-read the object definition and state and compare field by field against the snapshot.
- Confirm the offset or checkpoint position is where the rollback intended, not merely that the statement succeeded.
- Re-run the agreed reconciliation: target counts and control totals by window against the pre-change baseline and against the source.
- Confirm downstream consumers are receiving data again, from their own signal rather than by assumption — and confirm their totals reconcile, since a duplication is invisible to a liveness check.

## Data-loss and side-effect implications

- Rolling back a suspend does not backfill the window that was missed while the object was stopped; that gap needs its own bounded, approved backfill.
- Rolling back a resume leaves any partially processed data in place. Establish what was written before suspending again.
- A duplication introduced by a replay is not removed by suspending the pipeline. It requires a compensating delete or merge, authored and approved separately.
- Downstream dynamic tables and tasks that already consumed wrong data have propagated it; their own reconciliation and correction is a separate piece of work with its own owners.

## Where automatic rollback is unsafe

- The prior-state snapshot, offset position, or count baseline is missing — there is nothing to reconcile against and the correction will be guesswork on production data.
- The operation moved data and no compensating action has been authored and approved — suspending the pipeline does not undo the rows.
- Downstream consumers have already published figures derived from the affected window; correcting the table is then only part of the remediation and the published figures need their own restatement decision.
- The object is mid-run: wait for the run to complete or fail cleanly rather than interrupting it, which can leave partial state that neither the snapshot nor the reconciliation anticipates.

## Standing rule

The rollback owner is a **named human operator**, never this agent and never an automation. The rollback statement goes through the same preflight and approval path as the original mutation. If the rollback itself would be materially destructive, it requires its own sign-off.
