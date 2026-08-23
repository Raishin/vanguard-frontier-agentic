# Permissions — Snowflake Live RBAC Grant Guard Agent

## Execution tier

`mutating-runtime`. Exactly one mutation per invocation: One `GRANT <privilege> ON <securable_type> <securable> TO ROLE <custom_role>` or its exact `REVOKE` inverse. Gated by explicit written human approval. Never auto-dispatched.

## Run-as principal

| Component | Requirement |
|---|---|
| Identity type | Narrowly scoped **custom** Snowflake role. `ACCOUNTADMIN`, `SECURITYADMIN`, and `SYSADMIN` are hard stops: forbidden without exception, and no written justification, approval token, or incident makes them permissible. If the mutation seems to need one, the target lacks a purpose-built owning role — create and grant that role instead of widening this one. |
| User type | `TYPE = SERVICE` (or `SERVICE_AGENT` where the executor is an automated agent identity). Never `TYPE = PERSON`, never `LEGACY_SERVICE`. |
| Authentication | Key-pair, workload identity federation, or OAuth. Password authentication for a non-human identity is forbidden — and is being removed by Snowflake's strong-authentication rollout regardless. |
| Scope | Bounded to the single target named in the approval token: ONE privilege · ONE securable · ONE custom role · ONE statement per invocation. |

## Required read privileges

Needed to establish prior state and blast radius. Read privileges are granted permanently; the write privilege is not.

- `SHOW GRANTS ON <securable>` and `SHOW GRANTS TO ROLE <role>` — the prior-state capture and the post-change verification
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.GRANTS_TO_ROLES` — required to compute the effective-inheritance impact before execution
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY` — required to state, for a REVOKE, what the privilege was actually being used for

## Required write privilege

- OWNERSHIP on the single target securable named in the approval token — and nothing else. This is the entire write surface.

Why each is needed:

- OWNERSHIP on the target securable is needed because Snowflake permits a role to grant or revoke privileges only on objects it owns; it is the narrowest path that can perform the approved statement.
- `MANAGE GRANTS` would also permit the statement, but it is account-level and global — it cannot be scoped to one object, so granting it would give this guard authority over every securable in the account. It is denied for that reason and no other.
- Read access to the grant graph is needed to show the approver the effective-inheritance impact; without it the guard can execute a change whose consequence nobody has seen.
- Read access to access history is needed so a REVOKE can state what breaks, rather than discovering it from an incident.

## Explicitly forbidden privileges

- ACCOUNTADMIN
- SECURITYADMIN
- SYSADMIN
- PUBLIC
- MANAGE GRANTS (account-level global privilege — never granted to this guard's role)
- OWNERSHIP transfer capability beyond the single target securable
- CREATE ROLE / DROP ROLE / ALTER ROLE
- Any role granting privileges on securables other than the approved target

## Privilege escalation paths to check before first run

- OWNERSHIP of the target implies the ability to grant every privilege on that target to anyone — verify that the guard's role owns only securables it is intended to administer, and that its ownership set has not grown.
- If the guard's role is itself granted to another role, everything above inherits its ownership. Check `SHOW GRANTS OF ROLE <guard_role>` before first run and on every review.
- If the target securable is a schema or database, ownership of the container can reach objects inside it. Confirm the approved scope matches the securable type the approver intended.
- The service user's `DEFAULT_ROLE` must be the guard role and nothing broader; a system default role would give every session maximum privilege before it asks for anything.

## Credential posture

- Credentials are referenced by environment variable **name** only: `SNOWFLAKE_ACCOUNT`, `SNOWFLAKE_USER`, `SNOWFLAKE_AUTHENTICATOR`, `SNOWFLAKE_PRIVATE_KEY_PATH`. Values are never requested, echoed, logged, or stored.
- Private keys and tokens live in the organization's secrets manager, never in this repository, a chat transcript, an environment dump, or an attestation.
- Password authentication for the executing identity is a hard stop.

## Egress allow-list

- The Snowflake account endpoint for the approved account only — the private-connectivity hostname where the account uses private connectivity, and never the public account URL in that case

No other egress destination is required or permitted. Where the account uses private connectivity, the private endpoint hostname is used and the public account URL must not be.

## Privilege removal after use

- OWNERSHIP of the target securable is granted to the guard role by a named human administrator before first run, and is reviewed on a stated cadence; this guard never grants itself anything.
- Where the guard administers a securable only for a bounded project, ownership is returned to the owning role at project close, and the return is recorded.
- The service user's key is rotated on the organization's standard cadence; where workload identity federation is used there is no key to rotate, which is the preferred state.
- Any expansion of the guard role's ownership set is a change requiring the same review as any other privilege grant — it widens this guard's entire authority.

## Blast-radius boundary

- The immediate change is one privilege on one securable for one role. The effective change is that privilege reaching every principal that inherits that role, transitively — which is why the inheritance analysis is mandatory rather than advisory.
- Snowflake privilege changes take effect immediately; there is no staging state in which the change can be observed before it applies.
- For a GRANT: any data read during the window between grant and rollback cannot be recalled. State the window in the approval, and review access history for that window afterwards.
- For a REVOKE: sessions and workloads relying on the privilege fail from that moment. Identify them from access history before execution, not after — the ETL job that runs nightly is the one that notices last and loudest.
- A privilege on a schema or database can be a prerequisite for object-level access elsewhere; removing it can break access that appears unrelated. State the dependency in the blast-radius section of the proposal.
