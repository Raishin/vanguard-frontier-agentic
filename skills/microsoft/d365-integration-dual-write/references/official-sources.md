# Official sources

Use this reference only when you need source grounding for Dynamics 365 dual-write integration behavior, table map operations, initial sync, error handling, or Power Platform integration boundary design.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live integration state, table map health, or error posture:

- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/dual-write-overview
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/enable-entity-map
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/errors-and-alerts
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/dual-write-troubleshooting-live-sync
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/dual-write-home-page
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/initial-sync-guidance
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/dual-write-troubleshooting-initial-sync
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/dual-write-health-check
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/dual-write-integration-keys

## Grounding rule

Official documentation explains Dynamics 365 dual-write infrastructure behavior, table map lifecycle states, initial sync mechanics, and error handling capabilities. It does not prove the user's actual table map health, error queue size, master-data ownership decisions, or rollback readiness. Prefer documented artifacts (dual-write health check results, table map status exports, sync error dashboard screenshots, alert configuration records, master-data ownership decision log) over inference.

## Service facts (verified 2026-06-17)

Dual-write overview:
- Dual-write is an out-of-box infrastructure providing near-real-time, bidirectional integration between Finance & Operations apps and Dataverse (customer engagement apps). Data changes on either side are automatically written to the other side.
- Dual-write has two aspects: **infrastructure** (synchronous/bidirectional data flow, play/pause/catchup modes, initial sync, alert and error log UI) and **application** (pre-built table maps for customers, vendors, products, chart of accounts, and other master data).

Table map lifecycle:
- States: Not running → Initializing (initial write) → Running → Paused → Resuming.
- When a table map is enabled, it runs an **initial write** phase that copies pre-existing data from both sides before going to Running status.
- When paused, new changes are **queued**. For compliance reasons, queued data is retained for **24 hours only**. Maps must be resumed within 24 hours or queued changes are lost.
- Enabling a table map requires enabling all **dependent tables** first; the system presents a dependency list before enabling.
- **Master for initial sync**: when records with the same keys exist on both sides with conflicting values, a master must be declared per entity map to resolve conflicts. Dataverse is the default master.

Integration keys:
- Dual-write uses **integration keys** (alternate keys) to uniquely identify and link records across Finance & Operations and Dataverse. All integration key fields must be mapped in the table map. Lookup fields used as integration key components must also be mapped bidirectionally.

Error handling and monitoring:
- Errors are visible in the dual-write activity and error logs. Admins can configure **alert settings** per error type (e.g., Application error) with thresholds (e.g., 10 errors within 15 minutes) to trigger email notifications and automatic pause or stop actions.
- The dual-write async error dashboard (preview) provides centralized retry and dismiss functionality for sync errors, with a limit of 5 queued requests and 10,000 records per request.
- A dual-write transaction must succeed on both sides within a **two-minute window**; otherwise it fails on both sides.

Health check:
- The dual-write health check validates system requirements and configuration prerequisites. Run it as part of setup and as a troubleshooting step.

Certification anchor:
- MB-500 (Finance and Operations Apps Developer) and MB-700 (Microsoft Dynamics 365: Finance and Operations Apps Solution Architect) — verify current exam status and objectives on Microsoft Learn before citing. (E4: verify before citing.)
