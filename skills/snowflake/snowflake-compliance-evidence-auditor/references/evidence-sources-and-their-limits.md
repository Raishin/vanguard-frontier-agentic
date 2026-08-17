# Evidence Sources and Their Limits

What each Snowflake evidence source actually establishes, and the latency and retention that bound it. Load before mapping any control to evidence.

## The two bounds on every claim

- **Latency** — Account Usage views are not real time. Any assertion about the recent past is bounded by the view's documented latency, and an absence observed inside that window is `UNKNOWN` rather than a clean result.
- **Retention** — each view retains history for a documented period. An audit period longer than that retention cannot be evidenced from that view at all, and discovering this during fieldwork is the expensive path.
- Both bounds are documented per view and both change. Read the current Account Usage reference for the specific views in the control map rather than relying on remembered values, and record the values used in the report so the reader can re-check them.
- Where an audit period exceeds retention, the remediation is an evidence-export pipeline established going forward. It cannot be applied retroactively — which is why this check comes first.

## What each source proves

- `ACCESS_HISTORY` — that specific objects and columns were read or written by specific queries and users. It is the strongest evidence the platform produces for a data-access control, and it is also the source most likely to be assumed rather than checked for coverage of the object types in scope.
- `LOGIN_HISTORY` — that an authentication occurred and which factors were used. This is how an MFA control is evidenced; a configuration is not.
- `GRANTS_TO_ROLES` / `GRANTS_TO_USERS` with `created_on` and `deleted_on` — when access existed. An access-review control needs the interval, not the snapshot, because a privilege held for one week inside the period is still a privilege held.
- `POLICY_REFERENCES` — that a policy was attached. Establishing attachment *throughout* a period requires evidence captured over that period; a single current query proves today only.
- `QUERY_HISTORY` — the activity record. Useful for change evidence and for demonstrating that a control did not block legitimate work.
- Trust Center scanner results — that a specific packaged check passed at a specific time, within the scanner's own scope. Security-essentials and CIS-aligned scanner packages exist, and extensions can add scanners. A green scanner is a point-in-time result about a defined check, not a period assertion and not coverage of controls nobody wrote a scanner for.

## Evidence queries

Evidence an access-review control across a period — who held what, and when it was created and removed. The interval is the evidence, not the snapshot.

```sql
SELECT grantee_name    AS role_name,
       privilege,
       granted_on,
       name            AS object_name,
       created_on,
       deleted_on
  FROM SNOWFLAKE.ACCOUNT_USAGE.GRANTS_TO_ROLES
 WHERE created_on <  :period_end
   AND (deleted_on IS NULL OR deleted_on > :period_start)
 ORDER BY created_on;
-- Rows here include privileges that existed only briefly inside the period.
-- A snapshot query filtered on deleted_on IS NULL would miss every one of them.
```

Evidence an MFA control from what actually happened rather than from what is configured.

```sql
SELECT user_name,
       first_authentication_factor,
       second_authentication_factor,
       COUNT(*)              AS events,
       MIN(event_timestamp)  AS first_event,
       MAX(event_timestamp)  AS last_event
  FROM SNOWFLAKE.ACCOUNT_USAGE.LOGIN_HISTORY
 WHERE event_timestamp BETWEEN :period_start AND :period_end
   AND is_success = 'YES'
 GROUP BY 1, 2, 3
 ORDER BY user_name;
-- Rows with a NULL second factor on a human user are the finding.
-- State the view's retention: if it is shorter than the period, this is a gap.
```

Evidence access to a specifically scoped sensitive object across the period.

```sql
SELECT ah.user_name,
       ah.role_name,
       f.value:objectName::string AS object_accessed,
       COUNT(*)                   AS accesses,
       MIN(ah.query_start_time)   AS first_access,
       MAX(ah.query_start_time)   AS last_access
  FROM SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY ah,
       LATERAL FLATTEN(input => ah.base_objects_accessed) f
 WHERE ah.query_start_time BETWEEN :period_start AND :period_end
   AND f.value:objectName::string ILIKE '%CUSTOMER_PII%'
 GROUP BY 1, 2, 3
 ORDER BY last_access DESC;
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/sql-reference/account-usage — The Account Usage schema, including the documented latency and retention of each view — the two bounds on every evidence claim
- https://docs.snowflake.com/en/sql-reference/account-usage/access_history — What ACCESS_HISTORY records, at what granularity, and for which object types
- https://docs.snowflake.com/en/user-guide/trust-center/overview — That Trust Center provides packaged scanners, including security-essentials and CIS-aligned checks — and therefore what a green result does and does not cover
