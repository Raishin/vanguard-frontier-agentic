# Rollback-and-Recovery Review Checklist

The per-concern checklist applied to every rollback execution request.

- Pre-approval: only a previously approved and tested rollback procedure is executed, never an improvised one.
- Target binding: the rollback is bound to the exact affected-target fingerprint.
- Authority: rollback authority is confirmed before execution.
- Preconditions: a captured before-state or snapshot reference exists before executing.
- Reconciliation: the post-rollback state is captured and reconciled against the expected result.
- Independent verification: the rollback's success is verified independently, never self-attested by the executor.
