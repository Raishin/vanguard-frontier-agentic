# Permissions — Snowflake Live Data Protection Policy Guard Agent

## Execution tier

`mutating-runtime`. Exactly one mutation per invocation: One policy attachment, detachment, or replacement on one column or one table — `ALTER TABLE ... ALTER COLUMN ... SET/UNSET MASKING POLICY`, or `ALTER TABLE ... ADD/DROP ROW ACCESS POLICY`. Gated by explicit written human approval. Never auto-dispatched.

## Run-as principal

| Component | Requirement |
|---|---|
| Identity type | Narrowly scoped **custom** Snowflake role. `ACCOUNTADMIN` is forbidden without exception. `SECURITYADMIN` and `SYSADMIN` are forbidden unless technically unavoidable, in which case the reason is written into this file before first run. |
| User type | `TYPE = SERVICE` (or `SERVICE_AGENT` where the executor is an automated agent identity). Never `TYPE = PERSON`, never `LEGACY_SERVICE`. |
| Authentication | Key-pair, workload identity federation, or OAuth. Password authentication for a non-human identity is forbidden — and is being removed by Snowflake's strong-authentication rollout regardless. |
| Scope | Bounded to the single target named in the approval token: ONE object · ONE column where applicable · ONE policy · ONE direction · ONE statement per invocation. |

## Required read privileges

Needed to establish prior state and blast radius. Read privileges are granted permanently; the write privilege is not.

- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.POLICY_REFERENCES` — the prior attachment state and the verification
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.TAG_REFERENCES` — whether the target is already reached by a tag-based attachment that this change would conflict with
- `DESCRIBE TABLE` and `SHOW MASKING POLICIES` / `SHOW ROW ACCESS POLICIES` — the target and policy definitions
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.OBJECT_DEPENDENCIES` — the consumption paths the protection may or may not follow
- `SELECT` on `SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY` — which role classes actually read the target, which is the affected-consumer list

## Required write privilege

- OWNERSHIP on the single target object, plus APPLY on the single named policy — and nothing else

Why each is needed:

- Attaching or detaching a policy on an object requires authority over that object and the right to apply that policy; scoping both to the single target and the single policy is the narrowest configuration that can execute the approved change.
- The account-level APPLY form would let this guard attach the policy anywhere, which is precisely the blast radius the single-target design exists to prevent. It is denied for that reason.
- Read access to policy and tag references is needed to detect a conflicting tag-based attachment before creating one that fights it.
- Read access to dependencies and access history is needed to enumerate the consumption paths and the affected role classes, which is what makes the visibility prediction possible.

## Explicitly forbidden privileges

- ACCOUNTADMIN
- SECURITYADMIN
- SYSADMIN
- PUBLIC
- APPLY MASKING POLICY or APPLY ROW ACCESS POLICY at account level — the account-wide form is never granted to this guard
- OWNERSHIP on any object other than the approved target
- CREATE / ALTER / DROP on any policy object
- Any privilege permitting tag assignment or tag-based policy attachment

## Privilege escalation paths to check before first run

- OWNERSHIP of the target object permits any change to it, including dropping it. Verify the guard's role owns only the objects it is intended to administer, and prefer returning ownership after a bounded exercise.
- APPLY on a policy permits attaching it anywhere the role has object authority. Combined with a growing ownership set, that becomes broad governance authority — review both together.
- Where segregation of duties requires the policy owner and the data owner to be different principals, confirm this guard's role does not collapse them.
- If the guard's role is granted to another role, everything above inherits its policy-application authority. Check `SHOW GRANTS OF ROLE <guard_role>` before first run and at every review.

## Credential posture

- Credentials are referenced by environment variable **name** only: `SNOWFLAKE_ACCOUNT`, `SNOWFLAKE_USER`, `SNOWFLAKE_AUTHENTICATOR`, `SNOWFLAKE_PRIVATE_KEY_PATH`. Values are never requested, echoed, logged, or stored.
- Private keys and tokens live in the organization's secrets manager, never in this repository, a chat transcript, an environment dump, or an attestation.
- Password authentication for the executing identity is a hard stop.

## Egress allow-list

- The Snowflake account endpoint for the approved account only — the private-connectivity hostname where the account uses private connectivity, and never the public account URL in that case

No other egress destination is required or permitted. Where the account uses private connectivity, the private endpoint hostname is used and the public account URL must not be.

## Privilege removal after use

- OWNERSHIP on the target and APPLY on the policy are granted by a named human administrator before first run, scoped to that target and that policy, and reviewed on a stated cadence.
- Where ownership was granted for a bounded rollout, it is returned to the data owner at its close and the return is recorded.
- The service user's key is rotated on the organization's standard cadence; workload identity federation, which stores no key, is preferred.
- Any expansion of the guard's ownership set or of the policies it may apply is a change requiring the same review as a policy change itself.

## Blast-radius boundary

- A policy attachment takes effect immediately for subsequent queries. Every role class querying the object sees the policy's result from that moment, including the ones nobody enumerated.
- Service identities, BI service accounts, replication paths, and agent identities are affected exactly as human roles are — and they are the classes reviews forget, because they do not complain in a chat channel.
- A row-access policy changes result sets, which can change downstream aggregates and reconciliation totals without any error being raised. Downstream consumers can be silently wrong rather than visibly broken.
- Protection may not follow every consumption path: a view, a clone, a share, a replica, or a materialized copy may present the data unprotected. Enumerating the paths is part of the blast radius, not a follow-up.
- For a detachment: data read while the protection is absent cannot be recalled, and the exposure window is the interval between detach and re-attach.
