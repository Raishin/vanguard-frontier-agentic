# Rollback — Snowflake Live Auth and Network Policy Guard Agent

Rollback is a named, executable statement with a known window and known side effects. 'Undo the change' is not a rollback plan and is not accepted here.

## Rollback contract

| Property | Value |
|---|---|
| Trigger | Any report of access loss by a human or a non-human workload, a failed post-change verification, or the surviving-path holder being unable to connect |
| Owner | A named human administrator holding OWNERSHIP of the policy object, connecting from a path proven in preflight to survive the change |
| Statement | The exact inverse of the executed modification — the prior `ALLOWED_IP_LIST` / `ALLOWED_NETWORK_RULE_LIST` restored verbatim, the prior policy assignment reinstated, or the policy unset from the affected scope |
| Required state snapshot | The verbatim `DESCRIBE` of the policy and the effective assignment at every affected scope, captured in preflight |
| Maximum rollback window | Immediate and indefinite while the policy object exists and the rollback owner retains a working path — the practical bound is not time but connectivity, which is why the surviving path is proven first |
| Reversibility | Fully reversible in configuration terms. Not reversible in consequence terms: sessions terminated, workloads failed, and scheduled jobs missed during the window do not resume retroactively and need their own recovery |

## Verification after rollback

- Re-run `DESCRIBE NETWORK POLICY` / the authentication-policy read and compare field by field against the prior-state snapshot.
- Re-check the effective assignment at account scope and at every affected user scope.
- Confirm from login history that the previously-connecting principals — including the non-human clients — have successfully reconnected, rather than assuming they will.
- Confirm the surviving-path principal can still connect, so the account is not one change away from being unrecoverable.

## Data-loss and side-effect implications

- Workloads that failed during the window do not re-run themselves. Enumerate the scheduled jobs, pipelines, and reports that missed a run and hand them to their owners.
- Rolling back a tightening restores the exposure the change was made to remove; if the change was made in response to an incident, the rollback re-opens it and that trade must be explicit.
- Sessions terminated by a network policy are not restored by the rollback; clients must reconnect, and some will need manual intervention.

## Where automatic rollback is unsafe

- The prior-state snapshot is missing or partial — there is nothing authoritative to restore, and the policy will be reconstructed from memory under pressure.
- The rollback owner's path was not proven in preflight and cannot be established now.
- The change was made in response to an active security incident: rolling back re-opens the path an attacker was using, and that decision belongs to the incident commander rather than to this guard.
- Another policy change has been made since — resolve the sequence with the policy owner before reverting.

## Standing rule

The rollback owner is a **named human operator**, never this agent and never an automation. The rollback statement goes through the same preflight and approval path as the original mutation. If the rollback itself would be materially destructive, it requires its own sign-off.
