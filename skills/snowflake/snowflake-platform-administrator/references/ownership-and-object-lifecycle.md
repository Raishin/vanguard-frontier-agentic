# Ownership and Object Lifecycle

How to build an ownership map from account evidence and what the common ownership pathologies cost. Load when objects appear unowned, orphaned, or unsafe to change.

## Why ownership is an operational property, not a security detail

- The owning role decides who may grant on the object, who may alter or drop it, and — in practice — who is paged when it breaks. An object whose owner nobody can name has no operational owner either.
- OWNERSHIP concentrated in a system role means every change to that object requires that system role, which is precisely the escalation the role model was supposed to prevent.
- OWNERSHIP transferred without a recorded reason is a silent change of who can destroy the object. Transfers deserve the same record as a grant.
- An object owned by a role granted to a departed principal is not automatically safe: the role persists, and whoever inherits it inherits the object.
- Decommissioning is a change with a blast radius: check inbound grants, Time Travel and Fail-safe windows, replication membership, shares and listings, and downstream tasks and pipes before recommending a drop.

## Unused, idle, and suspended are three different findings

- **Unused** — no queries in the observation window. Owner question: should this exist? Route the cost consequence to FinOps.
- **Idle-heavy** — frequent resumes for trivial work, or a long auto-suspend on a bursty workload. Owner question: is auto-suspend tuned for the actual arrival pattern?
- **Suspended by policy** — working as designed. Reporting it as a finding erodes trust in the whole report.
- State the observation window explicitly. 'Unused' over seven days and over ninety days are different claims, and month-end and quarter-end workloads exist.

## Evidence queries

Build the ownership map and find objects owned by system roles.

```sql
SELECT grantee_name AS owning_role,
       granted_on,
       COUNT(*) AS objects_owned
  FROM SNOWFLAKE.ACCOUNT_USAGE.GRANTS_TO_ROLES
 WHERE privilege = 'OWNERSHIP'
   AND deleted_on IS NULL
 GROUP BY owning_role, granted_on
 ORDER BY objects_owned DESC;

-- The finding is the rows where owning_role is ACCOUNTADMIN, SYSADMIN,
-- SECURITYADMIN or PUBLIC: every change to those objects needs a system role.
```

Find warehouses with no observed usage, and warehouses whose configuration will never let them suspend.

```sql
-- Usage over an explicitly stated window.
SELECT w."name"                                   AS warehouse,
       w."auto_suspend"                           AS auto_suspend_seconds,
       w."owner"                                  AS owning_role,
       COALESCE(SUM(m.credits_used), 0)           AS credits_30d
  FROM TABLE(RESULT_SCAN(LAST_QUERY_ID())) w
  LEFT JOIN SNOWFLAKE.ACCOUNT_USAGE.WAREHOUSE_METERING_HISTORY m
         ON m.warehouse_name = w."name"
        AND m.start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
 GROUP BY 1, 2, 3
 ORDER BY credits_30d ASC;
-- Run SHOW WAREHOUSES immediately before this statement.
-- auto_suspend NULL or 0 means the warehouse does not auto-suspend.
```
