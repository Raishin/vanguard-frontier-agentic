# Architecture, Lifecycle, and Migration

Which Snowpipe Streaming architecture a design should use, what current guidance says, and how a migration is scoped. Load before recommending any streaming architecture.

## How to hold a lifecycle claim

- Lifecycle status is volatile and load-bearing: it decides whether a new build is starting on a supported path or booking a migration. Carry it with a verification date or do not carry it.
- 'Planned for deprecation' and 'retired on date X' are different claims. Snowflake's published material describes the classic architecture as planned for deprecation and directs new work to the high-performance architecture; it does not, in the material reviewed here, publish a retirement date.
- Where a date is needed for planning and none is documented, the correct output is `Status: unresolved — verify against the latest Snowflake release and deprecation notes`, plus a plan that does not depend on the date. Inventing one produces a real budget for a fictional deadline.
- For existing workloads: current guidance is that they remain supported and no immediate change is required, while customers are encouraged to assess pipelines and prioritize upgrading. That is a planning signal, not an emergency — say so, rather than manufacturing urgency.

## Scoping a classic-to-high-performance migration

- The migration is not only an SDK swap. Establish: which client or connector produces, what the channel naming will be on the new side, how offsets carry across, what the target table schema becomes, and how the cutover is reconciled.
- Offset migration is the risk centre. A migration that cannot recover the prior offsets either re-ingests (duplication) or skips (loss). Establish the offset-migration behaviour and its guarantees before scheduling a cutover.
- For Kafka Connect users, the connector's newer major version natively supports the high-performance architecture and is generally available, with a documented v3-to-v4 migration path including compatibility flags for connector class, converters, table-name sanitization, column-identifier normalization, schematization, and best-effort classic offset migration. Read the migration guide for the exact flags rather than reconstructing them.
- Snowflake states an intention to upgrade supported connectors to the high-performance architecture before the deprecation. That reduces the migration burden for managed connectors but does not remove it for custom SDK clients — scope those separately.
- Plan the cutover as a reconciled change: run the reconciliation before, during, and after, and define the rollback (return to the previous path) with its own offset consequence stated.

## Time-sensitive claims

Each row is volatile: re-verify against the cited primary source before encoding it in a recommendation. A status that has moved silently converts a safe recommendation into an unsafe one.

| Claim | Status / constraint | Verified | What the source proves | What it does NOT prove |
|---|---|---|---|---|
| Snowflake is deprecating the Snowpipe Streaming classic architecture in favour of the high-performance architecture; all future innovation is built on the high-performance architecture and the classic architecture will eventually be retired. | Deprecation announced; no retirement date established in the reviewed material | 2026-08-17 via Context7 `/websites/snowflake_en` (snowpipe-streaming-classic-deprecation) | That a new implementation should target the high-performance architecture | When the classic architecture stops working — that date is unresolved and must not be invented |
| Existing classic-architecture workloads remain fully supported with no immediate change required, while users are encouraged to review migration resources and prioritize upgrading. | Current documented position | 2026-08-17 via Context7 `/websites/snowflake_en` (data-load-snowpipe-streaming-overview) | That an existing pipeline is a planned migration, not an emergency | That any specific pipeline is healthy, or that its migration is simple |
| The Snowflake Connector for Kafka v4 natively supports the high-performance architecture and is generally available; a documented v3-to-v4 migration path exists with compatibility flags and best-effort classic offset migration. | GA as documented — confirm the version available to this deployment | 2026-08-17 via Context7 `/websites/snowflake_en` (kafka-connector/migrate-v3-to-v4) | That Kafka Connect users have a supported migration path with explicit compatibility controls | That offset migration will succeed for a specific deployment — 'best effort' is the documented characterization and must be reconciled |

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/snowpipe-streaming/snowpipe-streaming-classic-deprecation — That the classic architecture is being deprecated, that future innovation targets the high-performance architecture, and the guidance to assess and prioritize migration
- https://docs.snowflake.com/en/user-guide/snowpipe-streaming/data-load-snowpipe-streaming-overview — That existing classic workloads remain supported with no immediate change required, and where migration resources live
- https://docs.snowflake.com/en/user-guide/kafka-connector/migrate-v3-to-v4 — The connector class change, the compatibility flags, and the best-effort classic offset migration setting
