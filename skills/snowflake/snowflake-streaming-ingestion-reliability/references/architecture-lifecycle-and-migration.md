# Architecture, Lifecycle, and Migration

Which Snowpipe Streaming architecture a design should use, what current guidance says, and how a migration is scoped. Load before recommending any streaming architecture.

## How to hold a lifecycle claim

- Lifecycle status is volatile and load-bearing: it decides whether a new build is starting on a supported path or booking a migration. Carry it with a verification date or do not carry it.
- 'Planned for deprecation', 'announced with a date', and 'retired' are three different states. Snowflake's published material puts the classic architecture in the first: deprecation is stated, and a formal announcement carrying the final end-of-life date is documented as expected, with an 18-month sunset period beginning after it.
- Where a final date is needed for planning and none is published, the correct output is not a bare `unresolved`. State three things: no final end-of-life date is published; the documented expectation is a formal announcement followed by an 18-month sunset; and therefore the migration window is bounded by that clock rather than open-ended. Then give the action — re-check the deprecation page and the release notes, because the announcement is the event that starts it.
- For existing workloads: current guidance is that they remain supported and no immediate change is required, while customers are encouraged to assess pipelines and prioritize upgrading. That is a planning signal, not an emergency — say so, rather than manufacturing urgency.
- Check whether the announcement has actually landed before repeating the expected timeline. The documented target was mid-2026; if the page still reads in the future tense the announcement is due or overdue, and that is itself the finding a planner needs — not a reason to report nothing.

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
| Snowflake is deprecating the Snowpipe Streaming classic architecture in favour of the high-performance architecture; all future innovation is built on the high-performance architecture and the classic architecture will eventually be retired. | Deprecation stated; final end-of-life date not yet published — formal announcement documented as expected, with an 18-month sunset following it | 2026-08-17 via Context7 `/websites/snowflake_en` (snowpipe-streaming-classic-deprecation) | That a new implementation should target the high-performance architecture | When the classic architecture stops working — no final date is published, so it must not be invented; but the 18-month sunset window and the pending announcement ARE documented and must not be omitted either |
| Snowflake documents an expected timeline: a formal deprecation announcement — including the full transition timeline, milestones, migration guides, and the final end-of-life date — planned for mid-2026, followed by an 18-month sunset period for migrating workloads. | Expected timeline as documented — confirm whether the announcement has landed | 2026-08-17 via Context7 `/websites/snowflake_en` (snowpipe-streaming-classic-deprecation, 'Expected timeline and migration window') | That the migration window is bounded by an 18-month clock starting at announcement, so a programme can be scoped now rather than deferred for lack of a date | That the announcement has been issued, or what the final end-of-life date is — the documented mid-2026 target had arrived by this verification date while the page still read in the future tense, so re-check before planning on it |
| Existing classic-architecture workloads remain fully supported with no immediate change required, while users are encouraged to review migration resources and prioritize upgrading. | Current documented position | 2026-08-17 via Context7 `/websites/snowflake_en` (data-load-snowpipe-streaming-overview) | That an existing pipeline is a planned migration, not an emergency | That any specific pipeline is healthy, or that its migration is simple |
| The Snowflake Connector for Kafka v4 natively supports the high-performance architecture and is generally available; a documented v3-to-v4 migration path exists with compatibility flags and best-effort classic offset migration. | GA as documented — confirm the version available to this deployment | 2026-08-17 via Context7 `/websites/snowflake_en` (kafka-connector/migrate-v3-to-v4) | That Kafka Connect users have a supported migration path with explicit compatibility controls | That offset migration will succeed for a specific deployment — 'best effort' is the documented characterization and must be reconciled |

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/snowpipe-streaming/snowpipe-streaming-classic-deprecation — That the classic architecture is being deprecated, that future innovation targets the high-performance architecture, and the guidance to assess and prioritize migration
- https://docs.snowflake.com/en/user-guide/snowpipe-streaming/data-load-snowpipe-streaming-overview — That existing classic workloads remain supported with no immediate change required, and where migration resources live
- https://docs.snowflake.com/en/user-guide/kafka-connector/migrate-v3-to-v4 — The connector class change, the compatibility flags, and the best-effort classic offset migration setting
