# Account and Workload Topologies

The topology patterns that recur in enterprise Snowflake estates, what each one actually isolates, and where each one breaks. Load when the number or arrangement of accounts is in question.

## Patterns and their real isolation

- **Single account, environment-separated by database and role** — cheapest to operate, lowest isolation. A privileged mistake, a runaway cost event, or an account-level parameter change reaches production. Acceptable where the blast radius of the account is genuinely acceptable; state that explicitly rather than by omission.
- **Account per environment (dev/test/prod)** — isolates account-level parameters, account-level policies, and privileged blast radius. Costs: promotion tooling, replication or IaC to keep shape consistent, and duplicated integrations. This is the common enterprise default and its cost is the promotion pipeline.
- **Account per business domain or region** — buys residency, contractual, and administrative separation. Costs: cross-account sharing design, federated identity, per-account monitoring, and a real risk that governance policies diverge silently. Requires an organization-level governance answer before it is chosen, not after.
- **Account per tenant** — only defensible where the tenant boundary is contractual or regulatory. Otherwise row-access policies and separate roles achieve the isolation at a fraction of the operational cost.
- An **organization** groups accounts for administration and cross-account features. It is not itself a security boundary and does not unify governance policy across its accounts.

## Workload placement

- Place workloads by contention shape and cost attributability, not by team org chart. Two teams with identical query profiles can share a warehouse; one team with a nightly batch and an interactive dashboard should not.
- Ingestion, transformation, BI serving, ad-hoc exploration, and ML training have different concurrency, duration, and spill profiles. Sharing one warehouse across them makes every sizing decision a compromise and makes cost unattributable.
- Multi-cluster warehouses solve concurrency queuing, not slow individual queries. Reaching for multi-cluster to fix a single slow query buys credits and no latency.
- Serverless features bill independently of any warehouse. A topology that reasons only about warehouses has an unaccounted spend surface — hand that to FinOps explicitly.

## Evidence queries

Check whether workloads are actually isolated as the diagram claims — warehouse by distinct user, role, and query-type mix over the last 30 days.

```sql
SELECT warehouse_name,
       COUNT(DISTINCT user_name)            AS distinct_users,
       COUNT(DISTINCT role_name)            AS distinct_roles,
       COUNT(DISTINCT query_type)           AS distinct_query_types,
       COUNT(*)                             AS queries,
       MEDIAN(total_elapsed_time)/1000      AS median_seconds,
       MAX(total_elapsed_time)/1000         AS max_seconds
  FROM SNOWFLAKE.ACCOUNT_USAGE.QUERY_HISTORY
 WHERE start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
   AND warehouse_name IS NOT NULL
 GROUP BY warehouse_name
 ORDER BY queries DESC;
```

Establish the deployment facts a design must not assume — the accounts that exist, with their edition, cloud, and region.

```sql
SHOW ORGANIZATION ACCOUNTS;
-- Then, in the account being designed for:
SELECT CURRENT_ACCOUNT()   AS account,
       CURRENT_REGION()    AS region,
       CURRENT_VERSION()   AS version;
```
