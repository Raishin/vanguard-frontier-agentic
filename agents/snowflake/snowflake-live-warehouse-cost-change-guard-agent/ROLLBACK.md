# Rollback — Snowflake Live Warehouse and Cost Change Guard Agent

Rollback is a named, executable statement with a known window and known side effects. 'Undo the change' is not a rollback plan and is not accepted here.

## Rollback contract

| Property | Value |
|---|---|
| Trigger | The agreed observation, defined in the approval before execution — a p95 latency threshold, a queue-time threshold, a spill volume, a credit rate, or a failed workload |
| Owner | A named human administrator holding MODIFY on the target object |
| Statement | The exact inverse `ALTER WAREHOUSE ... SET <setting> = <prior value>`, the prior monitor assignment or threshold restored, or the prior budget configuration restored — with the prior value taken from the verbatim snapshot |
| Required state snapshot | The verbatim `SHOW WAREHOUSES` / `SHOW RESOURCE MONITORS` / `SHOW BUDGETS` output for the target, plus the 30-day baseline |
| Maximum rollback window | Immediate and indefinite while the object exists — the setting itself has no expiry. The practical urgency is set by the agreed rollback trigger, not by a clock |
| Reversibility | The setting is fully reversible. The consumption already incurred, the queries that already ran slowly or spilled, and any workload suspended by a monitor action are not — the credits are spent and the failed runs need their own recovery |

## Verification after rollback

- Re-run `SHOW WAREHOUSES LIKE '<name>'` (or the monitor/budget read) and compare field by field against the prior-state snapshot.
- Confirm from metering history that the credit rate has returned to the baseline range, allowing for the mixed-state window.
- Confirm from query history that the performance metric that triggered the rollback has returned to its baseline.
- Where a monitor action suspended warehouses, confirm each one has resumed and that the workloads on it have restarted.

## Data-loss and side-effect implications

- Credits consumed under the changed setting are spent and are not recovered by the rollback.
- Queries that ran slowly, queued, or spilled during the window do not re-run themselves; scheduled jobs that missed a window need their owners to re-run them.
- A warehouse suspended by a monitor action does not resume its failed workloads. Enumerate them and hand them to their owners.
- Reverting an auto-suspend change resets the caching behaviour again, so the first measurements after a rollback are also from a cold state.

## Where automatic rollback is unsafe

- The prior-state snapshot is missing — the prior value will be reconstructed from memory, and warehouse settings are exactly the kind of value people misremember.
- Another change has been made to the same object since; resolve the sequence with the object owner before reverting.
- Reverting a size increase during a live incident would restore the slower configuration mid-recovery — that is a decision for the incident owner, not an automatic rollback.
- Reverting a monitor threshold upward while spend is actively running away removes the only ceiling in place; pair it with a named owner watching the consumption.

## Standing rule

The rollback owner is a **named human operator**, never this agent and never an automation. The rollback statement goes through the same preflight and approval path as the original mutation. If the rollback itself would be materially destructive, it requires its own sign-off.
