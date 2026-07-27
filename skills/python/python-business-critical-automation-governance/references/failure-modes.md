# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A departed employee's personal script that closes the books every month has no named owner, and nobody can explain what it does when it breaks at quarter-end.
- The same on-call engineer who requests a production access grant also approves and executes it, with no second approver anywhere in the path.
- A payment-reconciliation job with no idempotency check double-posts a batch of refunds after a manual rerun.
- A notebook that computes month-end accruals runs cells out of order depending on who last edited it, producing a different number at every close.
- A critical automation with no retained run log cannot show auditors what inputs produced a disputed financial output.
