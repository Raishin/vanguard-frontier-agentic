# Replication, Failover, and Edition Constraints

What Snowflake's replication features actually provide, and which of them the account's edition and region permit. Load before asserting any recovery capability.

## Two different groups

- A **replication group** provides read-only replication and does not support promotion. A **failover group** supports promotion to primary. Teams frequently have the first and plan around the second, and the difference only surfaces at promotion time.
- Snowsight defaults reflect this split: Business Critical accounts and higher create a failover group by default with an option to create a replication group instead, while Standard and Enterprise accounts create a replication group by default.
- Group membership decides coverage. Objects and integrations outside the group are not replicated, and the gap is invisible until recovery. Enumerate what is excluded as a named list, not as a caveat.
- Refresh history is the empirical RPO. The gap between the last successful refresh and now is the data that would be lost by promoting at this moment — compute it, do not estimate it.
- Some object types, integrations, and features have specific replication behaviour or do not replicate at all. Check the replication considerations for each object class the recovery depends on rather than assuming uniform coverage.

## Edition and region gate the capability

- Database and share replication is available to all accounts. Replication of other account objects, failover and failback, and Client Redirect require Business Critical edition or higher.
- Failover groups require Business Critical or higher; replication groups require Standard or higher. A DR plan written for one and deployed on the other fails at exactly the wrong moment.
- These features are documented as not available in the People's Republic of China. Region availability can remove a capability independently of edition, so check both.
- Confirm edition and region from the account, never from the plan. This is the most common unverified assumption in a DR review and the one with the largest consequence.

## Client Redirect is not automatic client recovery

- Client Redirect provides a connection object that can be repointed, so clients using it follow the promotion. It requires Business Critical or higher.
- It only helps clients that actually connect through that connection URL. Every client, driver, BI tool, connector, and script with a hardcoded account URL is outside its reach — and those are the majority in most estates.
- Enumerate the clients: which use the redirect URL, which use the direct account URL, and which resolve through a customer-managed DNS name that must be changed separately.
- The clients nobody lists are the ones that fail. The inventory is the deliverable, not the redirect configuration.

## Time-sensitive claims

Each row is volatile: re-verify against the cited primary source before encoding it in a recommendation. A status that has moved silently converts a safe recommendation into an unsafe one.

| Claim | Status / constraint | Verified | What the source proves | What it does NOT prove |
|---|---|---|---|---|
| Database and share replication is available to all accounts; replication of other account objects, failover/failback, and Client Redirect require Business Critical edition or higher. | Edition-gated — confirm the account's edition | 2026-08-17 via Context7 `/websites/snowflake_en` (replication-intro) | That 'we replicate' and 'we can fail over' have different prerequisites | This account's edition, or that any configured group has ever been exercised |
| Failover groups require Business Critical edition or higher; replication groups require Standard edition or higher and provide read-only replication without failover support. | Edition-gated | 2026-08-17 via Context7 `/websites/snowflake_en` (dynamic-tables/replication) | That the group type in place determines whether promotion is even possible | Which type this account has — read it from SHOW output |
| Snowflake replication and failover features are documented as not available in the People's Republic of China. | Region-excluded | 2026-08-17 via Context7 `/websites/snowflake_en` (replication-intro) | That region availability can remove the capability independently of edition | Availability in any other specific region — check the target region directly |

## Evidence queries

Establish what actually exists, and compute the empirical RPO from refresh history.

```sql
SHOW REPLICATION GROUPS;
SHOW FAILOVER GROUPS;

-- Empirical RPO: the gap since the last successful refresh is the data that
-- would be lost by promoting right now.
SELECT replication_group_name,
       phase_name,
       start_time,
       end_time,
       DATEDIFF('minute', end_time, CURRENT_TIMESTAMP()) AS minutes_since_refresh,
       error_message
  FROM SNOWFLAKE.ACCOUNT_USAGE.REPLICATION_GROUP_REFRESH_HISTORY
 WHERE start_time >= DATEADD(day, -7, CURRENT_TIMESTAMP())
 ORDER BY start_time DESC;
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/replication-intro — That database and share replication is available to all accounts while failover, failback, and Client Redirect require Business Critical or higher, and the documented regional exclusion
- https://docs.snowflake.com/en/user-guide/account-replication-config — How replication and failover groups are created and the edition-dependent defaults in Snowsight
- https://docs.snowflake.com/en/user-guide/client-redirect — What Client Redirect provides and that clients must connect through its connection URL to benefit
