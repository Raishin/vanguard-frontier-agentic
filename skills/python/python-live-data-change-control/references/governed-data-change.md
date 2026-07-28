# Governed Data Change

Ownership, classification, reconciliation, and data-minimization requirements for a live production data change.

- A production data change requires owner sign-off, a recorded data classification, a bounded scope, a reconciliation plan, and a working rollback before execution.
- Technical completion of a migration/backfill/reprocessing job is not proof of data correctness; reconciliation evidence (counts, checksums) is required separately.
- Personal-data handling follows GDPR data-minimization and residency principles: process only what is necessary, and do not move regulated data into a third-party tool without review.
- Reconciliation of monetary/financial amounts should use Python's `decimal` module for exact arithmetic rather than binary floating point, which is not exact for base-10 fractions.

## Sources

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://gdpr-info.eu/
- https://docs.python.org/3/library/decimal.html
