# Network Policies and Effective Scope

How to establish which policy actually applies to a principal, and the privileges that gate changing one. Load before any policy analysis.

## Scope is the first question

- Network policies activate at account level and at individual user level. A user-level policy provides granular control for specific users and is what applies to that user — so the account-level policy is not automatically the effective one.
- Report the effective policy per principal class: human operators, the ETL or orchestration service, BI service accounts, replication, connectors, and any agent identity. These classes connect from different places and are removed by different rules.
- Activating a user-level policy is documented as requiring OWNERSHIP on the user and USAGE on the network policy; modifying a network policy requires OWNERSHIP on the policy object. Those privileges decide who can both cause and undo a lockout — establish who holds them before the change, not during the incident.
- Policies are built from allowed and blocked lists, either as IP lists or as references to network rules. Blocked entries are the ones that surprise people; read the whole definition rather than the name.
- A policy that restricts a user does not only block login: documented behaviour is that a non-compliant network location prevents further query execution for that user. A change therefore affects sessions in flight, not just new connections.

## Order of operations that stays reversible

- Step 1 — add the new allowed path and activate nothing. Step 2 — verify from login history that real traffic arrives on it. Step 3 — only then remove the old path, as a separate approved change.
- Never combine addition and removal. A combined change has no intermediate state to observe and no partial rollback.
- Before step 3, name the break-glass principal, the path it uses, the privilege it holds to revert, and the human who is awake and holds it. Write those four facts into the approval request.
- Where a policy advisor or equivalent tooling is available, use it to preview what a policy would block, and treat its output as supporting evidence alongside the login-history analysis rather than as a replacement for it.

## Evidence queries

Establish what exists and what is actually in force.

```sql
SHOW NETWORK POLICIES;
SHOW NETWORK RULES;
DESCRIBE NETWORK POLICY my_policy;

-- Account-level activation.
SHOW PARAMETERS LIKE 'NETWORK_POLICY' IN ACCOUNT;

-- User-level activation overrides the account picture for that user.
SHOW PARAMETERS LIKE 'NETWORK_POLICY' FOR USER my_service_user;
```

Build the lockout analysis input — who connects from where, including the non-human clients that notice at 3am.

```sql
SELECT user_name,
       client_ip,
       reported_client_type,
       COUNT(*)             AS logins,
       MIN(event_timestamp) AS first_seen,
       MAX(event_timestamp) AS last_seen
  FROM SNOWFLAKE.ACCOUNT_USAGE.LOGIN_HISTORY
 WHERE event_timestamp >= DATEADD(day, -30, CURRENT_TIMESTAMP())
   AND is_success = 'YES'
 GROUP BY 1, 2, 3
 ORDER BY user_name, logins DESC;
-- Every distinct (user_name, client_ip) pair here is a principal the change
-- may remove. State the window: a 7-day window misses monthly jobs.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/network-policies — Account-level and user-level activation, the privileges required for user-level activation, and that a non-compliant location prevents further query execution
- https://docs.snowflake.com/en/sql-reference/sql/alter-network-policy — The modification grammar and that modifying a policy requires OWNERSHIP on the policy object
- https://docs.snowflake.com/en/user-guide/network-policy-advisor — That Snowflake provides tooling to reason about network rules and policies before activation
