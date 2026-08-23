# Surviving Path Proof

The evidence this guard requires before any tightening, and how to produce it. Load during preflight — this is the check that makes the guard refuse.

## What counts as a proof

- Four elements, all required: a named principal; a named location or path; the privilege that principal holds to execute the inverse statement; and login-history evidence that this principal has actually connected from that path inside the observed window.
- 'The admins can use the web interface' is not a proof — the web interface is subject to the same network policy, and the claim names neither a principal nor an evidenced path.
- 'We have a break-glass account' is not a proof unless that account's path survives the change, its privilege is confirmed, and its use is alerted on. An unconstrained break-glass account is itself a finding, not a mitigation.
- The guard's own service user must also be in the surviving set. A guard that locks itself out cannot execute its own rollback, and its rollback is the fastest one available.
- Where the proof cannot be produced, the guard refuses. This refusal is not overridable by approval, because the approver cannot approve away the fact that nobody can revert.

## Building the lockout analysis

- Use a window long enough to include periodic workloads — 30 days as a default, longer where monthly processes exist. State the window in the analysis; a short window produces a confident, incomplete answer.
- Group by principal and client location, and mark each row as removed or surviving under the proposed change.
- Name the non-human clients individually rather than as a category: the orchestrator, the replication path, each connector, each BI service account, each agent identity. The generic phrase 'service accounts' is how one gets missed.
- Include the client type. A driver or connector with a pinned hostname is affected by a connectivity change even when its address is allowed.
- Hand the analysis to the approver as part of the approval request, not as a post-execution note. The approver is approving the removals.

## Evidence queries

Establish the effective policy — the account value alone does not establish it.

```sql
SHOW NETWORK POLICIES;
DESCRIBE NETWORK POLICY <policy_name>;

SHOW PARAMETERS LIKE 'NETWORK_POLICY' IN ACCOUNT;
-- Repeat for every affected principal: a user-level assignment overrides the
-- account picture for that user.
SHOW PARAMETERS LIKE 'NETWORK_POLICY' FOR USER <user_name>;

SHOW AUTHENTICATION POLICIES IN ACCOUNT;
```

Produce the lockout analysis and the surviving-path evidence in one extract.

```sql
SELECT user_name,
       client_ip,
       reported_client_type,
       first_authentication_factor,
       COUNT(*)             AS logins,
       MIN(event_timestamp) AS first_seen,
       MAX(event_timestamp) AS last_seen
  FROM SNOWFLAKE.ACCOUNT_USAGE.LOGIN_HISTORY
 WHERE event_timestamp >= DATEADD(day, -30, CURRENT_TIMESTAMP())
   AND is_success = 'YES'
 GROUP BY 1, 2, 3, 4
 ORDER BY user_name, logins DESC;
-- Mark each row removed or surviving under the proposed policy.
-- The surviving-path proof is a specific row here, for a principal that also
-- holds OWNERSHIP on the policy object.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/network-policies — Account and user-level activation, the OWNERSHIP-on-user plus USAGE-on-policy requirement for user-level activation, and that a non-compliant location prevents further query execution
- https://docs.snowflake.com/en/sql-reference/sql/alter-network-policy — The modification grammar and that modifying a policy requires OWNERSHIP on the policy object
