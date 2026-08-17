# Permissions — Snowflake Live Failover Promotion Guard Agent

## Execution tier

`mutating-runtime`. Exactly one mutation per invocation: One `ALTER FAILOVER GROUP <name> PRIMARY` — promotion of one failover group to primary in the target account. Gated by explicit written human approval. Never auto-dispatched.

## Run-as principal

| Component | Requirement |
|---|---|
| Identity type | Narrowly scoped **custom** Snowflake role. `ACCOUNTADMIN` is forbidden without exception. `SECURITYADMIN` and `SYSADMIN` are forbidden unless technically unavoidable, in which case the reason is written into this file before first run. |
| User type | `TYPE = SERVICE` (or `SERVICE_AGENT` where the executor is an automated agent identity). Never `TYPE = PERSON`, never `LEGACY_SERVICE`. |
| Authentication | Key-pair, workload identity federation, or OAuth. Password authentication for a non-human identity is forbidden — and is being removed by Snowflake's strong-authentication rollout regardless. |
| Scope | Bounded to the single target named in the approval token: ONE failover group · ONE target account · ONE promotion per invocation. |

## Required read privileges

Needed to establish prior state and blast radius. Read privileges are granted permanently; the write privilege is not.

- `SHOW FAILOVER GROUPS` and `SHOW REPLICATION GROUPS` in the target account — group type, membership, and state
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.REPLICATION_GROUP_REFRESH_HISTORY` — the last successful refresh, from which the data-loss window is computed rather than estimated
- `SHOW CONNECTIONS` — the Client Redirect configuration and which connection is currently primary
- `SELECT CURRENT_ACCOUNT(), CURRENT_REGION()` in both accounts where reachable — confirmation that the promotion targets the intended account
- `SHOW DATABASES` / group membership in the target — what will and will not become available on promotion

## Required write privilege

- The privilege to promote the single named failover group to primary in the target account — and nothing else. This guard has exactly one write capability.

Why each is needed:

- Promotion of a failover group is the entire mutation this guard performs; scoping the privilege to one named group means the guard cannot promote anything else, including during an incident when judgement is worst.
- No privilege over replication configuration is held, because changing group membership or schedule during an incident is a different decision with a different owner and a different blast radius.
- No standing privilege is held in the primary account: this guard operates in the target, which is the account that will still be reachable when the primary is not.
- Read access to refresh history is what converts an estimated data-loss window into a computed one, and the difference between those two is what a business acknowledgement is actually acknowledging.

## Explicitly forbidden privileges

- ACCOUNTADMIN
- SECURITYADMIN
- SYSADMIN
- PUBLIC
- Any privilege to promote a failover group other than the approved one
- Any privilege to alter replication or failover group membership or schedule
- Any privilege to create, drop, or modify accounts or connections
- Any standing privilege in the primary account — this guard operates in the target account only

## Privilege escalation paths to check before first run

- The ability to promote a failover group is the ability to change which account is authoritative for its data. Verify the guard's role can promote only the named group, and re-verify after any group membership change.
- If the guard's role is granted to another role, everything above inherits promotion authority. Check `SHOW GRANTS OF ROLE <guard_role>` before first run and at every DR review.
- The guard's service user must be usable when the primary region is unavailable. An identity federated through a provider reachable only via the primary is an escalation path in reverse — it removes the capability at the moment of need. Verify this in every drill.
- Credentials for the target account must be provisioned and tested in advance. Provisioning them during an incident is both a delay and an unreviewed privilege grant.

## Credential posture

- Credentials are referenced by environment variable **name** only: `SNOWFLAKE_TARGET_ACCOUNT`, `SNOWFLAKE_USER`, `SNOWFLAKE_AUTHENTICATOR`, `SNOWFLAKE_PRIVATE_KEY_PATH`. Values are never requested, echoed, logged, or stored.
- Private keys and tokens live in the organization's secrets manager, never in this repository, a chat transcript, an environment dump, or an attestation.
- Password authentication for the executing identity is a hard stop.

## Egress allow-list

- The target (secondary) account endpoint only — the private-connectivity hostname where that account uses private connectivity. Reachability of this endpoint independent of the primary region is verified during every drill, not assumed

No other egress destination is required or permitted. Where the account uses private connectivity, the private endpoint hostname is used and the public account URL must not be.

## Privilege removal after use

- The promotion privilege is granted by a named human administrator as a standing capability in the target account, because provisioning it during an incident is a delay and an unreviewed grant. It is reviewed at every DR review and after every group membership change.
- The service user's key is rotated on the organization's standard cadence, and the rotation is verified against the target account specifically — a key valid only in the primary is a capability that disappears when it is needed.
- After a promotion, the guard's role in the now-demoted account is reviewed: the direction of the topology has changed, and a standing promotion privilege pointing the wrong way is a hazard.
- Any expansion of the set of failover groups this guard may promote is a change requiring DR-owner review.

## Blast-radius boundary

- Promotion changes which account is authoritative for every object in the failover group. Every client, integration, pipeline, and consumer of those objects is affected simultaneously.
- Transactions committed to the primary after the last successful refresh are lost. That window is computed from refresh history and stated in minutes; it is the number the business acknowledgement is about.
- Objects, integrations, and features not in the group do not become available in the target. Enumerate them before promotion — they are the recovery gaps, and they are discovered at the worst possible moment otherwise.
- Clients that connect through the Client Redirect connection follow the promotion; every client with a hardcoded account URL does not. The second set is usually larger and is enumerated in preflight.
- Ingestion producers, orchestration, external stages, identity, and downstream consumers each need their own action. A promotion completed with these unready has relocated the outage rather than ended it — and the new location has less capacity, different connectivity, and no rehearsed runbook.
- Failback is a second promotion in the reverse direction with its own data-loss window. Promotion without a failback strategy is a permanent architecture change made under time pressure.
