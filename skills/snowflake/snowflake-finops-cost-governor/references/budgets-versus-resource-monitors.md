# Budgets Versus Resource Monitors

The single most consequential distinction in Snowflake cost control, and the gap most designs fall into. Load before designing or reviewing any spend control.

## The distinction

- Resource monitors track credit usage for user-managed virtual warehouses and the cloud services layer. When a limit is reached they can notify, suspend standard warehouses, or disable Adaptive Warehouses.
- Resource monitors are documented as NOT tracking spending for serverless features or AI services. That spend is managed with budgets.
- Budgets monitor credit usage for supported objects and serverless features as well as warehouses, and a helper function reports which budgets track a given resource.
- The practical consequence: an account controlled only by resource monitors has an uncontrolled surface whose size nobody has measured. Measure it first — the share of credits that is serverless and AI — and report it as the headline finding.
- A resource monitor's suspend action is a real availability control with real blast radius. It stops warehouses. Treat configuring one as a change requiring the same review as any other production change, not as a safety net that costs nothing.
- Re-verify the coverage boundary before every long-lived recommendation. Which serverless features a budget supports is a moving line, and a control design built on a stale boundary silently stops covering what it was built for.

## Designing the control surface

- Produce a coverage map, not a list of monitors: for each spend surface, the mechanism controlling it, the threshold, the action, and the named owner who is notified.
- A threshold with a notify action and no owner is not a control. Name the human.
- A suspend action on a production warehouse needs the same lockout thinking a network policy needs: what breaks when it fires, at what hour, and who can raise the limit at 3am.
- Where a spend surface has no available control mechanism, say so explicitly rather than leaving a blank row. An unmonitored surface that everyone believes is monitored is worse than one everyone knows is open.

## Time-sensitive claims

Each row is volatile: re-verify against the cited primary source before encoding it in a recommendation. A status that has moved silently converts a safe recommendation into an unsafe one.

| Claim | Status / constraint | Verified | What the source proves | What it does NOT prove |
|---|---|---|---|---|
| Resource monitors track credit usage for user-managed virtual warehouses and the cloud services layer, and do not track spending for serverless features or AI services; budgets are the documented mechanism for those. | Current documented behaviour — re-verify before every long-lived control design | 2026-08-17 via Context7 `/websites/snowflake_en` (resource-monitors, cost-management-overview) | That a resource-monitor-only cost control leaves serverless and AI spend uncontrolled | Which specific serverless features a budget covers in this account today — verify the current supported list |
| Resource monitor actions include notifying, suspending standard warehouses, and disabling Adaptive Warehouses when a credit limit is reached. | Current documented behaviour | 2026-08-17 via Context7 `/websites/snowflake_en` (resource-monitors) | That a resource monitor is an availability control as well as a cost control, with real blast radius | That any configured monitor's thresholds or actions are appropriate for this workload |

## Evidence queries

Measure the uncontrolled surface first — how much of the bill is not warehouse compute.

```sql
SELECT service_type,
       SUM(credits_used)      AS credits,
       ROUND(100 * RATIO_TO_REPORT(SUM(credits_used)) OVER (), 1) AS pct_of_total
  FROM SNOWFLAKE.ACCOUNT_USAGE.METERING_HISTORY
 WHERE start_time >= DATEADD(day, -30, CURRENT_TIMESTAMP())
 GROUP BY service_type
 ORDER BY credits DESC;
-- Every service_type that is not warehouse compute is outside a resource
-- monitor's reach. That percentage is the headline number.
```

Map what is actually controlled.

```sql
SHOW RESOURCE MONITORS;
SHOW BUDGETS IN ACCOUNT;

-- Which budgets, if any, track a specific resource.
SELECT SYSTEM$SHOW_BUDGETS_FOR_RESOURCE('WAREHOUSE', 'MY_WH');
```

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/resource-monitors — That resource monitors are warehouse and cloud-services scoped, their available actions, and that they do not track serverless or AI spend
- https://docs.snowflake.com/en/user-guide/cost-management-overview — The cost management framework and the documented split — budgets manage costs for serverless features and warehouses, resource monitors focus solely on warehouses
- https://docs.snowflake.com/en/user-guide/budgets — What a budget covers and how budget membership is determined for a resource
