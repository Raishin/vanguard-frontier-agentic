# Permissions — Snowflake Live Pipeline and Streaming Change Guard Agent

## Execution tier

`mutating-runtime`. Exactly one mutation per invocation: One pipeline or ingestion object operation: `ALTER TASK ... SUSPEND|RESUME|SET`, `ALTER PIPE ... SET PIPE_EXECUTION_PAUSED|REFRESH`, `ALTER DYNAMIC TABLE ... SUSPEND|RESUME|REFRESH|SET TARGET_LAG`, a stream recreation, or one bounded backfill statement. Gated by explicit written human approval. Never auto-dispatched.

## Run-as principal

| Component | Requirement |
|---|---|
| Identity type | Narrowly scoped **custom** Snowflake role. `ACCOUNTADMIN` is forbidden without exception. `SECURITYADMIN` and `SYSADMIN` are forbidden unless technically unavoidable, in which case the reason is written into this file before first run. |
| User type | `TYPE = SERVICE` (or `SERVICE_AGENT` where the executor is an automated agent identity). Never `TYPE = PERSON`, never `LEGACY_SERVICE`. |
| Authentication | Key-pair, workload identity federation, or OAuth. Password authentication for a non-human identity is forbidden — and is being removed by Snowflake's strong-authentication rollout regardless. |
| Scope | Bounded to the single target named in the approval token: ONE pipeline object · ONE operation · ONE bounded data window where the operation moves data · ONE statement per invocation. |

## Required read privileges

Needed to establish prior state and blast radius. Read privileges are granted permanently; the write privilege is not.

- `SHOW TASKS`, `SHOW STREAMS`, `SHOW PIPES`, `SHOW DYNAMIC TABLES` and their `DESCRIBE` forms — the object definitions and prior state
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.TASK_HISTORY`, `COPY_HISTORY`, `PIPE_USAGE_HISTORY`, and `DYNAMIC_TABLE_REFRESH_HISTORY` — the last successful processing state and the achieved lag
- `SYSTEM$PIPE_STATUS` and `SYSTEM$STREAM_HAS_DATA` — live object state and offset position
- `SELECT` on the target table for count and boundary queries — never for row export
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.OBJECT_DEPENDENCIES` and `ACCESS_HISTORY` — the downstream consumers the change affects

## Required write privilege

- OWNERSHIP on the single target pipeline object, plus the minimum write privilege on the target table that the approved operation requires — and nothing else

Why each is needed:

- OWNERSHIP of the target pipeline object is the privilege that permits suspending, resuming, refreshing, or altering it; scoping it to one object means this guard cannot affect any other pipeline in the account.
- Write privilege on the target table is requested only where the approved operation moves data — a backfill or a replay — and is bounded to that table.
- The account-level task-execution privilege is account-scoped by design in Snowflake and is therefore held only for the period of an approved change and returned immediately, rather than standing.
- Read access to processing history and to the target's counts is what makes the freshness baseline and the post-change reconciliation possible; without them the guard can only report that a statement ran.

## Explicitly forbidden privileges

- ACCOUNTADMIN
- SECURITYADMIN
- SYSADMIN
- PUBLIC
- OWNERSHIP on any pipeline object other than the approved target
- DROP or CREATE on the target table
- Any privilege permitting changes to other pipelines, tasks, streams, pipes, or dynamic tables in the account
- Any standing account-level task-execution privilege held outside the period of an approved change

## Privilege escalation paths to check before first run

- OWNERSHIP of a task or dynamic table permits changing what it executes, which is equivalent to changing what enters the target table. Review it as a data-integrity privilege, not an operational one.
- A standing account-level task-execution privilege would let this guard run any task in the account; it is held only for an approved change window and its return is verified.
- Write privilege on a target table is a data-modification capability. Confirm it is scoped to the single target and that it is not retained after a backfill completes.
- If the guard's role is granted to another role, everything above inherits its pipeline-modification authority. Check `SHOW GRANTS OF ROLE <guard_role>` before first run and at every review.

## Credential posture

- Credentials are referenced by environment variable **name** only: `SNOWFLAKE_ACCOUNT`, `SNOWFLAKE_USER`, `SNOWFLAKE_AUTHENTICATOR`, `SNOWFLAKE_PRIVATE_KEY_PATH`. Values are never requested, echoed, logged, or stored.
- Private keys and tokens live in the organization's secrets manager, never in this repository, a chat transcript, an environment dump, or an attestation.
- Password authentication for the executing identity is a hard stop.

## Egress allow-list

- The Snowflake account endpoint for the approved account only — the private-connectivity hostname where the account uses private connectivity, and never the public account URL in that case

No other egress destination is required or permitted. Where the account uses private connectivity, the private endpoint hostname is used and the public account URL must not be.

## Privilege removal after use

- OWNERSHIP on the target pipeline object is granted by a named human administrator before first run, scoped to that object, and reviewed on a stated cadence.
- Write privilege on the target table is granted for the approved backfill or replay only and revoked immediately after the reconciliation passes; the revocation is recorded in the attestation.
- Any account-level task-execution privilege is granted for the change window and returned immediately afterwards, with the return verified rather than assumed.
- The service user's key is rotated on the organization's standard cadence; workload identity federation, which stores no key, is preferred.

## Blast-radius boundary

- Suspending a task or a dynamic table stops data arriving downstream from that moment. Every consumer of the target — including the ones nobody enumerated — sees stale data with no error raised.
- Recreating a stream resets its offset. Changes that occurred before the recreation are not re-delivered, which is silent data loss presented as a routine operation.
- Refreshing a pipe or running a backfill can re-deliver data. Without an idempotent key and a deduplication mechanism in the target, that produces duplicates that survive indefinitely and corrupt every aggregate computed from the table.
- Changing a dynamic table's target lag changes its refresh frequency and therefore its cost, and can change whether the refresh remains incremental.
- A change to one object propagates through the dependency graph: downstream tasks and dynamic tables inherit both the staleness and any duplication. Enumerate the graph, not just the object.
