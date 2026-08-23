# Rollback — Snowflake Live Failover Promotion Guard Agent

Rollback is a named, executable statement with a known window and known side effects. 'Undo the change' is not a rollback plan and is not accepted here.

## Rollback contract

| Property | Value |
|---|---|
| Trigger | The incident owner determining that the promotion did not restore service, that the primary has recovered and the estate should return, or that the data-loss consequence is greater than assessed |
| Owner | The named incident or DR owner — never this agent, never an automation, and never the engineer who executed the promotion acting alone |
| Statement | There is no inverse statement. The corrective operation is **failback**: a second promotion in the reverse direction, once the original primary is healthy and has been re-synchronized, executed through this same guard with its own declaration, approval, data-loss assessment, and dependency readiness |
| Required state snapshot | The group definition and membership, the last successful refresh time and computed data-loss window, the dependency-readiness matrix, and the client inventory captured in preflight |
| Maximum rollback window | Not time-bounded and not automatic. Failback becomes possible only when the original primary is healthy and re-synchronized, and it carries its own data-loss window for everything written to the promoted account in the interim |
| Reversibility | Promotion is NOT reversible in the ordinary sense. The transactions lost in the original data-loss window are gone permanently. Returning to the original region is a forward operation with its own data loss, its own dependency readiness, and its own approval — which is why the failback strategy is required before the first promotion, not after it |

## Verification after rollback

- Confirm the promoted group reports as primary in the target account and that its objects are writable.
- Confirm each dependency owner has completed their action and reports their system operating against the target — from their signal, not from an assumption of readiness.
- Confirm clients have reconnected: those using Client Redirect automatically, and each hardcoded client by its named owner.
- Confirm ingestion has resumed and reconcile the landing tables for the promotion window — a promoted account with a stalled ingest path is a partial recovery presented as a complete one.
- Record what was actually lost in the data-loss window, from the refresh history and the ingestion reconciliation, so the incident record states a measured figure rather than the pre-promotion estimate.

## Data-loss and side-effect implications

- Transactions in the data-loss window are permanently lost. Identify the affected business processes and hand each to its owner for assessment; some will require manual re-entry or a restatement decision.
- The promoted account is now production in a region that was sized, priced, and connected as a secondary. Capacity, cost, and connectivity assumptions all need revisiting immediately, not at the next planning cycle.
- Objects outside the failover group are not available and their absence may not be obvious. Work through the pre-computed exclusion list rather than waiting for consumers to report gaps.
- Any write to the promoted account becomes data that a future failback must carry back — the longer the estate runs promoted, the larger the failback's own data-loss and reconciliation problem becomes.

## Where automatic rollback is unsafe

- The original primary has not been confirmed healthy and re-synchronized — failing back to an unsynchronized account loses everything written since the promotion.
- Dependency readiness for the return direction has not been re-confirmed; the same matrix applies in reverse and its state has changed since the promotion.
- The incident is still active and the cause is unresolved — returning to a region that is still failing is a second outage rather than a recovery.
- No business acknowledgement has been obtained for the failback's own data-loss window, which covers everything written while promoted.

## Standing rule

The rollback owner is a **named human operator**, never this agent and never an automation. The rollback statement goes through the same preflight and approval path as the original mutation. If the rollback itself would be materially destructive, it requires its own sign-off.
