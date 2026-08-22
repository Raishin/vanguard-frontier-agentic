# Effective Access Computation

How to compute what a principal can actually do, and the four queries that together answer it. Load for any 'who can access this' question.

## Four questions, four commands

- `SHOW GRANTS TO ROLE <role>` — what this role holds directly, including roles it has been granted.
- `SHOW GRANTS OF ROLE <role>` — who holds this role. This is the direction most reviews forget, and it is the one that reveals who inherits.
- `SHOW GRANTS TO USER <user>` — the roles granted to a principal directly. It is the starting set, not the answer.
- `SHOW GRANTS ON <object>` — everything granted on the object, including OWNERSHIP. This is where the surprise usually is.
- Effective access is the transitive closure over role-to-role edges, plus OWNERSHIP edges, plus database roles reachable through those roles, plus future grants that will apply to objects created later. Any analysis that stops at direct grants is answering a different question.

## Edges that are routinely missed

- **OWNERSHIP is an escalation edge.** The owning role can grant privileges on the object to anyone. A role that owns an object effectively holds every privilege on it plus the ability to distribute them.
- **Database roles** are scoped inside a database but are reached through account roles. A closure computed only over account roles under-reports.
- **Future grants** authorize access to objects that do not exist yet. A future grant at database scope means every table created there from now on carries the privilege — which is precisely why it is popular and why it is dangerous.
- **PUBLIC** is inherited by every user in the account, present and future, human and service. A grant to PUBLIC is a grant to the whole account.
- **Role hierarchy convenience edges** — a role granted to another role once, for one project, is permanent until someone finds it. These are the majority of unexpected paths in a mature account.

## Evidence queries

Find every role that can reach a specific sensitive object, directly or by inheritance, from the grant graph.

```sql
-- Direct grants on the object, including OWNERSHIP.
SHOW GRANTS ON TABLE my_db.my_schema.customer_pii;

-- Transitive closure: which roles inherit the roles found above.
WITH RECURSIVE role_edges AS (
  SELECT grantee_name AS child_role, name AS parent_role
    FROM SNOWFLAKE.ACCOUNT_USAGE.GRANTS_TO_ROLES
   WHERE granted_on = 'ROLE'
     AND privilege  = 'USAGE'
     AND deleted_on IS NULL
),
seed AS (
  SELECT DISTINCT grantee_name AS role_name
    FROM SNOWFLAKE.ACCOUNT_USAGE.GRANTS_TO_ROLES
   WHERE name = 'CUSTOMER_PII'
     AND table_schema = 'MY_SCHEMA'
     AND deleted_on IS NULL
),
closure AS (
  SELECT role_name, 0 AS hops FROM seed
  UNION ALL
  SELECT e.child_role, c.hops + 1
    FROM closure c
    JOIN role_edges e ON e.parent_role = c.role_name
   WHERE c.hops < 10
)
SELECT role_name, MIN(hops) AS shortest_path_hops
  FROM closure
 GROUP BY role_name
 ORDER BY shortest_path_hops;
-- State the hop limit in the finding. A closure truncated at 10 that is
-- reported as complete is a wrong answer.
```

Inventory future grants — standing authorization over objects that do not exist yet.

```sql
SHOW FUTURE GRANTS IN DATABASE my_db;
SHOW FUTURE GRANTS IN SCHEMA my_db.my_schema;

-- A future grant at DATABASE scope authorizes every object created in every
-- schema from now on. Report its scope explicitly; it is not a convenience.
```

Establish which privileges were actually exercised, so a proposed revocation has a known blast radius.

```sql
SELECT ah.user_name,
       f.value:objectName::string AS object_accessed,
       COUNT(*)                   AS accesses,
       MAX(ah.query_start_time)   AS last_access
  FROM SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY ah,
       LATERAL FLATTEN(input => ah.base_objects_accessed) f
 WHERE ah.query_start_time >= DATEADD(day, -90, CURRENT_TIMESTAMP())
 GROUP BY 1, 2
 ORDER BY last_access DESC;
-- Absence here inside the ACCESS_HISTORY latency window is UNKNOWN,
-- not proof that the privilege is unused.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/security-access-control-overview — The role types, ownership semantics, and inheritance model that make the transitive closure necessary
- https://docs.snowflake.com/en/user-guide/security-access-control-considerations — Snowflake's own least-privilege guidance, including the treatment of system roles and the separation of grant management from object creation
