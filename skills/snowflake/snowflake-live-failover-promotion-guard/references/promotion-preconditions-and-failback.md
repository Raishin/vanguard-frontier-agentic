# Promotion Preconditions and Failback

The preconditions that make a promotion a recovery, and why the failback strategy must exist before the first promotion. Load during preflight, and rehearse it in every drill.

## Why promotion is not recovery

- Promotion makes the secondary authoritative for the objects in the failover group. It does nothing about identity, DNS, client configuration, secrets, orchestration, ingestion, external stages, BI tools, shares, or downstream consumers.
- If those are not ready, the estate is now authoritative in a region nothing can reach, write to, or feed — which is an outage in a harder place, not a resolution.
- Confirm readiness item by item, with the confirming owner recorded per item. A matrix completed by the person running the promotion records an intention, not a state.
- Objects and integrations not in the failover group do not become available. That exclusion list is computed before the incident and read from during it.
- Client Redirect helps only the clients that connect through its connection URL. Every hardcoded client is a separate manual action with a separate owner, and that set is usually the larger one.

## The data-loss window

- Compute it from the last successful refresh in replication history: the gap between that timestamp and the moment of promotion is what is lost.
- State it in minutes and describe what those minutes contain in business terms — orders, transactions, events, patient records. A number without a description cannot be acknowledged meaningfully.
- Obtain written business acknowledgement where the window is material. The engineer running the promotion is not the person entitled to accept that loss.
- After promotion, measure what was actually lost — from refresh history and from the ingestion reconciliation — and record the measured figure in the incident record. The estimate was for the decision; the measurement is for the record and for the next drill.

## Failback

- Failback is a second promotion in the reverse direction, with its own declaration, approval, data-loss window, and dependency readiness. It is not an undo and it is not automatic.
- It becomes possible only when the original primary is healthy and re-synchronized. Failing back to an unsynchronized account loses everything written while promoted.
- Every write to the promoted account increases the failback's own data-loss and reconciliation problem. The longer the estate runs promoted, the more expensive returning becomes — which is why the strategy is agreed before the first promotion.
- State the last date the failback path was tested. Failover is rehearsed far more often than failback, and an untested return path means the promotion is effectively one-way.
- Because the return is not free and not automatic, a promotion made without a failback strategy is an architecture change executed under time pressure. Naming it as such before it happens is the point of this precondition.

## Evidence queries

Compute the data-loss window and confirm the group type — the two facts that must not be estimated.

```sql
-- Group type: a replication group cannot be promoted.
SHOW FAILOVER GROUPS;
SHOW REPLICATION GROUPS;

-- The data-loss window, computed rather than estimated.
SELECT replication_group_name,
       MAX(end_time)                                     AS last_successful_refresh,
       DATEDIFF('minute', MAX(end_time), CURRENT_TIMESTAMP()) AS data_loss_window_minutes
  FROM SNOWFLAKE.ACCOUNT_USAGE.REPLICATION_GROUP_REFRESH_HISTORY
 WHERE phase_name = 'COMPLETED'
   AND replication_group_name = '<GROUP>'
 GROUP BY replication_group_name;
-- State what these minutes contain in business terms before requesting
-- acknowledgement. A number alone cannot be meaningfully acknowledged.
```

Confirm what will and will not be available after promotion, and the client redirect state.

```sql
SHOW DATABASES IN FAILOVER GROUP <GROUP>;
-- Compare against the objects the business needs after recovery. The
-- difference is the exclusion list, and it is read during the incident,
-- not computed during it.

SHOW CONNECTIONS;
-- Identifies which connection is primary and whether Client Redirect is in
-- use. Clients not connecting through it require individual owner action.
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/account-replication-failover — The promotion operation, its prerequisites, and the failover workflow
- https://docs.snowflake.com/en/user-guide/replication-intro — That failover, failback, and Client Redirect require Business Critical edition or higher, and the documented regional exclusions
- https://docs.snowflake.com/en/user-guide/client-redirect — That only clients connecting through the redirect connection follow a promotion automatically
