# Job-Control Review Checklist

The per-concern checklist applied to every distributed job / business-automation operation.

- Technical idempotency: the job can be safely re-run without a technical side effect (duplicate write, error).
- Business idempotency: re-running the job produces no duplicate business effect (double charge, duplicate email, duplicate shipment).
- Retry bound: a retry is bounded, idempotency-guarded, and dead-lettered, never a blind retry-all-failed-jobs request.
- Owner approval: a bounded retry or job operation has owner approval before execution.
- Separation: process completion (the job ran) and business completion (the business effect is correct) are confirmed separately.
- Reconciliation: the actual business outcome is reconciled after the operation, not assumed from a success status.
