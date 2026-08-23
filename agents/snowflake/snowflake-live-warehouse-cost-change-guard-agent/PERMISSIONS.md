# Permissions — Snowflake Live Warehouse and Cost Change Guard Agent

## Execution tier

`mutating-runtime`. Exactly one mutation per invocation: One `ALTER WAREHOUSE` setting change (size, auto-suspend, auto-resume, min/max cluster count, scaling policy, or statement timeout), one resource-monitor assignment or threshold change, or one supported budget operation. Gated by explicit written human approval. Never auto-dispatched.

## Run-as principal

| Component | Requirement |
|---|---|
| Identity type | Narrowly scoped **custom** Snowflake role. `ACCOUNTADMIN`, `SECURITYADMIN`, and `SYSADMIN` are hard stops: forbidden without exception, and no written justification, approval token, or incident makes them permissible. If the mutation seems to need one, the target lacks a purpose-built owning role — create and grant that role instead of widening this one. |
| User type | `TYPE = SERVICE` (or `SERVICE_AGENT` where the executor is an automated agent identity). Never `TYPE = PERSON`, never `LEGACY_SERVICE`. |
| Authentication | Key-pair, workload identity federation, or OAuth. Password authentication for a non-human identity is forbidden — and is being removed by Snowflake's strong-authentication rollout regardless. |
| Scope | Bounded to the single target named in the approval token: ONE warehouse, monitor, or budget · ONE setting · ONE statement per invocation. |

## Required read privileges

Needed to establish prior state and blast radius. Read privileges are granted permanently; the write privilege is not.

- `SHOW WAREHOUSES`, `SHOW RESOURCE MONITORS`, `SHOW BUDGETS` — the prior state and the verification
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.WAREHOUSE_METERING_HISTORY` — the credit baseline the cost effect is measured against
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.WAREHOUSE_LOAD_HISTORY` — queueing and running load, which determines whether a scaling change is even addressing the right problem
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.QUERY_HISTORY` — the affected workload inventory and the performance baseline
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.QUERY_ATTRIBUTION_HISTORY` — the query-attributed share, which separates a query problem from an idle problem

## Required write privilege

- MODIFY on the single target warehouse (plus OPERATE where the approved change requires it), or the equivalent narrow privilege on the single target monitor or budget — and nothing else

Why each is needed:

- MODIFY on the target warehouse is the privilege that permits changing its settings; scoping it to one warehouse means this guard cannot affect any other compute in the account.
- OPERATE is requested only where the approved change requires suspending or resuming the target, and only for that warehouse.
- The metering, load, and query history reads are what make the quantified cost and performance effects possible; without them the guard would execute changes whose consequences nobody predicted.
- Query attribution is needed to distinguish an idle-cost problem from a query-cost problem, because the two have different correct changes and the wrong one costs credits without helping.

## Explicitly forbidden privileges

- ACCOUNTADMIN
- SECURITYADMIN
- SYSADMIN
- PUBLIC
- CREATE WAREHOUSE or any warehouse lifecycle privilege
- MODIFY or OPERATE on any warehouse other than the approved target
- Any privilege permitting account-level parameter changes
- Any privilege permitting grant management on the target

## Privilege escalation paths to check before first run

- MODIFY on a warehouse permits any setting change on it, including one that materially increases spend. Verify the guard's role holds it on exactly the warehouses it is intended to administer.
- A resource monitor with a suspend action is an availability control: whoever can set its threshold can stop production compute. Treat monitor-modification privilege as equivalent in blast radius to the ability to suspend the warehouses it covers.
- If the guard's role is granted to another role, everything above inherits its compute-modification authority. Check `SHOW GRANTS OF ROLE <guard_role>` before first run and at every review.
- The service user's `DEFAULT_ROLE` must be the guard role and nothing broader.

## Credential posture

- Credentials are referenced by environment variable **name** only: `SNOWFLAKE_ACCOUNT`, `SNOWFLAKE_USER`, `SNOWFLAKE_AUTHENTICATOR`, `SNOWFLAKE_PRIVATE_KEY_PATH`. Values are never requested, echoed, logged, or stored.
- Private keys and tokens live in the organization's secrets manager, never in this repository, a chat transcript, an environment dump, or an attestation.
- Password authentication for the executing identity is a hard stop.

## Egress allow-list

- The Snowflake account endpoint for the approved account only — the private-connectivity hostname where the account uses private connectivity, and never the public account URL in that case

No other egress destination is required or permitted. Where the account uses private connectivity, the private endpoint hostname is used and the public account URL must not be.

## Privilege removal after use

- MODIFY and OPERATE on the target warehouse are granted by a named human administrator before first run and reviewed on a stated cadence; the guard never grants itself anything.
- Where the guard administers a warehouse only for a bounded tuning exercise, the privilege is returned at its close and the return is recorded.
- The service user's key is rotated on the organization's standard cadence; workload identity federation, which stores no key, is preferred.
- Adding a warehouse to the guard's administered set is a change requiring the same review as a cost change itself — it widens what this guard can spend.

## Blast-radius boundary

- A warehouse setting change takes effect for subsequent activity immediately; running queries continue under the prior configuration, so the change has a mixed-state window that any measurement must exclude.
- A size increase raises the credit rate for every subsequent query on that warehouse, not only the query that motivated it. A size decrease can increase total credits by making queries spill and run longer — the cost effect of a reduction is not reliably negative.
- An auto-suspend reduction can increase resume frequency and cold-cache misses on a bursty workload; an increase leaves the warehouse running and billing on an idle one.
- A resource-monitor suspend action stops warehouses when the threshold is reached. Every workload on those warehouses fails from that moment, at whatever hour the threshold happens to be crossed.
- Every workload sharing the target warehouse is affected, not only the one that prompted the change. Enumerate them from query history before execution.
