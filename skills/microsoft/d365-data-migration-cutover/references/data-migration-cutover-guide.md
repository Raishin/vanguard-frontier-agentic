# Data Migration & Cutover Guide

Use this reference for Dynamics 365 data migration failure modes, safe review workflow, Data Management Framework patterns, reconciliation design, cutover runbook requirements, verification targets, and pushback criteria.

## What people get wrong

The lazy story is:

> Run an export from the legacy system, import it into Dynamics 365 using the Data Management workspace, check the record count, and you are done.

Wrong. Record count matching is a necessary but not sufficient reconciliation control. Field-level data quality issues, staging table transformation errors, dependency sequencing failures, and financial balance discrepancies are invisible to record count checks alone. A single failed mock migration that is declared "good enough" is the leading predictor of production cutover chaos.

Common bad assumptions:

- One mock migration is sufficient regardless of error rate or elapsed time.
- Staging table errors that were manually resolved in the test environment do not need to be re-validated after fix.
- Record count reconciliation is sufficient to prove data integrity.
- The cutover window is generous enough to absorb a 2x slowdown from production environment differences.
- Legacy data cleansing can be completed during the cutover window.
- Rollback is simple and can be planned ad hoc if needed.
- Delta loads between the final mock migration and production cutover do not need a separate test run.

## Data migration failure modes

- Legacy data quality issues (duplicates, nulls in required fields, invalid codes, format mismatches) are discovered during production migration after the cutover window has started.
- Staging table errors not resolved before production promotion cause partial entity loads, creating referential integrity violations in target tables.
- Entity dependency sequence not respected: transactional data imported before master data causes failed lookups.
- Migration throughput measured in development or sandbox environment does not reflect production environment capacity, causing the cutover window to be exceeded.
- Delta load strategy not designed: data changed in the legacy system between the final mock migration and production cutover is not captured, leaving gaps.
- Rollback plan not tested: when rollback is needed, the steps are unclear, the owner is unknown, and the rollback itself fails.
- Reconciliation performed only by the implementation team, not validated by business data owners who know the expected values.

## High-risk data domains (examples)

- **Chart of accounts and financial dimensions**: must be migrated and validated before any financial transaction data; errors here cascade across all financial reporting.
- **Customer and vendor master data**: deduplication failures create billing, payment, and compliance issues post-go-live.
- **Open purchase orders and open invoices**: in-flight transactions must be captured completely or business operations halt at go-live.
- **Fixed asset master and net book values**: incorrect opening balances trigger depreciation calculation errors.
- **Inventory on-hand balances**: quantity or cost inaccuracies surface immediately at go-live in procurement and production.
- **Bank account and payment method configuration**: missing or incorrect data blocks payment processing from day one.

## Minimum safe migration review workflow

1. Confirm scope: Dynamics 365 workloads, legal entities, data domains, legacy sources, go-live date, cutover window.
2. Review entity selection and dependency sequence: all required entities identified, sequenced in dependency order.
3. Confirm mock migration evidence: at least one completed mock migration with documented staging error counts, elapsed time per entity, and reconciliation totals.
4. Review data quality plan: legacy data cleansing completed or in progress, known issues logged with mitigations.
5. Review reconciliation design: record counts, field-level sampling plan, financial balance checks, business user validation sign-off.
6. Review cutover runbook: task sequence, owners, durations, system freeze point, delta load steps, go/no-go checkpoint.
7. Review rollback plan: trigger criteria, rollback owner, rollback steps, rollback validation, rollback time estimate.
8. Confirm stakeholder sign-off: business data owner and implementation lead approvals documented.
9. Provide minimum-safe-action recommendation scoped to highest-severity migration gaps.
10. Require live-guard escalation for any production migration authorization.

## Verification targets

- Mock migration results: completed runs, staging error count, elapsed time, entity coverage
- Data quality: cleansing status, known issues log, deduplication results
- Reconciliation: record counts, field samples, financial balances, business user validation
- Cutover runbook: task sequence, owners, durations, go/no-go criteria, rollback section
- Rollback plan: trigger criteria, owner, steps, validation, time estimate
- Delta load strategy: designed, tested, sequenced relative to freeze point
- Stakeholder sign-off: business data owner and implementation lead named and dated

## When to push back

Push back if the user asks to:

- approve production migration without at least one completed mock migration with documented results
- accept record count matching alone as sufficient reconciliation evidence
- proceed to production cutover with unresolved staging table errors
- approve a cutover window that has no throughput buffer based on measured migration performance
- proceed without a tested rollback plan that includes a named rollback owner and trigger criteria
- sign off on reconciliation without business data owner validation
- skip the go/no-go checkpoint before starting production migration
- authorize post-migration legacy data deletion before business users have validated the target environment
