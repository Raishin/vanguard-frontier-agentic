# Official sources

Use this reference only when you need source grounding for Dynamics 365 data migration, Data Management Framework behavior, or cutover strategy guidance.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live migration state or data quality posture:

- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-to-go-live
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-go-live-checklist
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-go-live-cutover-strategy
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/data-entities-data-packages
- https://learn.microsoft.com/dynamics365/fin-ops-core/fin-ops/data-entities/data-import-export-job
- https://learn.microsoft.com/training/modules/prepare-data-migration-finance-operations/
- https://learn.microsoft.com/power-platform/architecture/key-concepts/data-migration/cut-over-planning
- https://learn.microsoft.com/dynamics365/guidance/fasttrack/go-live-workshops

## Grounding rule

Official documentation explains Dynamics 365 data management framework behavior, data entity design, and cutover strategy guidance. It does not prove the user's actual migration job results, staging table validation outcomes, reconciliation totals, or rollback plan readiness. Prefer documented migration artifacts (mock migration results, staging error logs, reconciliation reports, cutover runbook sign-offs) over inference.

## Service facts (verified 2026-06-16)

Data Management Framework (DMF) structure:
- The DMF in finance and operations apps (Dynamics 365 Finance, Supply Chain Management, Commerce, Human Resources) provides the primary mechanism for data migration via **data entities**, **data projects**, **data jobs**, and **data packages**.
- **Data entity**: conceptual abstraction over one or more underlying tables representing a business concept (e.g., Customers, Vendors, Chart of Accounts).
- **Staging tables**: intermediate tables created per entity during import. Data is validated in staging before promotion to target tables. Staging table errors must be resolved before the job succeeds.
- **Data package**: a compressed file containing a data project manifest and data files, used for bulk import or export across environments.
- **Job history**: tracks source-to-staging and staging-to-target execution results, error counts, and status.

Migration sequencing:
- Entities must be migrated in dependency order: reference data and configuration data before transactional data (e.g., chart of accounts and legal entity setup before journal entries).
- Common sequence: legal entity configuration → number sequences → reference data (currencies, payment terms, tax codes) → master data (customers, vendors, items, fixed assets) → open transactions (open purchase orders, open invoices, open balances).

Mock migration (dry run) requirements:
- Mock migrations must be performed in a non-production environment using production-representative data volumes.
- Measure CRUD throughput (records per second) to estimate production migration duration. Include a 20–30% buffer for monitoring overhead.
- For large datasets (millions of records), plan full migration plus delta loads: full migration completed weeks before go-live, followed by incremental delta loads to reduce final cutover window.
- Mock migration results must be documented: entity job results, staging error counts, reconciliation totals, and elapsed time per entity.

Cutover planning:
- The cutover plan must include: all task sequences with owners and durations, system freeze timing for source and target, final delta migration steps, data validation steps, go/no-go checkpoint criteria, rollback triggers, and rollback execution steps.
- A go/no-go meeting with stakeholder sign-off is required before cutover begins.
- Rollback must be defined: what triggers rollback, who owns rollback execution, how long rollback takes, and what validation confirms successful rollback.

Go-live checklist data migration items:
- Test data migration several times before cutover (multiple mock migrations).
- Validate strategy and processes; identify and address data corruption or duplication.
- Test and sign off on all scripts and processes planned for cutover migration.
- Log any migration issues and risks with a mitigation plan.
- Business stakeholder sign-off on migration results required before production promotion.
