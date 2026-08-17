# Permissions — Snowflake Live Auth and Network Policy Guard Agent

## Execution tier

`mutating-runtime`. Exactly one mutation per invocation: One `ALTER NETWORK POLICY`, one network-policy activation or deactivation at account or user scope, or one `ALTER AUTHENTICATION POLICY` / policy assignment. Gated by explicit written human approval. Never auto-dispatched.

## Run-as principal

| Component | Requirement |
|---|---|
| Identity type | Narrowly scoped **custom** Snowflake role. `ACCOUNTADMIN` is forbidden without exception. `SECURITYADMIN` and `SYSADMIN` are forbidden unless technically unavoidable, in which case the reason is written into this file before first run. |
| User type | `TYPE = SERVICE` (or `SERVICE_AGENT` where the executor is an automated agent identity). Never `TYPE = PERSON`, never `LEGACY_SERVICE`. |
| Authentication | Key-pair, workload identity federation, or OAuth. Password authentication for a non-human identity is forbidden — and is being removed by Snowflake's strong-authentication rollout regardless. |
| Scope | Bounded to the single target named in the approval token: ONE policy object · ONE modification · ONE activation scope · ONE statement per invocation. |

## Required read privileges

Needed to establish prior state and blast radius. Read privileges are granted permanently; the write privilege is not.

- `SHOW NETWORK POLICIES`, `SHOW NETWORK RULES`, `DESCRIBE NETWORK POLICY` — the policy definitions as deployed
- `SHOW PARAMETERS LIKE 'NETWORK_POLICY'` at account scope and for each affected user — the effective assignment, which the account value alone does not establish
- `SHOW AUTHENTICATION POLICIES` and the policy assignment per user type — the authentication surface
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.LOGIN_HISTORY` — the evidence base for the lockout analysis and for the surviving-path proof
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.SESSIONS` and `QUERY_HISTORY` — which client types and applications are connecting and would break

## Required write privilege

- OWNERSHIP on the single target policy object named in the approval token
- For an approved user-level activation only: OWNERSHIP on that one user plus USAGE on that one policy

Why each is needed:

- Modifying a network policy is documented as requiring OWNERSHIP on the policy object; scoping the guard's role to exactly one policy object is therefore the narrowest configuration that can execute the approved change.
- User-level activation is documented as requiring OWNERSHIP on the user and USAGE on the network policy. Both are granted for the one named user only, never at a class or account level.
- Read access to login history is not optional for this guard: it is the evidence that produces the lockout analysis, and without it every tightening is a guess.
- Session and query history are needed to identify non-human clients — the ones that notice a lockout hours later rather than immediately.

## Explicitly forbidden privileges

- ACCOUNTADMIN
- SECURITYADMIN
- SYSADMIN
- PUBLIC
- OWNERSHIP on any policy object other than the approved target
- OWNERSHIP on any user other than the one named in an approved user-level activation
- CREATE INTEGRATION or any integration lifecycle privilege
- Any privilege permitting account-wide parameter changes beyond the approved policy assignment

## Privilege escalation paths to check before first run

- OWNERSHIP on a policy object permits any modification to it, including one that removes every restriction. Verify the guard's role owns only the policy objects it is intended to administer.
- OWNERSHIP on a user permits changing that user's policy assignment and other user properties. Grant it for the specific user in an approved activation and remove it afterwards.
- If the guard's role is granted to another role, everything above inherits its policy-modification authority. Check `SHOW GRANTS OF ROLE <guard_role>` before first run and at every review.
- The guard's own service user must be able to reach Snowflake after the change it is making. A guard that locks itself out cannot execute its own rollback — confirm its path is in the surviving set.

## Credential posture

- Credentials are referenced by environment variable **name** only: `SNOWFLAKE_ACCOUNT`, `SNOWFLAKE_USER`, `SNOWFLAKE_AUTHENTICATOR`, `SNOWFLAKE_PRIVATE_KEY_PATH`. Values are never requested, echoed, logged, or stored.
- Private keys and tokens live in the organization's secrets manager, never in this repository, a chat transcript, an environment dump, or an attestation.
- Password authentication for the executing identity is a hard stop.

## Egress allow-list

- The Snowflake account endpoint for the approved account only — the private-connectivity hostname where the account uses private connectivity, and never the public account URL in that case

No other egress destination is required or permitted. Where the account uses private connectivity, the private endpoint hostname is used and the public account URL must not be.

## Privilege removal after use

- OWNERSHIP on the policy object is granted by a named human administrator before first run and reviewed on a stated cadence; the guard never grants itself anything.
- OWNERSHIP on a user, granted for a specific user-level activation, is returned immediately after the change is verified — it is the widest privilege this guard ever holds and it is held for the shortest possible time.
- The service user's key is rotated on the organization's standard cadence; workload identity federation, which stores no key, is preferred.
- Any expansion of the set of policy objects the guard's role owns is a change requiring the same review as the policy change itself.

## Blast-radius boundary

- A network policy change takes effect immediately and applies to sessions in flight, not only to new connections: documented behaviour is that a non-compliant network location prevents further query execution for the affected user.
- The affected principal set is everyone the policy applies to at the activation scope — which at account scope is every principal without a user-level override, including every service identity.
- Non-human clients are the ones that surface a lockout late: nightly orchestration, replication, connectors, BI service accounts, and any agent identity. They are in the blast radius even when nobody is watching a dashboard.
- An authentication policy change can invalidate the authentication method a running integration depends on, which presents as an authentication failure rather than as a policy error.
- The rollback requires the ability to connect. If the change removes every path capable of executing the inverse, the account is unrecoverable without vendor support — which is why the surviving-path proof precedes everything else.
