# Edition, Cloud, and Region Constraints

The capability boundaries that make a design valid in one account and invalid in another. Load before asserting that any capability is available.

- Capability in Snowflake is a function of edition, cloud, and region simultaneously. A feature can be generally available in the product, unavailable in a region, and unavailable at the account's edition, all at once.
- Treat every capability assertion as `UNKNOWN` until account evidence confirms it. The failure mode is not a missing feature — it is a design signed off on the assumption that the feature is there.
- An edition upgrade is a recurring cost with a permanent floor. Justify it by naming the specific capability the design requires, and price the alternative design that does not require it.

## Time-sensitive claims

Each row is volatile: re-verify against the cited primary source before encoding it in a recommendation. A status that has moved silently converts a safe recommendation into an unsafe one.

| Claim | Status / constraint | Verified | What the source proves | What it does NOT prove |
|---|---|---|---|---|
| Database and share replication is available to all accounts; replication of other account objects, failover/failback, and Client Redirect require Business Critical edition or higher — Enterprise is NOT sufficient, and the edition matrix lists these business-continuity capabilities as Business Critical and VPS only. | Edition-gated — verify the account's actual edition | 2026-08-17 via Context7 `/websites/snowflake_en` | That a multi-region design claiming automatic failover has an edition prerequisite | That this account holds that edition, or that the capability is available in its region Also does not prove the account is not Enterprise: an Enterprise account can replicate but cannot fail over, which is the single most common edition misreading in a DR design. |
| Failover groups require Business Critical or higher; replication groups (read-only, no failover) require Standard or higher. | Edition-gated | 2026-08-17 via Context7 `/websites/snowflake_en` | That 'we replicate' and 'we can fail over' are different capabilities with different prerequisites | That any configured group is healthy, current, or has ever been exercised |
| Snowflake replication and failover features are documented as not available in the People's Republic of China. | Region-excluded | 2026-08-17 via Context7 `/websites/snowflake_en` | That region availability can remove a capability independently of edition | The current status of any other region — re-check the region availability page for the specific target |
| Snowflake Horizon Catalog is the recommended solution for new customers needing Apache Iceberg support and multi-engine interoperability; Snowflake Open Catalog is restricted to existing customers. | Current guidance — supersedes older Open-Catalog-first designs | 2026-08-17 via Context7 `/websites/snowflake_en` | That a new interoperability design should not default to Open Catalog because it was once the preferred path | That Horizon Catalog covers every engine or governance requirement a specific design has — check the current capability page |
| External query engine support for Snowflake-managed Iceberg tables through Horizon Catalog reached general availability, using the open Iceberg REST protocol with Snowflake users, roles, policies, and authentication. | GA as documented — re-verify before relying on it | 2026-08-17 via Context7 `/websites/snowflake_en` (release note dated 2026-02-06) | That an external-engine interoperability design can keep Snowflake's governance model rather than forking it | That every external engine, region, or table configuration is supported — verify the specific engine and region |

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/intro-editions — The feature and edition matrix, including which business-continuity capabilities require Business Critical or VPS
- https://docs.snowflake.com/en/user-guide/replication-intro — That database and share replication is available to all accounts while failover, failback, and Client Redirect require Business Critical or higher
- https://docs.snowflake.com/en/user-guide/intro-regions — Which regions exist on which cloud platform — the input to any residency or latency claim
- https://docs.snowflake.com/en/user-guide/tables-iceberg — The Iceberg table options Snowflake supports and the governance differences between Snowflake-managed and externally managed tables
