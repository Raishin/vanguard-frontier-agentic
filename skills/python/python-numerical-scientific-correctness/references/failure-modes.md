# High-Severity Failure Modes

The financial and analytical incidents each finding class maps to, for severity calibration.

- Float-based interest or tax accumulation drifts by cents across millions of rows and breaks reconciliation against the ledger.
- A tz-naive month-end timestamp interpreted in the wrong zone books a transaction in the wrong accounting period.
- A missing value upcasts an ID column to float, and downstream equality joins silently drop or mismatch rows.
- An unseeded model evaluation reports a metric that cannot be reproduced for audit or comparison.
- Catastrophic cancellation in a variance or difference calculation returns a materially wrong risk number.
