# Inheritance Impact and Usage Evidence

The two analyses that must precede any privilege change, and the queries that produce them. Load during preflight, every time.

## Why the inheritance impact is mandatory

- A grant to a role is a grant to every principal that inherits that role, transitively. The approver approves the statement; the account experiences the closure.
- Present it as a list of paths — role, then each inheriting role, then the users — not as a count. A count cannot be reviewed; a path can be recognized as wrong.
- State the transitive depth analysed. A closure truncated at ten hops and reported as complete is the same class of error as not computing it at all.
- Include ownership edges. A role that owns an object can grant on it, so an ownership edge in the closure means the privilege can be redistributed by anyone holding that role.
- If the closure reaches a role the approver did not expect, that is the finding, and the change goes back to the approver rather than proceeding with a note.

## Why a revoke needs usage evidence

- A revoke takes effect immediately and breaks whatever was using the privilege. The workloads that notice last are the unattended ones — the nightly job, the weekly report, the monthly close.
- Query access history for the securable over a window long enough to include periodic workloads. Ninety days is a reasonable default; state whatever window was used.
- An absence of usage inside the view's latency window is `UNKNOWN`, not proof of disuse. Say which it is.
- Where usage is found, name the principals and hand the list to the approver as part of the blast radius. A revoke with known consequences is a decision; a revoke with unknown consequences is an incident with a change ticket.

## Evidence queries

Capture prior state verbatim — the rollback basis.

```sql
SHOW GRANTS ON TABLE <db>.<schema>.<object>;
SHOW GRANTS TO ROLE <target_role>;
SHOW GRANTS OF ROLE <target_role>;

SELECT CURRENT_ACCOUNT() AS account,
       CURRENT_REGION()  AS region,
       CURRENT_ROLE()    AS executing_role,
       CURRENT_USER()    AS executing_user;
```

Compute the effective-inheritance impact — who actually gains or loses the privilege.

```sql
WITH RECURSIVE role_edges AS (
  SELECT grantee_name AS child_role, name AS parent_role
    FROM SNOWFLAKE.ACCOUNT_USAGE.GRANTS_TO_ROLES
   WHERE granted_on = 'ROLE'
     AND privilege  = 'USAGE'
     AND deleted_on IS NULL
),
closure AS (
  SELECT '<TARGET_ROLE>'::string AS role_name, 0 AS hops
  UNION ALL
  SELECT e.child_role, c.hops + 1
    FROM closure c
    JOIN role_edges e ON e.parent_role = c.role_name
   WHERE c.hops < 10
)
SELECT c.role_name,
       MIN(c.hops)        AS hops_from_target,
       COUNT(DISTINCT g.grantee_name) AS users_holding_role
  FROM closure c
  LEFT JOIN SNOWFLAKE.ACCOUNT_USAGE.GRANTS_TO_USERS g
         ON g.role = c.role_name AND g.deleted_on IS NULL
 GROUP BY c.role_name
 ORDER BY hops_from_target;
-- Report the hop limit alongside the result. Every role listed here gains
-- (or loses) the privilege being changed.
```

Usage evidence for a proposed REVOKE — what breaks, and for whom.

```sql
SELECT ah.user_name,
       ah.role_name,
       COUNT(*)                 AS accesses,
       MIN(ah.query_start_time) AS first_access,
       MAX(ah.query_start_time) AS last_access
  FROM SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY ah,
       LATERAL FLATTEN(input => ah.base_objects_accessed) f
 WHERE ah.query_start_time >= DATEADD(day, -90, CURRENT_TIMESTAMP())
   AND f.value:objectName::string = '<DB>.<SCHEMA>.<OBJECT>'
 GROUP BY 1, 2
 ORDER BY last_access DESC;
-- Ninety days to catch monthly workloads. State the window and the view's
-- latency; an empty result inside the latency window is UNKNOWN, not 'unused'.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/security-access-control-considerations — That a role may grant privileges only on objects it owns — the basis for this guard's ownership-scoped least-privilege run-as design
- https://docs.snowflake.com/en/sql-reference/sql/grant-privilege — The exact GRANT grammar and the access-control requirements for issuing it
