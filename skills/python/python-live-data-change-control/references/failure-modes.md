# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A backfill run against production just to test corrupts live customer records with no rollback path.
- A bounded one-partition approval quietly expanded to the rest of the table turns a reviewed change into an unreviewed one.
- A migration that completes without error but was never reconciled silently drops or duplicates rows.
- Reconciling monetary totals with binary floating point instead of exact decimal arithmetic hides a real mismatch behind rounding error.
- Copying a customer PII export into a third-party analytics tool without a data-flow review creates an unreviewed regulated-data exposure.
